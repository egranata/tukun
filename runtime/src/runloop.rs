use crate::{
    bytecode::Bytecode,
    environ::Environment,
    instruction_runtime::RuntimeInstruction,
    log::{LogSubsystem, StderrWriter},
    log_debug, log_subsystem,
    runtime_module::{RuntimeCallable, RuntimeModule},
    types::{array::ArrayType, record::RecordType, RuntimeType},
    values::{array::Array, record::Record, RuntimeValue},
};

static LOG_RUNLOOP: LogSubsystem = log_subsystem!("runloop", crate::log::LogLevel::Error);
static mut LOG_WRITER: StderrWriter = StderrWriter {};

macro_rules! err_ret {
    ($ptr:expr, $payload:expr) => {
        return Err(RunloopError {
            cur_ptr: $ptr,
            data: $payload,
        });
    };
}

macro_rules! stack_pop {
    ($ptr:expr, $env:expr, $inst:expr) => {{
        if let Some(val) = $env.runtime_stack.try_pop() {
            val
        } else {
            err_ret!($ptr, RunloopErrData::EmptyStack);
        }
    }};
}

macro_rules! typed_pop {
    ($ptr:expr, $env:expr, $inst:expr, $t:path) => {{
        if let Some(val) = $env.runtime_stack.try_pop() {
            if let $t(payload) = val {
                payload
            } else {
                err_ret!($ptr, RunloopErrData::InvalidOperands($inst, vec![val]));
            }
        } else {
            err_ret!($ptr, RunloopErrData::EmptyStack);
        }
    }};
}

macro_rules! typed_pop2 {
    ($ptr:expr, $env:expr, $inst:expr, $t1:path, $t2:path) => {{
        if let Some(v1) = $env.runtime_stack.try_pop() {
            if let Some(v2) = $env.runtime_stack.try_pop() {
                if matches!(v1, $t1(_)) && matches!(v2, $t2(_)) {
                    if let ($t1(p1), $t2(p2)) = (v1, v2) {
                        (p1, p2)
                    } else {
                        panic!("unwrapping failed");
                    }
                } else {
                    err_ret!($ptr, RunloopErrData::InvalidOperands($inst, vec![v1, v2]));
                }
            } else {
                err_ret!($ptr, RunloopErrData::EmptyStack);
            }
        } else {
            err_ret!($ptr, RunloopErrData::EmptyStack);
        }
    }};
}

struct BytecodeContext<'a> {
    m: &'a RuntimeModule,
    b: &'a Bytecode,
}

impl<'a> BytecodeContext<'a> {
    fn body(&self) -> &'a Bytecode {
        self.b
    }

    fn module(&self) -> &'a RuntimeModule {
        self.m
    }
}

#[derive(Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub struct InvalidTypeError {
    actual: RuntimeType,
    expected: String,
}

#[derive(Debug, PartialEq, Eq)]
pub enum RunloopErrData {
    EmptyStack,
    InstrutionOutOfBounds,
    InvalidBytecode,
    MissingInternValue(u16),
    InvalidOperands(RuntimeInstruction, Vec<RuntimeValue>),
    MissingFunction(String),
    MissingType(String),
    InvalidSlot(usize),
    InvalidType(InvalidTypeError),
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct RunloopError {
    pub cur_ptr: usize,
    pub data: RunloopErrData,
}

impl PartialEq for RunloopError {
    fn eq(&self, other: &Self) -> bool {
        self.cur_ptr == other.cur_ptr && self.data == other.data
    }
}

impl Eq for RunloopError {}

pub type RunloopResult = Result<(), RunloopError>;

fn bytecode_run_loop<'a>(ctx: &'a BytecodeContext<'a>, env: &mut Environment) -> RunloopResult {
    let mut slots: Vec<RuntimeValue> = vec![];

    let mut ip: usize = 0;
    loop {
        let cur_ptr = ip;
        env.unwinder.set_ip(cur_ptr);

        if cur_ptr >= ctx.body().len() {
            err_ret!(cur_ptr, RunloopErrData::InstrutionOutOfBounds);
        }
        let opcode_maybe = RuntimeInstruction::from_bytecode(ctx.body(), cur_ptr);
        if opcode_maybe.is_none() {
            err_ret!(cur_ptr, RunloopErrData::InvalidBytecode);
        }
        let (inst, new_ip) = opcode_maybe.unwrap();
        ip = new_ip;

        log_debug!(LOG_RUNLOOP, "running opcode {inst:?}");

        match inst {
            RuntimeInstruction::NOP => {}
            RuntimeInstruction::POP => {
                stack_pop!(cur_ptr, env, inst);
            }
            RuntimeInstruction::DUP => {
                let val = stack_pop!(cur_ptr, env, inst);
                env.runtime_stack.push(val.clone());
                env.runtime_stack.push(val);
            }
            RuntimeInstruction::SWAP => {
                let x = stack_pop!(cur_ptr, env, inst);
                let y = stack_pop!(cur_ptr, env, inst);
                env.runtime_stack.push(x);
                env.runtime_stack.push(y);
            }
            RuntimeInstruction::PUSH(idx) => {
                let iv = ctx.module().get_intern_value(idx);
                if iv.is_none() {
                    err_ret!(cur_ptr, RunloopErrData::MissingInternValue(idx));
                }
                let iv = iv.unwrap();
                env.runtime_stack.push(RuntimeValue::from(iv.as_ref()));
            }
            RuntimeInstruction::ADD => {
                let x = stack_pop!(cur_ptr, env, inst);
                let y = stack_pop!(cur_ptr, env, inst);
                match (&x, &y) {
                    (RuntimeValue::Integer(x), RuntimeValue::Integer(y)) => env
                        .runtime_stack
                        .push(RuntimeValue::Integer(x.wrapping_add(*y))),
                    (RuntimeValue::Float(x), RuntimeValue::Float(y)) => {
                        env.runtime_stack.push(RuntimeValue::Float(x + y))
                    }
                    (_, _) => {
                        err_ret!(cur_ptr, RunloopErrData::InvalidOperands(inst, vec![x, y]));
                    }
                }
            }
            RuntimeInstruction::SUB => {
                let x = stack_pop!(cur_ptr, env, inst);
                let y = stack_pop!(cur_ptr, env, inst);
                match (&x, &y) {
                    (RuntimeValue::Integer(x), RuntimeValue::Integer(y)) => {
                        env.runtime_stack
                            .push(RuntimeValue::Integer(x.wrapping_sub(*y)));
                    }
                    (RuntimeValue::Float(x), RuntimeValue::Float(y)) => {
                        env.runtime_stack.push(RuntimeValue::Float(x - y));
                    }
                    (_, _) => {
                        err_ret!(cur_ptr, RunloopErrData::InvalidOperands(inst, vec![x, y]));
                    }
                }
            }
            RuntimeInstruction::EQUAL => {
                let x = stack_pop!(cur_ptr, env, inst);
                let y = stack_pop!(cur_ptr, env, inst);
                env.runtime_stack.push(RuntimeValue::Logical(x == y))
            }
            RuntimeInstruction::JUMP(dst) => {
                ip = dst as usize;
            }
            RuntimeInstruction::JTRUE(dst) => {
                let b = typed_pop!(cur_ptr, env, inst, RuntimeValue::Logical);
                if b {
                    ip = dst as usize;
                }
            }
            RuntimeInstruction::NOT => {
                let b = typed_pop!(cur_ptr, env, inst, RuntimeValue::Logical);
                env.runtime_stack.push(crate::rv_bool!(!b));
            }
            RuntimeInstruction::AND => {
                let (b1, b2) = typed_pop2!(
                    cur_ptr,
                    env,
                    inst,
                    RuntimeValue::Logical,
                    RuntimeValue::Logical
                );
                env.runtime_stack.push(crate::rv_bool!(b1 && b2));
            }
            RuntimeInstruction::OR => {
                let (b1, b2) = typed_pop2!(
                    cur_ptr,
                    env,
                    inst,
                    RuntimeValue::Logical,
                    RuntimeValue::Logical
                );
                env.runtime_stack.push(crate::rv_bool!(b1 || b2));
            }
            RuntimeInstruction::RET => return Ok(()),
            RuntimeInstruction::FLOOKUP => {
                let n = typed_pop!(cur_ptr, env, inst, RuntimeValue::String);
                if let Some(f) = env.lookup_function(&n) {
                    env.runtime_stack.push(RuntimeValue::Function(f));
                } else {
                    err_ret!(cur_ptr, RunloopErrData::MissingFunction(n));
                }
            }
            RuntimeInstruction::TLOOKUP => {
                let n = typed_pop!(cur_ptr, env, inst, RuntimeValue::String);
                if let Some(t) = env.lookup_named_type(&n) {
                    env.runtime_stack
                        .push(RuntimeValue::Type(t.target().clone()));
                } else {
                    err_ret!(cur_ptr, RunloopErrData::MissingType(n));
                }
            }
            RuntimeInstruction::TYPEOF => {
                let x = stack_pop!(cur_ptr, env, inst);
                env.runtime_stack.push(RuntimeValue::Type(x.get_type()))
            }
            RuntimeInstruction::TOSLOT(slot) => {
                let slot = slot as usize;
                let x = stack_pop!(cur_ptr, env, inst);
                match slot.cmp(&slots.len()) {
                    std::cmp::Ordering::Less => {
                        slots[slot] = x;
                    }
                    std::cmp::Ordering::Equal => {
                        slots.push(x);
                    }
                    std::cmp::Ordering::Greater => {
                        err_ret!(cur_ptr, RunloopErrData::InvalidSlot(slot));
                    }
                }
            }
            RuntimeInstruction::FROMSLOT(slot) => {
                let x = slots[slot as usize].clone();
                env.runtime_stack.push(x);
            }
            RuntimeInstruction::CALL => {
                let f = typed_pop!(cur_ptr, env, inst, RuntimeValue::Function);
                let result = run_loop(&f, env);
                result?;
            }
            RuntimeInstruction::NEWARR => {
                let at = typed_pop!(cur_ptr, env, inst, RuntimeValue::Type);
                if let RuntimeType::Arr(at) = at {
                    let et = at.value_type;
                    let len = at.len;
                    let mut values = Vec::<RuntimeValue>::with_capacity(len);
                    for _ in 0..len {
                        let val = stack_pop!(cur_ptr, env, inst);
                        assert!(val.get_type() == et);
                        values.insert(0, val);
                    }
                    let arr = Array::new_inferred(&values);
                    env.runtime_stack.push(RuntimeValue::Arr(arr));
                } else {
                    err_ret!(
                        cur_ptr,
                        RunloopErrData::InvalidType(InvalidTypeError {
                            actual: at,
                            expected: "array".to_owned()
                        })
                    );
                }
            }
            RuntimeInstruction::NEWREC => {
                let rt = typed_pop!(cur_ptr, env, inst, RuntimeValue::Type);
                if let RuntimeType::Record(rt) = rt {
                    let len = rt.len();
                    let mut values = Vec::<RuntimeValue>::with_capacity(len);
                    for _ in 0..len {
                        let val = stack_pop!(cur_ptr, env, inst);
                        values.insert(0, val);
                    }
                    let rc = Record::new_typed(*rt, &values);
                    env.runtime_stack.push(RuntimeValue::Record(rc));
                } else {
                    err_ret!(
                        cur_ptr,
                        RunloopErrData::InvalidType(InvalidTypeError {
                            actual: rt,
                            expected: "record".to_owned()
                        })
                    );
                }
            }
            RuntimeInstruction::ARRGET => {
                let idx = typed_pop!(cur_ptr, env, inst, RuntimeValue::Integer);
                let arr = typed_pop!(cur_ptr, env, inst, RuntimeValue::Arr);
                let val = arr.get(idx as usize);
                env.runtime_stack.push(val);
            }
            RuntimeInstruction::ARRSET => {
                let val = env.runtime_stack.pop();
                let idx = typed_pop!(cur_ptr, env, inst, RuntimeValue::Integer);
                let mut arr = typed_pop!(cur_ptr, env, inst, RuntimeValue::Arr);
                arr.set(idx as usize, &val);
                env.runtime_stack.push(RuntimeValue::Arr(arr));
            }
            RuntimeInstruction::RECGET => {
                let idx = typed_pop!(cur_ptr, env, inst, RuntimeValue::Integer);
                let rec = typed_pop!(cur_ptr, env, inst, RuntimeValue::Record);
                let val = rec.get(idx as usize);
                env.runtime_stack.push(val);
            }
            RuntimeInstruction::RECSET => {
                let val = env.runtime_stack.pop();
                let idx = typed_pop!(cur_ptr, env, inst, RuntimeValue::Integer);
                let mut rec = typed_pop!(cur_ptr, env, inst, RuntimeValue::Record);
                rec.set(idx as usize, &val);
                env.runtime_stack.push(RuntimeValue::Record(rec));
            }
            RuntimeInstruction::ARRLEN => {
                let arr = typed_pop!(cur_ptr, env, inst, RuntimeValue::Arr);
                let len = RuntimeValue::Integer(arr.len() as u64);
                env.runtime_stack.push(len);
            }
            RuntimeInstruction::MKARRTYPE => {
                let len = typed_pop!(cur_ptr, env, inst, RuntimeValue::Integer);
                let vt = typed_pop!(cur_ptr, env, inst, RuntimeValue::Type);
                let at = ArrayType::new(vt, len as usize);
                let at: RuntimeValue = RuntimeValue::Type(RuntimeType::Arr(Box::new(at)));
                env.runtime_stack.push(at);
            }
            RuntimeInstruction::MKRECTYPE => {
                let mut len = typed_pop!(cur_ptr, env, inst, RuntimeValue::Integer);
                let mut vt: Vec<RuntimeType> = Vec::with_capacity(len as usize);
                while len > 0 {
                    let val = stack_pop!(cur_ptr, env, inst);
                    match val {
                        RuntimeValue::Type(rt) => vt.insert(0, rt),
                        _ => {
                            err_ret!(
                                cur_ptr,
                                RunloopErrData::InvalidType(InvalidTypeError {
                                    actual: val.get_type(),
                                    expected: "type".to_owned()
                                })
                            );
                        }
                    }
                    len -= 1;
                }
                let rt = RecordType::new(&vt);
                let rt = RuntimeValue::Type(RuntimeType::Record(Box::new(rt)));
                env.runtime_stack.push(rt);
            }
        }
    }
}

pub fn run_loop(callable: &RuntimeCallable, env: &mut Environment) -> RunloopResult {
    env.unwinder.push_frame(callable);

    let result = match &callable.f.content {
        either::Either::Left(f) => {
            let ctx = BytecodeContext {
                m: &callable.module(),
                b: f.body(),
            };

            bytecode_run_loop(&ctx, env)
        }
        either::Either::Right(f) => f.call(env),
    };

    match result {
        Ok(_) => {
            env.unwinder.pop_frame();
            result
        }
        Err(err) => Err(err),
    }
}
