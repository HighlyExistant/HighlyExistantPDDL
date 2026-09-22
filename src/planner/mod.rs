use std::{fmt::Debug, sync::Arc};

use crate::{strips_condition::StripsPredicate, strips_core::StripsAction};

pub mod planner;
#[derive(Debug,Clone)]
pub struct StripsPlan<Predicate: StripsPredicate> {
    actions: Vec<Arc<dyn StripsAction<Predicate>>>,
    steps_left: usize,
}

impl<Predicate: StripsPredicate> Iterator for StripsPlan<Predicate> {
    type Item = Arc<dyn StripsAction<Predicate>>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.steps_left == 0 {
            return None;
        }
        self.steps_left -= 1;
        Some(self.actions[self.steps_left].clone())
    }
}
