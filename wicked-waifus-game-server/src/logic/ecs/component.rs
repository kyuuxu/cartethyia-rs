use wicked_waifus_protocol::EntityPb;

use crate::logic::components::*;

macro_rules! impl_component_container {
    ($($comp:ident;)*) => {
        pub enum ComponentContainer {
        $(
            $comp($comp),
        )*
        }

        impl ComponentContainer {
            pub fn set_pb_data(&self, pb: &mut wicked_waifus_protocol::EntityPb) {
                match self {
                $(
                    Self::$comp(comp) => comp.set_pb_data(pb),
                )*
                }
            }
        }
    };
}

impl_component_container! {
    Position;
    EntityConfig;
    OwnerPlayer;
    Visibility;
    Attribute;
    PlayerOwnedEntityMarker;
    Movement;
    Equip;
    VisionSkill;
    MonsterAi;
    Fsm;
    RoleSkin;
    FightBuff;
    StateTag;
    Tag;
    Autonomous;
    Interact;
    Concomitant;
    Summoner;
    SoarWingSkin;
    ParaglidingSkin;
    WeaponSkin;
}

pub trait Component {
    fn set_pb_data(&self, pb: &mut EntityPb);
}
