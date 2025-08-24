use godot::{
    classes::{Node, Window},
    obj::{Gd, GodotClass},
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
