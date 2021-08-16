use crate::utils::Scale;
use crate::{
    asset_manager::AssetManager, block::BLOCK_SIZE, gold::GoldPile, monsters::monster::Monster,
    towers::tower::Tower,
};

use ggez::graphics::DrawParam;
use ggez::{audio::SoundSource, graphics, Context, GameResult};

pub struct BasicTower {
    pub position: [f32; 2],
    pub attack_cooldown: f32,
}

impl BasicTower {
    pub const ATTACK_RANGE: f32 = 100.0; // Pixels.
    pub const ATTACK_TIMER: f32 = 1.0; // Seconds.
    pub const DAMAGE: f32 = 10.0;

    pub fn new(position: [f32; 2]) -> BasicTower {
        BasicTower {
            position,
            attack_cooldown: 0.0,
        }
    }

    pub fn get_center_pos_abs(&self) -> [f32; 2] {
        [
            self.position[0] * BLOCK_SIZE + BLOCK_SIZE / 2.0,
            self.position[1] * BLOCK_SIZE + BLOCK_SIZE / 2.0,
        ]
    }

    fn position_is_in_attack_range(&self, position_abs: [f32; 2]) -> bool {
        let tower_center_pos_abs = self.get_center_pos_abs();
        debug!(
            "position_is_in_attack_range: position_abs ({:?}), tower_center_pos_abs ({:?}).",
            position_abs, tower_center_pos_abs
        );

        let dx = tower_center_pos_abs[0] - position_abs[0];
        let dy = tower_center_pos_abs[1] - position_abs[1];

        dx * dx + dy * dy < BasicTower::ATTACK_RANGE * BasicTower::ATTACK_RANGE
    }

    fn draw_attack(
        &mut self,
        ctx: &mut Context,
        scale: Scale,
        from_abs: [f32; 2],
        to_abs: [f32; 2],
    ) -> GameResult {
        debug!(
            "draw_attack: from_abs ({:?}), to_abs ({:?}).",
            from_abs, to_abs
        );

        if from_abs == to_abs {
            // Early exit, nothing to draw.
            return Ok(());
        }

        let _from_abs = scale.to_viewport_point(from_abs[0], from_abs[1]);
        let _to_abs = scale.to_viewport_point(to_abs[0], to_abs[1]);

        let line = graphics::Mesh::new_line(
            ctx,
            &[_from_abs, _to_abs],
            3.0,
            graphics::Color::new(0.0, 1.0, 1.0, 1.0),
        )?;

        let location = (ggez::mint::Point2 { x: 0.0, y: 0.0 },);

        graphics::draw(ctx, &line, location)?;
        Ok(())
    }
}

impl Tower for BasicTower {
    fn draw(
        &mut self,
        ctx: &mut Context,
        scale: Scale,
        asset_manager: &AssetManager,
    ) -> GameResult {
        let location = scale.to_viewport_point(
            self.position[0] * BLOCK_SIZE - 5.0,
            self.position[1] * BLOCK_SIZE - 35.0,
        );

        graphics::draw(
            ctx,
            &asset_manager.tower_assets.tower_sprite,
            DrawParam::default()
                .scale([scale.x, scale.y])
                .dest(location),
        )?;

        Ok(())
    }

    fn draw_abilities(
        &mut self,
        ctx: &mut Context,
        scale: Scale,
        monsters: &Vec<Box<dyn Monster>>,
    ) -> GameResult {
        for monster in monsters.iter() {
            let monster_center = monster.get_center_pos_abs();

            if self.position_is_in_attack_range(monster_center) {
                self.draw_attack(ctx, scale, self.get_center_pos_abs(), monster_center)?;
            }
        }
        Ok(())
    }

    fn update(
        &mut self,
        elapsed: f32,
        monsters: &mut Vec<Box<dyn Monster>>,
        gold_piles: &mut Vec<GoldPile>,
        asset_manager: &mut AssetManager,
    ) {
        debug!(
            "update: elapsed ({}), monsters length ({}), gold_piles length ({}).",
            elapsed,
            monsters.len(),
            gold_piles.len()
        );
        self.attack_cooldown -= elapsed;

        if self.attack_cooldown < 0.0 {
            self.attack_cooldown = 0.0;
        }

        if self.attack_cooldown == 0.0 {
            let mut damage_dealt = false;
            for monster in monsters.iter_mut() {
                if self.position_is_in_attack_range(monster.get_center_pos_abs()) {
                    damage_dealt = true;
                    monster.recieve_damage(BasicTower::DAMAGE, gold_piles, asset_manager);
                }
            }
            if damage_dealt {
                info!("update: attacked at least one monster! Playing attack soundeffect.");
                asset_manager
                    .tower_assets
                    .tower_attack_sound
                    .play()
                    .unwrap();
                self.attack_cooldown = BasicTower::ATTACK_TIMER;
            }
        }
    }

    fn get_block_position(&self) -> [f32; 2] {
        self.position
    }
}
