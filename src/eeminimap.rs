use chrono::Local;
use crate::eetypes::{minimap_cell_t, minimap_t, minimap_terrain_t, player_color_t};

use image::{Pixel, Rgb, Rgba, RgbaImage};

fn logentry(msg: String) {
    println!(
        "[eeminimap]\t{:?}\t{}\t{}",
        std::thread::current().id(),
        Local::now().format("%Y-%m-%d %H:%M:%S.%f"),
        msg
    );
}

fn parse_minimap_pixel(pixel: Rgba<u8>) -> minimap_cell_t {
    match pixel.to_rgb() {
        // fog
        Rgb([0, 0, 0]) => minimap_cell_t::fog,

        // frame: this is a tricky one prob. But useful af
        Rgb([255, 75, 125]) => minimap_cell_t::frame,


        // Terrain
        //
        // grass
        Rgb([0, 68, 0]) => minimap_cell_t::terrain { t_type: minimap_terrain_t::grass },
        Rgb([0, 52, 0]) => minimap_cell_t::obstacle { t_type: minimap_terrain_t::grass },

        // snow
        Rgb([176, 196, 192]) => minimap_cell_t::terrain { t_type: minimap_terrain_t::snow },
        Rgb([136, 156, 152]) => minimap_cell_t::obstacle { t_type: minimap_terrain_t::snow },

        // lava
        Rgb([72, 76, 72]) => minimap_cell_t::terrain { t_type: minimap_terrain_t::lava },
        Rgb([40, 44, 40]) => minimap_cell_t::obstacle { t_type: minimap_terrain_t::lava },

        // dirt
        Rgb([80, 60, 8]) => minimap_cell_t::terrain { t_type: minimap_terrain_t::dirt },
        Rgb([56, 44, 8]) => minimap_cell_t::obstacle { t_type: minimap_terrain_t::dirt },

        // subterranean
        Rgb([128, 48, 0]) => minimap_cell_t::terrain { t_type: minimap_terrain_t::subterranean },
        Rgb([88, 8, 0]) => minimap_cell_t::obstacle { t_type: minimap_terrain_t::subterranean },

        // rough
        Rgb([128, 112, 24]) => minimap_cell_t::terrain { t_type: minimap_terrain_t::rough },
        Rgb([96, 84, 32]) => minimap_cell_t::obstacle { t_type: minimap_terrain_t::rough },

        // swamp
        Rgb([72, 128, 104]) => minimap_cell_t::terrain { t_type: minimap_terrain_t::swamp },
        Rgb([32, 88, 64]) => minimap_cell_t::obstacle { t_type: minimap_terrain_t::swamp },

        // highlands
        Rgb([40, 112, 24]) => minimap_cell_t::terrain { t_type: minimap_terrain_t::highlands },
        Rgb([32, 84, 16]) => minimap_cell_t::obstacle { t_type: minimap_terrain_t::highlands },

        // wasteland
        Rgb([184, 88, 8]) => minimap_cell_t::terrain { t_type: minimap_terrain_t::wasteland },
        Rgb([152, 64, 8]) => minimap_cell_t::obstacle { t_type: minimap_terrain_t::wasteland },

        // sand
        Rgb([216, 200, 136]) => minimap_cell_t::terrain { t_type: minimap_terrain_t::sand },
        Rgb([160, 152, 104]) => minimap_cell_t::obstacle { t_type: minimap_terrain_t::sand },

        // players
        Rgb([128, 132, 128]) => minimap_cell_t::object { owner: player_color_t::neutral },
        Rgb([248, 0, 0]) => minimap_cell_t::object { owner: player_color_t::red },
        Rgb([48, 80, 248]) => minimap_cell_t::object { owner: player_color_t::blue },
        Rgb([160, 128, 80]) => minimap_cell_t::object { owner: player_color_t::tan },
        Rgb([32, 148, 0]) => minimap_cell_t::object { owner: player_color_t::green },
        Rgb([248, 132, 0]) => minimap_cell_t::object { owner: player_color_t::orange },
        Rgb([136, 40, 160]) => minimap_cell_t::object { owner: player_color_t::purple },
        Rgb([8, 156, 160]) => minimap_cell_t::object { owner: player_color_t::teal },
        Rgb([192, 120, 136]) => minimap_cell_t::object { owner: player_color_t::pink },


        _ => {
            logentry(format!("Unhandled pix {:?}", pixel));
            minimap_cell_t::fog
        }
    }
}

pub fn parse_minimap(img: &RgbaImage, map_dim: usize) -> minimap_t {
    // currently this works well only for map_dim == 144
    minimap_t {
        map_size: map_dim,
        cells: (0..map_dim).map(|x|
            (0..map_dim).map(|y| parse_minimap_pixel(*img.get_pixel(x as u32, y as u32))).collect()
        ).collect()
    }
}

pub fn locate_topleft_corner(img: &RgbaImage, map_dim: usize) -> Option<(u32, u32)> {
    // currently this works well only for map_dim == 144
    for x in 0..map_dim {
        for y in 0..map_dim {
            if img.get_pixel(x as u32, y as u32).0[..3] == [255, 75, 125] {
                return Some((x as u32, y as u32));
            }
        }
    }
    logentry(format!("Unable to locate topleft frame corner on a minimap screenshot"));
    None
}
