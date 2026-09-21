use std::{fmt::Debug, sync::Arc};

use crate::{pddl_condition::PDDLPredicateName, pddl_core::PDDLAction};


pub mod planner;
#[derive(Debug,Clone)]
pub struct PDDLPlan<Name: PDDLPredicateName> {
    actions: Vec<Arc<dyn PDDLAction<Name>>>,
    steps_left: usize,
}

impl<Name: PDDLPredicateName> Iterator for PDDLPlan<Name> {
    type Item = Arc<dyn PDDLAction<Name>>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.steps_left == 0 {
            return None;
        }
        self.steps_left -= 1;
        Some(self.actions[self.steps_left].clone())
    }
}
