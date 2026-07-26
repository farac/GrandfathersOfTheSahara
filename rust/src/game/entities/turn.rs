use crate::game::entities::player::PlayerName;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TurnPhase {
    Move,
    Explore,
}

#[derive(Debug, Clone, Copy)]
pub struct TurnState {
    active_player: PlayerName,
    phase: TurnPhase,
}

impl Default for TurnState {
    fn default() -> Self {
        Self {
            active_player: PlayerName::default(),
            phase: TurnPhase::Move,
        }
    }
}

impl TurnState {
    pub fn active_player(&self) -> PlayerName {
        self.active_player
    }
    pub fn can_move(&self) -> bool {
        self.phase == TurnPhase::Move
    }
    /// Move -> Explore: the player is done moving and can now explore.
    /// Only legal from the Move phase.
    pub fn advance_to_explore(&mut self) {
        debug_assert!(
            self.phase == TurnPhase::Move,
            "advance_to_explore is only legal from the Move phase, was {:?}",
            self.phase
        );

        self.phase = TurnPhase::Explore;
    }
    /// Explore -> Move: hand the turn to the next player. A desert tile keeps
    /// the same player for another move. Only legal from the Explore phase.
    pub fn advance_turn(&mut self, was_desert_tile: bool) {
        debug_assert!(
            self.phase == TurnPhase::Explore,
            "advance_turn is only legal from the Explore phase, was {:?}",
            self.phase
        );

        if !was_desert_tile {
            self.active_player = self.active_player.cycle();
        }

        self.phase = TurnPhase::Move;
    }
}

#[cfg(test)]
mod tests {
    use super::TurnState;
    use crate::game::entities::player::PlayerName;

    #[test]
    fn starts_in_move_phase_for_the_first_player() {
        let turn = TurnState::default();

        assert!(turn.can_move());
        assert_eq!(turn.active_player(), PlayerName::White);
    }

    #[test]
    fn using_a_move_switches_to_explore() {
        let mut turn = TurnState::default();

        turn.advance_to_explore();

        assert!(!turn.can_move());
    }

    #[test]
    fn advancing_a_regular_turn_cycles_to_the_next_player() {
        let mut turn = TurnState::default();

        turn.advance_to_explore();
        turn.advance_turn(false);

        assert!(turn.can_move());
        assert_eq!(turn.active_player(), PlayerName::White.cycle());
    }

    #[test]
    fn a_desert_tile_grants_the_same_player_another_turn() {
        let mut turn = TurnState::default();
        let player = turn.active_player();

        turn.advance_to_explore();
        turn.advance_turn(true);

        assert!(turn.can_move());
        assert_eq!(turn.active_player(), player);
    }

    #[test]
    #[should_panic]
    fn advancing_the_turn_from_the_move_phase_is_rejected() {
        let mut turn = TurnState::default();

        turn.advance_turn(false);
    }

    #[test]
    #[should_panic]
    fn advancing_to_explore_from_the_explore_phase_is_rejected() {
        let mut turn = TurnState::default();

        turn.advance_to_explore();
        turn.advance_to_explore();
    }
}
