use crate::game_state::GameState;
use bevy::prelude::*;
use bevy_asset_loader::{AssetCollection, AssetLoader};
use bevy_kira_audio::AudioSource;

pub struct Plugin;

/// This plugin loads all assets using [AssetLoader] from a third party bevy plugin
/// Alternatively you can write the logic to load assets yourself
/// If interested, take a look at https://bevy-cheatbook.github.io/features/assets.html
impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut AppBuilder) {
        // AssetLoader will move to the 2nd gamestate provided here once all assets are loaded
        AssetLoader::new(GameState::Loading, GameState::Setup)
            .with_collection::<FontAssets>()
            .with_collection::<AudioAssets>()
            .with_collection::<TextureAssets>()
            .build(app);
    }
}

// The following asset collections will be loaded during the State `GameState::Loading`
#[derive(AssetCollection)]
pub struct FontAssets {
    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    pub fira_sans: Handle<Font>,
}

#[derive(AssetCollection)]
pub struct AudioAssets {
    #[asset(path = "sfx/countdown.mp3")]
    pub countdown: Handle<AudioSource>,
    #[asset(path = "sfx/kenney_uiaudio/Audio/click1.ogg")]
    pub snare: Handle<AudioSource>,
    #[asset(path = "sfx/kenney_uiaudio/Audio/click2.ogg")]
    pub bass: Handle<AudioSource>,
}

#[derive(AssetCollection)]
pub struct TextureAssets {
    #[asset(path = "sprites/sword.png")]
    pub icon_sword: Handle<Texture>,
    #[asset(path = "sprites/shield.png")]
    pub icon_shield: Handle<Texture>,
    #[asset(path = "sprites/magic_ball.png")]
    pub icon_magic: Handle<Texture>,
    #[asset(path = "sprites/arrow.png")]
    pub icon_arrow: Handle<Texture>,
    #[asset(path = "sprites/pointer.png")]
    pub icon_pointer: Handle<Texture>,

    #[asset(path = "sprites/lpc-medieval-fantasy-character/our_work/archer/walk_down/00.png")]
    pub archer_idle: Handle<Texture>,
    #[asset(path = "sprites/lpc-medieval-fantasy-character/our_work/archer/bow_down/09.png")]
    pub archer_attack: Handle<Texture>,
    #[asset(path = "sprites/lpc-medieval-fantasy-character/our_work/archer/die/05.png")]
    pub archer_dead: Handle<Texture>,

    #[asset(path = "sprites/lpc-medieval-fantasy-character/our_work/knight/walk_down/00.png")]
    pub knight_idle: Handle<Texture>,
    #[asset(path = "sprites/lpc-medieval-fantasy-character/our_work/knight/spear_down/05.png")]
    pub knight_attack: Handle<Texture>,
    #[asset(path = "sprites/lpc-medieval-fantasy-character/our_work/knight/die/05.png")]
    pub knight_dead: Handle<Texture>,

    #[asset(path = "sprites/lpc-medieval-fantasy-character/our_work/mage/walk_down/00.png")]
    pub mage_idle: Handle<Texture>,
    #[asset(path = "sprites/lpc-medieval-fantasy-character/our_work/mage/cast_down/05.png")]
    pub mage_attack: Handle<Texture>,
    #[asset(path = "sprites/lpc-medieval-fantasy-character/our_work/mage/die/05.png")]
    pub mage_dead: Handle<Texture>,

    #[asset(path = "sprites/lpc-medieval-fantasy-character/our_work/player/walk_up/00.png")]
    pub player_idle: Handle<Texture>,
    #[asset(path = "sprites/lpc-medieval-fantasy-character/our_work/player/bow_up/09.png")]
    pub player_attack_arrow: Handle<Texture>,
    #[asset(path = "sprites/lpc-medieval-fantasy-character/our_work/player/spear_up/05.png")]
    pub player_attack_sword: Handle<Texture>,
    #[asset(path = "sprites/lpc-medieval-fantasy-character/our_work/player/cast_up/05.png")]
    pub player_attack_magic: Handle<Texture>,
    #[asset(path = "sprites/lpc-medieval-fantasy-character/our_work/player/die/05.png")]
    pub player_dead: Handle<Texture>,

    #[asset(path = "sprites/david_dawn/hit.png")]
    pub damage_hit: Handle<Texture>,
    #[asset(path = "sprites/david_dawn/blocked.png")]
    pub damage_blocked: Handle<Texture>,

    #[asset(path = "sprites/david_dawn/background.png")]
    pub background: Handle<Texture>,

    #[asset(path = "sprites/david_dawn/space_bar_anim.png")]
    pub space_bar_atlas: Handle<Texture>,
}
