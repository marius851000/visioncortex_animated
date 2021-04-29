use image::{DynamicImage, Pixel};
use visioncortex::{Color, ColorImage};

pub fn dynamicimage_to_colorimage(img: DynamicImage) -> ColorImage {
    let image_rgba = img.to_rgba();
    let mut new_image =
        ColorImage::new_w_h(image_rgba.width() as usize, image_rgba.height() as usize);

    for (x, y, pixel) in image_rgba.enumerate_pixels() {
        let channels = pixel.channels();
        new_image.set_pixel(
            x as usize,
            y as usize,
            &Color {
                r: channels[0] as u8,
                g: channels[1] as u8,
                b: channels[2] as u8,
                a: channels[3] as u8,
            },
        )
    }

    new_image
}
