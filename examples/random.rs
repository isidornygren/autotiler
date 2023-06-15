use autotiler::{build_autotile_texture, get_marching_tile_position, Neighbours};
use image::{
    imageops::{crop, overlay},
    ImageBuffer, ImageFormat, RgbImage,
};
use rand::Rng;

const WIDTH: u32 = 10;
const HEIGHT: u32 = 10;
const TILE_SIZE: u32 = 8;

#[derive(PartialEq)]
enum Tile {
    Grass,
    Water,
}

struct Map {
    pub width: u32,
    pub height: u32,
    pub data: Vec<Tile>,
}

impl Map {
    pub fn get(&self, x: i32, y: i32) -> Option<&Tile> {
        if x < 0 || y < 0 || x > (self.width - 1) as i32 || y > (self.height - 1) as i32 {
            return None;
        }
        let index = y as u32 * self.width + x as u32;
        self.data.get(index as usize)
    }

    pub fn index_to_coord(&self, index: usize) -> Option<(u32, u32)> {
        let index_u32: u32 = index as u32;
        let x = index_u32 % self.width;
        let y = index_u32 / self.width;
        if y > self.height {
            return None;
        }
        Some((x, y))
    }

    pub fn get_neighbours(&self, x: i32, y: i32) -> [Option<&Tile>; 8] {
        [
            self.get(x - 1, y - 1),
            self.get(x, y - 1),
            self.get(x + 1, y - 1),
            self.get(x - 1, y),
            self.get(x + 1, y),
            self.get(x - 1, y + 1),
            self.get(x, y + 1),
            self.get(x + 1, y + 1),
        ]
    }
}

fn main() {
    let grass_img: image::ImageBuffer<image::Rgb<u8>, Vec<u8>> =
        image::open("grass.png").unwrap().into_rgb8();
    let mut water_img: image::ImageBuffer<image::Rgb<u8>, Vec<u8>> =
        image::open("water.png").unwrap().into_rgb8();
    let autotile_texture: ImageBuffer<image::Rgb<u8>, Vec<u8>> =
        build_autotile_texture(&mut water_img).unwrap();

    let mut rng = rand::thread_rng();

    let map = Map {
        width: WIDTH,
        height: HEIGHT,
        data: (0..(HEIGHT * WIDTH))
            .map(|_| {
                if rng.gen_bool(0.5) {
                    Tile::Grass
                } else {
                    Tile::Water
                }
            })
            .collect(),
    };

    let mut map_image = RgbImage::new(WIDTH * TILE_SIZE, HEIGHT * TILE_SIZE);

    for (index, tile) in map.data.iter().enumerate() {
        let (x, y) = map.index_to_coord(index).unwrap();
        let mut image = match tile {
            Tile::Grass => grass_img.clone(),
            Tile::Water => autotile_texture.clone(),
        };
        let (tile_x, tile_y) = match tile {
            Tile::Water => {
                let neighbours = map.get_neighbours(x as i32, y as i32);

                let autotiler_neighbours = Neighbours::from_array(neighbours.map(|maybe_tile| {
                    maybe_tile.map_or(false, |tile| matches!(tile, Tile::Water))
                }));

                get_marching_tile_position(autotiler_neighbours)
            }
            Tile::Grass => (rng.gen_range(0..4), rng.gen_range(0..6)),
        };

        let current_sprite = crop(
            &mut image,
            tile_x as u32 * TILE_SIZE, // Get the marching square position here
            tile_y as u32 * TILE_SIZE, // Get the marching square position here
            TILE_SIZE,
            TILE_SIZE,
        );

        overlay(
            &mut map_image,
            &current_sprite.to_image(),
            (x * TILE_SIZE) as i64,
            (y * TILE_SIZE) as i64,
        )
    }

    map_image
        .save_with_format("random.png", ImageFormat::Png)
        .unwrap();
}
