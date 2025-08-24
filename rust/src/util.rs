use godot::{
    classes::{INode, Node, Os, Window},
    global::{godot_error, godot_print, godot_warn},
    obj::{Base, Gd},
    prelude::{godot_api, GodotClass},
};

pub mod debug;
pub mod flags;
pub mod input;
pub mod loader;

pub trait RootWindow
where
    Self: GodotClass,
{
    fn get_tree_root(&self) -> Gd<Window>;
}

impl RootWindow for Node {
    /// Gets the root of the tree this node belongs to
    ///
    /// # Panics
    ///
    /// Panics if the result of `get_tree()` or `get_root()` is None
    fn get_tree_root(&self) -> Gd<Window> {
        self.get_tree()
            .expect("Expected node to be part of a tree")
            .get_root()
            .expect("Expected tree to have root node")
    }
}

enum LogLevel {
    Debug,
    Warn,
    Error,
}

#[derive(GodotClass, Debug)]
#[class(base=Node)]
struct Env {
    base: Base<Node>,
}

pub struct Logger;

impl Logger {
    /// Use `format!()` macro to construct a formatted `output` parameter
    fn print(level: LogLevel, output: &str) {
        let debug = Os::singleton()
            .get_environment("DEBUG")
            .to_string()
            .as_str()
            .parse()
            .unwrap_or(false);

        match level {
            LogLevel::Debug => {
                if debug {
                    godot_print!("{}", output)
                }
            }
            LogLevel::Warn => godot_warn!("{}", output),
            LogLevel::Error => godot_error!("{}", output),
        }
    }
    pub fn debug(message: &str) {
        Self::print(LogLevel::Debug, message)
    }
    pub fn warn(message: &str) {
        Self::print(LogLevel::Warn, message)
    }
    pub fn error(message: &str) {
        Self::print(LogLevel::Error, message)
    }
}

#[godot_api]
impl INode for Env {
    fn init(base: Base<Node>) -> Self {
        #[cfg(debug_assertions)]
        {
            Os::singleton().set_environment("DEBUG", "true");
        }

        Self { base }
    }
}
