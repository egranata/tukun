use crate::{parser::Rule, result::AssemblerResult};

use super::block::Block;

#[derive(Debug)]
pub struct Function {
    pub(crate) name: String,
    pub(crate) body: Vec<Block>,
}

impl Function {
    pub(crate) fn from_parse_tree(p: pest::iterators::Pair<'_, Rule>) -> AssemblerResult<Self> {
        assert!(p.as_rule() == Rule::function);
        let f = p.into_inner();
        let name = f.find_first_tagged("name").expect("need a name");

        let mut ret = Self {
            name: name.as_str().to_owned(),
            body: vec![],
        };

        for bb in f {
            match bb.as_rule() {
                Rule::ident => {}
                Rule::block => {
                    let b = Block::from_parse_tree(bb)?;
                    ret.body.push(b);
                }
                _ => panic!("unexpected entry: {bb}"),
            }
        }

        Ok(ret)
    }
}
