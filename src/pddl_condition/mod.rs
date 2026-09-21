use std::{collections::{HashMap, HashSet}, hash::Hash, ops::Deref, sync::Arc};

pub trait PDDLPredicateName: Sized + Clone + Eq + Hash + 'static {
}

/// Parameters should be handled inside of the type.
pub trait PDDLPredicate: 'static + Send + Sync {
    type PredicateName: PDDLPredicateName;
    fn name(&self) -> Self::PredicateName;
    fn eval(&self) -> bool;
}

pub trait PDDLState {
    fn has_predicate(&self, name: String, parameters: &Vec<String>) -> bool;
}