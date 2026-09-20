use std::{cell::Cell, collections::HashMap, fmt::Debug, sync::{Arc, Mutex}};

use crate::pddl_core::{PDDLDomain, PDDLAction, PDDLGoal};
mod change_tree;

struct PlanNode {
    parent: Option<Arc<PlanNode>>,
    action: Arc<dyn PDDLAction>,
    cost: f32,
    // This way of doing things would most likely
    // waste lots of memory, but for now, this is 
    // what I will doe
    // new_domain: HashMap<String, bool>,
}
impl Debug for PlanNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PlanNode")
            .field("action", &self.action)
            .finish()
    }
}
#[derive(Debug,Clone)]
pub struct PDDLPlan {
    actions: Vec<Arc<dyn PDDLAction>>,
    steps_left: usize,
}

pub struct PDDLPlanner {
    actions: Vec<Arc<dyn PDDLAction>>,
}

impl PDDLPlanner {
    pub fn new(actions: Vec<Arc<dyn PDDLAction>>) -> Self {
        Self {
            actions
        }
    }
    fn conditions_satisfied(current_conditions: &HashMap<String, bool>, desired_conditions: &HashMap<String, bool>) -> bool {
        for (name, precondition) in current_conditions.iter().map(|v|{(v.0.clone(), v.1.clone())}) {
            let condition = desired_conditions.get(&name).cloned().unwrap_or(false);
            if condition != precondition {
                return false;
            }
        }
        true
    }
    fn effects_satisfied(action: &Arc<dyn PDDLAction>, desired_conditions: &HashMap<String, bool>) -> bool {
        Self::conditions_satisfied(action.effects(), desired_conditions)
    }
    fn preconditions_satisfied(action: &Arc<dyn PDDLAction>, desired_conditions: &HashMap<String, bool>) -> bool {
        Self::conditions_satisfied(action.preconditions(), desired_conditions)
    }
    fn find_plans(&self, desired_conditions: &HashMap<String, bool>, parent: Option<Arc<PlanNode>>) -> Vec<Arc<PlanNode>> {
        let mut plan_list = vec![];
        for action in &self.actions {
            if Self::effects_satisfied(&action, desired_conditions) {
                let cost = if let Some(parent) = &parent {
                    parent.cost + action.action_cost()
                } else {
                    action.action_cost()
                };
                plan_list.push(
                    Arc::new(PlanNode { 
                        parent: parent.clone(), 
                        action: action.clone(), 
                        cost: cost, 
                    })
                );
            }
        }
        plan_list
    }
    pub fn get_plan(&self, goal: &dyn PDDLGoal, domain: &dyn PDDLDomain) -> Option<PDDLPlan> {
        let initial_conditions = domain.domain();
        // Get actions which will satisfy the initial goal
        // ignoring whether the preconditions are satisfied
        let mut initial_actions = self.find_plans(goal.desired_state(), None);

        // Create a node_stack to store nodes for processing
        let mut node_stack = vec![];
        node_stack.append(&mut initial_actions);

        let mut cheapest_cost = f32::INFINITY;
        let mut cheapest_node: Option<Arc<PlanNode>> = None;

        // Loop through the actions recieved
        while !node_stack.is_empty() {
            // Pop node from stack to process
            let node = node_stack.pop().unwrap();
            
            // If the node cost is higher than a solution already found, then continue
            if node.cost > cheapest_cost {
                continue;
            }
            // Check if the nodes preconditions satisfy the initial domain conditions
            // in which case set it as a possible solution.
            if Self::preconditions_satisfied(&node.action, initial_conditions) {
                cheapest_cost = node.cost;
                cheapest_node = Some(node.clone());
                continue;
            }

            // Search for actions which satisfy the preconditions necessary
            // for the action to be satisfied
            let preconditions = node.action.preconditions();
            let mut possible_actions = self.find_plans(preconditions, Some(node.clone()));

            // Append those actions to be further processed
            node_stack.append(&mut possible_actions);

        }
        let mut next_node = if let Some(next_node) = cheapest_node {
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

// impl PDDLPlanner {
//     pub fn new(actions: Vec<Arc<dyn PDDLAction>>) -> Self {
//         Self {
//             actions
//         }
//     }
//     fn find_plans(&self, desired_state: &HashMap<String, bool>, parent: Option<Arc<PlanNode>>, blackboard: &HashMap<String, bool>, minimum_cost: &mut f32) -> Vec<Arc<PlanNode>> {
//         let mut plan_list = vec![];
//         if blackboard {
            
//         }
//         for action in &self.actions {
//             let cost = if let Some(parent) = &parent {
//                 parent.cost + action.action_cost()
//             } else {
//                 action.action_cost()
//             };
//             if cost > *minimum_cost {
//                 continue;
//             }
//             let effect = action.effects();
//             if effect == desired_state {
//                 plan_list.push(Arc::new(PlanNode {
//                     action: action.clone(),
//                     parent: parent.clone(),
//                     // next: Cell::new(vec![]),
//                     cost: cost,
//                 }));
//             }
//         }
//         plan_list
//     }
//     fn change_conditions(conditions: &mut HashMap<String, bool>, effects: &HashMap<String, bool>) {
//         for (name, value) in effects {
//             if let Some(element) = conditions.get_mut(name) {
//                 *element = *value;
//             } else {
//                 conditions.insert(name.clone(), *value);
//             }
//         }
//     }
//     /// Finds plans and calculates their cost
//     fn build_plans(&self, desired_state: &HashMap<String, bool>, blackboard: &dyn PDDLDomain) -> Option<PDDLPlan> {
//         let mut minimum_cost = f32::INFINITY;

//         // Get initial list of plausible actions which can help reach a desired state.
//         let initial_conditions: &HashMap<String, bool> = blackboard.domain();
//         let mut node_stack = self.find_plans(desired_state, None, initial_conditions, &mut minimum_cost);
//         println!("selected {:?} as initial nodes", node_stack);
//         let mut cheapest_node: Option<Arc<PlanNode>> = None;
//         let mut cheapest_cost = f32::INFINITY;
//         while !node_stack.is_empty() {
//             let node = node_stack.pop().unwrap();
//             if node.cost > cheapest_cost { // There is no need to continue through this path, if it is too expensive.
//                 continue;
//             }
//             let preconditions = node.action.preconditions();
            
//             if self.preconditions_satisfied(preconditions, &conditions) { // If this is true, then it must be the current cheapest node
//                 cheapest_cost = node.cost;
//                 cheapest_node = Some(node.clone());
//                 continue;
//             }
//             // Preconditions past this point should not be satisfied.

//             // Find nodes which don't surpass the minimum cost and append them for future processing next iteration.
//             let mut sub_nodes = self.find_plans(desired_state, Some(node.clone()), blackboard, &mut minimum_cost);
//             println!("{:#?}", sub_nodes);
//             println!("desired_state {:#?}", desired_state);
//             println!("initial_conditions {:#?}", initial_conditions);
//             node_stack.append(&mut sub_nodes);
//         }
//         // After this point cheapest_node should be the leaf node to the plan with least cost, so all
//         // we have to do is work our way backwards through parents, appending to the plan as we go along.
//         let mut next_node = if let Some(next_node) = cheapest_node {
//             next_node
//         } else { // If no node was found, no plan can be generated
//             return None;
//         };
//         let mut plan = PDDLPlan {
//             actions: vec![next_node.action.clone()],
//             steps_left: 0,
//         };
//         while let Some(parent) = &next_node.parent {
//             plan.actions.push(parent.action.clone());
//         }
//         plan.steps_left = plan.actions.len();
//         Some(plan)
//     }
//     fn preconditions_satisfied(&self, preconditions: &HashMap<String, bool>, blackboard: &HashMap<String, bool>) -> bool {
//         for (name, precondition_value) in preconditions.iter() {
//             println!("{} = {:?}, {} = {}", name, blackboard.get(name), name, precondition_value);
//             if let Some(blackboard_value) = blackboard.get(name) {
                
//                 if *blackboard_value != *precondition_value {
//                     return false;
//                 }
//             } else {
//                 return false;
//             }
//         }
//         true
//     }
//     pub fn get_plan(&self, goal: &dyn PDDLGoal, blackboard: &dyn PDDLDomain) -> Option<PDDLPlan> {
//         let desired_state = goal.desired_state();
//         return self.build_plans(&desired_state, blackboard);
//     }
// }