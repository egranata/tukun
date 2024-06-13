use runtime::{
    environ::Environment, runtime_module::NativeCallable, rv_arr, rv_int, values::RuntimeValue,
};

use crate::util::ArrayCopy;

#[test]
fn test_arraycopy() {
    let src = rv_arr!(rv_int!(2), rv_int!(4), rv_int!(5), rv_int!(6), rv_int!(8));
    let dst = rv_arr!(
        rv_int!(0),
        rv_int!(1),
        rv_int!(0),
        rv_int!(0),
        rv_int!(0),
        rv_int!(0),
        rv_int!(0),
        rv_int!(0),
        rv_int!(5),
        rv_int!(0)
    );
    let ac = ArrayCopy {};
    let mut env = Environment::default();
    env.push_value(src); // src
    env.push_value(rv_int!(0)); // src_idx
    env.push_value(dst); // dst
    env.push_value(rv_int!(1)); // dst_idx
    env.push_value(rv_int!(3)); // len
    assert!(ac.call(&mut env).is_ok());
    assert!(!env.is_stack_empty());
    let dst = typed_pop!(env, RuntimeValue::Arr);
    assert_eq!(10, dst.len());
    assert_eq!(
        dst.as_runtime_value(),
        rv_arr!(
            rv_int!(0),
            rv_int!(2),
            rv_int!(4),
            rv_int!(5),
            rv_int!(0),
            rv_int!(0),
            rv_int!(0),
            rv_int!(0),
            rv_int!(5),
            rv_int!(0)
        )
    )
}
