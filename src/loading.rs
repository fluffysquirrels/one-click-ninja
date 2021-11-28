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
        AssetLoader::new(GameState::Loading, GameState::CreateResources)
            .with_collection::<Fonts>()
            .with_collection::<Sounds>()
            .with_collection::<TextureAssets>()
            .with_collection::<CountdownTextures>()
            .build(app);
    }
}

// The following asset collections will be loaded during the State `GameState::Loading`
#[derive(AssetCollection)]
pub struct Fonts {
    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    pub fira_sans: Handle<Font>,
    #[asset(path = "fonts/fiendish/Fiendish.ttf")]
    pub fiendish: Handle<Font>,
}

#[derive(AssetCollection)]
pub struct Sounds {
    #[asset(path = "sfx/countdown_from_3.ogg")]
    pub countdown: Handle<AudioSource>,
    #[asset(path = "sfx/game_over_loud.ogg")]
    pub game_over: Handle<AudioSource>,
    #[asset(path = "sfx/kenney_uiaudio/Audio/click1.ogg")]
    pub snare: Handle<AudioSource>,
    #[asset(path = "sfx/kenney_uiaudio/Audio/click2.ogg")]
    pub bass: Handle<AudioSource>,
    #[asset(path = "sfx/hyperbeam_-_ninja_song.ogg")]
    pub playing_loop: Handle<AudioSource>,
    #[asset(path = "sfx/game_over_loop.ogg")]
    pub game_over_loop: Handle<AudioSource>,
    #[asset(path = "sfx/main_menu_loop.ogg")]
    pub main_menu_loop: Handle<AudioSource>,
    #[asset(path = "sfx/boss_growl.ogg")]
    pub boss_intro: Handle<AudioSource>,
    #[asset(path = "sfx/grunt_loud.ogg")]
    pub grunt: Handle<AudioSource>,
    #[asset(path = "sfx/hits/hit08.ogg")]
    pub hit: Handle<AudioSource>,
    #[asset(path = "sfx/sword_clash/2.ogg")]
    pub shield: Handle<AudioSource>,
    #[asset(path = "sfx/aargh/aargh6.ogg")]
    pub scream: Handle<AudioSource>,
    #[asset(path = "sfx/zombies/3.ogg")]
    pub zombie_death: Handle<AudioSource>,
}

#[derive(AssetCollection)]
pub struct TextureAssets {
    #[asset(path = "sprites/sword.png")]
    pub icon_sword: Handle<Texture>,
    #[asset(path = "sprites/shield.png")]
    pub icon_shield: Handle<Texture>,
    #[asset(path = "sprites/david_dawn/orb_blue.png")]
    pub icon_magic: Handle<Texture>,
    #[asset(path = "sprites/arrow.png")]
    pub icon_arrow: Handle<Texture>,
    #[asset(path = "sprites/david_dawn/sword_highlight.png")]
    pub icon_sword_highlight: Handle<Texture>,
    #[asset(path = "sprites/david_dawn/shield_highlight.png")]
    pub icon_shield_highlight: Handle<Texture>,
    #[asset(path = "sprites/david_dawn/orb_highlight.png")]
    pub icon_magic_highlight: Handle<Texture>,
    #[asset(path = "sprites/david_dawn/arrow_highlight.png")]
    pub icon_arrow_highlight: Handle<Texture>,
    #[asset(path = "sprites/david_dawn/pointer.png")]
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

    #[asset(path = "sprites/david_dawn/golem/idle.png")]
    pub boss_idle: Handle<Texture>,
    #[asset(path = "sprites/david_dawn/golem/flash_sheet.png")]
    pub boss_attack_sheet: Handle<Texture>,
    #[asset(path = "sprites/david_dawn/golem/death_sheet.png")]
    pub boss_death_sheet: Handle<Texture>,
    #[asset(path = "sprites/david_dawn/golem/godray_sheet.png")]
    pub boss_ray_sheet: Handle<Texture>,

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

    #[asset(path = "sprites/david_dawn/blood.png")]
    pub blood_splatter: Handle<Texture>,
    #[asset(path = "sprites/david_dawn/shield_flash_sheet.png")]
    pub shield_flash_sheet: Handle<Texture>,

    #[asset(path = "sprites/david_dawn/background.png")]
    pub background: Handle<Texture>,
    #[asset(path = "sprites/david_dawn/platform.png")]
    pub platform: Handle<Texture>,

    #[asset(path = "sprites/david_dawn/space_bar_anim.png")]
    pub space_bar_atlas: Handle<Texture>,

    #[asset(path = "sprites/david_dawn/game_over.png")]
    pub game_over_text: Handle<Texture>,
    #[asset(path = "sprites/david_dawn/space_to_restart.png")]
    pub game_over_press_space: Handle<Texture>,

    #[asset(path = "sprites/david_dawn/menu_text.png")]
    pub menu_text: Handle<Texture>,
    #[asset(path = "sprites/david_dawn/click_here.png")]
    pub menu_click_here: Handle<Texture>,

    #[asset(path = "sprites/david_dawn/boss_text.png")]
    pub boss_text: Handle<Texture>,

    #[asset(path = "sprites/david_dawn/you_survived.png")]
    pub win_text: Handle<Texture>,

    #[asset(path = "sprites/david_dawn/health_player.png")]
    pub health_player: Handle<Texture>,
    #[asset(path = "sprites/david_dawn/health_enemy.png")]
    pub health_enemy: Handle<Texture>,
}

#[derive(AssetCollection)]
pub struct CountdownTextures {
    #[asset(color_material)]
    #[asset(path = "sprites/david_dawn/countdown/3.png")]
    pub number_3: Handle<ColorMaterial>,
    #[asset(color_material)]
    #[asset(path = "sprites/david_dawn/countdown/2.png")]
    pub number_2: Handle<ColorMaterial>,
    #[asset(color_material)]
    #[asset(path = "sprites/david_dawn/countdown/1.png")]
    pub number_1: Handle<ColorMaterial>,
    #[asset(color_material)]
    #[asset(path = "sprites/david_dawn/countdown/fight.png")]
    pub fight: Handle<ColorMaterial>,
}
