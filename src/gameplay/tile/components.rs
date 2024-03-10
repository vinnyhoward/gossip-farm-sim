pub struct CSVMapData {
    pub csv_file_path: &'static str,
    pub spritesheet_resource_name: &'static str,
    pub collision_indices: Vec<i32>,
    pub passive_collision_indices: Vec<i32>,
    pub z_index: f32,
    pub is_vertically_inverted: bool,
}