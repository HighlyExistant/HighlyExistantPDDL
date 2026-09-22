use std::{collections::{HashMap, HashSet}, hash::Hash, ops::Deref, sync::Arc};

/// Corresponds to a parameter in a [`StripsPredicate`], to assign 
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct StripsVariable(usize);

pub trait StripsPredicate: Sized + Clone + Eq + Hash + 'static {
}
