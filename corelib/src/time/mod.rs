use std::time::SystemTime;

use runtime::{
    runtime_module::{NativeCallable, RuntimeModule},
    rv_int,
};

struct NowCallable {}
impl NativeCallable for NowCallable {
    fn call(&self, env: &mut runtime::environ::Environment) {
        let now = SystemTime::now();
        let duration = now
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("not time travel")
            .as_millis() as u64;
        env.push_value(rv_int!(duration));
    }

    fn name(&self) -> String {
        String::from("now")
    }
}

pub(crate) fn register_corelib(rm: &mut RuntimeModule) {
    rm.add_function_native(Box::new(NowCallable {}));
}
