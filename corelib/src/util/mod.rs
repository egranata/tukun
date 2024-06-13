use runtime::{
    runloop::RunloopResult,
    runtime_module::{NativeCallable, RuntimeModule},
    values::RuntimeValue,
};

struct PrintCallable {}
impl NativeCallable for PrintCallable {
    fn call(&self, env: &mut runtime::environ::Environment) -> RunloopResult {
        let value = env.pop_value();
        match value {
            runtime::values::RuntimeValue::Integer(n) => {
                println!("{}", n);
            }
            runtime::values::RuntimeValue::Logical(x) => {
                println!("{}", x);
            }
            runtime::values::RuntimeValue::String(s) => {
                println!("{}", s);
            }
            runtime::values::RuntimeValue::Function(f) => {
                println!("{}", f.fullname());
            }
            runtime::values::RuntimeValue::Arr(a) => {
                println!("{}", a);
            }
            runtime::values::RuntimeValue::Record(r) => {
                println!("{}", r);
            }
            runtime::values::RuntimeValue::Type(t) => {
                println!("{}", t);
            }
        }

        Ok(())
    }

    fn name(&self) -> String {
        String::from("print")
    }
}

pub(crate) struct ArrayCopy {}
impl NativeCallable for ArrayCopy {
    fn call(&self, env: &mut runtime::environ::Environment) -> RunloopResult {
        let len = typed_pop!(env, RuntimeValue::Integer) as usize;
        let dst_idx = typed_pop!(env, RuntimeValue::Integer) as usize;
        let mut dst = typed_pop!(env, RuntimeValue::Arr);
        let src_idx = typed_pop!(env, RuntimeValue::Integer) as usize;
        let src = typed_pop!(env, RuntimeValue::Arr);
        for i in 0..len {
            let src_item = src.get(i + src_idx);
            dst.set(i + dst_idx, &src_item);
        }
        env.push_value(RuntimeValue::Arr(dst));

        Ok(())
    }

    fn name(&self) -> String {
        String::from("arraycopy")
    }
}

pub(crate) fn register_corelib(rm: &mut RuntimeModule) {
    rm.add_function_native(Box::new(PrintCallable {}));
    rm.add_function_native(Box::new(ArrayCopy {}));
}
