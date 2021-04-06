use crate::world_blocks::{WorldBlocks, Block};
use std::thread::current;

mod algorithms;
mod world_blocks;

pub trait Problem<Action> {
    fn apply(&mut self, action: &Action);
    fn restore(&mut self, action: &Action);
    fn actions(&self) -> Vec<Action>;
}

fn h0(_current_state: &WorldBlocks, _goal_state: &WorldBlocks) -> f64 { 0.0 }

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
    let goal = WorldBlocks::new_solution(nb_blocks, nb_stacks);

    println!("{:#?}", state);
    let path = algorithms::ida_star(state, &goal, h0);
    if let Some(path) = path {
        println!("Found in {} moves", path.len() - 1);
    } else {
        println!("Not found");
    }
}

