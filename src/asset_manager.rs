use ggez::{audio, graphics, Context};

pub struct AssetManager {
    pub tower_sprite: graphics::Image,
    pub tower_ninja_sprite: graphics::Image,
    pub tower_attack_sound: audio::Source,
    pub ninja_tower_strong_attack_sound: audio::Source,
    pub chicken_sprite: graphics::Image,
    pub cool_chicken_sprite: graphics::Image,
    pub monster_hurt_sound: audio::Source,
    pub gold_sprite: graphics::Image,
    pub gold_sound: audio::Source,
    pub base_sprite: graphics::Image,
}

impl AssetManager {
    pub fn new(ctx: &mut Context) -> AssetManager {
        AssetManager {
            tower_sprite: graphics::Image::new(ctx, "/tower2.png").unwrap(),
            tower_ninja_sprite: graphics::Image::new(ctx, "/tower_ninja.png").unwrap(),
            tower_attack_sound: audio::Source::new(ctx, "/tower_attack_pop.ogg").unwrap(),
            ninja_tower_strong_attack_sound: audio::Source::new(ctx, "/NinjaTowerStrongAttack.mp3")
                .unwrap(),
            chicken_sprite: graphics::Image::new(ctx, "/chicken.png").unwrap(),
            cool_chicken_sprite: graphics::Image::new(ctx, "/cool_chicken.png").unwrap(),
            monster_hurt_sound: audio::Source::new(ctx, "/chicken_hurt.ogg").unwrap(),
            gold_sprite: graphics::Image::new(ctx, "/gold_pile.png").unwrap(),
            gold_sound: audio::Source::new(ctx, "/gold.ogg").unwrap(),
            base_sprite: graphics::Image::new(ctx, "/base.png").unwrap(),
        }
    }
}
