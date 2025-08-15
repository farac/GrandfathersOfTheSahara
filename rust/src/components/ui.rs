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
pub struct OnClickButton {
    base: Base<Button>,
}

#[godot_api]
impl IButton for OnClickButton {
    fn pressed(&mut self) {
        self.to_gd().call("on_click", &[]);
    }
}

#[cfg(test)]
mod tests {}
