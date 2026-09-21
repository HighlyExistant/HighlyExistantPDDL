use std::{fmt::Debug, sync::Arc};

use crate::{pddl_condition::PDDLPredicate, pddl_core::PDDLAction};

pub mod planner;
#[derive(Debug,Clone)]
pub struct PDDLPlan<Predicate: PDDLPredicate> {
    actions: Vec<Arc<dyn PDDLAction<Predicate>>>,
    steps_left: usize,
}

impl<Predicate: PDDLPredicate> Iterator for PDDLPlan<Predicate> {
    type Item = Arc<dyn PDDLAction<Predicate>>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.steps_left == 0 {
            return None;
        }
        self.steps_left -= 1;
        Some(self.actions[self.steps_left].clone())
    }
}
