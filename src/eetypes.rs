#[derive(Clone, Debug)]
pub enum minimap_terrain_t {
    grass,
    snow,
    lava,
    dirt,
    subterranean,
    rough,
    swamp,
    highlands,
    wasteland,
    sand,
    // water, // later!
    rock // might not be a play here. But might be differentiatable on cell screens who knows. Still should be useful
}

#[derive(Clone, Debug)]
pub enum player_color_t {
    neutral,
    red,
    blue,
    tan,
    green,
    orange,
    purple,
    teal,
    pink
}

#[derive(Clone, Debug)]
pub enum minimap_cell_t {
    fog,
    terrain {t_type: minimap_terrain_t},
    obstacle {t_type: minimap_terrain_t},
    object {owner: player_color_t}, // there may be many different types of object. And what about heroes? castles?
    frame,
}

#[derive(Debug)]
pub struct minimap_t {
    pub map_size: usize, // does this struct need to know its size?
    pub cells: Vec<Vec<minimap_cell_t>>
    // todo think of levels later (on/under ground)
}

pub struct worldmap_cell_object_t {

}

pub struct worldmap_cell_t {
    contents: Vec<worldmap_cell_object_t>
}

pub struct worldmap_snapshot_ {
    map_size: usize,
    minimap: minimap_t,
    cells: Vec<Vec<minimap_cell_t>>
}
