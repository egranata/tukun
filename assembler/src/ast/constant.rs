use runtime::intern_value::InternValue;

use crate::{ast::parse_string_trim, parser::Rule, result::AssemblerResult};

#[derive(Clone, Debug)]
pub struct Constant {
    pub(crate) name: String,
    pub(crate) val: InternValue,
}

impl Constant {
    pub(crate) fn from_parse_tree(p: pest::iterators::Pair<'_, Rule>) -> AssemblerResult<Self> {
        assert!(p.as_rule() == Rule::interned_value);

        let mut f = p.into_inner();
        if let Some(bb) = f.next() {
            match bb.as_rule() {
                Rule::interned_float => {
                    let inner = bb.into_inner();
                    let value = inner.find_first_tagged("value").expect("need a value");
                    let value = value.as_str().parse::<f64>().expect("invalid float");
                    let name = inner.find_first_tagged("name").expect("need a name");
                    let name = parse_string_trim(name.as_str());
                    Ok(Self {
                        name,
                        val: InternValue::Float(value),
                    })
                }
                Rule::interned_integer => {
                    let inner = bb.into_inner();
                    let value = inner.find_first_tagged("value").expect("need a value");
                    let value = value.as_str().parse::<u64>().expect("invalid integer");
                    let name = inner.find_first_tagged("name").expect("need a name");
                    let name = parse_string_trim(name.as_str());
                    Ok(Self {
                        name,
                        val: InternValue::Integer(value),
                    })
                }
                Rule::interned_string => {
                    let inner = bb.into_inner();
                    let value = inner.find_first_tagged("value").expect("need a value");
                    let value = parse_string_trim(value.as_str());
                    let name = inner.find_first_tagged("name").expect("need a name");
                    let name = parse_string_trim(name.as_str());
                    Ok(Self {
                        name,
                        val: InternValue::String(value),
                    })
                }
                _ => Err(crate::result::AssemblerError::AstGenerationError(format!(
                    "not a valid interned value {}",
                    bb
                ))),
            }
        } else {
            Err(crate::result::AssemblerError::AstGenerationError(format!(
                "not a valid directive {}",
                f
            )))
        }
    }
}
