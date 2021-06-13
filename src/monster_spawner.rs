use crate::{monster::Monster, Board};

pub struct MonsterSpawner {
    pub spawn_schedule: Vec<f32>,
    pub elapsed_time: f32,
}

impl MonsterSpawner {
    pub fn new() -> MonsterSpawner {
        let spawn_schedule = vec![0.0, 3.0, 5.0, 7.0, 8.0, 8.5, 8.6, 8.7, 8.8];
        MonsterSpawner {
            spawn_schedule,
            elapsed_time: 0.0,
        }
    }

    pub fn update(&mut self, elapsed: f32, board: &mut Board) {
        self.elapsed_time += elapsed;

        for i in 0..self.spawn_schedule.len() {
            if self.spawn_schedule[i] < self.elapsed_time {
                board.monsters.push(Monster::new_basic_monster());
                if i == self.spawn_schedule.len() - 1 {
                    self.spawn_schedule = vec![];
                }
            } else {
                self.spawn_schedule = self.spawn_schedule.split_off(i);
                break; // Schedule is cronological, no reason to check further.
            }
        }
    }
}
