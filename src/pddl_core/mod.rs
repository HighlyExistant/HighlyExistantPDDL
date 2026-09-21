use std::{collections::{HashMap, HashSet}, fmt::Debug, sync::Arc};

use crate::pddl_condition::{PDDLPredicate, PDDLPredicateName};

/// The parameters of an action, should be held
/// within the action itself.
pub trait PDDLAction<Name: PDDLPredicateName> {
    fn preconditions(&self) -> &HashMap<Name, bool>;
    fn effects(&self) -> &HashMap<Name, bool>;
    fn action_cost(&self) -> f32;
    fn perform(&mut self, domain: &dyn PDDLDomain<Name>, delta: f32) -> bool;
}

impl<Name: PDDLPredicateName + Debug> Debug for dyn PDDLAction<Name> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PDDLAction")
            .field("preconditions", self.preconditions())
            .field("effects", self.effects())
            .field("action_cost", &self.action_cost())
            .finish()
    }
}

/// The parameters of a goal, should be held
/// within the goal itself.
pub trait PDDLGoal<Name: PDDLPredicateName> {
    fn priority(&self) -> f32;
    fn desired_state(&self) -> &HashMap<Name, bool>;
}
impl<Name: PDDLPredicateName + Debug> Debug for dyn PDDLGoal<Name> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PDDLGoal")
            .field("priority", &self.priority())
            .field("desired_state", self.desired_state())
            .finish()
    }
}

pub trait PDDLDomain<Name: PDDLPredicateName> {
    fn predicates(&self) -> &HashMap<Name, Arc<dyn PDDLPredicate<PredicateName = Name>>>;
}