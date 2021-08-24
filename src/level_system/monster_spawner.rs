use crate::game_components::{
    board::Board,
    monsters::{chicken::Chicken, cool_chicken::CoolChicken, monster::MonsterType},
};
use crate::game_views::monsters::{ChickenView, CoolChickenView};

pub struct MonsterSpawner {
    pub spawn_schedule: Vec<(MonsterType, f32)>,
    pub elapsed_time: f32,
}

impl MonsterSpawner {
    pub fn new() -> MonsterSpawner {
        let spawn_schedule = vec![
            (MonsterType::Chicken, 0.0),
            (MonsterType::Chicken, 3.0),
            (MonsterType::Chicken, 5.0),
            (MonsterType::Chicken, 7.0),
            (MonsterType::Chicken, 8.0),
            (MonsterType::Chicken, 8.5),
            (MonsterType::Chicken, 8.6),
            (MonsterType::Chicken, 8.7),
            (MonsterType::Chicken, 8.8),
            (MonsterType::CoolChicken, 14.0),
        ];
        MonsterSpawner {
            spawn_schedule,
            elapsed_time: 0.0,
        }
    }

    pub fn update(&mut self, elapsed: f32, board: &mut Board) {
        self.elapsed_time += elapsed;

        for i in 0..self.spawn_schedule.len() {
            if self.spawn_schedule[i].1 < self.elapsed_time {
                if self.spawn_schedule[i].0 == MonsterType::Chicken {
                    board.monster_views.push(Box::new(ChickenView {
                        chicken: Chicken::new(),
                    }));
                } else if self.spawn_schedule[i].0 == MonsterType::CoolChicken {
                    board.monster_views.push(Box::new(CoolChickenView {
                        cool_chicken: CoolChicken::new(),
                    }));
                }

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
