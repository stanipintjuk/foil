use std::collections::HashMap;
use super::evaluator::{Evaluator, EvalResult};

#[derive(PartialEq)]
#[derive(Debug)]
pub enum Scope<'parent, 'ast: 'parent> {
    Open(&'parent OpenScope<'parent, 'ast>),
    Closed(&'parent ClosedScope),
}
impl<'parent, 'ast: 'parent> Scope<'parent, 'ast> {
    pub fn to_closed(&self) -> ClosedScope {
        match self {
            &Scope::Open(ref scope) => scope.to_closed(),
            &Scope::Closed(ref scope) => Clone::clone(scope),
        }
    }

    pub fn get_value(&self, id_name: &str) -> Option<EvalResult> {
        match self {
            &Scope::Open(ref scope) => scope.get_value(id_name),
            &Scope::Closed(ref scope) => scope.get_value(id_name),
        }
    }
}
impl<'parent, 'ast: 'parent> Clone for Scope<'parent, 'ast> {

    fn clone(&self) -> Scope<'parent, 'ast> {
        match self {
            &Scope::Open(ref s) => Scope::Open(s),
            &Scope::Closed(ref s) => Scope::Closed(s),
        }
    }
}

#[derive(PartialEq)]
#[derive(Debug)]
pub struct OpenScope<'parent, 'ast: 'parent> {
    pub parent: Option<Scope<'parent, 'ast>>,
    pub map: HashMap<&'ast str, Evaluator<'parent, 'ast>>,
}
impl<'parent, 'ast: 'parent> OpenScope<'parent, 'ast> {
    pub fn new() -> Self {
        OpenScope{parent: None, map: HashMap::new()}
    }

    pub fn get_value(&self, id_name: &str) -> Option<EvalResult> {
        if let Some(eval) = self.map.get(id_name) {
            Some(eval.eval())
        } else if let Some(ref parent) = self.parent {
            parent.get_value(id_name)
        } else {
            None
        }
    }

    /// Copies all of its variables into 
    /// one owned, closed scope.
    pub fn to_closed(&self) -> ClosedScope {
        let mut closed_map = HashMap::new();
        for (key, value) in self.map.iter() {
            closed_map.insert(key.to_string(), value.eval());
        }

        let mut closed_scope = ClosedScope{map: closed_map};

        if let Some(ref parent) = self.parent {
            closed_scope.merge(&parent.to_closed());
        }

        closed_scope
    }
}

#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Clone)]
pub struct ClosedScope {
    pub map: HashMap<String, EvalResult>,
}
impl ClosedScope {
    pub fn get_value(&self, id_name: &str) -> Option<EvalResult> {
        if let Some(eval) = self.map.get(id_name) {
            Some(Clone::clone(eval))
        } else {
            None
        }
    }

    pub fn merge(&mut self, other: &ClosedScope) {
        for (key, value) in other.map.iter() {
            if self.map.get(key) == None {
                self.map.insert(key.to_string(), Clone::clone(value));
            }
        }
    }
}
