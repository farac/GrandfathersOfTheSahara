#[macro_export]
macro_rules! godot_dbg {
    ( $var: expr ) => {
        godot_print!("{:?}", $var);
    };
}
