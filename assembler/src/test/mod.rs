use runtime::{
    environ::Environment,
    module_definition::ModuleDef,
    runloop,
    runtime_module::RuntimeModule,
    rv_arr, rv_bool, rv_int, rv_str,
    types::{array::ArrayType, record::RecordType, RuntimeType},
    values::{record::Record, RuntimeValue},
};

use crate::assembler::do_assemble;

#[allow(dead_code)]
fn run_source<'a: 'static>(input: &'a str, stack: &[RuntimeValue]) -> Environment {
    let mut env = Environment::default();
    corelib::register_corelib(&mut env);

    let mdef = do_assemble(input).expect("invalid input");
    let mdef: ModuleDef = bincode::deserialize(&mdef).expect("invalid bytecode");
    let rm = RuntimeModule::from(&mdef);
    env.add_module(rm);

    let main = env.lookup_function("com.tukunc.testmodule.main");
    runloop::run_loop(&main.expect("missing main function"), &mut env);

    let mut i = 0;

    assert!(env.stack_len() >= stack.len());

    while i < stack.len() {
        let actual_value = env.pop_value();
        let expected_value = &stack[i];
        assert_eq!(actual_value, *expected_value);
        i += 1;
    }

    env
}

#[test]
fn test_define_module() {
    let input = r#"
@modname "com.tukunc.testmodule"
fn main
  :entry
    ret
"#;
    run_source(input, &[]);
}

#[test]
fn test_call_function_with_name() {
    let input = r#"
@modname "com.tukunc.testmodule"
%const "com.tukunc.testmodule.callee" = "com.tukunc.testmodule.callee"
%const "five" = 5
fn callee
  :entry
     push 1
     ret
fn main
  :entry
    push "com.tukunc.testmodule.callee"
    flookup
    call
    ret
"#;
    run_source(input, &[RuntimeValue::Integer(5)]);
}

#[test]
fn test_fcall() {
    let input = r#"
@modname "com.tukunc.testmodule"
%const "com.tukunc.testmodule.callee" = "com.tukunc.testmodule.callee"
%const "five" = 5
fn callee
  :entry
     push 1
     ret
fn main
  :entry
    fcall "com.tukunc.testmodule.callee"
    ret
"#;
    run_source(input, &[RuntimeValue::Integer(5)]);
}

#[test]
fn test_call_function_with_index() {
    let input = r#"
@modname "com.tukunc.testmodule"
%const "name" = "com.tukunc.testmodule.callee"
%const "five" = 5
fn callee
  :entry
     push 1
     ret
fn main
  :entry
    push 0
    flookup
    call
    ret
"#;
    run_source(input, &[RuntimeValue::Integer(5)]);
}

#[test]
fn test_jump_label() {
    let input = r#"
@modname "com.tukunc.testmodule"
%const "six" = 6
fn main
  :entry
    jump :do
    ret
  :do
    push 0
    ret
"#;
    run_source(input, &[RuntimeValue::Integer(6)]);
}

#[test]
fn test_use_named_constant() {
    let input = r#"
@modname "com.tukunc.testmodule"
%const "forty_two" = 42
fn main
  :entry
    push "forty_two"
    ret
"#;
    run_source(input, &[RuntimeValue::Integer(42)]);
}

#[test]
fn test_mkarrtype() {
    let input = r#"
@modname "com.tukunc.testmodule"
%const "three" = 3
fn main
  :entry
    push "three"
    typeof
    push "three"
    mkarrtype
    ret
"#;
    run_source(
        input,
        &[RuntimeValue::Type(runtime::types::RuntimeType::Arr(
            Box::new(ArrayType::new(runtime::types::RuntimeType::Integer, 3)),
        ))],
    );
}

#[test]
fn test_newarr() {
    let input = r#"
@modname "com.tukunc.testmodule"
%const "five" = 5
%const "one" = 1
%const "two" = 2
%const "three" = 3
%const "four" = 4
fn main
  :entry
    push "one"
    push "two"
    push "three"
    push "four"
    push "four"
    typeof
    push "four"
    mkarrtype
    newarr
    ret
"#;
    run_source(
        input,
        &[rv_arr!(rv_int!(1), rv_int!(2), rv_int!(3), rv_int!(4))],
    );
}

#[test]
fn test_fcall_corelib() {
    let input = r#"
@modname "com.tukunc.testmodule"
%const "corelib.now" = "corelib.now"
fn main
  :entry
    fcall "corelib.now"
    ret
"#;
    let mut env = run_source(input, &[]);
    assert!(!env.is_stack_empty());
    let value = env.pop_value();
    assert!(matches!(value, RuntimeValue::Integer(x) if x > 0));
}

#[test]
fn test_call_function_with_implicit_name() {
    let input = r#"
@modname "com.tukunc.testmodule"
%const "five" = 5
fn callee
  :entry
     push "five"
     ret
fn main
  :entry
    fcall "com.tukunc.testmodule.callee"
    ret
"#;
    run_source(input, &[RuntimeValue::Integer(5)]);
}

#[test]
fn test_jtrue() {
    let input = r#"
@modname "com.tukunc.testmodule"
%const "six" = 6
%const "seven" = 7
fn main
  :entry
    push "six"
    push "seven"
    equal
    jtrue :fail
    push "six"
    push "six"
    equal
    jtrue :pass
  :fail
    push "six"
    ret
  :pass
    push "seven"
    ret
"#;
    run_source(input, &[RuntimeValue::Integer(7)]);
}

#[test]
fn test_dup() {
    let input = r#"
@modname "com.tukunc.testmodule"
%const "six" = 6
fn main
  :entry
    push "six"
    dup
    add
    ret
"#;
    run_source(input, &[RuntimeValue::Integer(12)]);
}

#[test]
fn test_pop() {
    let input = r#"
@modname "com.tukunc.testmodule"
%const "six" = 6
%const "seven" = 7
fn main
  :entry
    push "six"
    push "seven"
    push "six"
    pop
    add
    ret
"#;
    run_source(input, &[RuntimeValue::Integer(13)]);
}

#[test]
fn test_slot() {
    let input = r#"
@modname "com.tukunc.testmodule"
%const "six" = 6
%const "seven" = 7
fn callee
  :entry
    toslot 0
    toslot 1
    fromslot 0
    ret
fn main
  :entry
    push "six"
    push "seven"
    push "six"
    fcall "com.tukunc.testmodule.callee"
    add
    ret
"#;
    run_source(input, &[RuntimeValue::Integer(12)]);
}

#[test]
fn test_arrget() {
    let input = r#"
@modname "com.tukunc.testmodule"
%const "five" = 5
%const "one" = 1
%const "two" = 2
%const "three" = 3
%const "four" = 4
%const "zero" = 0
fn main
  :entry
    push "one"
    push "two"
    push "three"
    push "four"
    push "four"
    typeof
    push "four"
    mkarrtype
    newarr
    push "one"
    arrget
    ret
"#;
    run_source(input, &[rv_int!(2)]);
}

#[test]
fn test_arrset() {
    let input = r#"
@modname "com.tukunc.testmodule"
%const "five" = 5
%const "six" = 6
%const "one" = 1
%const "two" = 2
%const "three" = 3
%const "four" = 4
%const "zero" = 0
fn main
  :entry
    push "one"
    push "two"
    push "three"
    push "four"
    push "five"
    push "zero"
    push "six"
    typeof
    push "six"
    mkarrtype
    newarr
    push "one"
    push "five"
    arrset
    dup
    push "one"
    arrget
    ret
"#;
    run_source(
        input,
        &[
            rv_int!(5),
            rv_arr!(
                rv_int!(1),
                rv_int!(5),
                rv_int!(3),
                rv_int!(4),
                rv_int!(5),
                rv_int!(0)
            ),
        ],
    );
}

#[test]
fn test_arrlen() {
    let input = r#"
@modname "com.tukunc.testmodule"
%const "five" = 5
%const "one" = 1
%const "two" = 2
%const "three" = 3
%const "four" = 4
%const "zero" = 0
fn main
  :entry
    push "one"
    push "two"
    push "three"
    push "four"
    push "four"
    typeof
    push "four"
    mkarrtype
    newarr
    arrlen
    ret
"#;
    run_source(input, &[rv_int!(4)]);
}

#[test]
fn test_typeof() {
    let input = r#"
@modname "com.tukunc.testmodule"
%const "one" = 1
%const "hello" = "hello"
fn main
  :entry
    push "one"
    typeof
    push "hello"
    typeof
    push "one"
    push "one"
    equal
    typeof
    ret
"#;
    run_source(
        input,
        &[
            RuntimeValue::Type(runtime::types::RuntimeType::Logical),
            RuntimeValue::Type(runtime::types::RuntimeType::String),
            RuntimeValue::Type(runtime::types::RuntimeType::Integer),
        ],
    );
}

#[test]
fn test_arrlen_with_corelib() {
    let input = r#"
@modname "com.tukunc.testmodule"
%const "five" = 5
%const "one" = 1
%const "two" = 2
%const "three" = 3
%const "four" = 4
%const "zero" = 0
%const "integer" = "corelib.integer"
fn main
  :entry
    push "one"
    push "two"
    push "three"
    push "four"
    push "integer"
    tlookup
    push "four"
    mkarrtype
    newarr
    arrlen
    ret
"#;
    run_source(input, &[rv_int!(4)]);
}

#[test]
fn test_mkrectype() {
    let input = r#"
@modname "com.tukunc.testmodule"
%const "integer" = "corelib.integer"
%const "logical" = "corelib.logical"
%const "four" = 4
fn main
  :entry
    push "integer"
    tlookup
    push "logical"
    tlookup
    push "integer"
    tlookup
    dup
    push "four"
    mkrectype
    ret
"#;
    run_source(
        input,
        &[RuntimeValue::Type(RuntimeType::Record(Box::new(
            RecordType::new(&[
                RuntimeType::Integer,
                RuntimeType::Logical,
                RuntimeType::Integer,
                RuntimeType::Integer,
            ]),
        )))],
    );
}

#[test]
fn test_newrec() {
    let input = r#"
@modname "com.tukunc.testmodule"
%const "integer" = "corelib.integer"
%const "string" = "corelib.string"
%const "logical" = "corelib.logical"
%const "three" = 3
%const "four" = 4
%const "hello" = "hi"
fn main
  :entry
    push "four"
    dup
    dup
    equal
    push "hello"
    push "integer"
    tlookup
    push "logical"
    tlookup
    push "string"
    tlookup
    push "three"
    mkrectype
    newrec
    ret
"#;
    let rv = Record::new_inferred(&[rv_int!(4), rv_bool!(true), rv_str!("hi")]);
    run_source(input, &[RuntimeValue::Record(rv)]);
}

#[test]
fn test_newrec_from_decl() {
    let input = r#"
@modname "com.tukunc.testmodule"
%typedef "rec" = record("integer", "logical", "string")
%const "three" = 3
%const "four" = 4
%const "hello" = "hi"
fn main
  :entry
    push "three"
    dup
    push "four"
    equal
    push "hello"
    push "com.tukunc.testmodule.rec"
    tlookup
    newrec
    ret
"#;
    let rv = Record::new_inferred(&[rv_int!(3), rv_bool!(false), rv_str!("hi")]);
    run_source(input, &[RuntimeValue::Record(rv)]);
}

#[test]
fn test_recget() {
    let input = r#"
@modname "com.tukunc.testmodule"
%typedef "rec" = record("integer", "logical", "string")
%const "three" = 3
%const "four" = 4
%const "hello" = "hi"
%const "one" = 1
fn main
  :entry
    push "three"
    dup
    push "four"
    equal
    push "hello"
    push "com.tukunc.testmodule.rec"
    tlookup
    newrec
    push "one"
    recget
    ret
"#;
    run_source(input, &[rv_bool!(false)]);
}

#[test]
fn test_recset() {
    let input = r#"
@modname "com.tukunc.testmodule"
%typedef "rec" = record("integer", "logical", "string")
%const "three" = 3
%const "four" = 4
%const "hello" = "hi"
%const "one" = 1
fn main
  :entry
    push "three"
    dup
    push "four"
    equal
    push "hello"
    push "com.tukunc.testmodule.rec"
    tlookup
    newrec
    push "one"
    push "three"
    dup
    equal
    recset
    ret
"#;
    let rv = Record::new_inferred(&[rv_int!(3), rv_bool!(true), rv_str!("hi")]);
    run_source(input, &[RuntimeValue::Record(rv)]);
}

#[test]
fn test_decl_arrtype() {
    let input = r#"
@modname "com.tukunc.testmodule"
%typedef "arrt" = array(3,"integer")
fn main
  :entry
    push "com.tukunc.testmodule.arrt"
    tlookup
    ret
"#;
    run_source(
        input,
        &[RuntimeValue::Type(runtime::types::RuntimeType::Arr(
            Box::new(ArrayType::new(runtime::types::RuntimeType::Integer, 3)),
        ))],
    );
}

#[test]
fn test_comment() {
    let input = r#"
@modname "com.tukunc.testmodule" # name the module
%typedef "arrt" = array(3,"integer") # a typedef
%const "foo" = 3 # an integer value
fn main # a function
  :entry # a label
    # ret not a statement
    push "foo" # push 3 on the stack
    dup # duplicate
    add
    # add
    ret
    # an empty comment here
"#;
    run_source(input, &[rv_int!(6)]);
}

#[test]
fn test_not() {
    let input = r#"
@modname "com.tukunc.testmodule"
%const "five" = 5
%const "four" = 4
fn main
  :entry
    push "five"
    push "five"
    equal not
    push "five"
    push "four"
    equal not
    ret
"#;
    run_source(input, &[rv_bool!(true), rv_bool!(false)]);
}

#[test]
#[should_panic(expected = "type name corelib.integer is undefined")]
fn test_type_remoting() {
    let input = r#"
@modname "com.tukunc.testmodule"
# does not work because cannot resolve typename from external module
%typedef "fourint" = array(4,"corelib.integer")
fn main
  :entry
    ret
"#;
    run_source(input, &[]);
}
