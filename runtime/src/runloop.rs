use crate::{
    bytecode::Bytecode,
    environ::Environment,
    instruction_runtime::RuntimeInstruction,
    log::{LogSubsystem, StderrWriter},
    log_debug, log_subsystem,
    runtime_module::{RuntimeCallable, RuntimeModule},
    types::{array::ArrayType, record::RecordType, RuntimeType},
    values::{array::Array, RuntimeValue},
};

static LOG_RUNLOOP: LogSubsystem = log_subsystem!("runloop", crate::log::LogLevel::Error);
static mut LOG_WRITER: StderrWriter = StderrWriter {};

macro_rules! stack_pop {
    ($env:expr, $inst:expr) => {{
        if let Some(val) = $env.runtime_stack.try_pop() {
            val
        } else {
            panic!("empty stack unexpected for instruction {:?}", $inst)
        }
    }};
}

macro_rules! typed_pop {
    ($env:expr, $inst:expr, $t:path) => {{
        if let Some(val) = $env.runtime_stack.try_pop() {
            if let $t(payload) = val {
                payload
            } else {
                panic!("invalid value {} for instruction {:?}", val, $inst)
            }
        } else {
            panic!("empty stack unexpected for instruction {:?}", $inst)
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

fn bytecode_run_loop<'a>(ctx: &'a BytecodeContext<'a>, env: &mut Environment) {
    let mut slots: Vec<RuntimeValue> = vec![];

    let mut cur_ptr: usize = 0;
    loop {
        if cur_ptr >= ctx.body().len() {
            panic!("invalid opcode index {cur_ptr}");
        }
        let (inst, i) = RuntimeInstruction::from_bytecode(ctx.body(), cur_ptr);
        cur_ptr = i;

        log_debug!(LOG_RUNLOOP, "running opcode {inst:?}");

        match inst {
            RuntimeInstruction::NOP => {}
            RuntimeInstruction::POP => {
                stack_pop!(env, inst);
            }
            RuntimeInstruction::DUP => {
                let val = stack_pop!(env, inst);
                env.runtime_stack.push(val.clone());
                env.runtime_stack.push(val);
            }
            RuntimeInstruction::PUSH(idx) => {
                #[allow(clippy::expect_fun_call)]
                let iv = RuntimeValue::from(
                    ctx.module()
                        .get_intern_value(idx)
                        .expect(&format!("invalid intern value {idx}"))
                        .as_ref(),
                );
                env.runtime_stack.push(iv);
            }
            RuntimeInstruction::ADD => {
                let x = stack_pop!(env, inst);
                let y = stack_pop!(env, inst);
                match (&x, &y) {
                    (RuntimeValue::Integer(x), RuntimeValue::Integer(y)) => {
                        env.runtime_stack.push(RuntimeValue::Integer(x + y))
                    }
                    (_, _) => {
                        panic!("invalid operands for {inst:?}: {},{}", x, y)
                    }
                }
            }
            RuntimeInstruction::EQUAL => {
                let x = stack_pop!(env, inst);
                let y = stack_pop!(env, inst);
                env.runtime_stack.push(RuntimeValue::Logical(x == y))
            }
            RuntimeInstruction::JUMP(dst) => {
                cur_ptr = dst as usize;
            }
            RuntimeInstruction::JTRUE(dst) => {
                let b = typed_pop!(env, inst, RuntimeValue::Logical);
                if b {
                    cur_ptr = dst as usize;
                }
            }
            RuntimeInstruction::NOT => {
                let b = typed_pop!(env, inst, RuntimeValue::Logical);
                env.runtime_stack.push(crate::rv_bool!(!b));
            }
            RuntimeInstruction::RET => return,
            RuntimeInstruction::FLOOKUP => {
                let n = typed_pop!(env, inst, RuntimeValue::String);
                if let Some(f) = env.lookup_function(&n) {
                    env.runtime_stack.push(RuntimeValue::Function(f));
                } else {
                    panic!("lookup failed of function {n}");
                }
            }
            RuntimeInstruction::TLOOKUP => {
                let n = typed_pop!(env, inst, RuntimeValue::String);
                if let Some(t) = env.lookup_named_type(&n) {
                    env.runtime_stack
                        .push(RuntimeValue::Type(t.target().clone()));
                } else {
                    panic!("lookup failed of type {n}");
                }
            }
            RuntimeInstruction::TYPEOF => {
                let x = stack_pop!(env, inst);
                env.runtime_stack.push(RuntimeValue::Type(x.get_type()))
            }
            RuntimeInstruction::TOSLOT(slot) => {
                let slot = slot as usize;
                let x = stack_pop!(env, inst);
                match slot.cmp(&slots.len()) {
                    std::cmp::Ordering::Less => {
                        slots[slot] = x;
                    }
                    std::cmp::Ordering::Equal => {
                        slots.push(x);
                    }
                    std::cmp::Ordering::Greater => {
                        panic!("slot access {slot} is out of bounds");
                    }
                }
            }
            RuntimeInstruction::FROMSLOT(slot) => {
                let x = slots[slot as usize].clone();
                env.runtime_stack.push(x);
            }
            RuntimeInstruction::CALL => {
                let f = typed_pop!(env, inst, RuntimeValue::Function);
                run_loop(&f, env);
            }
            RuntimeInstruction::NEWARR => {
                let at = typed_pop!(env, inst, RuntimeValue::Type);
                if let RuntimeType::Arr(at) = at {
                    let et = at.value_type;
                    let len = at.len;
                    let mut values = Vec::<RuntimeValue>::with_capacity(len);
                    for _ in 0..len {
                        let val = stack_pop!(env, inst);
                        assert!(val.get_type() == et);
                        values.insert(0, val);
                    }
                    let arr = Array::new_inferred(&values);
                    env.runtime_stack.push(RuntimeValue::Arr(arr));
                } else {
                    panic!("invalid type: expected array");
                }
            }
            RuntimeInstruction::ARRGET => {
                let idx = typed_pop!(env, inst, RuntimeValue::Integer);
                let arr = typed_pop!(env, inst, RuntimeValue::Arr);
                let val = arr.get(idx as usize);
                env.runtime_stack.push(val);
            }
            RuntimeInstruction::ARRSET => {
                let val = env.runtime_stack.pop();
                let idx = typed_pop!(env, inst, RuntimeValue::Integer);
                let mut arr = typed_pop!(env, inst, RuntimeValue::Arr);
                arr.set(idx as usize, &val);
                env.runtime_stack.push(RuntimeValue::Arr(arr));
            }
            RuntimeInstruction::ARRLEN => {
                let arr = typed_pop!(env, inst, RuntimeValue::Arr);
                let len = RuntimeValue::Integer(arr.len() as u64);
                env.runtime_stack.push(len);
            }
            RuntimeInstruction::MKARRTYPE => {
                let len = typed_pop!(env, inst, RuntimeValue::Integer);
                let vt = typed_pop!(env, inst, RuntimeValue::Type);
                let at = ArrayType::new(vt, len as usize);
                let at: RuntimeValue = RuntimeValue::Type(RuntimeType::Arr(Box::new(at)));
                env.runtime_stack.push(at);
            }
            RuntimeInstruction::MKRECTYPE => {
                let mut len = typed_pop!(env, inst, RuntimeValue::Integer);
                let mut vt: Vec<RuntimeType> = Vec::with_capacity(len as usize);
                while len > 0 {
                    let val = stack_pop!(env, inst);
                    match val {
                        RuntimeValue::Type(rt) => vt.insert(0, rt),
                        _ => panic!("expected type, found {val}"),
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

pub fn run_loop(callable: &RuntimeCallable, env: &mut Environment) {
    match &callable.f.content {
        either::Either::Left(f) => {
            let ctx = BytecodeContext {
                m: &callable.module(),
                b: f.body(),
            };

            bytecode_run_loop(&ctx, env);
        }
        either::Either::Right(f) => {
            f.call(env);
        }
    }
}
