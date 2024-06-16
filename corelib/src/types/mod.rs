use runtime::{runtime_module::RuntimeModule, types::RuntimeType};

pub(crate) fn register_corelib(rm: &mut RuntimeModule) {
    rm.add_named_type(&RuntimeType::Integer.to_typedef("integer"));
    rm.add_named_type(&RuntimeType::Logical.to_typedef("logical"));
    rm.add_named_type(&RuntimeType::String.to_typedef("string"));
    rm.add_named_type(&RuntimeType::Float.to_typedef("float"));
}
