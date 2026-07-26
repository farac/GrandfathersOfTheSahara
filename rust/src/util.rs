use godot::classes::INode;
use godot::classes::Node;
use godot::classes::Os;
use godot::classes::Window;
use godot::global::godot_error;
use godot::global::godot_print;
use godot::global::godot_warn;
use godot::obj::Base;
use godot::obj::Gd;
use godot::obj::Singleton;
use godot::prelude::godot_api;
use godot::prelude::GodotClass;

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
            .get_root()
            .expect("Expected tree to have root node")
    }
}

#[derive(PartialEq, PartialOrd)]
enum LogLevel {
    Debug,
    Info,
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
    /// Lowest severity that prints. `Error` in release, `Info` in debug
    /// builds, or `Debug` when the `DEBUG` environment flag is set.
    fn threshold() -> LogLevel {
        let debug_flag = Os::singleton()
            .get_environment("DEBUG")
            .to_string()
            .parse()
            .unwrap_or(false);

        if debug_flag {
            LogLevel::Debug
        } else if cfg!(debug_assertions) {
            LogLevel::Info
        } else {
            LogLevel::Error
        }
    }
    /// Use `format!()` macro to construct a formatted `output` parameter
    fn print(level: LogLevel, output: &str) {
        if level < Self::threshold() {
            return;
        }

        match level {
            LogLevel::Debug => godot_print!("[DEBUG]: {}", output),
            LogLevel::Info => godot_print!("[INFO]: {}", output),
            LogLevel::Warn => godot_warn!("[WARN]: {}", output),
            LogLevel::Error => godot_error!("[ERROR]: {}", output),
        }
    }
    pub fn debug(message: &str) {
        Self::print(LogLevel::Debug, message)
    }
    pub fn info(message: &str) {
        Self::print(LogLevel::Info, message)
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
        Self { base }
    }
}
