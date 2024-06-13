use crate::{
    builder::Builder,
    bytecode::Bytecode,
    environ::Environment,
    iv_str,
    module_definition::{FunctionDef, ModuleDef},
    opcodes::Opcode,
    runloop::{run_loop, RunloopError},
    runtime_module::{NativeCallable, RuntimeModule},
    rv_int,
    types::{array::ArrayType, record::RecordType, RuntimeType},
    values::{array::Array, RuntimeValue},
};

use crate::instruction_def::InstructionDef;

#[test]
fn test_stack() {
    use crate::stack::Stack;

    let mut foo: Stack<i32> = Default::default();
    assert!(foo.is_empty());

    foo.push(123);
    assert!(!foo.is_empty());
    assert_eq!(123, *foo.peek());

    foo.push(321);
    assert!(!foo.is_empty());
    assert_eq!(321, *foo.peek());

    assert_eq!(321, foo.pop());

    assert!(!foo.is_empty());
    assert_eq!(123, *foo.peek());
    assert_eq!(123, foo.pop());

    assert!(foo.is_empty());

    foo.push(1);
    foo.push(2);
    foo.push(3);

    assert!(!foo.is_empty());
    assert_eq!(3, foo.len());
    assert_eq!(3, *foo.peek());
    assert_eq!(3, *foo.peek_at(0));
    assert_eq!(2, *foo.peek_at(1));
    assert_eq!(1, *foo.peek_at(2));

    assert_eq!(3, foo.pop());
    assert!(!foo.is_empty());
    assert_eq!(2, foo.len());
    assert_eq!(2, *foo.peek_at(0));
    assert_eq!(1, *foo.peek_at(1));
}

#[test]
fn test_bytecode() {
    let mut bc = Bytecode::default();
    assert!(bc.is_empty());
    assert_eq!(0, bc.len());

    bc.write_u8(12);
    assert!(!bc.is_empty());
    assert_eq!(1, bc.len());
    assert_eq!(12, bc.read_u8(0));

    bc.write_u8(33);
    assert!(!bc.is_empty());
    assert_eq!(2, bc.len());
    assert_eq!(12, bc.read_u8(0));
    assert_eq!(33, bc.read_u8(1));

    bc.write_u16(4142);
    assert!(!bc.is_empty());
    assert_eq!(4, bc.len());
    assert_eq!(12, bc.read_u8(0));
    assert_eq!(33, bc.read_u8(1));
    assert_eq!(4142, bc.read_u16(2));

    bc.write_u8(5).write_u8(6).write_u16(7);
    assert_eq!(8, bc.len());
    assert_eq!(5, bc.read_u8(4));
    assert_eq!(6, bc.read_u8(5));
    assert_eq!(7, bc.read_u16(6));

    bc.write_u32(12345678);
    assert_eq!(12, bc.len());
    assert_eq!(12345678, bc.read_u32(8));
}

#[test]
fn test_runloop() {
    use crate::bytecode::Bytecode;
    use crate::environ::Environment;
    use crate::intern_value::InternValue;
    use crate::opcodes::Opcode;

    let mut bc = Bytecode::default();

    bc.write_u8(u8::from(Opcode::PUSH));
    bc.write_u16(0);
    bc.write_u8(u8::from(Opcode::DUP));
    bc.write_u8(u8::from(Opcode::PUSH));
    bc.write_u16(1);
    bc.write_u8(u8::from(Opcode::ADD));
    bc.write_u8(u8::from(Opcode::RET));

    let mut env = Environment::default();

    let mut module = RuntimeModule::new("module");
    let function = FunctionDef::new("main", bc);
    let function = module.add_function_fdef(&function);
    module.add_intern_value(&InternValue::Integer(5));
    module.add_intern_value(&InternValue::Integer(6));
    env.add_module(module);

    assert!(crate::runloop::run_loop(&function, &mut env).is_ok());

    assert!(!env.is_stack_empty());
    assert_eq!(RuntimeValue::Integer(11), env.pop_value());
    assert!(!env.is_stack_empty());
    assert_eq!(RuntimeValue::Integer(5), env.pop_value());
    assert!(env.is_stack_empty());
}

#[test]
fn test_function_lookup() {
    use crate::bytecode::Bytecode;
    use crate::environ::Environment;
    use crate::opcodes::Opcode;

    let mut bc = Bytecode::default();
    bc.write_u8(u8::from(Opcode::RET));

    let mut m1 = RuntimeModule::new("com.sys.module1");
    let mut m2 = RuntimeModule::new("com.sys.module2");

    let func1 = FunctionDef::new("func1", bc.clone());
    let func2 = FunctionDef::new("func2", bc.clone());
    let funcfoo = FunctionDef::new("foo", bc.clone());
    let funcbar = FunctionDef::new("bar", bc.clone());

    m1.add_function_fdef(&funcfoo);
    m1.add_function_fdef(&func2);
    m2.add_function_fdef(&func1);
    m2.add_function_fdef(&funcbar);

    let mut env = Environment::default();
    env.add_module(m1);
    env.add_module(m2);

    assert!(env.lookup_function("com.sys.module1.func1").is_none());
    assert!(env.lookup_function("com.sys.module2.none").is_none());

    assert!(env.lookup_function("no").is_none());
    assert!(env.lookup_function("m3.foo").is_none());
    assert!(env.lookup_function("coms.sys.module1.foo.bar").is_none());

    assert!(env
        .lookup_function("com.sys.module2.bar")
        .is_some_and(|f| f.name().eq("bar")));
    assert!(env
        .lookup_function("com.sys.module1.func2")
        .is_some_and(|f| f.name().eq("func2")));

    assert!(env.lookup_named_type("com.sys.module1.func2").is_none());
    assert!(env.lookup_named_type("com.sys.module2.bar").is_none());
}

#[test]
fn test_module_def() {
    let mut md = ModuleDef::new("module");

    let mut bc = Bytecode::default();
    bc.write_u8(u8::from(Opcode::RET));
    let func1 = FunctionDef::new("func1", bc.clone());
    let func2 = FunctionDef::new("func2", bc.clone());
    let funcfoo = FunctionDef::new("foo", bc.clone());
    let funcbar = FunctionDef::new("bar", bc.clone());

    md.add_function(func1);
    md.add_function(func2);
    md.add_function(funcfoo);
    md.add_function(funcbar);

    let rd = RuntimeModule::from(&md);

    let mut env = Environment::default();
    env.add_module(rd);

    assert!(env
        .lookup_function("module.bar")
        .is_some_and(|f| f.name().eq("bar")));
    assert!(env
        .lookup_function("module.func2")
        .is_some_and(|f| f.name().eq("func2")));
    assert!(env
        .lookup_function("module.func1")
        .is_some_and(|f| f.name().eq("func1")));
    assert!(env
        .lookup_function("module.foo")
        .is_some_and(|f| f.name().eq("foo")));
}

#[test]
fn test_fcall() {
    let mut bc1 = Bytecode::default();
    bc1.write_u8(u8::from(Opcode::ADD));
    bc1.write_u8(u8::from(Opcode::RET));

    let mut bc2 = Bytecode::default();
    bc2.write_u8(u8::from(Opcode::PUSH));
    bc2.write_u16(1);
    bc2.write_u8(u8::from(Opcode::PUSH));
    bc2.write_u16(2);
    bc2.write_u8(u8::from(Opcode::PUSH));
    bc2.write_u16(0);
    bc2.write_u8(u8::from(Opcode::FLOOKUP));
    bc2.write_u8(u8::from(Opcode::CALL));
    bc2.write_u8(u8::from(Opcode::RET));

    let main = FunctionDef::new("main", bc2.clone());
    let f = FunctionDef::new("f", bc1.clone());

    let mut md = ModuleDef::new("module");
    md.add_interned_value(crate::intern_value::InternValue::String(
        "module.f".to_owned(),
    ));
    md.add_interned_value(crate::intern_value::InternValue::Integer(5));
    md.add_interned_value(crate::intern_value::InternValue::Integer(7));
    md.add_function(main);
    md.add_function(f);

    let rd = RuntimeModule::from(&md);
    let mut env = Environment::default();
    env.add_module(rd);

    let main = env
        .lookup_function("module.main")
        .expect("main function missing");
    assert!(run_loop(&main, &mut env).is_ok());

    assert!(!env.is_stack_empty());
    assert_eq!(RuntimeValue::Integer(12), env.pop_value());
}

#[test]
fn test_builder() {
    let mut builder = Builder::new("main");
    let mut block = builder.append_block("entry");
    block.append_instruction(InstructionDef::PUSH(0));
    block.append_instruction(InstructionDef::PUSH(1));
    block.append_instruction(InstructionDef::ADD);
    block.append_instruction(InstructionDef::RET);

    let main = builder.generate();
    let mut md = ModuleDef::new("module");
    md.add_interned_value(crate::intern_value::InternValue::Integer(5));
    md.add_interned_value(crate::intern_value::InternValue::Integer(7));
    md.add_function(main);

    let rd = RuntimeModule::from(&md);
    let mut env = Environment::default();
    env.add_module(rd);

    let main = env
        .lookup_function("module.main")
        .expect("main function missing");
    assert!(run_loop(&main, &mut env).is_ok());

    assert!(!env.is_stack_empty());
    assert_eq!(RuntimeValue::Integer(12), env.pop_value());
}

#[test]
fn test_builder_block_lookup() {
    let mut builder = Builder::new("main");
    builder.append_block("main");
    builder.append_block("this");
    builder.append_block("that");

    assert!(builder.find_block("main").is_some());
    assert!(builder.find_block("this").is_some());
    assert!(builder.find_block("notthat").is_none());

    assert!(builder
        .find_block("main")
        .is_some_and(|f| f.name() == "main"));
    assert!(builder
        .find_block("that")
        .is_some_and(|f| f.name() == "that"));
}

#[test]
fn test_builder_jump() {
    let mut builder = Builder::new("main");
    let mut entry = builder.append_block("entry");
    let mut exit = builder.append_block("exit");
    let mut next = builder.append_block("next");

    let mut md = ModuleDef::new("module");
    let five = md.add_interned_value(crate::intern_value::InternValue::Integer(5));
    let seven = md.add_interned_value(crate::intern_value::InternValue::Integer(7));
    let three = md.add_interned_value(crate::intern_value::InternValue::Integer(3));

    exit.append_instruction(InstructionDef::RET);

    entry.append_instruction(InstructionDef::PUSH(five as u16));
    entry.append_instruction(InstructionDef::PUSH(seven as u16));
    entry.append_instruction(InstructionDef::ADD);
    entry.append_instruction(InstructionDef::JUMP(next.clone()));
    entry.append_instruction(InstructionDef::JUMP(entry.clone()));

    next.append_instruction(InstructionDef::PUSH(three as u16));
    next.append_instruction(InstructionDef::ADD);
    next.append_instruction(InstructionDef::JUMP(exit.clone()));

    let main = builder.generate();
    md.add_function(main);

    let rd = RuntimeModule::from(&md);
    let mut env = Environment::default();
    env.add_module(rd);

    let main = env
        .lookup_function("module.main")
        .expect("main function missing");
    assert!(run_loop(&main, &mut env).is_ok());

    assert!(!env.is_stack_empty());
    assert_eq!(RuntimeValue::Integer(15), env.pop_value());
}

#[test]
fn test_builder_jtrue() {
    let mut builder = Builder::new("main");
    let mut entry = builder.append_block("entry");
    let mut exit = builder.append_block("exit");
    let mut next = builder.append_block("next");

    let mut md = ModuleDef::new("module");
    let five = md.add_interned_value(crate::intern_value::InternValue::Integer(5));
    let seven = md.add_interned_value(crate::intern_value::InternValue::Integer(7));
    let three = md.add_interned_value(crate::intern_value::InternValue::Integer(3));

    exit.append_instruction(InstructionDef::RET);

    entry.append_instruction(InstructionDef::PUSH(five as u16));
    entry.append_instruction(InstructionDef::PUSH(seven as u16));
    entry.append_instruction(InstructionDef::EQUAL);
    entry.append_instruction(InstructionDef::JTRUE(entry.clone()));
    entry.append_instruction(InstructionDef::PUSH(five as u16));
    entry.append_instruction(InstructionDef::PUSH(five as u16));
    entry.append_instruction(InstructionDef::EQUAL);
    entry.append_instruction(InstructionDef::JTRUE(next.clone()));
    entry.append_instruction(InstructionDef::JUMP(entry.clone()));

    next.append_instruction(InstructionDef::PUSH(three as u16));
    next.append_instruction(InstructionDef::PUSH(five as u16));
    next.append_instruction(InstructionDef::ADD);
    next.append_instruction(InstructionDef::JUMP(exit.clone()));

    let main = builder.generate();
    md.add_function(main);

    let rd = RuntimeModule::from(&md);
    let mut env = Environment::default();
    env.add_module(rd);

    let main = env
        .lookup_function("module.main")
        .expect("main function missing");
    assert!(run_loop(&main, &mut env).is_ok());

    assert!(!env.is_stack_empty());
    assert_eq!(RuntimeValue::Integer(8), env.pop_value());
}

#[test]
fn test_intern_index() {
    let mut md = ModuleDef::new("module");
    assert_eq!(
        0,
        md.add_interned_value(crate::intern_value::InternValue::Integer(5))
    );
    assert_eq!(
        1,
        md.add_interned_value(crate::intern_value::InternValue::Integer(7))
    );
}

#[test]
fn test_terminated_block() {
    let mut builder = Builder::new("main");
    let mut x = builder.append_block("x");
    let mut y: crate::builder::BasicBlock = builder.append_block("y");

    assert!(!x.is_terminated());
    x.append_instruction(InstructionDef::ADD);
    assert!(!x.is_terminated());
    x.append_instruction(InstructionDef::RET);
    assert!(x.is_terminated());
    x.append_instruction(InstructionDef::JUMP(y.clone()));
    assert!(x.is_terminated());

    assert!(!y.is_terminated());
    assert!(!builder.is_terminated());

    y.append_instruction(InstructionDef::RET);
    assert!(y.is_terminated());
    assert!(builder.is_terminated());
}

#[test]
fn test_typeof() {
    let mut builder = Builder::new("main");
    let mut block = builder.append_block("entry");
    block.append_instruction(InstructionDef::PUSH(0));
    block.append_instruction(InstructionDef::TYPEOF);
    block.append_instruction(InstructionDef::RET);

    let main = builder.generate();
    let mut md = ModuleDef::new("module");
    md.add_interned_value(crate::intern_value::InternValue::Integer(7));
    md.add_function(main);

    let rd = RuntimeModule::from(&md);
    let mut env = Environment::default();
    env.add_module(rd);

    let main = env
        .lookup_function("module.main")
        .expect("main function missing");
    assert!(run_loop(&main, &mut env).is_ok());

    assert!(!env.is_stack_empty());
    assert_eq!(RuntimeValue::Type(RuntimeType::Integer), env.pop_value());
}

#[test]
fn test_type_equality() {
    let t1 = RuntimeType::Integer;
    let t2 = RuntimeType::Type(Box::new(RuntimeType::String));

    assert_eq!(RuntimeType::Integer, t1);
    assert_ne!(RuntimeType::String, t1);
    assert_ne!(t2, t1);
    assert_ne!(RuntimeType::String, t2);
    assert_eq!(RuntimeType::Type(Box::new(RuntimeType::String)), t2);
    assert_ne!(RuntimeType::Type(Box::new(RuntimeType::Integer)), t2);
}

#[test]
fn test_runloop_slots() {
    let mut builder = Builder::new("main");
    let mut block = builder.append_block("entry");
    block.append_instruction(InstructionDef::PUSH(1));
    block.append_instruction(InstructionDef::TOSLOT(0));
    block.append_instruction(InstructionDef::PUSH(1));
    block.append_instruction(InstructionDef::TOSLOT(1));
    block.append_instruction(InstructionDef::FROMSLOT(0));
    block.append_instruction(InstructionDef::FROMSLOT(1));
    block.append_instruction(InstructionDef::ADD);
    block.append_instruction(InstructionDef::RET);

    let main = builder.generate();
    let mut md = ModuleDef::new("module");
    md.add_interned_value(crate::intern_value::InternValue::Integer(5));
    md.add_interned_value(crate::intern_value::InternValue::Integer(7));
    md.add_function(main);

    let rd = RuntimeModule::from(&md);
    let mut env = Environment::default();
    env.add_module(rd);

    let main = env
        .lookup_function("module.main")
        .expect("main function missing");
    assert!(run_loop(&main, &mut env).is_ok());

    assert!(!env.is_stack_empty());
    assert_eq!(RuntimeValue::Integer(14), env.pop_value());
}

#[test]
fn test_array_typed() {
    let mut a = Array::new_typed(
        RuntimeType::Logical,
        &vec![
            RuntimeValue::Logical(false),
            RuntimeValue::Logical(true),
            RuntimeValue::Logical(false),
            RuntimeValue::Logical(true),
        ],
    );

    let eat = RuntimeType::Arr(Box::new(ArrayType::new(RuntimeType::Logical, 4)));

    assert_eq!(4, a.len());
    assert_eq!(RuntimeValue::Logical(false), a.get(0));
    assert_eq!(RuntimeValue::Logical(true), a.get(1));
    assert_eq!(RuntimeValue::Logical(false), a.get(2));
    assert_eq!(RuntimeValue::Logical(true), a.get(3));

    a.set(0, &RuntimeValue::Logical(true));
    a.set(1, &RuntimeValue::Logical(false));

    assert_eq!(4, a.len());
    assert_eq!(RuntimeValue::Logical(true), a.get(0));
    assert_eq!(RuntimeValue::Logical(false), a.get(1));
    assert_eq!(RuntimeValue::Logical(false), a.get(2));
    assert_eq!(RuntimeValue::Logical(true), a.get(3));

    assert_eq!(eat, a.get_type());

    let rv = RuntimeValue::Arr(a);
    assert_eq!(eat, rv.get_type());
}

#[test]
fn test_array_inferred() {
    let mut a = Array::new_inferred(&vec![
        RuntimeValue::Logical(false),
        RuntimeValue::Logical(true),
        RuntimeValue::Logical(false),
        RuntimeValue::Logical(true),
    ]);

    let eat = RuntimeType::Arr(Box::new(ArrayType::new(RuntimeType::Logical, 4)));

    assert_eq!(4, a.len());
    assert_eq!(RuntimeValue::Logical(false), a.get(0));
    assert_eq!(RuntimeValue::Logical(true), a.get(1));
    assert_eq!(RuntimeValue::Logical(false), a.get(2));
    assert_eq!(RuntimeValue::Logical(true), a.get(3));

    a.set(0, &RuntimeValue::Logical(true));
    a.set(1, &RuntimeValue::Logical(false));

    assert_eq!(4, a.len());
    assert_eq!(RuntimeValue::Logical(true), a.get(0));
    assert_eq!(RuntimeValue::Logical(false), a.get(1));
    assert_eq!(RuntimeValue::Logical(false), a.get(2));
    assert_eq!(RuntimeValue::Logical(true), a.get(3));

    assert_eq!(eat, a.get_type());

    let rv = RuntimeValue::Arr(a);
    assert_eq!(eat, rv.get_type());
}

#[test]
fn test_newarr_instruction() {
    let mut md = ModuleDef::new("module");
    md.add_interned_value(crate::intern_value::InternValue::Integer(5));
    md.add_interned_value(crate::intern_value::InternValue::Integer(1));
    md.add_interned_value(crate::intern_value::InternValue::Integer(2));
    md.add_interned_value(crate::intern_value::InternValue::Integer(3));
    md.add_interned_value(crate::intern_value::InternValue::Integer(4));
    md.add_interned_value(crate::intern_value::InternValue::Integer(6));

    let mut builder = Builder::new("main");
    let mut block = builder.append_block("entry");
    block.append_instruction(InstructionDef::PUSH(1));
    block.append_instruction(InstructionDef::PUSH(2));
    block.append_instruction(InstructionDef::PUSH(3));
    block.append_instruction(InstructionDef::PUSH(4));
    block.append_instruction(InstructionDef::PUSH(5));
    block.append_instruction(InstructionDef::PUSH(0));
    block.append_instruction(InstructionDef::TYPEOF);
    block.append_instruction(InstructionDef::PUSH(0));
    block.append_instruction(InstructionDef::MKARRTYPE);
    block.append_instruction(InstructionDef::NEWARR);
    block.append_instruction(InstructionDef::RET);

    let main = builder.generate();
    md.add_function(main);

    let rd = RuntimeModule::from(&md);
    let mut env = Environment::default();
    env.add_module(rd);

    let main = env
        .lookup_function("module.main")
        .expect("main function missing");
    assert!(run_loop(&main, &mut env).is_ok());

    assert!(!env.is_stack_empty());
    if let RuntimeValue::Arr(arr) = env.pop_value() {
        assert_eq!(5, arr.len());
        assert_eq!(RuntimeValue::Integer(1), arr.get(0));
        assert_eq!(RuntimeValue::Integer(2), arr.get(1));
        assert_eq!(RuntimeValue::Integer(3), arr.get(2));
        assert_eq!(RuntimeValue::Integer(4), arr.get(3));
        assert_eq!(RuntimeValue::Integer(6), arr.get(4));
    } else {
        assert!(false, "not an array on stack");
    }
}

#[test]
fn test_native_function() {
    struct NativeReturn42 {}
    impl NativeCallable for NativeReturn42 {
        fn call(&self, env: &mut Environment) -> Result<(), RunloopError> {
            env.push_value(rv_int!(42));
            Ok(())
        }

        fn name(&self) -> String {
            "fortytwo".to_owned()
        }
    }

    let mut native_module = RuntimeModule::new("tukun");
    let call42: Box<dyn NativeCallable> = Box::new(NativeReturn42 {});
    native_module.add_function_native(call42);

    let mut env = Environment::default();
    env.add_module(native_module);

    let mut builder = Builder::new("main");
    let mut block = builder.append_block("entry");
    block.append_instruction(InstructionDef::PUSH(0));
    block.append_instruction(InstructionDef::FLOOKUP);
    block.append_instruction(InstructionDef::CALL);
    block.append_instruction(InstructionDef::RET);

    let main = builder.generate();
    let mut md = ModuleDef::new("module");
    md.add_interned_value(iv_str!("tukun.fortytwo"));

    md.add_function(main);

    let rd = RuntimeModule::from(&md);
    env.add_module(rd);

    let main = env
        .lookup_function("module.main")
        .expect("main function missing");
    assert!(run_loop(&main, &mut env).is_ok());

    assert!(!env.is_stack_empty());
    assert_eq!(rv_int!(42), env.pop_value());
}

#[test]
fn test_record_def() {
    let rdef = RecordType::new(&[RuntimeType::Integer, RuntimeType::Logical]);
    let tdef = rdef.to_typedef("test");
    let mut mdef = ModuleDef::new("module");
    mdef.add_named_type(&tdef);

    let rm = RuntimeModule::from(&mdef);
    let rmrdef = rm.find_named_type("test");
    assert!(rmrdef.is_some());
    let rmrdef = rmrdef.unwrap();
    assert_eq!(rmrdef.name(), "test");
    let rmtdef = rmrdef.target();
    if let RuntimeType::Record(rt) = rmtdef {
        assert_eq!(2, rt.len());
        assert_eq!(&RuntimeType::Integer, rt.get(0));
        assert_eq!(&RuntimeType::Logical, rt.get(1));
    } else {
        panic!("not a record");
    }
}

#[test]
fn test_record_lookup() {
    let mut env = Environment::default();
    let mut rm1 = RuntimeModule::new("com.module1");
    let mut rm2 = RuntimeModule::new("com.module2");

    let rd1 = RecordType::new(&[RuntimeType::Integer, RuntimeType::Logical]).to_typedef("test");
    rm1.add_named_type(&rd1);
    env.add_module(rm1);

    let rd2 = RecordType::new(&[
        RuntimeType::String,
        RuntimeType::Logical,
        RuntimeType::Integer,
    ])
    .to_typedef("again");
    rm2.add_named_type(&rd2);
    env.add_module(rm2);

    let test_rdef = env
        .lookup_named_type("com.module1.test")
        .expect("test type exists");
    let test_rdef = test_rdef.target();
    if let RuntimeType::Record(test_rdef) = test_rdef {
        assert_eq!(2, test_rdef.len());
        assert_eq!(&RuntimeType::Integer, test_rdef.get(0));
        assert_eq!(&RuntimeType::Logical, test_rdef.get(1));
    } else {
        panic!("not a record");
    }

    let again_rdef = env
        .lookup_named_type("com.module2.again")
        .expect("again type exists");
    let again_rdef = again_rdef.target();
    if let RuntimeType::Record(again_rdef) = again_rdef {
        assert_eq!(3, again_rdef.len());
        assert_eq!(&RuntimeType::String, again_rdef.get(0));
        assert_eq!(&RuntimeType::Logical, again_rdef.get(1));
        assert_eq!(&RuntimeType::Integer, again_rdef.get(2));
    } else {
        panic!("not a record");
    }

    assert!(env.lookup_named_type("com.module1.no").is_none());
    assert!(env.lookup_named_type("com.module2.test").is_none());

    assert!(env.lookup_function("com.module1.test").is_none());
    assert!(env.lookup_function("com.module2.again").is_none());
}
