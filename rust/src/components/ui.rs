use godot::engine::{Button, IButton};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base=Button)]
pub struct SceneChangeButton {
    base: Base<Button>,

    #[export]
    scene_on_click: GString,
}

#[godot_api]
impl IButton for SceneChangeButton {
    fn pressed(&mut self) {
        let mut scene_tree = self.base().get_tree().expect("Scene tree should exist.");

        scene_tree.change_scene_to_file(self.scene_on_click.clone());
    }
}

#[derive(GodotClass)]
#[class(init, base=Button)]
pub struct QuitButton {
    base: Base<Button>,
}

#[godot_api]
impl IButton for QuitButton {
    fn pressed(&mut self) {
        let scene_tree = self.base().get_tree().expect("Scene tree should exist.");

        scene_tree
            .get_root()
            .expect("Scene tree root should exist.")
            .emit_signal("close_requested".into(), &[]);
    }
}

#[cfg(test)]
mod tests {}
