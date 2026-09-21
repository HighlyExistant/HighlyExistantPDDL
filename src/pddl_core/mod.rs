use std::{collections::{HashMap, HashSet}, fmt::Debug, sync::Arc};

use crate::pddl_condition::{PDDLPredicate, PDDLPredicateName};

/// The parameters of an action, should be held
/// within the action itself.
pub trait PDDLAction<Name: PDDLPredicateName>: Debug {
    fn preconditions(&self) -> &HashMap<Name, bool>;
    fn effects(&self) -> &HashMap<Name, bool>;
    fn action_cost(&self) -> f32;
    fn perform(&mut self, domain: &dyn PDDLDomain<Name>, delta: f32) -> bool;
}

/// The parameters of a goal, should be held
/// within the goal itself.
pub trait PDDLGoal<Name: PDDLPredicateName>: Debug {
    fn priority(&self) -> f32;
    fn desired_state(&self) -> &HashMap<Name, bool>;
}

pub trait PDDLDomain<Name: PDDLPredicateName> {
    fn predicates(&self) -> &HashMap<Name, Arc<dyn PDDLPredicate<PredicateName = Name>>>;
}