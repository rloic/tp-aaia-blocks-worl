use crate::Problem;
use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::io::{stdout, Write};

pub fn ida_star<State, Action, Heuristic>(mut state: State, goal_state: &State, heuristic: Heuristic) -> Option<Vec<State>>
    where State: Problem<Action> + Eq + Clone,
          Action: Clone,
          Heuristic: Fn(&State, &State) -> f64 {
    let mut nub: f64 = heuristic(&state, goal_state);
    let mut path = vec![state.clone()];
    let mut nb_visited_nodes = 0;

    loop {
        let ub = nub;
        nub = f64::INFINITY;

        print!("upper bound: {}", ub);
        let _ = stdout().flush();
        let res = search(&mut state, goal_state, &heuristic, &mut path, ub, &mut nub, &mut nb_visited_nodes);
        println!(" ; nb_visited_state: {}", nb_visited_nodes);

        if res.is_some() {
            return res;
        }
        if nub == f64::INFINITY {
            return None;
        }
    }
}

fn search<State, Action, Heuristic>(
    state: &mut State,
    goal_state: &State,
    heuristic: &Heuristic,
    path: &mut Vec<State>,
    ub: f64,
    nub: &mut f64,
    nb_visited_state: &mut usize,
) -> Option<Vec<State>>
    where State: Problem<Action> + Eq + Clone,
          Action: Clone,
          Heuristic: Fn(&State, &State) -> f64
{
    *nb_visited_state += 1;

    if state == goal_state {
        return Some(path.clone());
    }

    let mut neighbors = BinaryHeap::new();
    for action in state.actions() {
        state.apply(&action);
        if !path.contains(state) {
            neighbors.push(Edge::new(action.clone(),heuristic(state, goal_state)));
        }
        state.restore(&action);
    }

    let current_cost = (path.len() - 1) as f64;
    while let Some(Edge { action, heuristic_cost }) = neighbors.pop() {
        let f = current_cost + 1.0 + heuristic_cost;
        if f > ub {
            if f < *nub {
                *nub = f;
            }
        } else {
            state.apply(&action);
            path.push(state.clone());
            let res = search(state, goal_state, heuristic, path, ub, nub, nb_visited_state);
            path.pop();
            state.restore(&action);
            if res.is_some() { return res; }
        }
    }

    None
}

struct Edge<A> {
    action: A,
    heuristic_cost: f64
}

impl <A> Edge<A> {
    fn new(action: A, cost: f64) -> Self {
        Edge { action, heuristic_cost: cost }
    }
}

impl <A: Clone> Clone for Edge<A> {
    fn clone(&self) -> Self {
        Edge {
            action: self.action.clone(),
            heuristic_cost: self.heuristic_cost.clone()
        }
    }
}

impl <A> Eq for Edge<A> {}

impl <A> PartialEq for Edge<A> {
    fn eq(&self, other: &Self) -> bool {
        self.heuristic_cost.eq(&other.heuristic_cost)
    }
}

impl <A> Ord for Edge<A> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.heuristic_cost.partial_cmp(&other.heuristic_cost).unwrap()
    }
}

impl <A> PartialOrd for Edge<A> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.heuristic_cost.partial_cmp(&other.heuristic_cost)
    }
}

