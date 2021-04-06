use crate::world_blocks::{WorldBlocks, Block};
use std::thread::current;

mod algorithms;
mod world_blocks;

pub trait Problem<Action> {
    fn apply(&mut self, action: &Action);
    fn restore(&mut self, action: &Action);
    fn actions(&self) -> Vec<Action>;
}

fn weighted<P, H>(epsilon: f64, h: &'static H) -> impl for<'r> Fn(&'r P) -> f64
    where H: Fn(&P) -> f64 {
    move |problem: &P| { (1.0 + epsilon) * h(problem) }
}

fn max<P>(heuristics: Vec<&'static dyn Fn(&P) -> f64>) -> impl for<'r> Fn(&'r P) -> f64 {
    move |problem: &P| {
        let mut best_score = 0.0;
        for heuristic in &heuristics {
            let score = heuristic(problem);
            if score > best_score {
                best_score = score;
            }
        }
        best_score
    }
}

fn h0(_: &WorldBlocks) -> f64 { 0.0 }

fn h1(state: &WorldBlocks) -> f64 {
    let mut cost = 0.0;
    for block in 0..state.nb_blocks() {
        if state.stack[block] != state.nb_stacks() - 1 {
            cost += 1.0;
        }
    }
    cost
}

fn h2(state: &WorldBlocks, _target: &WorldBlocks) -> f64 {
    let mut score = 0.0;

    for block in 0..state.nb_blocks() {
        if state.stack[block] != state.last_stack() {
            score += 1.0;
        } else {
            let mut next = state.under[block];
            while next != state.table() && state.under[next] == next + 1 {
                next = state.under[next];
            }
            if next != state.table() {
                score += 2.0;
            }
        }
    }

    score
}

fn custom(state: &WorldBlocks, target: &WorldBlocks) -> f64 {
    let mut score = 0.0;

    for block in 0..state.nb_blocks() {
        if state.stack[block] != target.stack[block] {
            score += 1.0;
        } else {
            let mut curr = state.under[block];
            let mut tar = target.under[block];

            while curr == tar && curr != state.table() && tar != target.table() {
                curr = state.under[curr];
                tar = state.under[tar];
            }

            if curr != state.table() || tar != target.table() {
                score += 2.0;
            }
        }
    }

    score
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();

    let nb_blocks = args
        .get(1)
        .map(|arg| arg.parse::<usize>().expect("Cannot parse nb_blocks"))
        .unwrap_or(4);

    let nb_stacks = args.get(2)
        .map(|arg| arg.parse::<usize>().expect("Cannot parse nb_stacks"))
        .unwrap_or(3);

    println!("nb_blocks: {}, nb_stacks: {}", nb_blocks, nb_stacks);
    let state = WorldBlocks::new(nb_blocks, nb_stacks);
    let solution = WorldBlocks::new_solution(nb_blocks, nb_stacks);

    println!("{:#?}", state);
    let path = algorithms::ida_star(state, &solution, custom);
    if let Some(path) = path {
        println!("Found in {} moves", path.len() - 1);
    } else {
        println!("Not found");
    }
}

