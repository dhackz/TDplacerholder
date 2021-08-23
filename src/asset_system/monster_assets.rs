use ggez::{audio, graphics, Context};

pub struct ChickenAssets {
    pub chicken_sprite: graphics::Image,
}

pub struct MonsterAssets {
    pub chicken_assets: ChickenAssets,
    pub cool_chicken_sprite: graphics::Image,
    pub monster_hurt_sound: audio::Source,
}

impl MonsterAssets {
    pub fn new(ctx: &mut Context) -> MonsterAssets {
        MonsterAssets {
            chicken_assets: ChickenAssets {
                chicken_sprite: graphics::Image::new(ctx, "/chicken.png").unwrap(),
            },
            cool_chicken_sprite: graphics::Image::new(ctx, "/cool_chicken.png").unwrap(),
            monster_hurt_sound: audio::Source::new(ctx, "/chicken_hurt.ogg").unwrap(),
        }
    }
}
