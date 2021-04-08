use std::cmp::Ordering;

pub mod ida_star;

pub trait UniformProblem<Action> {
    fn apply(&mut self, action: &Action);
    fn restore(&mut self, action: &Action);
    fn weighted_actions(&self) -> Vec<Action>;
}

pub trait NonUniformProblem<Action, Cost> {
    fn apply(&mut self, action: &Action);
    fn restore(&mut self, action: &Action);
    fn weighted_actions(&self) -> Vec<WeightedAction<Action, Cost>>;
}

pub struct WeightedAction<A, C> {
    pub action: A,
    pub cost: C
}

impl <A, C> WeightedAction<A, C> {
    pub fn new(action: A, cost: C) -> Self {
        WeightedAction { action, cost }
    }
}

impl <A: Clone, C: Clone> Clone for  WeightedAction<A, C> {
    fn clone(&self) -> Self {
        WeightedAction {
            action: self.action.clone(),
            cost: self.cost.clone()
        }
    }
}

pub trait Cost {
    fn best() -> Self;
    fn is_better_than(&self, other: &Self) -> bool;
    fn is_worst_than(&self, other: &Self) -> bool;
    fn aggregate(&self, other: &Self) -> Self;
}

impl<C: Cost> Cost for Option<C> {
    fn best() -> Self {
        Some(C::best())
    }

    fn is_better_than(&self, other: &Self) -> bool {
        match (self, other) {
            (Option::None, _) => false,
            (Option::Some(_), Option::None) => true,
            (Option::Some(lhs), Option::Some(rhs)) => lhs.is_better_than(rhs)
        }
    }

    fn is_worst_than(&self, other: &Self) -> bool {
        match (self, other) {
            (Option::None, _) => true,
            (Option::Some(_), Option::None) => false,
            (Option::Some(lhs), Option::Some(rhs)) => lhs.is_worst_than(rhs)
        }
    }

    fn aggregate(&self, other: &Self) -> Self {
        match (self, other) {
            (Some(lhs), Some(rhs)) => Some(lhs.aggregate(rhs)),
            _ => None
        }
    }
}

#[allow(dead_code)]
fn best_cost_first<C: Cost> (lhs: &C, rhs: &C) -> Ordering {
    if lhs.is_better_than(&rhs) {
        Ordering::Less
    } else if lhs.is_worst_than(&rhs) {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}

#[allow(dead_code)]
fn worst_cost_first<C: Cost> (lhs: &C, rhs: &C) -> Ordering {
    if rhs.is_better_than(&lhs) {
        Ordering::Less
    } else if rhs.is_worst_than(&lhs) {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}