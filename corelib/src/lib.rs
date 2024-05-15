use runtime::{environ::Environment, runtime_module::RuntimeModule};

macro_rules! typed_pop {
    ($env:expr, $t:path) => {{
        if $env.is_stack_empty() {
            panic!("empty stack unexpected");
        }
        let val = $env.pop_value();
        if let $t(payload) = val {
            payload
        } else {
            panic!("invalid value {}", val)
        }
    }};
}

mod time;
mod types;
mod util;

#[cfg(test)]
pub mod test;

pub fn register_corelib(env: &mut Environment) {
    let mut rm = RuntimeModule::new("corelib");
    crate::time::register_corelib(&mut rm);
    crate::types::register_corelib(&mut rm);
    crate::util::register_corelib(&mut rm);

    env.add_module(rm);
}
