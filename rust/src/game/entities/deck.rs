use godot::builtin::{Color, GString, Vector2};
use godot::classes::{INode2D, Label, Node2D};
use godot::obj::{Base, Gd, WithBaseField};
use godot::prelude::{godot_api, GodotClass};
use phf::{phf_map, Map};

use crate::game::components::hover_outline::HoverableOutline;
use crate::game::components::tile_component::{
    NextTileData, NextTileDataRemaining, TileComponent, TileDeckComponent,
};
use crate::game::entities::tile::Tile;
use crate::game::entities::BoardComponent;
use crate::game::RunningGameScene;
use crate::util::loader::SceneLoader;

// TODO: Move this color information to the mapping in config/tileset.toml
const TILE_COLOR_MAP: Map<&'static str, &'static str> = phf_map! {
    "1" => "#fee17c",
    "2" => "#b3d7ed",
    "3" => "#99d761",
    "4" => "#f89b49",
    "5" => "#c97db4",
};

#[derive(Debug, GodotClass)]
#[class(init, base=Node2D)]
struct TileDeck {
    base: Base<Node2D>,

    #[export]
    deck_index: u8,
}

impl TileDeck {
    fn get_hover_outline(&self) -> Gd<HoverableOutline> {
        self.base()
            .get_node_as::<HoverableOutline>("./HoverOutline")
    }
    fn disable_outline(&self) {
        let mut gd_hover_outline = self.get_hover_outline();
        gd_hover_outline.set_visible(false);

        let mut hover_outline = gd_hover_outline.bind_mut();

        hover_outline.disable_collision();
    }
    fn enable_collision(&self) {
        let mut gd_hover_outline = self.get_hover_outline();

        gd_hover_outline.set_visible(true);

        let mut hover_outline = gd_hover_outline.bind_mut();

        hover_outline.enable_collision();
    }
    fn spawn_new_tile(&self, tile_component: Gd<TileComponent>) {
        let gd_scene_loader = SceneLoader::get(&self.base());
        let scene_loader = gd_scene_loader.bind();
        let mut new_tile: Gd<Tile>;

        {
            let tile_component = tile_component.bind();
            new_tile = scene_loader.instantiate_tile_scene_from_tile_component(&tile_component);
        }

        {
            let mut new_tile = new_tile.bind_mut();
            new_tile.set_active();
        }

        let mut gd_scene = RunningGameScene::get_running_game(&self.base());

        gd_scene.add_child(&new_tile);
        new_tile.set_owner(&gd_scene);

        let mouse_position = new_tile
            .get_viewport()
            .expect("Expected game to have a viewport")
            .get_mouse_position();
        new_tile.set_position(mouse_position);
        new_tile.set_scale(Vector2::from_tuple((0.3, 0.3)));
    }
}

#[godot_api]
impl TileDeck {
    #[func]
    fn get_next_tile(&mut self) {
        let mut board_component = BoardComponent::get(&self.to_gd());

        if board_component.bind().active_tile_deck != self.deck_index {
            return;
        }

        let mut gd_tile_deck_component = self.get_tile_deck_component();
        let next_tile: Option<NextTileDataRemaining>;

        {
            next_tile = gd_tile_deck_component.bind_mut().get_next_tile_data();
        }

        if let Some(tile) = next_tile.map(|nt| TileComponent::from_tile_data(nt.0)) {
            self.spawn_new_tile(tile);

            let mut rem_label = self.get_remaining_label();

            let tile_deck_component = gd_tile_deck_component.bind();
            let new_remaining =
                tile_deck_component.tiles.len() - tile_deck_component.index as usize;
            let new_remaining_label = new_remaining.to_string();

            rem_label.set_text(&new_remaining_label);

            if new_remaining == 0 {
                self.disable_outline();
            }
        } else {
            board_component.bind_mut().active_tile_deck += 1;
            self.disable_outline();
        }
    }
    fn get_tile_deck_component(&self) -> Gd<TileDeckComponent> {
        self.to_gd()
            .get_node_as::<TileDeckComponent>("./TileDeckComponent")
    }
    fn get_idx_label(&self) -> Gd<Label> {
        self.to_gd()
            .get_node_as::<Label>("./Control/VBoxContainer/CenterContainer/Label")
    }
    fn get_remaining_label(&self) -> Gd<Label> {
        self.to_gd()
            .get_node_as::<Label>("./Control/VBoxContainer/CenterContainer2/Remaining")
    }
}

#[godot_api]
impl INode2D for TileDeck {
    fn ready(&mut self) {
        let board_component = BoardComponent::get(&self.to_gd());

        if board_component.bind().active_tile_deck != self.deck_index {
            self.disable_outline();
        }

        let mut tile_deck_component: Gd<TileDeckComponent>;
        let base = self.base();

        tile_deck_component = TileDeckComponent::from_tile_deck_index(&base, self.deck_index);
        tile_deck_component.set_name("TileDeckComponent");

        {
            let mut base = self.base_mut();
            base.add_child(&tile_deck_component);
        }

        let tile_deck = self.to_gd();
        tile_deck_component.set_owner(&tile_deck);

        let mut label = self.get_idx_label();
        let index_string = (self.deck_index + 1).to_string();
        let color_string = *TILE_COLOR_MAP
            .get(&index_string)
            .expect("Expected valid deck index");

        label.set_text(&index_string);
        label.add_theme_color_override(
            &format!("Deck {index_string} color"),
            Color::from_html(color_string).expect("Couldn't parse color string for deck"),
        );
    }
    fn process(&mut self, _dt: f64) {
        let active_index = BoardComponent::get(&self.to_gd()).bind().active_tile_deck;

        if active_index == self.deck_index {
            self.enable_collision();
        }
    }
}
