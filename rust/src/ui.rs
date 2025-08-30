use godot::{
    classes::{Area2D, Button, CollisionShape2D, IArea2D, IButton, Label, MarginContainer},
    prelude::*,
};

use crate::scenes::GameScene;

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
            let on_click_string = self.scene_on_click.to_string();

            let target_scene: GameScene = on_click_string.as_str().try_into().expect(
                "Provided invalid scene name as `scene_on_click` to `SceneChangeButton.pressed()`",
            );

            let target_scene_path = target_scene.to_path();

            scene_tree.change_scene_to_file(target_scene_path);
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

#[derive(GodotClass)]
#[class(init, base=Area2D)]
pub struct LabelTooltip {
    base: Base<Area2D>,

    is_visible: bool,

    #[init(val = GString::from("Tooltip"))]
    #[export]
    label: GString,
    #[init(val = GString::from(""))]
    #[export]
    value: GString,
}

impl LabelTooltip {
    fn get_panel(&self) -> Gd<MarginContainer> {
        self.base().get_node_as("./Panel")
    }
    fn show_tooltip(&mut self) {
        self.get_panel().set_visible(true);
        self.is_visible = true;
    }
    fn hide_tooltip(&mut self) {
        self.get_panel().set_visible(false);
        self.is_visible = false;
    }
    fn get_collision_shape(&self) -> Gd<CollisionShape2D> {
        self.base().get_node_as("./CollisionShape2D")
    }
    fn disable_collision(&mut self) {
        let mut gd_collision_shape = self.get_collision_shape();
        gd_collision_shape.set_disabled(true);
    }
    fn enable_collision(&mut self) {
        let mut gd_collision_shape = self.get_collision_shape();
        gd_collision_shape.set_disabled(false);
    }
    fn get_tooltip_label(&mut self) -> Gd<Label> {
        self.base()
            .get_node_as("./Panel/Outline/MarginContainer/VBoxContainer/Label")
    }
    fn get_tooltip_value(&mut self) -> Gd<Label> {
        self.base()
            .get_node_as("./Panel/Outline/MarginContainer/VBoxContainer/Highlight")
    }
}

#[godot_api]
impl IArea2D for LabelTooltip {
    fn ready(&mut self) {
        self.base()
            .signals()
            .mouse_entered()
            .connect_other(&self.to_gd(), |this| this.show_tooltip());

        self.base()
            .signals()
            .mouse_exited()
            .connect_other(&self.to_gd(), |this| this.hide_tooltip());

        let mut label = self.get_tooltip_label();
        label.set_text(&self.label);

        if !self.value.is_empty() {
            let mut value = self.get_tooltip_value();
            value.set_visible(true);
            value.set_text(&self.value);
        }
    }
    fn process(&mut self, _dt: f64) {
        let mut tooltip = self.get_panel();
        let mouse_position = self
            .base()
            .get_viewport()
            .expect("Expected node to have a viewport")
            .get_mouse_position();

        tooltip.set_global_position(mouse_position);
    }
}

#[cfg(test)]
mod tests {}
