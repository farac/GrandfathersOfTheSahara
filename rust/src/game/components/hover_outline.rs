use godot::{
    classes::{Area2D, CollisionPolygon2D, CollisionShape2D, IArea2D, PanelContainer},
    obj::{Base, Gd, WithBaseField},
    prelude::{godot_api, GodotClass},
};

#[derive(Debug, GodotClass)]
#[class(init, base=Area2D)]
pub struct HoverableOutline {
    base: Base<Area2D>,
}

impl HoverableOutline {
    fn get_outline_node(&self) -> Gd<PanelContainer> {
        self.base().get_node_as("./Outline")
    }
    fn get_collision_shape(&self) -> Gd<CollisionShape2D> {
        self.base().get_node_as("./CollisionShape2D")
    }
    fn show_outline(&mut self) {
        let mut outline: Gd<PanelContainer> = self.get_outline_node();

        outline.set_visible(true);
    }
    fn hide_outline(&mut self) {
        let mut outline: Gd<PanelContainer> = self.get_outline_node();

        outline.set_visible(false);
    }
    pub fn enable_collision(&mut self) {
        let mut collision_shape: Gd<CollisionShape2D> = self.get_collision_shape();

        collision_shape.set_disabled(false);
    }
    pub fn disable_collision(&mut self) {
        let mut collision_shape: Gd<CollisionShape2D> = self.get_collision_shape();

        collision_shape.set_disabled(true);
    }
}

#[godot_api]
impl IArea2D for HoverableOutline {
    fn ready(&mut self) {
        self.base()
            .signals()
            .mouse_entered()
            .connect_other(&*self, |this| this.show_outline());

        self.base()
            .signals()
            .mouse_exited()
            .connect_other(&*self, |this| this.hide_outline());
    }
}

#[derive(Debug, GodotClass)]
#[class(init, base=Area2D)]
pub struct CollisionOutline {
    base: Base<Area2D>,
}

impl CollisionOutline {
    fn get_outline_node(&self) -> Gd<PanelContainer> {
        self.base().get_node_as("./Outline")
    }
    fn get_collision_shape(&self) -> Gd<CollisionPolygon2D> {
        self.base().get_node_as("./CollisionShape2D")
    }
    fn show_outline(&mut self) {
        let mut outline: Gd<PanelContainer> = self.get_outline_node();

        outline.set_visible(true);
    }
    fn hide_outline(&mut self) {
        let mut outline: Gd<PanelContainer> = self.get_outline_node();

        outline.set_visible(false);
    }
    pub fn enable_collision(&mut self) {
        let mut collision_shape: Gd<CollisionPolygon2D> = self.get_collision_shape();

        collision_shape.set_disabled(false);
    }
    pub fn disable_collision(&mut self) {
        let mut collision_shape: Gd<CollisionPolygon2D> = self.get_collision_shape();

        collision_shape.set_disabled(true);
    }
}

#[godot_api]
impl IArea2D for CollisionOutline {
    fn ready(&mut self) {
        self.base()
            .signals()
            .mouse_entered()
            .connect_other(&*self, |this| this.show_outline());

        self.base()
            .signals()
            .mouse_exited()
            .connect_other(&*self, |this| this.hide_outline());
    }
}
