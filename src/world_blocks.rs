use crate::NonUniformProblem;
use std::fmt::{Debug, Formatter, Write};
use std::fmt;
use std::ops::Range;
use crate::algorithms::{Cost, WeightedAction};

pub type Stack = usize;
pub type Stacks = Range<Stack>;
pub type Block = usize;
pub type Blocks = Range<Block>;

#[derive(Clone, Eq, Hash)]
pub struct WorldBlocks {
    pub stack: Vec<Stack>,
    pub next: Vec<Option<Block>>,
    pub top: Vec<Option<Block>>,
}

impl PartialEq for WorldBlocks {
    fn eq(&self, other: &Self) -> bool {
        assert_eq!(self.nb_blocks(), other.nb_blocks());
        assert_eq!(self.nb_stacks(), other.nb_stacks());
        for block in self.blocks() {
            if self.stack[block as usize] != other.stack[block as usize] {
                return false;
            }
            if self.next[block as usize] != other.next[block as usize] {
                return false;
            }
        }
        true
    }
}

#[derive(Copy, Clone)]
pub struct Move { from: Stack, to: Stack }

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct NbMoves(u32);

impl NbMoves {
    pub fn new(nb_moves: u32) -> Self {
        NbMoves(nb_moves)
    }
}

impl Cost for NbMoves {
    fn best() -> Self {
        NbMoves(0)
    }

    fn is_better_than(&self, other: &Self) -> bool {
        self.0 < other.0
    }

    fn is_worst_than(&self, other: &Self) -> bool {
        self.0 > other.0
    }

    fn aggregate(&self, other: &Self) -> Self {
        NbMoves(self.0 + other.0)
    }
}

impl WorldBlocks {
    pub fn initial(nb_blocks: usize, nb_stacks: usize) -> Self {
        let mut top = vec![None; nb_stacks];
        let mut stack = Vec::with_capacity(nb_blocks);
        let mut under = vec![None; nb_blocks];
        for block in 0..nb_blocks as Block {
            let s = block % nb_stacks as Stack;
            stack.push(s);
            under[block as usize] = top[s as usize];
            top[s as usize] = Some(block);
        }
        WorldBlocks { stack, next: under, top }
    }

    pub fn nb_blocks(&self) -> usize {
        self.next.len()
    }

    pub fn nb_stacks(&self) -> usize {
        self.top.len()
    }

    pub fn stacks(&self) -> Stacks {
        0..self.nb_stacks() as Stack
    }

    pub fn is_not_empty_stack(&self, stack: Stack) -> bool {
        self.top[stack as usize].is_some()
    }

    #[allow(dead_code)]
    pub fn is_empty_stack(&self, stack: Stack) -> bool {
        self.top[stack as usize].is_none()
    }

    pub fn blocks(&self) -> Blocks {
        0..self.nb_blocks() as Block
    }
}

impl Debug for WorldBlocks {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        fn display_block_stack(f: &mut Formatter<'_>, s: &WorldBlocks, block: Option<Block>, next: &Vec<Option<Block>>) -> fmt::Result {
            if let Some(block) = block {
                display_block_stack(f, s, next[block as usize], next)?;
                f.write_fmt(format_args!("{} ", block))?;
            }
            Ok(())
        }

        for stack in 0..self.nb_stacks() {
            f.write_fmt(format_args!("stack {}: ", stack))?;
            display_block_stack(f, self, self.top[stack], &self.next)?;
            f.write_char('\n')?;
        }

        Ok(())
    }
}

impl NonUniformProblem<Move, NbMoves> for WorldBlocks {
    fn apply(&mut self, action: &Move) {
        if let Some(moved_block) = self.top[action.from as usize] {
            // The new top of the stack from is the block under the moved_block
            let new_top = self.next[moved_block as usize];
            self.top[action.from as usize] = new_top;

            // Move the block to it new position
            self.next[moved_block as usize] = self.top[action.to as usize];
            self.top[action.to as usize] = Some(moved_block);
            self.stack[moved_block as usize] = action.to;
        } else {
            panic!("Try to pick a block on an empty stack");
        }
    }

    fn restore(&mut self, action: &Move) {
        let reverse = Move { from: action.to, to: action.from };
        self.apply(&reverse)
    }

    fn weighted_actions(&self) -> Vec<WeightedAction<Move, NbMoves>> {
        let mut actions = Vec::with_capacity(self.nb_stacks() * (self.nb_stacks() - 1));
        for lhs in 0..self.nb_stacks() as Stack {
            for rhs in lhs + 1..self.nb_stacks() as Stack {
                if !self.is_empty_stack(lhs) {
                    actions.push(WeightedAction::new(Move { from: lhs, to: rhs }, NbMoves(1)));
                }
                if !self.is_empty_stack(rhs) {
                    actions.push(WeightedAction::new(Move { from: rhs, to: lhs }, NbMoves(1)));
                }
            }
        }
        actions
    }
}

