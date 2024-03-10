use crate::gameplay::tile::components::{NpcSpawnData, PlayerSpawnData};

pub fn get_npc_spawn_data() -> Vec<NpcSpawnData> {
    vec![NpcSpawnData {
        csv_file_path: "assets/tiled_maps/csv/gossip_farm_sim_v2_cow_spawn_points.csv",
        z_index: 901.,
    }]
}

pub fn get_player_spawn_data() -> Vec<PlayerSpawnData> {
    vec![PlayerSpawnData {
        csv_file_path: "assets/tiled_maps/csv/gossip_farm_sim_v2_character_spawn_points.csv",
        z_index: 900.,
    }]
}
