use crate::NonUniformProblem;
use std::io::{stdout, Write};
use crate::algorithms::{Cost, best_cost_first, WeightedAction};
use std::fmt::Debug;
use std::option::Option;

pub fn ida_star<State, C, Action, Predicate, Heuristic>(mut state: State, is_goal: Predicate, heuristic: Heuristic, is_strong_heuristic: bool) -> Option<Vec<State>>
    where State: NonUniformProblem<Action, C> + Eq + Clone,
          C: Cost + Copy + Debug,
          Action: Copy,
          Predicate: Fn(&State) -> bool,
          Heuristic: Fn(&State) -> C {
    let mut next_upper_bound = Some(heuristic(&state));
    let mut path = vec![state.clone()];
    let mut nb_visited_nodes = 0;

    loop {
        let upper_bound = next_upper_bound;
        next_upper_bound = None;

        print!("upper bound: {:?}", upper_bound);
        let _ = stdout().flush();
        let res = if is_strong_heuristic {
            strong_search(&mut state, C::best(), &is_goal, &heuristic, &mut path, &upper_bound, &mut next_upper_bound, &mut nb_visited_nodes)
        } else {
            weak_search(&mut state, C::best(), &is_goal, &heuristic, &mut path, &upper_bound, &mut next_upper_bound, &mut nb_visited_nodes)
        };
        println!(" ; nb_visited_state: {}", nb_visited_nodes);

        if res.is_some() {
            return res;
        }
        if let None = next_upper_bound {
            return None;
        }
    }
}

fn strong_search<State, C, Action, Predicate, Heuristic>(
    state: &mut State,
    cost: C,
    is_goal: &Predicate,
    heuristic: &Heuristic,
    path: &mut Vec<State>,
    upper_bound: &Option<C>,
    next_upper_bound: &mut Option<C>,
    nb_visited_state: &mut usize,
) -> Option<Vec<State>>
    where State: NonUniformProblem<Action, C> + Eq + Clone,
          C: Cost + Copy + Debug,
          Action: Copy,
          Predicate: Fn(&State) -> bool,
          Heuristic: Fn(&State) -> C
{
    *nb_visited_state += 1;

    if is_goal(state) {
        return Some(path.clone());
    }

    let mut actions: Vec<(WeightedAction<Action, C>, C)> = Vec::new();
    for weighted_action in &state.weighted_actions() {
        state.apply(&weighted_action.action);
        if !path.contains(state) {
            actions.push((weighted_action.clone(), heuristic(state)));
        }
        state.restore(&weighted_action.action);
    }
    actions.sort_by(|lhs, rhs| best_cost_first(&lhs.1, &rhs.1));

    for (WeightedAction { action, cost: action_cost }, heuristic_cost) in &actions {
        let g = cost.aggregate(action_cost);
        let f = Some(g.aggregate(&heuristic_cost));
        if f.is_worst_than(&upper_bound) {
            if f.is_better_than(next_upper_bound) {
                *next_upper_bound = f;
            }
        } else {
            state.apply(action);
            path.push(state.clone());
            let result = strong_search(state, g, is_goal, heuristic, path, upper_bound, next_upper_bound, nb_visited_state);
            path.pop();
            state.restore(action);
            if result.is_some() { return result; }
        }
    }

    None
}

fn weak_search<State, C, Action, Predicate, Heuristic>(
    state: &mut State,
    cost: C,
    is_goal: &Predicate,
    heuristic: &Heuristic,
    path: &mut Vec<State>,
    upper_bound: &Option<C>,
    next_upper_bound: &mut Option<C>,
    nb_visited_state: &mut usize,
) -> Option<Vec<State>>
    where State: NonUniformProblem<Action, C> + Eq + Clone,
          C: Cost + Copy + Debug,
          Action: Copy,
          Predicate: Fn(&State) -> bool,
          Heuristic: Fn(&State) -> C
{
    *nb_visited_state += 1;

    if is_goal(state) {
        return Some(path.clone());
    }

    for weighted_action in &state.weighted_actions() {
        state.apply(&weighted_action.action);
        if !path.contains(state) {
            let g = cost.aggregate(&weighted_action.cost);
            let heuristic_cost = heuristic(&state);
            let f = Some(g.aggregate(&heuristic_cost));
            if f.is_worst_than(&upper_bound) {
                if f.is_better_than(next_upper_bound) {
                    *next_upper_bound = f;
                }
            } else {
                path.push(state.clone());
                let result = weak_search(state, g, is_goal, heuristic, path, upper_bound, next_upper_bound, nb_visited_state);
                path.pop();
                if result.is_some() { return result; }
            }
        }
        state.restore(&weighted_action.action);
    }
    None
}