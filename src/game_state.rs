#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub (crate) enum GameState {
    /// During the loading State the LoadingPlugin will load our assets
    Loading,
    /// Asset loading is complete at this point but resources derived
    /// from the loaded assets need to be created
    CreateResources,
    /// Use resources to spawn entities
    Setup,
    /// Here the menu is drawn and waiting for player interaction
    Menu,
    /// During this state the actual game logic is executed
    Playing,
    /// The player has died and there is the option to restart
    GameOver,
}
