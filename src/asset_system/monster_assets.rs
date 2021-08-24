use ggez::{audio, graphics, Context};

pub struct ChickenAssets {
    pub walking_sprites: Vec<graphics::Image>,
}

pub struct MonsterAssets {
    pub chicken_assets: ChickenAssets,
    pub cool_chicken_sprite: graphics::Image,
    pub monster_hurt_sound: audio::Source,
}

impl MonsterAssets {
    pub fn new(ctx: &mut Context) -> MonsterAssets {
        let mut walking_sprites = Vec::new();
        walking_sprites
            .push(graphics::Image::new(ctx, "/monsters/chicken/chicken_run1.png").unwrap());
        walking_sprites
            .push(graphics::Image::new(ctx, "/monsters/chicken/chicken_run2.png").unwrap());

        MonsterAssets {
            chicken_assets: ChickenAssets { walking_sprites },
            cool_chicken_sprite: graphics::Image::new(
                ctx,
                "/monsters/cool_chicken/cool_chicken.png",
            )
            .unwrap(),
            monster_hurt_sound: audio::Source::new(ctx, "/monsters/chicken_hurt.ogg").unwrap(),
        }
    }
}
