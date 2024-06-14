pub mod builder;
pub mod bytecode;
pub mod environ;
pub mod frame;
pub mod instruction_def;
pub mod instruction_runtime;
pub mod intern_value;
pub mod log;
pub mod module_definition;
pub mod opcodes;
pub mod runloop;
pub mod runtime_module;
pub mod stack;
pub mod types;
pub mod unwinder;
pub mod values;

#[cfg(test)]
pub mod test;

pub fn foo() -> String {
    String::from("tukun library")
}
