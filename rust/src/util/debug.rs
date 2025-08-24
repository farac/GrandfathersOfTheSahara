#[macro_export]
macro_rules! godot_dbg {
    ( $var: expr ) => {
        Logger::debug(format!("{:?}", $var));
    };
}
