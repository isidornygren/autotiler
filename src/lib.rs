extern crate image;

use image::{
    error::{ParameterError, ParameterErrorKind},
    imageops::{crop, overlay},
    ImageError, ImageFormat, RgbImage,
};

mod constants;
mod neighbours;

pub fn build<P>(from: P, to: P) -> Result<(), ImageError>
where
    P: AsRef<std::path::Path>,
{
    let mut from_img = image::open(from)?.into_rgb8();
    let built = build_autotile_texture(&mut from_img)?;

    built.save_with_format(to, ImageFormat::Png)?;
    Ok(())
}

fn build_autotile_texture(image: &mut RgbImage) -> Result<RgbImage, ImageError> {
    if image.width() % 6 != 0
        || image.height() % 8 != 0
        || image.height() != ((image.width() / 6) * 8)
    {
        return Err(ImageError::Parameter(ParameterError::from_kind(
            ParameterErrorKind::DimensionMismatch,
        )));
    }

    let sprite_width = image.width() / 6;
    let sprite_height = image.height() / 8;

    let sprites: Vec<(u32, u32)> = [0; 6 * 8]
        .iter()
        .enumerate()
        .map(|(index, _)| {
            let col = index as u32 % 6;
            let row = index as u32 / 6;

            (col * sprite_width, row * sprite_height)
        })
        .collect();

    // Create a texture canvas to draw the autotile on
    let mut draw_image = RgbImage::new(sprite_width * 16, sprite_height * 12);

    for (n, value) in constants::AUTOTILE_REORDERING.iter().enumerate() {
        for (m, sprite_value) in value.iter().enumerate() {
            let (sprite_x, sprite_y) = sprites
                .get(*sprite_value as usize)
                .expect("Could not load autotile sprite");
            println!(
                "Sprite: {:?}, {:?}, {:?}, {:?}",
                sprite_x, sprite_y, sprite_width, sprite_height
            );
            let current_sprite = crop(image, *sprite_x, *sprite_y, sprite_width, sprite_height);

            let x = m as u32 % 2;
            let y = m as u32 / 2;

            let x_pos = n as u32 % 8;
            let y_pos = n as u32 / 8;

            overlay(
                &mut draw_image,
                &current_sprite.to_image(),
                (x_pos * sprite_width * 2 + x * sprite_width) as i64,
                (y_pos * sprite_height * 2 + y * sprite_height) as i64,
            )
        }
    }

    Ok(draw_image)
}

fn get_marching_tile_byte(neighbours: u8) -> u8 {
    let mut sample = 0;
    // top left corner
    if neighbours & 0b11010000 == 0b11010000 {
        sample += 1;
    }
    // North
    if neighbours & 0b01000000 == 0b01000000 {
        sample += 2;
    }
    // top right corner
    if neighbours & 0b01101000 == 0b01101000 {
        sample += 4;
    }
    // West
    if neighbours & 0b00010000 == 0b00010000 {
        sample += 8;
    }
    // East
    if neighbours & 0b00001000 == 0b00001000 {
        sample += 16;
    }
    // bottom left corner
    if neighbours & 0b00010110 == 0b00010110 {
        sample += 32;
    }
    // South
    if neighbours & 0b00000010 == 0b00000010 {
        sample += 64;
    }
    // bottom right corner
    if neighbours & 0b00001011 == 0b00001011 {
        sample += 128;
    }
    sample
}

/// Input is a byte which represents [nw, n, ne, w, e, sw, s, se]
pub fn get_marching_tile_index(neighbours: u8) -> u8 {
    let index = get_marching_tile_byte(neighbours);
    constants::MARCHING_TILES[index as usize]
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{open, ImageBuffer};

    #[test]
    fn it_builds_an_autotile_texture() {
        let mut image = ImageBuffer::from_fn(6, 8, |x, y| {
            image::Rgb([x as u8 * (255 / 6), y as u8 * (255 / 8), 255])
        });
        let results = build_autotile_texture(&mut image).unwrap();
        let expected_output = open("expected_output.png").unwrap().into_rgb8();

        assert_eq!(results, expected_output);
    }

    #[test]
    fn it_opens_and_saves_an_autotile_texture() {
        let results = build("input.png", "output.png");

        assert!(results.is_ok());
    }

    #[test]
    fn it_should_get_marching_tile_index_with_all_ones() {
        assert_eq!(get_marching_tile_byte(0xff), 0xff);
    }

    #[test]
    fn it_should_get_marching_non_corner_tiles() {
        assert_eq!(get_marching_tile_byte(0b01011010), 0x5a);
    }

    #[test]
    fn it_should_get_marching_tile_index_with_all_zeroes() {
        assert_eq!(get_marching_tile_byte(0x00), 0x00);
    }
}
