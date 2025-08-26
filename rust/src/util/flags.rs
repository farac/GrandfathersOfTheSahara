use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, PartialEq)]
    pub struct CardinalDirectionFlags: u8 {
        const N = 0b0001;
        const E = 0b0010;
        const S = 0b0100;
        const W = 0b1000;
    }
}

impl CardinalDirectionFlags {
    fn rotate_left(&self, amount: u32) -> CardinalDirectionFlags {
        let rotated_u8 = self.bits().rotate_left(amount);
        let upper_half = rotated_u8 >> 4;

        CardinalDirectionFlags::from_bits_truncate(upper_half + rotated_u8)
    }
    fn rotate_right(&self, amount: u32) -> CardinalDirectionFlags {
        let rotated_u8 = self.bits().rotate_right(amount);
        let upper_half = rotated_u8 >> 4;

        CardinalDirectionFlags::from_bits_truncate(upper_half + rotated_u8)
    }
}

impl From<CardinalDirection> for CardinalDirectionFlags {
    fn from(value: CardinalDirection) -> Self {
        match value {
            CardinalDirection::N => CardinalDirectionFlags::N,
            CardinalDirection::E => CardinalDirectionFlags::E,
            CardinalDirection::S => CardinalDirectionFlags::S,
            CardinalDirection::W => CardinalDirectionFlags::W,
        }
    }
}

bitflags! {
    #[derive(Debug, Clone, PartialEq)]
    pub struct OasisLayoutFlags: u16 {
        const N1 = CardinalDirectionFlags::N.bits() as u16;
        const E1 = CardinalDirectionFlags::E.bits() as u16;
        const S1 = CardinalDirectionFlags::S.bits() as u16;
        const W1 = CardinalDirectionFlags::W.bits() as u16;

        const N2 = (CardinalDirectionFlags::N.bits() as u16) << 4;
        const E2 = (CardinalDirectionFlags::E.bits() as u16) << 4;
        const S2 = (CardinalDirectionFlags::S.bits() as u16) << 4;
        const W2 = (CardinalDirectionFlags::W.bits() as u16) << 4;

        const N3 = (CardinalDirectionFlags::N.bits() as u16) << (2 * 4);
        const E3 = (CardinalDirectionFlags::E.bits() as u16) << (2 * 4);
        const S3 = (CardinalDirectionFlags::S.bits() as u16) << (2 * 4);
        const W3 = (CardinalDirectionFlags::W.bits() as u16) << (2 * 4);

        const N4 = (CardinalDirectionFlags::N.bits() as u16) << (3 * 4);
        const E4 = (CardinalDirectionFlags::E.bits() as u16) << (3 * 4);
        const S4 = (CardinalDirectionFlags::S.bits() as u16) << (3 * 4);
        const W4 = (CardinalDirectionFlags::W.bits() as u16) << (3 * 4);
    }
}

impl OasisLayoutFlags {
    pub fn from_cardinal_direction_flags(f: &CardinalDirectionFlags, idx: u8) -> Self {
        let bits = (f.bits() as u16) << (4 * idx);
        OasisLayoutFlags::from_bits_truncate(bits)
    }
    fn to_chunks(&self) -> Vec<CardinalDirectionFlags> {
        let mut flags = self.bits();
        let mut vec_flags = vec![];
        let mut idx = 0;

        while flags > 0 {
            flags >>= 4 * idx;

            vec_flags.push(CardinalDirectionFlags::from_bits_truncate(flags as u8));
            idx += 1;
        }

        vec_flags
    }
    pub fn rotate_right(&self, amount: u32) -> Self {
        let chunks: Vec<CardinalDirectionFlags> = self.to_chunks();

        OasisLayoutFlags::from_bits_truncate(
            chunks
                .iter()
                .enumerate()
                .map(|(idx, f)| f.rotate_right(amount).bits().wrapping_shl(4 * idx as u32))
                .fold(0, |acc, f| acc + (f as u16)),
        )
    }
    pub fn rotate_left(&self, amount: u32) -> Self {
        let chunks: Vec<CardinalDirectionFlags> = self.to_chunks();

        OasisLayoutFlags::from_bits_truncate(
            chunks
                .iter()
                .enumerate()
                .map(|(idx, f)| f.rotate_left(amount).bits().wrapping_shl(4 * idx as u32))
                .fold(0, |acc, f| acc + (f as u16)),
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CardinalDirection {
    N,
    E,
    S,
    W,
}

impl CardinalDirection {
    pub fn get_coordinate_offset(&self) -> (i32, i32) {
        match self {
            CardinalDirection::N => (0, 1),
            CardinalDirection::E => (1, 0),
            CardinalDirection::S => (0, -1),
            CardinalDirection::W => (-1, 0),
        }
    }
}

impl From<usize> for CardinalDirection {
    fn from(value: usize) -> Self {
        match value % 4 {
            0 => CardinalDirection::N,
            1 => CardinalDirection::E,
            2 => CardinalDirection::S,
            3 => CardinalDirection::W,
            // It makes no sense to have any other number if we're using mod 4
            _ => panic!(),
        }
    }
}

impl From<OasisLayoutFlags> for CardinalDirectionFlags {
    fn from(value: OasisLayoutFlags) -> CardinalDirectionFlags {
        let mut acc = CardinalDirectionFlags::empty();
        let bits = value.bits();

        for idx in 0..4 {
            let res =
                CardinalDirectionFlags::from_bits_truncate((bits.rotate_right(4 * idx)) as u8);

            acc = res | acc;
        }

        acc
    }
}

impl CardinalDirection {
    fn _is_opposite(&self, side: &CardinalDirection) -> bool {
        self.invert() == *side
    }
    pub fn invert(&self) -> Self {
        match self {
            CardinalDirection::N => CardinalDirection::S,
            CardinalDirection::E => CardinalDirection::W,
            CardinalDirection::S => CardinalDirection::N,
            CardinalDirection::W => CardinalDirection::E,
        }
    }
}

pub const DIRECTIONS: [CardinalDirection; 4] = [
    CardinalDirection::N,
    CardinalDirection::E,
    CardinalDirection::S,
    CardinalDirection::W,
];

impl From<CardinalDirectionFlags> for Vec<CardinalDirection> {
    fn from(value: CardinalDirectionFlags) -> Self {
        let mut direction_vec = vec![];

        value.iter_names().for_each(|f| match f.0 {
            "N" => direction_vec.push(CardinalDirection::N),
            "E" => direction_vec.push(CardinalDirection::E),
            "S" => direction_vec.push(CardinalDirection::S),
            "W" => direction_vec.push(CardinalDirection::W),
            _ => (),
        });

        direction_vec
    }
}

impl From<&CardinalDirection> for &'static str {
    fn from(value: &CardinalDirection) -> Self {
        match value {
            CardinalDirection::N => "N",
            CardinalDirection::E => "E",
            CardinalDirection::S => "S",
            CardinalDirection::W => "W",
        }
    }
}

impl From<&CardinalDirection> for CardinalDirectionFlags {
    fn from(value: &CardinalDirection) -> Self {
        match value {
            CardinalDirection::N => CardinalDirectionFlags::N,
            CardinalDirection::E => CardinalDirectionFlags::E,
            CardinalDirection::S => CardinalDirectionFlags::S,
            CardinalDirection::W => CardinalDirectionFlags::W,
        }
    }
}

pub const OASIS_CONNECTION_LABELS: [&str; 6] =
    ["N | W", "N | E", "S | W", "S | E", "E | W", "N | S"];
const OASIS_CONNECTION_FLAGS: [CardinalDirectionFlags; 6] = [
    CardinalDirectionFlags::N.union(CardinalDirectionFlags::W),
    CardinalDirectionFlags::N.union(CardinalDirectionFlags::E),
    CardinalDirectionFlags::S.union(CardinalDirectionFlags::W),
    CardinalDirectionFlags::S.union(CardinalDirectionFlags::E),
    CardinalDirectionFlags::E.union(CardinalDirectionFlags::W),
    CardinalDirectionFlags::N.union(CardinalDirectionFlags::S),
];

impl From<CardinalDirectionFlags> for Vec<&str> {
    fn from(value: CardinalDirectionFlags) -> Self {
        OASIS_CONNECTION_FLAGS
            .into_iter()
            .enumerate()
            .flat_map(|(idx, connection)| {
                if value.contains(connection) {
                    Some(OASIS_CONNECTION_LABELS[idx])
                } else {
                    None
                }
            })
            .collect()
    }
}
