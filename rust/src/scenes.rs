use const_format::concatcp;
use thiserror::Error;

static GAME_SCENES_ROOT: &str = "res://game/";
static MENU_SCENES_ROOT: &str = "res://menus/";

#[derive(Clone, Copy, Debug)]
pub enum GameScene {
    MainMenu,
    Running,
    Settings,
}

#[derive(Error, Debug)]
pub enum GameSceneParseError<'a> {
    #[error("GameScene expected one of running, settings, main_menu, received {0}")]
    ParseGameScene(&'a str),
}

impl<'a> TryFrom<&'a str> for GameScene {
    type Error = GameSceneParseError<'a>;

    fn try_from(value: &'a str) -> Result<Self, GameSceneParseError<'a>> {
        match value {
            "running" => Ok(GameScene::Running),
            "settings" => Ok(GameScene::Settings),
            "main_menu" => Ok(GameScene::MainMenu),
            _ => Err(GameSceneParseError::ParseGameScene(value)),
        }
    }
}

impl GameScene {
    pub fn to_path(&self) -> &'static str {
        match &self {
            GameScene::Running => concatcp!(GAME_SCENES_ROOT, "screens/running/running.tscn"),
            GameScene::Settings => concatcp!(MENU_SCENES_ROOT, "settings.tscn"),
            GameScene::MainMenu => concatcp!(MENU_SCENES_ROOT, "main_menu.tscn"),
        }
    }
}
