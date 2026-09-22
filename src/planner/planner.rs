use std::{cell::RefCell, collections::{HashMap, HashSet}, fmt::Debug, sync::Arc};

use crate::{strips_condition::StripsPredicate, strips_core::{StripsAction, StripsDomain, StripsGoal}, planner::StripsPlan};

type StripsState<Predicate> = HashMap<Predicate, bool>;


fn build_plan<Predicate: StripsPredicate>(node: Option<Arc<StripsPlanNode<Predicate>>>) -> Option<StripsPlan<Predicate>> {
    let mut next_node = if let Some(next_node) = node {
        next_node
    } else { // If no node was found, no plan can be generated
        return None;
    };
    let mut plan = StripsPlan {
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

struct StripsPlanNode<Predicate: StripsPredicate> {
    parent: Option<Arc<StripsPlanNode<Predicate>>>,
    action: Arc<dyn StripsAction<Predicate>>,
    cost: f32,
    desired_state: RefCell<StripsState<Predicate>>,
}

impl<Predicate: StripsPredicate> StripsPlanNode<Predicate> {
    pub fn new(parent: Option<Arc<StripsPlanNode<Predicate>>>, action: Arc<dyn StripsAction<Predicate>>) -> Self {
        let (cost, desired_state) = if let Some(parent) = &parent {
            let desired_state = parent.desired_state.clone();
            
            Self::erase_effects(&desired_state, action.effects());
            
            (parent.cost + action.action_cost(), desired_state)
        } else {
            (action.action_cost(), RefCell::new(HashMap::new()))
        };
        {
            let mut borrowed = desired_state.borrow_mut();
            for precondition in action.preconditions() {
                borrowed.insert(precondition.0.clone(), precondition.1.clone());
            }
        }
        
        Self { 
            parent, 
            action, 
            cost, 
            desired_state
        }
    }
    fn erase_effects(desired_state: &RefCell<StripsState<Predicate>>, effects: &StripsState<Predicate>) {
        let mut borrowed = desired_state.borrow_mut();
        // If a desired value is reached, we no longer have to look for it
        // and therefore we remove it from the list.
        for predicate in effects {
            let desired_value = if let Some(desired_value) = borrowed.get(predicate.0).cloned() {
                desired_value
            } else {
                continue
            };
            if desired_value == *predicate.1 {
                borrowed.remove(predicate.0);
            }
        }
    }
    pub fn erase(&self, predicate: &Predicate) {
        let mut state = self.desired_state.borrow_mut();
        let _ = state.remove(&predicate);
    }
}
impl<Predicate: StripsPredicate + Debug> Debug for StripsPlanNode<Predicate> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PlanNode")
            .field("action", &self.action)
            .finish()
    }
}

pub struct StripsPlanner<Predicate: StripsPredicate> {
    actions: Vec<Arc<dyn StripsAction<Predicate>>>,
}

impl<Predicate: StripsPredicate> StripsPlanner<Predicate> {
    pub fn new(actions: Vec<Arc<dyn StripsAction<Predicate>>>) -> Self {
        Self {
            actions,
            // factories: vec![]
        }
    }
    fn has_condition(effects: &StripsState<Predicate>, desired_state: &StripsState<Predicate>) -> bool {
        for effect in effects {
            if let Some(value) = desired_state.get(effect.0) {
                if *value ==  *effect.1 {
                    return true
                }
            }
        }
        false
    }
    /// We assume everything that is not in the HashSet is false, and everything that is in
    /// the HashSet is true.
    fn erase_effects(desired_state: &RefCell<HashMap<Predicate, bool>>, effects: &HashSet<Predicate>) {
        let mut borrowed = desired_state.borrow_mut();
        // If a desired value is reached, we no longer have to look for it
        // and therefore we remove it from the list.
        borrowed.retain(|p, add_del| {
            effects.contains(p) != *add_del
        });
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
    fn possible_actions(&self, parent: Option<Arc<StripsPlanNode<Predicate>>>, goal: &dyn StripsGoal<Predicate>) -> Vec<Arc<StripsPlanNode<Predicate>>> {
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
                let plan = Arc::new(StripsPlanNode::new(
                    parent.clone(), 
                    action.clone(), 
                ));
                possible.push(plan);
            }
        }
        
        possible
    }
    pub fn build_plan(&self, goal: &dyn StripsGoal<Predicate>, domain: &dyn StripsDomain<Predicate>) -> Option<StripsPlan<Predicate>> {
        let initial_conditions = domain.predicates();
        // Get actions which will satisfy the initial goal
        // ignoring whether the preconditions are satisfied
        let mut possible_plans = self.possible_actions(None, goal);

        // Create a node_stack to store nodes for processing
        let mut node_stack: Vec<Arc<StripsPlanNode<Predicate>>> = vec![];
        node_stack.append(&mut possible_plans);

        let mut cheapest_cost = f32::INFINITY;
        let mut cheapest_plan: Option<StripsPlan<Predicate>> = None;

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
            StripsPlanner::erase_effects(&node.desired_state, initial_conditions);

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