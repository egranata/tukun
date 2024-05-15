use crate::{ast::parse_string_trim, parser::Rule, result::AssemblerResult};

pub(crate) struct Attribute {
    pub(crate) name: String,
    pub(crate) value: String,
}

impl Attribute {
    pub(crate) fn from_parse_tree(p: pest::iterators::Pair<'_, Rule>) -> AssemblerResult<Self> {
        assert!(p.as_rule() == Rule::attribute);
        let mut f = p.into_inner();
        let value = f.find_first_tagged("value").expect("need a value");
        let value = parse_string_trim(value.as_str());

        if let Some(bb) = f.next() {
            if bb.as_rule() == Rule::attribute_module_name {
                return Ok(Self {
                    name: "modname".to_string(),
                    value,
                });
            } else {
                return Err(crate::result::AssemblerError::AstGenerationError(format!(
                    "invalid attribute {}",
                    bb
                )));
            }
        }

        panic!("did not find a valid attribute");
    }
}
