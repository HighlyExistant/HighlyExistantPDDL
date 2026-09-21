use std::{cell::RefCell, collections::HashMap, fmt::Debug, sync::Arc};

use crate::{pddl_condition::{PDDLPredicate, PDDLPredicateName}, pddl_core::{PDDLAction, PDDLDomain, PDDLGoal}, planner::PDDLPlan};

type PDDLState<Name: PDDLPredicateName> = HashMap<Name, bool>;


fn build_plan<Name: PDDLPredicateName>(node: Option<Arc<PDDLPlanNode<Name>>>) -> Option<PDDLPlan<Name>> {
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

struct PDDLPlanNode<Name: PDDLPredicateName> {
    parent: Option<Arc<PDDLPlanNode<Name>>>,
    action: Arc<dyn PDDLAction<Name>>,
    cost: f32,
    desired_state: RefCell<PDDLState<Name>>,
}

impl<Name: PDDLPredicateName> PDDLPlanNode<Name> {
    pub fn new(parent: Option<Arc<PDDLPlanNode<Name>>>, action: Arc<dyn PDDLAction<Name>>) -> Self {
        let (cost, desired_state) = if let Some(parent) = &parent {
            let desired_state = parent.desired_state.clone();
            
            Self::erase_effects(&desired_state, action.effects());
            
            (parent.cost + action.action_cost(), desired_state)
        } else {
            (action.action_cost(), RefCell::new(HashMap::new()))
        };
        {
            let mut borrowed = desired_state.borrow_mut();
            for (name, value) in action.preconditions() {
                borrowed.insert(name.clone(), *value);
            }
        }
        
        Self { 
            parent, 
            action, 
            cost, 
            desired_state
        }
    }
    fn erase_effects(desired_state: &RefCell<HashMap<Name, bool>>, effects: &PDDLState<Name>) {
        let mut borrowed = desired_state.borrow_mut();
        // If a desired value is reached, we no longer have to look for it
        // and therefore we remove it from the list.
        for (name, value) in effects {
            let desired_value = if let Some(desired_value) = borrowed.get(name).cloned() {
                desired_value
            } else {
                continue
            };
            if desired_value == *value {
                borrowed.remove(name);
            }
        }
    }
    pub fn erase(&self, predicate: &Name) {
        let mut state = self.desired_state.borrow_mut();
        let _ = state.remove(&predicate);
    }
}
impl<Name: PDDLPredicateName> Debug for PDDLPlanNode<Name> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PlanNode")
            .field("action", &self.action)
            .finish()
    }
}

pub struct PDDLPlanner<Name: PDDLPredicateName> {
    actions: Vec<Arc<dyn PDDLAction<Name>>>,
}

impl<Name: PDDLPredicateName> PDDLPlanner<Name> {
    pub fn new(actions: Vec<Arc<dyn PDDLAction<Name>>>) -> Self {
        Self {
            actions
        }
    }
    fn has_condition(effects: &PDDLState<Name>, desired_state: &PDDLState<Name>) -> bool {
        for (name, value) in effects {
            if desired_state.get(name).cloned().unwrap_or(false) == *value {
                return true
            }
        }
        false
    }
    fn erase_effects(desired_state: &RefCell<HashMap<Name, bool>>, effects: &HashMap<Name, Arc<dyn PDDLPredicate<PredicateName = Name>>>) {
        let mut borrowed = desired_state.borrow_mut();
        // If a desired value is reached, we no longer have to look for it
        // and therefore we remove it from the list.
        for (name, value) in effects {
            let desired_value = if let Some(desired_value) = borrowed.get(name).cloned() {
                desired_value
            } else {
                continue
            };
            if desired_value == value.eval() {
                borrowed.remove(name);
            }
        }
    }
    /// # Returns
    /// A list of actions which satisfy atleast one
    /// condition in the desired_state of the particular parent.
    /// If the parent does not exist, then it must be the root,
    /// in which case it will default to a list of actions which
    /// satisfy the goals desired_state.
    /// 
    /// This also means that if parent does exist, then the goal
    /// does not matter.
    fn possible_actions(&self, parent: Option<Arc<PDDLPlanNode<Name>>>, goal: &dyn PDDLGoal<Name>) -> Vec<Arc<PDDLPlanNode<Name>>> {
        let mut possible = vec![];

        let desired_state=  if let Some(parent) = &parent {
            // This is a bit unsafe, but we do this to ensure that
            // it has the same type as the else condition.
            unsafe { parent.desired_state.as_ptr().as_ref().unwrap() }
        } else {
            goal.desired_state()
        };
        
        for action in &self.actions {
            let effects = action.effects();
            if Self::has_condition(effects, &desired_state) {
                let plan = Arc::new(PDDLPlanNode::new(
                    parent.clone(), 
                    action.clone(), 
                ));
                possible.push(plan);
            }
        }
        
        possible
    }
    pub fn build_plan(&self, goal: &dyn PDDLGoal<Name>, domain: &dyn PDDLDomain<Name>) -> Option<PDDLPlan<Name>> {
        let initial_conditions = domain.predicates();
        // Get actions which will satisfy the initial goal
        // ignoring whether the preconditions are satisfied
        let mut possible_plans = self.possible_actions(None, goal);

        // Create a node_stack to store nodes for processing
        let mut node_stack: Vec<Arc<PDDLPlanNode<Name>>> = vec![];
        node_stack.append(&mut possible_plans);

        let mut cheapest_cost = f32::INFINITY;
        let mut cheapest_plan: Option<PDDLPlan<Name>> = None;

        // Loop through the actions recieved until the node_stack is empty
        while !node_stack.is_empty() {
            // Pop node from stack to process
            let mut node = node_stack.pop().unwrap();
            // If the node cost is higher than a solution already 
            // found, then continue
            if node.cost > cheapest_cost {
                continue;
            }

            // Erase desired_state from node which is satisfied by
            // initial conditions
            PDDLPlanner::erase_effects(&node.desired_state, initial_conditions);

            // Check if the nodes preconditions satisfy the initial 
            // domain conditions in which case set it as a possible 
            // solution. The way we check this is by seeing whether
            // the desired_state is empty.
            if node.desired_state.borrow().is_empty() {
                cheapest_cost = node.cost;
                cheapest_plan = build_plan(Some(node.clone()));
            }
            
            // Search for actions which satisfy atleast one condition 
            // in the desired state
            let mut possible_actions = self.possible_actions(Some(node.clone()), goal);
            
            // Append those actions to be further processed
            node_stack.append(&mut possible_actions);
        }
        cheapest_plan
    }
}