use godot::{
    classes::{Button, IButton},
    prelude::*,
};

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
        let scene_tree = self.base().get_tree();

        if let Some(mut scene_tree) = scene_tree {
            let on_click = &self.scene_on_click;

            scene_tree.change_scene_to_file(on_click);
        } else {
            godot_error!("Scene tree should exist.");
        }
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
        let scene_tree = self.base().get_tree();

        if let Some(scene_tree) = scene_tree {
            let tree_root = scene_tree.get_root();

            if let Some(mut tree_root) = tree_root {
                let signal = "close_requested";

                tree_root.emit_signal(signal, &[]);
            } else {
                godot_error!("Scene tree root should exist.");
            }
        } else {
            godot_error!("Scene tree should exist.");
        }
    }
}

#[cfg(test)]
mod tests {}
