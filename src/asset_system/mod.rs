pub mod asset_manager;
pub mod monster_assets;

// Re-export assets.
pub use self::asset_manager::AssetManager;
pub use self::asset_manager::BaseAssets;
pub use self::asset_manager::BuilderUIAssets;
pub use self::asset_manager::ItemAssets;
pub use self::asset_manager::TowerAssets;
pub use self::monster_assets::ChickenAssets;
pub use self::monster_assets::MonsterAssets;
