use std::{collections::{HashMap, HashSet}, hash::Hash, ops::Deref, sync::Arc};

/// Corresponds to a parameter in a [`PDDLPredicate`], to assign 
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PDDLVariable(usize);

pub trait PDDLPredicate: Sized + Clone + Eq + Hash + 'static {
    // fn eval(&self) -> bool;
}
