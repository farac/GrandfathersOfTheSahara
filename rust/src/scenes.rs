use const_format::concatcp;

static GAME_SCENES_ROOT: &str = "res://game/";
static MENU_SCENES_ROOT: &str = "res://game/";

enum GameScene {
    Running(concatcp!(GAME_SCENES_ROOT, "/screens/running/running.tscn")),
    Settings(concatcp!(MENU_SCENES_ROOT, "/settings.tscn")),
}

impl TryFrom<&str> for GameScene {
    fn try_from
}
