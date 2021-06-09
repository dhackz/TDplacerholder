use crate::{
    block::Block,
    tower::*,
    monster::Monster,
    base::Base,
    gold::GoldPile,
};

pub struct Board {
    pub path_blocks: Vec<Block>,
    pub towers: Vec<NinjaTower>,
    pub monsters: Vec<Monster>,
    pub gold_piles: Vec<GoldPile>,
    pub base: Base,
}

impl Board {
    pub fn generate(seed: u64, length: u32) -> Board {
        let mut path_blocks = Vec::new();
        path_blocks.push(Block { pos: (0.0, 0.0) });
        path_blocks.push(Block { pos: (0.0, 1.0) });
        path_blocks.push(Block { pos: (0.0, 2.0) });
        path_blocks.push(Block { pos: (1.0, 2.0) });
        path_blocks.push(Block { pos: (2.0, 2.0) });
        path_blocks.push(Block { pos: (2.0, 3.0) });
        path_blocks.push(Block { pos: (2.0, 3.0) });
        path_blocks.push(Block { pos: (2.0, 4.0) });
        path_blocks.push(Block { pos: (2.0, 5.0) });
        path_blocks.push(Block { pos: (3.0, 5.0) });
        path_blocks.push(Block { pos: (4.0, 5.0) });
        path_blocks.push(Block { pos: (5.0, 5.0) });
        path_blocks.push(Block { pos: (6.0, 5.0) });
        path_blocks.push(Block { pos: (7.0, 5.0) });
        path_blocks.push(Block { pos: (8.0, 5.0) });
        path_blocks.push(Block { pos: (9.0, 5.0) });
        path_blocks.push(Block { pos: (10.0, 5.0) });
        path_blocks.push(Block { pos: (11.0, 5.0) });
        path_blocks.push(Block { pos: (12.0, 5.0) });
        path_blocks.push(Block { pos: (13.0, 5.0) });
        path_blocks.push(Block { pos: (14.0, 5.0) });
        path_blocks.push(Block { pos: (15.0, 5.0) });
        path_blocks.push(Block { pos: (16.0, 5.0) });
        path_blocks.push(Block { pos: (17.0, 5.0) });
        path_blocks.push(Block { pos: (18.0, 5.0) });
        path_blocks.push(Block { pos: (19.0, 5.0) });
        path_blocks.push(Block { pos: (20.0, 5.0) });
        path_blocks.push(Block { pos: (20.0, 6.0) });
        path_blocks.push(Block { pos: (20.0, 7.0) });
        path_blocks.push(Block { pos: (20.0, 8.0) });
        path_blocks.push(Block { pos: (20.0, 9.0) });
        path_blocks.push(Block { pos: (19.0, 9.0) });
        path_blocks.push(Block { pos: (18.0, 9.0) });
        path_blocks.push(Block { pos: (17.0, 9.0) });
        path_blocks.push(Block { pos: (16.0, 9.0) });
        path_blocks.push(Block { pos: (15.0, 9.0) });
        path_blocks.push(Block { pos: (14.0, 9.0) });
        path_blocks.push(Block { pos: (13.0, 9.0) });
        path_blocks.push(Block { pos: (12.0, 9.0) });
        path_blocks.push(Block { pos: (11.0, 9.0) });
        path_blocks.push(Block { pos: (10.0, 9.0) });
        path_blocks.push(Block { pos: (9.0, 9.0) });
        path_blocks.push(Block { pos: (8.0, 9.0) });
        path_blocks.push(Block { pos: (7.0, 9.0) });
        path_blocks.push(Block { pos: (6.0, 9.0) });
        path_blocks.push(Block { pos: (5.0, 9.0) });
        path_blocks.push(Block { pos: (4.0, 9.0) });
        path_blocks.push(Block { pos: (3.0, 9.0) });
        path_blocks.push(Block { pos: (2.0, 9.0) });

        Board {
            path_blocks,
            towers: Vec::new(),
            monsters: Vec::new(),
            gold_piles: Vec::new(),
            base: Base { pos: [0.0, 8.0] },
        }
    }
}
