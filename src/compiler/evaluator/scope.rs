use std::collections::HashMap;
use super::evaluator::Evaluator;

pub struct Scope<'parent, 'ast: 'parent, 'text: 'ast> {
    pub parent: Option<&'parent Scope<'parent, 'ast, 'text>>,
    pub map: HashMap<&'text str, Evaluator<'parent, 'ast, 'text>>,
}
impl<'parent, 'ast: 'parent, 'text: 'ast> Scope<'parent, 'ast, 'text> {
    pub fn new() -> Self {
        Scope{parent: None, map: HashMap::new()}
    }

    fn empty(parent: &'parent Scope<'parent, 'ast, 'text>) -> Self {
        Scope{parent: Some(parent), map: HashMap::new()}
    }

    pub fn get_value(&self, id_name: &'text str) -> Option<&Evaluator<'parent, 'ast, 'text>> {
        if let Some(eval) = self.map.get(id_name) {
            Some(eval)
        } else if let Some(parent) = self.parent {
            parent.get_value(id_name)
        } else {
            None
        }
    }
}


