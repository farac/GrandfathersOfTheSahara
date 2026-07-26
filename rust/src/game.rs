use godot::builtin::Vector2;
use godot::classes::INode2D;
use godot::classes::Node;
use godot::classes::Node2D;
use godot::classes::PackedScene;
use godot::obj::Base;
use godot::obj::Gd;
use godot::obj::WithBaseField;
use godot::prelude::godot_api;
use godot::prelude::load;
use godot::prelude::GodotClass;

use crate::game::entities::movement::STARTING_POSITIONS;
use crate::game::entities::player_token::PlayerToken;
use crate::game::entities::BoardComponent;
use crate::util::Logger;
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
    fn place_starting_tokens(&mut self) {
        let mut gd_board = BoardComponent::get(&self.base());
        let token_scene = load::<PackedScene>("res://game/objects/player_token.tscn");
        let mut container = self.base().get_node_as::<Node2D>("./PlayerTokens");
        let running = self.to_gd();

        for (coordinates, player) in STARTING_POSITIONS {
            if let Err(error) = gd_board.bind().get_tile_at(coordinates.0, coordinates.1) {
                Logger::error(&format!("Couldn't place starting token: {error:?}"));
                continue;
            }

            gd_board.bind_mut().set_player_position(player, coordinates);

            let mut token = token_scene.instantiate_as::<PlayerToken>();
            container.add_child(&token);
            token.set_owner(&running);
            token.bind_mut().assign_token_to_player(player);
            token.set_scale(Vector2::new(0.25, 0.25));

            Logger::debug(&format!("Placed {player:?} caravan at {coordinates:?}"));
        }

        Logger::info("Placed starting caravans on the cross arm-ends");

        gd_board.bind_mut().enter_move_phase();
        gd_board.call_deferred("reposition_tokens", &[]);
    }
}

#[godot_api]
impl INode2D for RunningGameScene {
    fn ready(&mut self) {
        self.place_starting_tokens();
    }
}
