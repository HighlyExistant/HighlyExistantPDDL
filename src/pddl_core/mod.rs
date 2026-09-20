use std::{collections::HashMap, fmt::Debug};
/// The parameters of an action, should be held
/// within the action itself.
pub trait PDDLAction: Debug {
    fn preconditions(&self) -> &HashMap<String, bool>;
    fn effects(&self) -> &HashMap<String, bool>;
    fn action_cost(&self) -> f32;
    fn perform(&mut self, delta: f32) -> bool;
}

/// The parameters of a goal, should be held
/// within the goal itself.
pub trait PDDLGoal: Debug {
    fn priority(&self) -> f32;
    fn desired_state(&self) -> &HashMap<String, bool>;
}

pub trait PDDLDomain {
    fn domain(&self) -> &HashMap<String, bool>;
    fn modify(&self, name: String, value: bool);
}