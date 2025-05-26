use wicked_waifus_protocol::{AnimalDestroyRequest, AnimalDestroyResponse, AnimalDieRequest, AnimalDieResponse, AnimalDropRequest, AnimalDropResponse, EEntityType, ERemoveEntityType, EntityLivingStatusNotify, ErrorCode, LivingStatus};

use crate::logic::ecs::component::ComponentContainer;
use crate::logic::player::Player;
use crate::logic::utils::world_util;
use crate::query_components;

pub fn on_animal_die_request(
    player: &mut Player,
    request: AnimalDieRequest,
    response: &mut AnimalDieResponse,
) {
    tracing::warn!("AnimalDieRequest not fully implemented");
    player.notify(EntityLivingStatusNotify {
        id: request.entity_id,
        living_status: LivingStatus::Dead.into(),
        drop_vision_item: vec![],
    });
    get_animal_reward();
    response.error_code = ErrorCode::Success.into();
}

pub fn on_animal_drop_request(
    _player: &mut Player,
    _request: AnimalDropRequest,
    response: &mut AnimalDropResponse,
) {
    tracing::warn!("AnimalDropRequest not fully implemented");
    // Example for: 16818 - 166901770
    get_animal_reward();
    response.error_code = ErrorCode::Success.into();
}

pub fn on_animal_destroy_request(
    player: &mut Player,
    request: AnimalDestroyRequest,
    response: &mut AnimalDestroyResponse,
) {
    {
        let entity_id = request.entity_id;
        let world_ref = player.world.borrow();
        let world = world_ref.get_world_entity();
        let (Some(config), ) = query_components!(world,entity_id,EntityConfig) else {
            response.error_code = ErrorCode::ErrAnimalEntityNotExist.into();
            return;
        };
        if config.entity_type != EEntityType::Animal {
            response.error_code = ErrorCode::ErrNotAnimalEntity.into();
        }
    }
    world_util::remove_entity(player, request.entity_id, ERemoveEntityType::RemoveTypeNormal);
    response.error_code = ErrorCode::Success.into();
}

fn get_animal_reward() {
    // TODO:
    //      get entity->config_id,
    //      from level entity get reference,
    //      merge level entity with template,
    //      extract reward component
    //      get by reward id from DropPackage.json
    // NormalItemUpdateNotify
    // UpdateHandBookActiveStateMapNotify
    // ItemRewardNotify
}