use ggez::{audio, graphics, Context};

pub struct TowerAssets {
    pub tower_sprite: graphics::Image,
    pub tower_ninja_sprite: graphics::Image,
    pub tower_attack_sound: audio::Source,
    pub ninja_tower_strong_attack_sound: audio::Source,
}

pub struct MonsterAssets {
    pub chicken_sprite: graphics::Image,
    pub cool_chicken_sprite: graphics::Image,
    pub monster_hurt_sound: audio::Source,
}

pub struct ItemAssets {
    pub gold_sprite: graphics::Image,
    pub gold_sound: audio::Source,
}

pub struct BaseAssets {
    pub base_sprite: graphics::Image,
}

pub struct BuilderUIAssets {
    pub tower_sprite: graphics::Image,
    pub ninja_tower_sprite: graphics::Image,
}

pub struct AssetManager {
    pub tower_assets: TowerAssets,
    pub monster_assets: MonsterAssets,
    pub item_assets: ItemAssets,
    pub base_assets: BaseAssets,
    pub builder_ui_assets: BuilderUIAssets,
}

impl AssetManager {
    pub fn new(ctx: &mut Context) -> AssetManager {
        let tower_assets = TowerAssets {
            tower_sprite: graphics::Image::new(ctx, "/tower2.png").unwrap(),
            tower_ninja_sprite: graphics::Image::new(ctx, "/tower_ninja.png").unwrap(),
            tower_attack_sound: audio::Source::new(ctx, "/tower_attack_pop.ogg").unwrap(),
            ninja_tower_strong_attack_sound: audio::Source::new(ctx, "/NinjaTowerStrongAttack.mp3")
                .unwrap(),
        };

        let monster_assets = MonsterAssets {
            chicken_sprite: graphics::Image::new(ctx, "/chicken.png").unwrap(),
            cool_chicken_sprite: graphics::Image::new(ctx, "/cool_chicken.png").unwrap(),
            monster_hurt_sound: audio::Source::new(ctx, "/chicken_hurt.ogg").unwrap(),
        };

        let item_assets = ItemAssets {
            gold_sprite: graphics::Image::new(ctx, "/gold_pile.png").unwrap(),
            gold_sound: audio::Source::new(ctx, "/gold.ogg").unwrap(),
        };

        let base_assets = BaseAssets {
            base_sprite: graphics::Image::new(ctx, "/base.png").unwrap(),
        };

        let builder_ui_assets = BuilderUIAssets {
            tower_sprite: graphics::Image::new(ctx, "/ui/tower.png").unwrap(),
            ninja_tower_sprite: graphics::Image::new(ctx, "/ui/ninja_tower.png").unwrap(),
        };

        AssetManager {
            tower_assets,
            monster_assets,
            item_assets,
            base_assets,
            builder_ui_assets,
        }
    }
}
