#[cfg(test)]
mod tests {
    use assert_matches::assert_matches;
    use toml::Table;

    use crate::util::{
        flags::OasisLayoutFlags,
        loader::{CrossConfig, TilesetConfig},
    };

    #[test]
    fn test_parse_tileset_config() {
        let input = String::from(
            r#"
# Configuration for the cross center
[cross.c]
is_desert = false

# 4 lists describing each part of the starting cross
# Numbered going outwards from the center
# N1
[[cross.n]]
is_desert = false
oasis = ["W"]
# N2
[[cross.n]]
is_desert = false
oasis = ["E"]
# N3
[[cross.n]]
is_desert = false
oasis = ["W"]
# N4
[[cross.n]]
is_desert = false
oasis = ["E"]
# N5
[[cross.n]]
is_desert = false

# E1
[[cross.e]]
is_desert = false
# E2
[[cross.e]]
is_desert = false
oasis = ["S"]
# E3
[[cross.e]]
is_desert = false
oasis = ["N"]
# E4
[[cross.e]]
oasis = ["S"]
is_desert = false
# E5
[[cross.e]]
is_desert = false

# S1
[[cross.s]]
is_desert = false
# S2
[[cross.s]]
is_desert = false
# S3
[[cross.s]]
is_desert = false
oasis = ["E", "W"]
# S4
[[cross.s]]
is_desert = false
# S5
[[cross.s]]
is_desert = false

# W1
[[cross.w]]
is_desert = false
# W2
[[cross.w]]
is_desert = false
oasis = ["N", "S"]
# W3
[[cross.w]]
is_desert = false
# W4
[[cross.w]]
is_desert = false
oasis = ["N", "S"]
# W5
[[cross.w]]
is_desert = false

# Lists describing each of the "decks" used in the game

# Deck 1 
[[decks]]
# 1
[[decks.deck]]
is_desert = true
oasis = ["E | S"]
treasure_n = "none"
treasure_e = "salt"
treasure_s = "rumors"
treasure_w = "none"

# 2
[[decks.deck]]
is_desert = false
oasis = ["N"]
treasure_n = "none"
treasure_e = "none"
treasure_s = "none"
treasure_w = "none"

# 3
[[decks.deck]]
is_desert = true
oasis = []
treasure_n = "none"
treasure_e = "none"
treasure_s = "none"
treasure_w = "none"

# 4
[[decks.deck]]
is_desert = false
oasis = ["E"]
treasure_n = "none"
treasure_e = "none"
treasure_s = "goods:gems"
treasure_w = "none"

# 5
[[decks.deck]]
is_desert = false
oasis = []
treasure_n = "none"
treasure_e = "none"
treasure_s = "none"
treasure_w = "none"

# 6
[[decks.deck]]
is_desert = false
oasis = []
treasure_n = "none"
treasure_e = "none"
treasure_s = "none"
treasure_w = "none"

# 7
[[decks.deck]]
is_desert = false
oasis = []
treasure_n = "none"
treasure_e = "none"
treasure_s = "none"
treasure_w = "none"

# 8
[[decks.deck]]
is_desert = false
oasis = []
treasure_n = "none"
treasure_e = "none"
treasure_s = "none"
treasure_w = "none"

# 9
[[decks.deck]]
is_desert = false
oasis = []
treasure_n = "none"
treasure_e = "none"
treasure_s = "none"
treasure_w = "none"

# 10
[[decks.deck]]
is_desert = false
oasis = []
treasure_n = "none"
treasure_e = "none"
treasure_s = "none"
treasure_w = "none"

# 11
[[decks.deck]]
is_desert = false
oasis = []
treasure_n = "none"
treasure_e = "none"
treasure_s = "none"
treasure_w = "none"

# 12
[[decks.deck]]
is_desert = false
oasis = []
treasure_n = "none"
treasure_e = "none"
treasure_s = "none"
treasure_w = "none"

# 13
[[decks.deck]]
is_desert = false
oasis = []
treasure_n = "none"
treasure_e = "none"
treasure_s = "none"
treasure_w = "none"

# 14
[[decks.deck]]
is_desert = false
oasis = []
treasure_n = "none"
treasure_e = "none"
treasure_s = "none"
treasure_w = "none"

# 15
[[decks.deck]]
is_desert = false
oasis = []
treasure_n = "none"
treasure_e = "none"
treasure_s = "none"
treasure_w = "none"

# 16
[[decks.deck]]
is_desert = false
oasis = []
treasure_n = "none"
treasure_e = "none"
treasure_s = "none"
treasure_w = "none"

# 17
[[decks.deck]]
is_desert = false
oasis = []
treasure_n = "none"
treasure_e = "none"
treasure_s = "none"
treasure_w = "none"


# Deck 2 
[[decks]]
# 1
[[decks.deck]]
is_desert = false
oasis = []
treasure_n = "none"
treasure_e = "none"
treasure_s = "none"
treasure_w = "none"

# 2
[[decks.deck]]
is_desert = false
oasis = []
treasure_n = "none"
treasure_e = "none"
treasure_s = "none"
treasure_w = "none"

# 3
[[decks.deck]]
is_desert = false
oasis = []
treasure_n = "none"
treasure_e = "none"
treasure_s = "none"
treasure_w = "none"

# 4
[[decks.deck]]
is_desert = false
oasis = []
treasure_n = "none"
treasure_e = "none"
treasure_s = "none"
treasure_w = "none"

# 5
[[decks.deck]]
is_desert = false
oasis = []
treasure_n = "none"
treasure_e = "none"
treasure_s = "none"
treasure_w = "none"

# 6
[[decks.deck]]
is_desert = false
oasis = []
treasure_n = "none"
treasure_e = "none"
treasure_s = "none"
treasure_w = "none"

# 7
[[decks.deck]]
is_desert = false
oasis = []
treasure_n = "none"
treasure_e = "none"
treasure_s = "none"
treasure_w = "none"

# 8
[[decks.deck]]
is_desert = false
oasis = []
treasure_n = "none"
treasure_e = "none"
treasure_s = "none"
treasure_w = "none"

# 9
[[decks.deck]]
is_desert = false
oasis = []
treasure_n = "none"
treasure_e = "none"
treasure_s = "none"
treasure_w = "none"

# 10
[[decks.deck]]
is_desert = false
oasis = []
treasure_n = "none"
treasure_e = "none"
treasure_s = "none"
treasure_w = "none"

# 11
[[decks.deck]]
is_desert = false
oasis = []
treasure_n = "none"
treasure_e = "none"
treasure_s = "none"
treasure_w = "none"

# 12
[[decks.deck]]
is_desert = false
oasis = []
treasure_n = "none"
treasure_e = "none"
treasure_s = "none"
treasure_w = "none"

# 13
[[decks.deck]]
is_desert = false
oasis = []
treasure_n = "none"
treasure_e = "none"
treasure_s = "none"
treasure_w = "none"

# 14
[[decks.deck]]
is_desert = false
oasis = []
treasure_n = "none"
treasure_e = "none"
treasure_s = "none"
treasure_w = "none"

# 15
[[decks.deck]]
is_desert = false
oasis = []
treasure_n = "none"
treasure_e = "none"
treasure_s = "none"
treasure_w = "none"

# 16
[[decks.deck]]
is_desert = false
oasis = []
treasure_n = "none"
treasure_e = "none"
treasure_s = "none"
treasure_w = "none"

# 17
[[decks.deck]]
is_desert = false
oasis = []
treasure_n = "none"
treasure_e = "none"
treasure_s = "none"
treasure_w = "none"


# Deck 3
[[decks]]
# 1
[[decks.deck]]
is_desert = true
oasis = []
treasure_n = "none"
treasure_e = "none"
treasure_s = "none"
treasure_w = "none"

# 2
[[decks.deck]]
is_desert = false
oasis = ["E"]
treasure_n = "none"
treasure_e = "none"
treasure_s = "none"
treasure_w = "none"

# 3
[[decks.deck]]
is_desert = false
oasis = ["S"]
treasure_n = "none"
treasure_e = "none"
treasure_s = "rumors"
treasure_w = "none"

# 4
[[decks.deck]]
is_desert = false
oasis = []
treasure_n = "none"
treasure_e = "none"
treasure_s = "none"
treasure_w = "none"

# 5
[[decks.deck]]
is_desert = false
oasis = []
treasure_n = "none"
treasure_e = "none"
treasure_s = "none"
treasure_w = "none"

# 6
[[decks.deck]]
is_desert = false
oasis = []
treasure_n = "none"
treasure_e = "none"
treasure_s = "none"
treasure_w = "none"

# 7
[[decks.deck]]
is_desert = false
oasis = []
treasure_n = "none"
treasure_e = "none"
treasure_s = "none"
treasure_w = "none"

# 8
[[decks.deck]]
is_desert = false
oasis = []
treasure_n = "none"
treasure_e = "none"
treasure_s = "none"
treasure_w = "none"

# 9
[[decks.deck]]
is_desert = false
oasis = []
treasure_n = "none"
treasure_e = "none"
treasure_s = "none"
treasure_w = "none"

# 10
[[decks.deck]]
is_desert = false
oasis = []
treasure_n = "none"
treasure_e = "none"
treasure_s = "none"
treasure_w = "none"

# 11
[[decks.deck]]
is_desert = false
oasis = []
treasure_n = "none"
treasure_e = "none"
treasure_s = "none"
treasure_w = "none"

# 12
[[decks.deck]]
is_desert = false
oasis = []
treasure_n = "none"
treasure_e = "none"
treasure_s = "none"
treasure_w = "none"

# 13
[[decks.deck]]
is_desert = false
oasis = []
treasure_n = "none"
treasure_e = "none"
treasure_s = "none"
treasure_w = "none"

# 14
[[decks.deck]]
is_desert = false
oasis = []
treasure_n = "none"
treasure_e = "none"
treasure_s = "none"
treasure_w = "none"

# 15
[[decks.deck]]
is_desert = false
oasis = []
treasure_n = "none"
treasure_e = "none"
treasure_s = "none"
treasure_w = "none"

# 16
[[decks.deck]]
is_desert = false
oasis = []
treasure_n = "none"
treasure_e = "none"
treasure_s = "none"
treasure_w = "none"

# 17
[[decks.deck]]
is_desert = false
oasis = []
treasure_n = "none"
treasure_e = "none"
treasure_s = "none"
treasure_w = "none"


# Deck 4
[[decks]]
# 1
[[decks.deck]]
is_desert = false
oasis = []
treasure_n = "none"
treasure_e = "none"
treasure_s = "none"
treasure_w = "none"

# 2
[[decks.deck]]
is_desert = false
oasis = []
treasure_n = "none"
treasure_e = "none"
treasure_s = "none"
treasure_w = "none"

# 3
[[decks.deck]]
is_desert = false
oasis = []
treasure_n = "none"
treasure_e = "none"
treasure_s = "none"
treasure_w = "none"

# 4
[[decks.deck]]
is_desert = false
oasis = []
treasure_n = "none"
treasure_e = "none"
treasure_s = "none"
treasure_w = "none"

# 5
[[decks.deck]]
is_desert = false
oasis = []
treasure_n = "none"
treasure_e = "none"
treasure_s = "none"
treasure_w = "none"

# 6
[[decks.deck]]
is_desert = false
oasis = []
treasure_n = "none"
treasure_e = "none"
treasure_s = "none"
treasure_w = "none"

# 7
[[decks.deck]]
is_desert = false
oasis = []
treasure_n = "none"
treasure_e = "none"
treasure_s = "none"
treasure_w = "none"

# 8
[[decks.deck]]
is_desert = false
oasis = []
treasure_n = "none"
treasure_e = "none"
treasure_s = "none"
treasure_w = "none"

# 9
[[decks.deck]]
is_desert = false
oasis = []
treasure_n = "none"
treasure_e = "none"
treasure_s = "none"
treasure_w = "none"

# 10
[[decks.deck]]
is_desert = false
oasis = []
treasure_n = "none"
treasure_e = "none"
treasure_s = "none"
treasure_w = "none"

# 11
[[decks.deck]]
is_desert = false
oasis = []
treasure_n = "none"
treasure_e = "none"
treasure_s = "none"
treasure_w = "none"

# 12
[[decks.deck]]
is_desert = false
oasis = []
treasure_n = "none"
treasure_e = "none"
treasure_s = "none"
treasure_w = "none"

# 13
[[decks.deck]]
is_desert = false
oasis = []
treasure_n = "none"
treasure_e = "none"
treasure_s = "none"
treasure_w = "none"

# 14
[[decks.deck]]
is_desert = false
oasis = []
treasure_n = "none"
treasure_e = "none"
treasure_s = "none"
treasure_w = "none"

# 15
[[decks.deck]]
is_desert = false
oasis = []
treasure_n = "none"
treasure_e = "none"
treasure_s = "none"
treasure_w = "none"

# 16
[[decks.deck]]
is_desert = false
oasis = []
treasure_n = "none"
treasure_e = "none"
treasure_s = "none"
treasure_w = "none"

# 17
[[decks.deck]]
is_desert = false
oasis = []
treasure_n = "none"
treasure_e = "none"
treasure_s = "none"
treasure_w = "none"


# Deck 5
[[decks]]
# 1
[[decks.deck]]
is_desert = false
oasis = []
treasure_n = "none"
treasure_e = "none"
treasure_s = "none"
treasure_w = "none"

# 2
[[decks.deck]]
is_desert = false
oasis = []
treasure_n = "none"
treasure_e = "none"
treasure_s = "none"
treasure_w = "none"

# 3
[[decks.deck]]
is_desert = false
oasis = []
treasure_n = "none"
treasure_e = "none"
treasure_s = "none"
treasure_w = "none"

# 4
[[decks.deck]]
is_desert = false
oasis = []
treasure_n = "none"
treasure_e = "none"
treasure_s = "none"
treasure_w = "none"

# 5
[[decks.deck]]
is_desert = false
oasis = []
treasure_n = "none"
treasure_e = "none"
treasure_s = "none"
treasure_w = "none"

# 6
[[decks.deck]]
is_desert = false
oasis = []
treasure_n = "none"
treasure_e = "none"
treasure_s = "none"
treasure_w = "none"

# 7
[[decks.deck]]
is_desert = false
oasis = []
treasure_n = "none"
treasure_e = "none"
treasure_s = "none"
treasure_w = "none"

# 8
[[decks.deck]]
is_desert = false
oasis = []
treasure_n = "none"
treasure_e = "none"
treasure_s = "none"
treasure_w = "none"

# 9
[[decks.deck]]
is_desert = false
oasis = []
treasure_n = "none"
treasure_e = "none"
treasure_s = "none"
treasure_w = "none"

# 10
[[decks.deck]]
is_desert = false
oasis = []
treasure_n = "none"
treasure_e = "none"
treasure_s = "none"
treasure_w = "none"

# 11
[[decks.deck]]
is_desert = false
oasis = []
treasure_n = "none"
treasure_e = "none"
treasure_s = "none"
treasure_w = "none"

# 12
[[decks.deck]]
is_desert = false
oasis = []
treasure_n = "none"
treasure_e = "none"
treasure_s = "none"
treasure_w = "none"

# 13
[[decks.deck]]
is_desert = false
oasis = []
treasure_n = "none"
treasure_e = "none"
treasure_s = "none"
treasure_w = "none"

# 14
[[decks.deck]]
is_desert = false
oasis = []
treasure_n = "none"
treasure_e = "none"
treasure_s = "none"
treasure_w = "none"

# 15
[[decks.deck]]
is_desert = false
oasis = []
treasure_n = "none"
treasure_e = "none"
treasure_s = "none"
treasure_w = "none"

# 16
[[decks.deck]]
is_desert = false
oasis = []
treasure_n = "none"
treasure_e = "none"
treasure_s = "none"
treasure_w = "none"

# 17
[[decks.deck]]
is_desert = false
oasis = []
treasure_n = "none"
treasure_e = "none"
treasure_s = "none"
treasure_w = "none"
"#,
        );

        let table = toml::from_str::<Table>(&input).unwrap();

        let parsed_config = TilesetConfig::try_from(&table);

        assert_matches!(parsed_config, Ok(_));

        let tileset_config = parsed_config.unwrap();

        assert_matches!(
            tileset_config,
            TilesetConfig {
                deck: [deck_one, _, deck_three, _, _],
                cross: CrossConfig { c, n, e:_, w, s:_ }
            } => {
                assert_eq!(c.is_desert, Some(false));
                assert_eq!(c.oasis, None);
                assert_matches!(n, array => {
                    assert_eq!(array[0].is_desert, Some(false));
                    assert_eq!(array[0].oasis, Some(OasisLayoutFlags::W1));
                    assert_eq!(array[1].oasis, Some(OasisLayoutFlags::E1));
                    assert_eq!(array[2].oasis, Some(OasisLayoutFlags::W1));
                });
                assert_matches!(w, array => {
                    assert_eq!(array[1].is_desert, Some(false));
                    assert_eq!(array[0].oasis, None);
                    assert_eq!(array[1].oasis, Some(OasisLayoutFlags::N1 | OasisLayoutFlags::S2));
                    assert_eq!(array[2].oasis, None);
                });
                assert_matches!(deck_one, deck => {
                    assert_eq!(deck.0[0].is_desert, Some(true));
                    assert_eq!(deck.0[1].is_desert, Some(false));
                    assert_eq!(deck.0[0].oasis, Some(OasisLayoutFlags::E1 | OasisLayoutFlags::S1));
                    assert_eq!(deck.0[0].treasure_e, Some(String::from("salt")));
                    assert_eq!(deck.0[0].treasure_s, Some(String::from("rumors")));
                    assert_eq!(deck.0[1].oasis, Some(OasisLayoutFlags::N1));
                    assert_eq!(deck.0[2].oasis, Some(OasisLayoutFlags::empty()));
                });
                assert_matches!(deck_three, deck => {
                    dbg!(&deck);
                    assert_eq!(deck.0[0].is_desert, Some(true));
                    assert_eq!(deck.0[1].is_desert, Some(false));
                    assert_eq!(deck.0[0].oasis, Some(OasisLayoutFlags::empty()));
                    assert_eq!(deck.0[1].oasis, Some(OasisLayoutFlags::E1));
                    assert_eq!(deck.0[2].oasis, Some(OasisLayoutFlags::S1));
                });
            }
        );
    }
}
