use std::collections::HashMap;

use crate::{ast::parse_string_trim, parser::Rule, result::AssemblerResult};

#[derive(Clone, Debug)]
pub(crate) enum ValueType {
    B(BuiltinType),
    A(Box<ArrayType>),
    R(Box<RecordType>),
}

impl ValueType {
    fn from_parse_tree(
        p: pest::iterators::Pair<'_, Rule>,
        type_map: &HashMap<String, ValueType>,
    ) -> AssemblerResult<Self> {
        assert!(p.as_rule() == Rule::type_descriptor);
        let mut value_f = p.into_inner();
        if let Some(bb) = value_f.next() {
            match bb.as_rule() {
                Rule::type_name => {
                    let underlying_name = bb.as_str();
                    let underlying_name = parse_string_trim(underlying_name);
                    if let Some(underlying_type) = type_map.get(&underlying_name) {
                        Ok(underlying_type.clone())
                    } else {
                        panic!("type name {underlying_name} is undefined");
                    }
                }
                Rule::type_array => {
                    let underlying_type = ArrayType::from_parse_tree(bb, type_map)?;
                    Ok(ValueType::A(Box::new(underlying_type)))
                }
                Rule::type_record => {
                    let underlying_type = RecordType::from_parse_tree(bb, type_map)?;
                    Ok(ValueType::R(Box::new(underlying_type)))
                }
                _ => Err(crate::result::AssemblerError::AstGenerationError(format!(
                    "unexpected typedef result {}",
                    bb
                ))),
            }
        } else {
            panic!("unexpected rule {value_f}");
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) enum BuiltinType {
    Integer,
    String,
    Logical,
}

#[derive(Clone, Debug)]
pub(crate) struct ArrayType {
    len: usize,
    vt: ValueType,
}

impl ArrayType {
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn value_type(&self) -> &ValueType {
        &self.vt
    }
}

impl ArrayType {
    fn from_parse_tree(
        p: pest::iterators::Pair<'_, Rule>,
        type_map: &HashMap<String, ValueType>,
    ) -> AssemblerResult<Self> {
        assert!(p.as_rule() == Rule::type_array);
        let f = p.into_inner();
        let len = f.find_first_tagged("count").expect("need a count").as_str();
        let len = len.parse::<usize>().expect("invalid count");
        let content = f.find_first_tagged("of").expect("need element type");
        let vt = ValueType::from_parse_tree(content, type_map)?;
        Ok(Self { len, vt })
    }
}

impl RecordType {
    fn from_parse_tree(
        p: pest::iterators::Pair<'_, Rule>,
        type_map: &HashMap<String, ValueType>,
    ) -> AssemblerResult<Self> {
        assert!(p.as_rule() == Rule::type_record);
        let f = p.into_inner();
        let et: Vec<ValueType> = f
            .map(|i| ValueType::from_parse_tree(i, type_map).expect("invalid element type"))
            .collect();
        Ok(Self { et })
    }
}

#[derive(Clone, Debug)]
pub(crate) struct RecordType {
    et: Vec<ValueType>,
}

impl RecordType {
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.et.len()
    }

    #[allow(dead_code)]
    pub fn element_at(&self, idx: usize) -> &ValueType {
        &self.et[idx]
    }

    pub fn slice(&self) -> &[ValueType] {
        &self.et
    }
}

#[derive(Clone, Debug)]
pub(crate) struct TypeAlias {
    name: String,
    vt: ValueType,
}

impl TypeAlias {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn target(&self) -> &ValueType {
        &self.vt
    }
}

impl TypeAlias {
    pub(crate) fn from_parse_tree(
        p: pest::iterators::Pair<'_, Rule>,
        type_map: &HashMap<String, ValueType>,
    ) -> AssemblerResult<Self> {
        assert!(p.as_rule() == Rule::typedef);
        let f = p.into_inner();
        let name = f.find_first_tagged("name").expect("need a name").as_str();
        let name = parse_string_trim(name);
        let value = f.find_first_tagged("value").expect("need a value");
        let vt = ValueType::from_parse_tree(value, type_map)?;
        Ok(Self { name, vt })
    }
}
