use ggez::{
    audio,
    graphics,
    Context,
};

pub struct AssetManager {
    pub tower_sprite: graphics::Image,
    pub tower_attack_sound: audio::Source,
    pub monster_sprite: graphics::Image,
    // monster_hurt_sound: graphics::Image,
}

impl AssetManager {
    pub fn new(ctx: &mut Context) -> AssetManager {
        AssetManager {
            tower_sprite: graphics::Image::new(ctx, "/tower2.png").unwrap(),
            tower_attack_sound: audio::Source::new(ctx, "/tower_attack.ogg").unwrap(),
            monster_sprite: graphics::Image::new(ctx, "/monster1.png").unwrap(),
        }
    }
}
