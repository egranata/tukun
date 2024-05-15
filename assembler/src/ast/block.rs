use crate::{parser::Rule, result::AssemblerResult};

use super::instructions::Instruction;

#[derive(Debug)]
pub struct Block {
    pub name: String,
    pub body: Vec<Instruction>,
}

impl Block {
    pub(crate) fn from_parse_tree(p: pest::iterators::Pair<'_, Rule>) -> AssemblerResult<Self> {
        assert!(p.as_rule() == Rule::block);

        let f = p.into_inner();
        let name = f.find_first_tagged("name").expect("need a name");

        let mut ret = Self {
            name: name.as_str().to_owned(),
            body: vec![],
        };

        for bi in f {
            match bi.as_rule() {
                Rule::label => {}
                Rule::statement => {
                    let i = Instruction::from_parse_tree(bi.into_inner().last().unwrap())?;
                    ret.body.push(i);
                }
                _ => panic!("unexpected entry {bi}"),
            }
        }

        Ok(ret)
    }
}
