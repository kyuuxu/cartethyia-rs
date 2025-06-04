use std::string::ToString;
use std::sync::OnceLock;

use indexmap::IndexMap;
use wicked_waifus_protocol::entity_component_pb::ComponentPb;
use wicked_waifus_protocol::{
    DFsm, DFsmBlackBoard, EntityComponentPb, EntityFsmComponentPb, FsmCustomBlackboardDatas,
};

use wicked_waifus_data::{
    ai_base_data, ai_state_machine_config_data, AiStateMachineConfigData, StateMachineJson,
    StateMachineNode, StateMachineNodeCommon,
};

use crate::logic::ecs::component::Component;

static COMMON_FSM: OnceLock<AiStateMachineConfigData> = OnceLock::new();

#[derive(Default)]
pub struct Fsm {
    pub hash_code: i32,
    pub common_hash_code: i32,
    pub state_list: Vec<i32>,
    pub node_list: IndexMap<i32, StateMachineNodeCommon>,
}

impl Component for Fsm {
    fn set_pb_data(&self, pb: &mut wicked_waifus_protocol::EntityPb) {
        pb.component_pbs.push(EntityComponentPb {
            component_pb: Some(ComponentPb::EntityFsmComponentPb(EntityFsmComponentPb {
                fsms: self.get_initial_fsm(),
                hash_code: self.hash_code,
                common_hash_code: self.common_hash_code,
                black_board: self.get_black_board(),
                fsm_custom_blackboard_datas: self.get_fsm_custom_blackboard_datas(),
            })),
        })
    }
}

impl Fsm {
    pub fn from_ai_id(ai_id: i32) -> Self {
        let ai_base = ai_base_data::get(&ai_id);
        let Some(base) = ai_base else {
            tracing::error!("Ai Base not found for AI ID: {}", ai_id);
            return Self::default();
        };

        if base.state_machine.is_empty() {
            return Self::default();
        }

        let common_state_machine: &StateMachineJson = &get_common_fsm().state_machine_json;
        // Should always be defined since it comes from bindata
        let Some(state_machine_config_data) =
            ai_state_machine_config_data::get(&base.state_machine)
        else {
            tracing::error!("State machine config not found for AI ID: {}", ai_id);
            return Self::default();
        };
        let state_machine_config: &StateMachineJson = &state_machine_config_data.state_machine_json;
        let mut fsm_tree: IndexMap<i32, StateMachineNodeCommon> =
            IndexMap::with_capacity(state_machine_config.nodes.len());
        for state_machine in &state_machine_config.state_machines {
            for node in &state_machine_config.nodes {
                let uuid = match node {
                    StateMachineNode::Reference(n) => n.common.uuid,
                    StateMachineNode::Override(n) => n.common.uuid,
                    StateMachineNode::Custom(n) => n.common.uuid,
                };

                if uuid != *state_machine {
                    continue;
                };

                match node {
                    StateMachineNode::Reference(node) => {
                        if let Some(reference) = common_state_machine
                            .nodes
                            .iter()
                            .filter_map(|state_machine_node| match state_machine_node {
                                StateMachineNode::Reference(_) => None,
                                StateMachineNode::Override(_) => None,
                                StateMachineNode::Custom(custom) => Some(custom),
                            })
                            .find(|f| f.common.uuid == node.reference_uuid)
                        {
                            fsm_tree.insert(reference.common.uuid, reference.common.clone());
                        }
                    }
                    StateMachineNode::Override(node) => {
                        // TODO:
                        tracing::warn!(
                            "FSM: {state_machine} with override node {} for {} unimplemented",
                            node.common.uuid,
                            node.override_common_uuid
                        );
                    }
                    StateMachineNode::Custom(node) => {
                        fsm_tree.insert(node.common.uuid, node.common.clone());
                    }
                }
            }
        }
        Self {
            hash_code: state_machine_config.version as i32,
            common_hash_code: common_state_machine.version as i32,
            // state_list: state_machine_config.state_machines.clone(), // idk what this for ??
            state_list: Vec::new(),
            node_list: fsm_tree,
        }
    }

    fn get_initial_fsm(&self) -> Vec<DFsm> {
        self.node_list
            .iter()
            .map(|(&id, node)| DFsm {
                fsm_id: id,
                current_state: node
                    .children
                    .as_ref()
                    .and_then(|c| c.get(0).cloned())
                    .unwrap_or_default(),
                flag: if node.is_anim_state_machine.unwrap_or(false) {
                    1
                } else {
                    0
                },
                k_ts: 0, // TODO
            })
            .collect()
    }

    fn get_black_board(&self) -> Vec<DFsmBlackBoard> {
        vec![]
    }

    fn get_fsm_custom_blackboard_datas(&self) -> Option<FsmCustomBlackboardDatas> {
        None
    }
}

fn get_common_fsm() -> &'static AiStateMachineConfigData {
    COMMON_FSM.get_or_init(|| {
        let name = "SM_Common".to_string();
        // Common shall be always defined
        ai_state_machine_config_data::get(&name).cloned().unwrap()
    })
}
