use crate::{
    BLOCK_SIZE,
    base::Base, block::Block, gold::GoldPile, monsters::monster::Monster, towers::tower::Tower,
};

pub struct Board {
    pub path_blocks: Vec<Block>,
    pub towers: Vec<Box<dyn Tower>>,
    pub monsters: Vec<Box<dyn Monster>>,
    pub gold_piles: Vec<GoldPile>,
    pub base: Base,
}

impl Board {
    pub fn generate(_seed: u64, _length: u32) -> Board {
        let mut path_blocks = Vec::new();
        path_blocks.push(Block { position: [0.0, 0.0] });
        path_blocks.push(Block { position: [0.0, 1.0] });
        path_blocks.push(Block { position: [0.0, 2.0] });
        path_blocks.push(Block { position: [1.0, 2.0] });
        path_blocks.push(Block { position: [2.0, 2.0] });
        path_blocks.push(Block { position: [2.0, 3.0] });
        path_blocks.push(Block { position: [2.0, 3.0] });
        path_blocks.push(Block { position: [2.0, 4.0] });
        path_blocks.push(Block { position: [2.0, 5.0] });
        path_blocks.push(Block { position: [3.0, 5.0] });
        path_blocks.push(Block { position: [4.0, 5.0] });
        path_blocks.push(Block { position: [5.0, 5.0] });
        path_blocks.push(Block { position: [6.0, 5.0] });
        path_blocks.push(Block { position: [7.0, 5.0] });
        path_blocks.push(Block { position: [8.0, 5.0] });
        path_blocks.push(Block { position: [9.0, 5.0] });
        path_blocks.push(Block { position: [10.0, 5.0] });
        path_blocks.push(Block { position: [11.0, 5.0] });
        path_blocks.push(Block { position: [12.0, 5.0] });
        path_blocks.push(Block { position: [13.0, 5.0] });
        path_blocks.push(Block { position: [14.0, 5.0] });
        path_blocks.push(Block { position: [15.0, 5.0] });
        path_blocks.push(Block { position: [16.0, 5.0] });
        path_blocks.push(Block { position: [17.0, 5.0] });
        path_blocks.push(Block { position: [18.0, 5.0] });
        path_blocks.push(Block { position: [19.0, 5.0] });
        path_blocks.push(Block { position: [20.0, 5.0] });
        path_blocks.push(Block { position: [20.0, 6.0] });
        path_blocks.push(Block { position: [20.0, 7.0] });
        path_blocks.push(Block { position: [20.0, 8.0] });
        path_blocks.push(Block { position: [20.0, 9.0] });
        path_blocks.push(Block { position: [19.0, 9.0] });
        path_blocks.push(Block { position: [18.0, 9.0] });
        path_blocks.push(Block { position: [17.0, 9.0] });
        path_blocks.push(Block { position: [16.0, 9.0] });
        path_blocks.push(Block { position: [15.0, 9.0] });
        path_blocks.push(Block { position: [14.0, 9.0] });
        path_blocks.push(Block { position: [13.0, 9.0] });
        path_blocks.push(Block { position: [12.0, 9.0] });
        path_blocks.push(Block { position: [11.0, 9.0] });
        path_blocks.push(Block { position: [10.0, 9.0] });
        path_blocks.push(Block { position: [9.0, 9.0] });
        path_blocks.push(Block { position: [8.0, 9.0] });
        path_blocks.push(Block { position: [7.0, 9.0] });
        path_blocks.push(Block { position: [6.0, 9.0] });
        path_blocks.push(Block { position: [5.0, 9.0] });
        path_blocks.push(Block { position: [4.0, 9.0] });
        path_blocks.push(Block { position: [3.0, 9.0] });
        path_blocks.push(Block { position: [2.0, 9.0] });

        Board {
            path_blocks,
            towers: Vec::new(),
            monsters: Vec::new(),
            gold_piles: Vec::new(),
            base: Base { position: [0.0, 8.0] },
        }
    }

    pub fn position_is_occupied(&self, click_position: [f32; 2]) -> bool {
        let block_position = [
            (click_position[0] / BLOCK_SIZE).floor(),
            (click_position[1] / BLOCK_SIZE).floor(),
        ];

        for tower in self.towers.iter() {
            if tower.get_block_position() == block_position {
                return true
            }
        }

        for path_block in self.path_blocks.iter() {
            if path_block.position == block_position {
                return true
            }
        }

        if self.base.is_position_in_base(click_position) {
            return true
        }

        return false
    }

    /// Special function used to ensure Towers are sorted by y position, this
    /// is required since ggez does not have z-indexing.
    pub fn add_tower(&mut self, tower: Box<dyn Tower>) {
        debug!("Trying to place new tower at position {:?}.", tower.get_block_position());

        // Find index where to insert new tower, based on sorted y position.
        let mut index = 0;
        for i in 0..self.towers.iter().len() {
            if self.towers[i].get_block_position()[1] >= tower.get_block_position()[1] {
                break;
            }
            index += 1;
        }

        debug!("New tower put at list index {}.", index);
        self.towers.insert(index, tower);
    }
}

#[cfg(test)]
mod tests{
    use crate::{
        Board,
        towers::basic_tower::BasicTower,
        towers::tower::Tower,
    };

    fn _check_towers_in_order(towers: &Vec<Box<dyn Tower>>) -> bool {
        for i in 0..towers.len()-1 {
            if towers[i].get_block_position()[1] > towers[i+1].get_block_position()[1] {
                return false
            }
        }
        return true
    }

    fn _fill_tower_positions(board: &mut Board, tower_y_positions: Vec::<f32>) {
        for y in tower_y_positions {
            board.add_tower(Box::new(BasicTower::new([0.0, y])));
        }
    }

    #[test]
    fn tower_order_0_1_2() {
        let mut board = Board::generate(0, 0);
        assert_eq!(board.towers.len(), 0);

        _fill_tower_positions(&mut board, vec![0.0, 1.0, 2.0]);
        assert_eq!(board.towers.len(), 3);

        // Check sorted on y position.
        assert!(_check_towers_in_order(&board.towers));
    }

    #[test]
    fn tower_order_2_1_0() {
        let mut board = Board::generate(0, 0);
        assert_eq!(board.towers.len(), 0);

        _fill_tower_positions(&mut board, vec![2.0, 1.0, 0.0]);
        assert_eq!(board.towers.len(), 3);

        // Check sorted on y position.
        assert!(_check_towers_in_order(&board.towers));
    }

    #[test]
    fn tower_order_2_0_1() {
        let mut board = Board::generate(0, 0);
        assert_eq!(board.towers.len(), 0);

        _fill_tower_positions(&mut board, vec![2.0, 0.0, 1.0]);
        assert_eq!(board.towers.len(), 3);

        // Check sorted on y position.
        assert!(_check_towers_in_order(&board.towers));
    }
}
