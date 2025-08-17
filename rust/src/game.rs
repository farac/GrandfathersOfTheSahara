use godot::{
    classes::{Node, Node2D},
    obj::{Base, Gd},
    prelude::GodotClass,
};

pub mod components;
pub mod entities;

#[derive(Debug, GodotClass)]
#[class(init, base=Node2D)]
pub struct RunningGameScene {
    base: Base<Node2D>,
}

impl RunningGameScene {
    fn get_running_game(node: &Node) -> Gd<RunningGameScene> {
        let tree = node
            .get_tree()
            .expect("Expected node to be part of a tree")
            .get_root()
            .expect("Expected tree to have root node");

        tree.get_node_as("./Running")
    }
}
