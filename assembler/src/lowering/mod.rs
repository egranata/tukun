mod instructions;

use runtime::{
    builder::{BasicBlock, Builder},
    intern_value::InternValue,
    module_definition::{FunctionDef, ModuleDef},
    types::{array::ArrayType, record::RecordType, typedef::TypeDef, RuntimeType},
};

use crate::ast::{
    block::Block, constant::Constant, function::Function, module::Module, types::ValueType,
};

use self::instructions::lower_instruction;

fn lower_type(input: &ValueType) -> RuntimeType {
    match input {
        ValueType::B(bt) => match bt {
            crate::ast::types::BuiltinType::Integer => RuntimeType::Integer,
            crate::ast::types::BuiltinType::String => RuntimeType::String,
            crate::ast::types::BuiltinType::Logical => RuntimeType::Logical,
        },
        ValueType::A(at) => {
            let et = at.value_type();
            let et = lower_type(et);
            RuntimeType::Arr(Box::new(ArrayType::new(et, at.len())))
        }
        ValueType::R(rt) => {
            let et = rt
                .slice()
                .iter()
                .map(lower_type)
                .collect::<Vec<RuntimeType>>();
            RuntimeType::Record(Box::new(RecordType::new(&et)))
        }
    }
}

fn lower_constant(input: &Constant) -> InternValue {
    input.val.clone()
}

fn create_basic_block(input: &Block, b: &mut Builder) -> BasicBlock {
    b.append_block(&input.name)
}

fn lower_basic_block(mdef: &Module, input: &Block, b: &mut Builder) -> BasicBlock {
    let mut ret = b.find_block(&input.name).expect("invalid block");

    for i in &input.body {
        let lis = lower_instruction(mdef, i, b);
        for li in lis {
            ret.append_instruction(li);
        }
    }

    ret
}

fn lower_function(mdef: &Module, input: &Function) -> FunctionDef {
    let mut b = Builder::new(&input.name);

    for k in &input.body {
        create_basic_block(k, &mut b);
    }

    for k in &input.body {
        lower_basic_block(mdef, k, &mut b);
    }

    b.generate()
}

fn lower_name_symbol(ast: &mut Module, fname: &str) -> InternValue {
    let name = format!("{}.{}", ast.name, fname);
    let c = Constant {
        name: name.clone(),
        val: InternValue::String(name),
    };
    ast.add_constant(c)
}

pub fn lower_ast(mut input: Module) -> ModuleDef {
    let mut ret = ModuleDef::new(&input.name);

    for c in &input.constants {
        ret.add_interned_value(lower_constant(c));
    }

    for t in &input.types {
        let urt = lower_type(t.1);
        ret.add_named_type(&TypeDef::new(t.0, &urt));
    }

    {
        let names = input.types.keys().cloned().collect::<Vec<String>>();
        names.iter().for_each(|tname| {
            ret.add_interned_value(lower_name_symbol(&mut input, tname));
        });
    }

    {
        let names = input
            .functions
            .iter()
            .map(|f| f.name.clone())
            .collect::<Vec<String>>();
        names.iter().for_each(|tname| {
            ret.add_interned_value(lower_name_symbol(&mut input, tname));
        });
    }

    for f in &input.functions {
        ret.add_function(lower_function(&input, f));
    }

    ret
}
