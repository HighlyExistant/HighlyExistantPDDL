use std::{collections::{HashMap, HashSet}, fmt::Debug, sync::Arc};

use crate::pddl_condition::PDDLPredicate;

/// The parameters of an action, should be held
/// within the action itself.
pub trait PDDLAction<Predicate: PDDLPredicate> {
    fn preconditions(&self) -> &HashMap<Predicate, bool>;
    fn effects(&self) -> &HashMap<Predicate, bool>;
    fn action_cost(&self) -> f32;
    fn perform(&mut self, domain: &dyn PDDLDomain<Predicate>, delta: f32) -> bool;
}
pub trait PDDLActionFactory<Predicate: PDDLPredicate> {
    // fn construct(&self, parameters: HashMap<>) -> Arc<dyn PDDLAction<Predicate>>;
}

impl<Predicate: PDDLPredicate + Debug> Debug for dyn PDDLAction<Predicate> {
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
pub trait PDDLGoal<Predicate: PDDLPredicate> {
    fn priority(&self) -> f32;
    fn desired_state(&self) -> &HashMap<Predicate, bool>;
}
impl<Predicate: PDDLPredicate + Debug> Debug for dyn PDDLGoal<Predicate> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PDDLGoal")
            .field("priority", &self.priority())
            .field("desired_state", self.desired_state())
            .finish()
    }
}

pub trait PDDLDomain<Predicate: PDDLPredicate> {
    fn predicates(&self) -> &HashSet<Predicate>;
}