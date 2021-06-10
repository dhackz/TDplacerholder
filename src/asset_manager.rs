use ggez::{
    audio,
    graphics,
    Context,
};

pub struct AssetManager {
    pub tower_sprite: graphics::Image,
    pub tower_ninja_sprite: graphics::Image,
    pub tower_attack_sound: audio::Source,
    pub ninja_tower_strong_attack_sound: audio::Source,
    pub monster_sprite: graphics::Image,
    pub gold_sprite: graphics::Image,
    pub base_sprite: graphics::Image,
    // monster_hurt_sound: graphics::Image,
}

impl AssetManager {
    pub fn new(ctx: &mut Context) -> AssetManager {
        AssetManager {
            tower_sprite: graphics::Image::new(ctx, "/tower2.png").unwrap(),
            tower_ninja_sprite: graphics::Image::new(ctx, "/tower_ninja.png").unwrap(),
            tower_attack_sound: audio::Source::new(ctx, "/tower_attack.ogg").unwrap(),
            ninja_tower_strong_attack_sound: audio::Source::new(ctx, "/NinjaTowerStrongAttack.mp3").unwrap(),
            monster_sprite: graphics::Image::new(ctx, "/monster1.png").unwrap(),
            gold_sprite: graphics::Image::new(ctx, "/gold_pile.png").unwrap(),
            base_sprite: graphics::Image::new(ctx, "/base.png").unwrap(),
        }
    }
}
