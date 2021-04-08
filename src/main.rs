use crate::world_blocks::{WorldBlocks, Stack, Block, NbMoves};
use algorithms::ida_star::ida_star;
use algorithms::NonUniformProblem;

mod algorithms;
mod world_blocks;

#[allow(dead_code)]
fn h0(_current_state: &WorldBlocks) -> NbMoves { NbMoves::new(0) }

#[allow(dead_code)]
fn h1(current_state: &WorldBlocks) -> NbMoves {
    let mut score = 0;

    for block in current_state.blocks() {
        if current_state.stack[block as usize] != current_state.nb_stacks() as Stack - 1 {
            score += 1;
        }
    }

    NbMoves::new(score)
}

#[allow(dead_code)]
fn h2(current_state: &WorldBlocks) -> NbMoves {
    let mut score = 0;

    for block in current_state.blocks() {
        if current_state.stack[block as usize] != current_state.nb_stacks() as Stack - 1 {
            score += 1;
        } else {
            let mut block = block;

            while Some(block + 1) == current_state.next[block as usize] { block = block + 1; }

            if current_state.next[block as usize].is_some() || block != current_state.nb_blocks() as Block - 1 {
                score += 2;
            }
        }
    }

    NbMoves::new(score)
}

#[allow(dead_code)]
fn h4(current_state: &WorldBlocks) -> NbMoves {
    let mut score = 0;

    for block in current_state.blocks() {
        if current_state.stack[block as usize] != current_state.nb_stacks() as Stack - 1 {
            score += 1;
            let mut next = current_state.next[block as usize];
            while next.is_some() && next.unwrap() < block {
                next = current_state.next[next.unwrap() as usize];
            }
            if next.is_some() {
                score += 1;
            }
        } else {
            let mut block = block;

            while Some(block + 1) == current_state.next[block as usize] {
                block = block + 1;
            }

            if current_state.next[block as usize].is_some() || block != current_state.nb_blocks() as Block - 1 {
                score += 2;
            }
        }
    }

    NbMoves::new(score)
}

fn is_goal(state: &WorldBlocks) -> bool {
    for stack in 0..state.nb_stacks() as Stack - 1 {
        if state.is_not_empty_stack(stack) {
            return false;
        }
    }

    if state.top[state.nb_stacks() - 1] != Some(0) {
        return false;
    }

    for block in 0..state.nb_blocks() as Block - 1 {
        if state.stack[block as usize] != state.nb_stacks() as Block - 1 { return false; }
        if state.next[block as usize] != Some(block + 1) {
            return false;
        }
    }

    state.next[state.nb_blocks() - 1] == None
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();

    let nb_blocks = args
        .get(1)
        .map(|arg| arg.parse::<usize>().expect("Cannot parse nb_blocks"))
        .unwrap_or(20);

    let nb_stacks = args.get(2)
        .map(|arg| arg.parse::<usize>().expect("Cannot parse nb_stacks"))
        .unwrap_or(4);

    println!("nb_blocks: {}, nb_stacks: {}", nb_blocks, nb_stacks);
    let state = WorldBlocks::initial(nb_blocks, nb_stacks);

    println!("{:#?}", state);
    let path = ida_star(state, is_goal, h4, false);
    if let Some(path) = path {
        println!("Found in {} moves", path.len() - 1);
    } else {
        println!("Not found");
    }
}

