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
        const N3 = (CardinalDirectionFlags::N.bits() as u16) << 8;
        const E3 = (CardinalDirectionFlags::E.bits() as u16) << 8;
        const S3 = (CardinalDirectionFlags::S.bits() as u16) << 8;
        const W3 = (CardinalDirectionFlags::W.bits() as u16) << 8;
        const N4 = (CardinalDirectionFlags::N.bits() as u16) << 12;
        const E4 = (CardinalDirectionFlags::E.bits() as u16) << 12;
        const S4 = (CardinalDirectionFlags::S.bits() as u16) << 12;
        const W4 = (CardinalDirectionFlags::W.bits() as u16) << 12;
    }
}

impl OasisLayoutFlags {
    pub fn from_cardinal_direction_flags(f: &CardinalDirectionFlags, idx: u8) -> Self {
        let bits = (f.bits() as u16) << (4 * idx);
        OasisLayoutFlags::from_bits_truncate(bits)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CardinalDirection {
    N,
    E,
    S,
    W,
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
