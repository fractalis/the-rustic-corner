use rltk::{Rltk, RGB, to_cp437};

pub const MAP_WIDTH: usize = 80;
pub const MAP_HEIGHT: usize = 50;

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall, Floor
}

pub fn xy_idx(x: i32, y: i32) -> usize {
    (y as usize * 80) + x as usize
}

pub fn new_map() -> Vec<TileType> {
    let mut map = vec![TileType::Floor; MAP_WIDTH*MAP_HEIGHT];

    for x in 0..80 {
        map[xy_idx(x, 0)] = TileType::Wall;
        map[xy_idx(x, 49)] = TileType::Wall;
    }

    for y in 0..50 {
        map[xy_idx(0, y)] = TileType::Wall;
        map[xy_idx(79, y)] = TileType::Wall;
    }

    let mut rng = rltk::RandomNumberGenerator::new();

    for _i in 0..400 {
        let x = rng.roll_dice(1, MAP_WIDTH as i32 - 1);
        let y = rng.roll_dice(1, MAP_HEIGHT as i32 - 1);
        let idx = xy_idx(x, y);
        
        if idx != xy_idx(40, 25) {
            map[idx] = TileType::Wall;
        }
    }

    map
}

pub fn draw_map(map: &[TileType], ctx: &mut Rltk) {
    let mut y = 0;
    let mut x = 0;
    for tile in map.iter() {
        match tile {
            TileType::Floor => {
                ctx.set(x, y, RGB::from_f32(0.5, 0.5, 0.5), RGB::from_f32(0., 0., 0.), to_cp437('.'));
            }
            TileType::Wall => {
                ctx.set(x, y, RGB::from_f32(0., 1.0, 0.), RGB::from_f32(0., 0., 0.), to_cp437('#'));
            }
        }
        x += 1;
        if x > 79 {
            x = 0;
            y += 1;
        }
    }
}