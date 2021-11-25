#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub (crate) enum GameState {
    // During the loading State the LoadingPlugin will load our assets
    Loading,
    // Asset loading is complete at this point but resources derived from the loaded assets need to be created
    Setup,
    // Here the menu is drawn and waiting for player interaction
    Menu,
    // During this State the actual game logic is executed
    Playing,
}
