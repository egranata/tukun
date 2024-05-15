use crate::{ast, lowering, parser, result::AssemblerResult};

pub(crate) fn do_assemble(src: &str) -> AssemblerResult<Vec<u8>> {
    let parse_result = parser::derive_parse_tree(src)?;
    let ast = ast::parse_tree_to_ast(parse_result)?;
    let mdef = lowering::lower_ast(ast);
    match bincode::serialize(&mdef) {
        Ok(bytes) => Ok(bytes),
        Err(err) => Err(crate::result::AssemblerError::SerializationError(format!(
            "{err}"
        ))),
    }
}
