use wicked_waifus_protocol::entity_component_pb::ComponentPb;
use wicked_waifus_protocol::{
    BuffEffectCd, EntityComponentPb, FightBuffComponentPb, FightBuffInformation,
};

use crate::logic::ecs::component::Component;

#[derive(Default, Clone)]
pub struct FightBuff {
    pub fight_buff_infos: Vec<FightBuffInformation>,
    pub list_buff_effect_cd: Vec<BuffEffectCd>,
}

impl FightBuff {
    fn get_pb_data(pb: &wicked_waifus_protocol::EntityPb) -> Vec<&FightBuffComponentPb> {
        pb.component_pbs
            .iter()
            .filter_map(|pb| {
                if let Some(value) = &pb.component_pb {
                    match value {
                        ComponentPb::FightBuffComponent(result) => Some(result),
                        _ => None,
                    }
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
    }

    pub fn from_pb_data(pb: &wicked_waifus_protocol::EntityPb) -> Self {
        let pb_data = Self::get_pb_data(pb);
        match pb_data.get(0) {
            None => Self::default(),
            Some(pb) => Self {
                fight_buff_infos: pb.fight_buff_infos.clone(),
                list_buff_effect_cd: pb.list_buff_effect_cd.clone(),
                ..Default::default()
            },
        }
    }
}

impl Component for FightBuff {
    fn set_pb_data(&self, pb: &mut wicked_waifus_protocol::EntityPb) {
        // Remove existing FightBuffComponent
        pb.component_pbs.retain(|component_pb| {
            if let Some(value) = &component_pb.component_pb {
                match value {
                    ComponentPb::FightBuffComponent(_) => false,
                    _ => true,
                }
            } else {
                true
            }
        });

        // Add new FightBuffComponent
        pb.component_pbs.push(EntityComponentPb {
            component_pb: Some(ComponentPb::FightBuffComponent(FightBuffComponentPb {
                fight_buff_infos: self.fight_buff_infos.clone(),
                list_buff_effect_cd: self.list_buff_effect_cd.clone(),
            })),
        });
    }
}
