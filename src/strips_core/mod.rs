use std::{collections::{HashMap, HashSet}, fmt::Debug, sync::Arc};

use crate::strips_condition::StripsPredicate;

/// The parameters of an action, should be held
/// within the action itself.
pub trait StripsAction<Predicate: StripsPredicate> {
    fn preconditions(&self) -> &HashMap<Predicate, bool>;
    fn effects(&self) -> &HashMap<Predicate, bool>;
    fn action_cost(&self) -> f32;
    fn perform(&mut self, domain: &dyn StripsDomain<Predicate>, delta: f32) -> bool;
}

impl<Predicate: StripsPredicate + Debug> Debug for dyn StripsAction<Predicate> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StripsAction")
            .field("preconditions", self.preconditions())
            .field("effects", self.effects())
            .field("action_cost", &self.action_cost())
            .finish()
    }
}

/// The parameters of a goal, should be held
/// within the goal itself.
pub trait StripsGoal<Predicate: StripsPredicate> {
    fn priority(&self) -> f32;
    fn desired_state(&self) -> &HashMap<Predicate, bool>;
}
impl<Predicate: StripsPredicate + Debug> Debug for dyn StripsGoal<Predicate> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StripsGoal")
            .field("priority", &self.priority())
            .field("desired_state", self.desired_state())
            .finish()
    }
}

pub trait StripsDomain<Predicate: StripsPredicate> {
    fn predicates(&self) -> &HashSet<Predicate>;
}