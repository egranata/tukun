use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

use crate::result::AssemblerResult;

#[derive(Parser)]
#[grammar = "src/tukun.pest.common"]
#[grammar = "src/tukun.pest.ops"]
pub struct TukunGrammar;

#[allow(clippy::result_large_err)]
pub fn derive_parse_tree(input: &str) -> AssemblerResult<Pair<'_, Rule>> {
    let maybe_parsed = TukunGrammar::parse(Rule::module, input);

    match maybe_parsed {
        Ok(result) => Ok(result.last().expect("multiple modules not acceptable")),
        Err(err) => Err(crate::result::AssemblerError::ParseError(format!("{err}"))),
    }
}
