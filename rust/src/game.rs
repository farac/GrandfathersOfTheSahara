use godot::{
    classes::{Node, Node2D},
    obj::{Base, Gd},
    prelude::GodotClass,
};

use crate::util::RootWindow;

pub mod components;
pub mod entities;

#[derive(Debug, GodotClass)]
#[class(init, base=Node2D)]
pub struct RunningGameScene {
    base: Base<Node2D>,
}

impl RunningGameScene {
    fn get_running_game(node: &Node) -> Gd<RunningGameScene> {
        let root = node.get_tree_root();

        root.get_node_as("./Running")
    }
}
