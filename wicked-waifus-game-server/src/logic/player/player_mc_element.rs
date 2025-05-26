use crate::config;
use crate::logic::player::player_mc_element::Element::{
    Aero, Electro, Fusion, Glacio, Havoc, Spectro,
};
use wicked_waifus_protocol_internal::PlayerMcElementData;

pub enum Element {
    Glacio,
    Fusion,
    Electro,
    Aero,
    Spectro,
    Havoc,
}

pub struct PlayerMcElement {
    pub unlocked_elements: Vec<Element>,
    pub current_element: Element,
}

impl PlayerMcElement {
    pub fn load_from_save(data: PlayerMcElementData) -> Self {
        Self {
            unlocked_elements: data
                .unlocked_elements
                .iter()
                .map(|&element| Element::from(element))
                .collect(),
            current_element: Element::from(data.current_element),
        }
    }

    pub fn build_save_data(&self) -> PlayerMcElementData {
        PlayerMcElementData {
            unlocked_elements: self
                .unlocked_elements
                .iter()
                .map(|element| element.into())
                .collect(),
            current_element: (&self.current_element).into(),
        }
    }
}

impl From<i32> for Element {
    fn from(value: i32) -> Self {
        match value {
            0 => Glacio,
            1 => Fusion,
            2 => Electro,
            3 => Aero,
            4 => Spectro,
            5 => Havoc,
            _ => unreachable!(),
        }
    }
}

impl Into<i32> for &Element {
    fn into(self) -> i32 {
        match self {
            Glacio => 0,
            Fusion => 1,
            Electro => 2,
            Aero => 3,
            Spectro => 4,
            Havoc => 5,
        }
    }
}

impl Default for PlayerMcElement {
    fn default() -> Self {
        Self {
            unlocked_elements: match config::get_config().default_unlocks.unlock_all_mc_elements {
                true => vec![Spectro, Havoc, Aero],
                false => vec![Spectro],
            },
            current_element: Spectro,
        }
    }
}
