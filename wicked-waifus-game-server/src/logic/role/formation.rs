use wicked_waifus_protocol_internal::RoleFormationData;

pub struct RoleFormation {
    pub id: i32,
    pub cur_role: i32,
    pub role_ids: Vec<i32>,
    pub is_current: bool,
}

// Will be updated every version
// const DEFAULT_FORMATION: &[i32] = &[5101, 1407, 1507];
const DEFAULT_FORMATION: &[i32] = &[1205, 1207, 1409];

impl RoleFormation {
    pub fn default_roles() -> &'static [i32] {
        DEFAULT_FORMATION
    }

    pub fn load_from_save(data: RoleFormationData) -> Self {
        Self {
            id: data.formation_id,
            cur_role: data.cur_role,
            role_ids: data.role_id_list,
            is_current: data.is_current,
        }
    }

    pub fn build_save_data(&self) -> RoleFormationData {
        RoleFormationData {
            formation_id: self.id,
            cur_role: self.cur_role,
            role_id_list: self.role_ids.iter().copied().collect(),
            is_current: self.is_current,
        }
    }
}

impl Default for RoleFormation {
    fn default() -> Self {
        Self {
            id: 1,
            cur_role: DEFAULT_FORMATION[0],
            role_ids: DEFAULT_FORMATION.to_vec(),
            is_current: true,
        }
    }
}
