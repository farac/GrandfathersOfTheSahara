pub enum InputActions {
    Primary,
    _Secondary,
    RotateCw,
    RotateCcw,
}

impl From<InputActions> for String {
    fn from(value: InputActions) -> Self {
        match value {
            InputActions::Primary => String::from("Primary"),
            InputActions::_Secondary => String::from("Secondary"),
            InputActions::RotateCw => String::from("Rotate CW"),
            InputActions::RotateCcw => String::from("Rotate CCW"),
        }
    }
}
