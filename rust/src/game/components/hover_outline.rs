use godot::{
    classes::{Area2D, IArea2D, PanelContainer},
    obj::{Base, Gd, WithBaseField},
    prelude::{godot_api, GodotClass},
};

#[derive(GodotClass)]
#[class(init, base=Area2D)]
struct HoverableOutline {
    base: Base<Area2D>,
}

impl HoverableOutline {
    fn get_outline_node(&self) -> Gd<PanelContainer> {
        self.base().get_node_as("./Outline")
    }
    fn show_outline(&mut self) {
        let mut outline: Gd<PanelContainer> = self.get_outline_node();

        outline.set_visible(true);
    }
    fn hide_outline(&mut self) {
        let mut outline: Gd<PanelContainer> = self.get_outline_node();

        outline.set_visible(false);
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

#[derive(GodotClass)]
#[class(init, base=Area2D)]
struct CollisionOutline {
    base: Base<Area2D>,
}

impl CollisionOutline {
    fn get_outline_node(&self) -> Gd<PanelContainer> {
        self.base().get_node_as("./Outline")
    }
    fn show_outline(&mut self) {
        let mut outline: Gd<PanelContainer> = self.get_outline_node();

        outline.set_visible(true);
    }
    fn hide_outline(&mut self) {
        let mut outline: Gd<PanelContainer> = self.get_outline_node();

        outline.set_visible(false);
    }
}

#[godot_api]
impl IArea2D for CollisionOutline {
    fn ready(&mut self) {
        // TODO: Show snap location when setting a tile
        // self.base()
        //     .signals()
        //     .mouse_entered()
        //     .connect_other(&*self, |this| this.show_outline());
        //
        // self.base()
        //     .signals()
        //     .mouse_exited()
        //     .connect_other(&*self, |this| this.hide_outline());
    }
}
