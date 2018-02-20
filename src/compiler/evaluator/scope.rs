use std::collections::HashMap;
use super::output::Output;
use super::evaluator::{Evaluator, EvalResult};

#[derive(PartialEq)]
#[derive(Debug)]
pub enum Scope<'parent, 'ast: 'parent, 'text: 'ast> {
    Open(&'parent OpenScope<'parent, 'ast, 'text>),
    Closed(&'parent ClosedScope<'ast, 'text>),
}
impl<'parent, 'ast: 'parent, 'text: 'ast> Scope<'parent, 'ast, 'text> {
    pub fn to_closed(&self) -> ClosedScope<'ast, 'text> {
        match self {
            &Scope::Open(ref scope) => scope.to_closed(),
            &Scope::Closed(ref scope) => Clone::clone(scope),
        }
    }

    pub fn get_value(&self, id_name: &'text str) -> Option<EvalResult<'ast, 'text>> {
        match self {
            &Scope::Open(ref scope) => scope.get_value(id_name),
            &Scope::Closed(ref scope) => scope.get_value(id_name),
        }
    }
}
impl<'parent, 'ast: 'parent, 'text: 'ast> Clone for Scope<'parent, 'ast, 'text> {

    fn clone(&self) -> Scope<'parent, 'ast, 'text> {
        match self {
            &Scope::Open(ref s) => Scope::Open(s),
            &Scope::Closed(ref s) => Scope::Closed(s),
        }
    }
}

#[derive(PartialEq)]
#[derive(Debug)]
pub struct OpenScope<'parent, 'ast: 'parent, 'text: 'ast> {
    pub parent: Option<Scope<'parent, 'ast, 'text>>,
    pub map: HashMap<&'text str, Evaluator<'parent, 'ast, 'text>>,
}
impl<'parent, 'ast: 'parent, 'text: 'ast> OpenScope<'parent, 'ast, 'text> {
    pub fn new() -> Self {
        OpenScope{parent: None, map: HashMap::new()}
    }

    pub fn get_value(&self, id_name: &'text str) -> Option<EvalResult<'ast,'text>> {
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
    pub fn to_closed(&self) -> ClosedScope<'ast, 'text> {
        let mut closed_map = HashMap::new();
        for (key, value) in self.map.iter() {
            closed_map.insert(*key, value.eval());
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
pub struct ClosedScope<'ast, 'text: 'ast> {
    pub map: HashMap<&'text str, EvalResult<'ast, 'text>>,
}
impl<'ast, 'text: 'ast> ClosedScope<'ast, 'text> {
    pub fn get_value(&self, id_name: &'text str) -> Option<EvalResult<'ast, 'text>> {
        if let Some(eval) = self.map.get(id_name) {
            Some(Clone::clone(eval))
        } else {
            None
        }
    }

    pub fn merge(&mut self, other: &ClosedScope<'ast, 'text>) {
        for (key, value) in other.map.iter() {
            if self.map.get(key) == None {
                self.map.insert(key, Clone::clone(value));
            }
        }
    }
}
