use crate::Problem;
use std::fmt::{Debug, Formatter, Write};
use std::fmt;

pub type Stack = usize;
pub type Block = usize;

#[derive(Clone, Eq, PartialEq)]
pub struct WorldBlocks {
    pub stack: Vec<Stack>,
    pub under: Vec<Block>,
    pub top: Vec<Block>,
}

#[derive(Clone)]
pub struct WorldBlocksMove { from: Stack, to: Stack }

impl WorldBlocks {
    pub fn new(nb_blocks: usize, nb_stacks: usize) -> Self {
        let mut top: Vec<Block> = vec![nb_blocks; nb_stacks];
        let mut stack: Vec<Stack> = vec![0; nb_blocks];
        let mut under: Vec<Block> = vec![0; nb_blocks];
        for block in 0..nb_blocks {
            let s = block % nb_stacks;
            stack[block] = s;
            under[block] = top[s];
            top[s] = block as Block;
        }
        WorldBlocks { stack, under, top }
    }

    pub fn new_solution(nb_blocks: usize, nb_stacks: usize) -> Self {
        let mut top: Vec<Block> = vec![nb_blocks; nb_stacks];
        let mut stack: Vec<Stack> = vec![0; nb_blocks];
        let mut under: Vec<Block> = vec![0; nb_blocks];
        top[nb_stacks - 1] = 0;
        for block in 0..nb_blocks {
            stack[block] = nb_stacks - 1;
            under[block] = block + 1;
        }
        WorldBlocks { stack, under, top }
    }

    #[inline]
    pub fn table(&self) -> Block {
        self.nb_blocks()
    }

    #[inline]
    pub fn nb_blocks(&self) -> usize {
        self.under.len()
    }

    #[inline]
    pub fn nb_stacks(&self) -> usize {
        self.top.len()
    }

    #[inline]
    pub fn last_stack(&self) -> Stack {
        self.nb_stacks() - 1
    }

    pub fn stack_len(&self, stack: Stack) -> usize {
        let mut block = self.top[stack];
        let mut len = 0;
        while block != self.table() {
            len += 1;
            block = self.under[block];
        }
        len
    }

    #[inline]
    pub fn is_not_empty_stack(&self, stack: Stack) -> bool {
        self.top[stack] != self.table()
    }

    #[inline]
    pub fn is_empty_stack(&self, stack: Stack) -> bool {
        self.top[stack] == self.table()
    }
}

impl Debug for WorldBlocks {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        fn display_block_stack(f: &mut Formatter<'_>, s: &WorldBlocks, block: Block, next: &Vec<Block>) -> fmt::Result {
            if block != s.table() {
                display_block_stack(f, s, next[block], next)?;
                f.write_fmt(format_args!("{} ", block))?;
            }
            Ok(())
        }

        for stack in 0..self.nb_stacks() {
            f.write_fmt(format_args!("stack {}: ", stack))?;
            display_block_stack(f, self, self.top[stack], &self.under)?;
            f.write_char('\n')?;
        }

        Ok(())
    }
}

impl Problem<WorldBlocksMove> for WorldBlocks {
    fn apply(&mut self, action: &WorldBlocksMove) {
        let b1 = self.top[action.from];
        let bb1 = self.under[b1];
        let b2 = self.top[action.to];
        self.under[b1] = b2;
        self.top[action.to] = b1;
        self.top[action.from] = bb1;
        self.stack[b1] = action.to;
    }

    #[inline]
    fn restore(&mut self, action: &WorldBlocksMove) {
        let reverse = WorldBlocksMove { from: action.to, to: action.from };
        self.apply(&reverse)
    }

    fn actions(&self) -> Vec<WorldBlocksMove> {
        let mut actions = Vec::with_capacity(self.nb_stacks() * (self.nb_stacks() - 1));
        for from in 0..self.nb_stacks() {
            for to in from + 1..self.nb_stacks() {
                if !self.is_empty_stack(from) {
                    actions.push(WorldBlocksMove { from, to });
                }
                if !self.is_empty_stack(to) {
                    actions.push(WorldBlocksMove { from: to, to: from });
                }
            }
        }
        actions
    }
}

