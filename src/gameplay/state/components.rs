use bevy::prelude::*;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum CameraState {
    #[default]
    MainMenuCamera,
    FollowPlayer,
    ManualCameraControl,
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    MainMenu,
    Loading,
    Playing,
    Pause,
    MainDialogue,
    SideDialogue,
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum HttpRequestState {
    #[default]
    Idle,
    Loading,
    Success,
    Error,
}
