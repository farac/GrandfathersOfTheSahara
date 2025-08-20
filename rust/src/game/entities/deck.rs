use godot::builtin::{Color, Vector2};
use godot::classes::{INode2D, Label, Node2D};
use godot::obj::{Base, Gd, WithBaseField};
use godot::prelude::{godot_api, GodotClass};
use phf::{phf_map, Map};

use crate::game::components::tile_component::{
    NextTileData, NextTileDataRemaining, TileComponent, TileDeckComponent,
};
use crate::game::entities::tile::Tile;
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
        let mut gd_tile_deck_component = self.get_tile_deck_component();
        let next_tile: Option<NextTileDataRemaining>;

        {
            next_tile = gd_tile_deck_component.bind_mut().get_next_tile_data();
        }

        if let Some(tile) = next_tile.map(|nt| TileComponent::from_tile_data(nt.0)) {
            self.spawn_new_tile(tile)
        }
    }
    fn get_tile_deck_component(&self) -> Gd<TileDeckComponent> {
        self.to_gd()
            .get_node_as::<TileDeckComponent>("./TileDeckComponent")
    }
    fn get_label(&self) -> Gd<Label> {
        self.to_gd()
            .get_node_as::<Label>("./Control/CenterContainer/Label")
    }
}

#[godot_api]
impl INode2D for TileDeck {
    fn ready(&mut self) {
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

        let mut label = self.get_label();
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
}
