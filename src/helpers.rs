use crate::{constants, neighbours::Neighbours};

fn get_marching_tile_byte(neighbours: &Neighbours) -> u8 {
    let mut sample = 0;
    if neighbours.north_west() {
        sample += 1;
    }
    if neighbours.north() {
        sample += 2;
    }
    if neighbours.north_east() {
        sample += 4;
    }
    if neighbours.west() {
        sample += 8;
    }
    if neighbours.east() {
        sample += 16;
    }
    if neighbours.south_west() {
        sample += 32;
    }
    if neighbours.south() {
        sample += 64;
    }
    if neighbours.south_east() {
        sample += 128;
    }
    sample
}

pub fn get_tile_position(neighbours: Neighbours) -> (u8, u8) {
    let index = get_marching_tile_byte(&neighbours);
    let img_index = constants::MARCHING_TILES[index as usize];
    let width = 8;

    let x = img_index % width;
    let y = img_index / width;

    (x, y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_get_marching_tile_index_with_all_ones() {
        assert_eq!(get_marching_tile_byte(&Neighbours::from_byte(0xff)), 0xff);
    }

    #[test]
    fn it_should_get_marching_non_corner_tiles() {
        assert_eq!(
            get_marching_tile_byte(&Neighbours::from_byte(0b01011010)),
            0x5a
        );
    }

    #[test]
    fn it_should_get_marching_tile_index_with_all_zeroes() {
        assert_eq!(get_marching_tile_byte(&Neighbours::from_byte(0x00)), 0x00);
    }
}
