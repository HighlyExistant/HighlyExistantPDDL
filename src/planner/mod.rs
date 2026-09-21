use std::{cell::Cell, collections::HashMap, fmt::Debug, sync::{Arc, Mutex}};

use crate::{pddl_condition::{PDDLPredicate, PDDLPredicateName}, pddl_core::{PDDLAction, PDDLDomain, PDDLGoal}};
pub mod planner;
struct PlanNode<Name: PDDLPredicateName> {
    parent: Option<Arc<PlanNode<Name>>>,
    action: Arc<dyn PDDLAction<Name>>,
    cost: f32,
}
impl<Name: PDDLPredicateName> Debug for PlanNode<Name> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PlanNode")
            .field("action", &self.action)
            .finish()
    }
}
#[derive(Debug,Clone)]
pub struct PDDLPlan<Name: PDDLPredicateName> {
    actions: Vec<Arc<dyn PDDLAction<Name>>>,
    steps_left: usize,
}

impl<Name: PDDLPredicateName> PDDLPlan<Name> {
    fn build_plan(node: Option<Arc<PlanNode<Name>>>) -> Option<PDDLPlan<Name>> {
        let mut next_node = if let Some(next_node) = node {
            next_node
        } else { // If no node was found, no plan can be generated
            return None;
        };
        let mut plan = PDDLPlan {
            actions: vec![next_node.action.clone()],
            steps_left: 0,
        };
        while let Some(parent) = &next_node.parent {
            plan.actions.push(parent.action.clone());
            next_node = parent.clone();
        }
        plan.steps_left = plan.actions.len();
        Some(plan)
    }
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
