use visioncortex::color_clusters::{Runner, RunnerConfig};
use visioncortex::image::ColorImage;
use visioncortex::color::Color;
use visioncortex::path::PathSimplifyMode;

use image::{DynamicImage, Pixel};

use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use clap::{Arg, App};

fn convert_to_color_image(img: DynamicImage) -> ColorImage {
    let image_rgba = img.to_rgba();
    let mut new_image = ColorImage::new_w_h(image_rgba.width() as usize, image_rgba.height() as usize);

    for (x, y, pixel) in image_rgba.enumerate_pixels() {
        let channels = pixel.channels();
        new_image.set_pixel(x as usize, y as usize, &Color {
            r: channels[0] as u8,
            g: channels[1] as u8,
            b: channels[2] as u8,
            a: channels[3] as u8
        })
    }

    new_image
}

struct SvgFill {
    path_string: String,
    color: Color
}

struct SvgBuilder {
    fill: Vec<SvgFill>,
    width: usize,
    height: usize,
}

impl SvgBuilder {
    fn new(width: usize, height: usize) -> SvgBuilder {
        SvgBuilder {
            fill: Vec::new(),
            width,
            height,
        }
    }

    fn add_fill(&mut self, path: String, color: Color) {
        self.fill.push(SvgFill {
            path_string: path,
            color,
        })
    }

    fn to_vector_file(&self) -> String {
        let mut result = format!(r#"<?xml version="1.0" encoding="UTF-8"?>
        <svg width="{}" height="{}">
        "#, self.width, self.height);

        for fill in &self.fill {
            let color = fill.color;
            result.push_str(&format!("<path d=\"{}\" fill=\"#{:02x}{:02x}{:02x}\"/>\n", fill.path_string, color.r, color.g, color.b));
        };

        result.push_str("</svg>");
        result
    }
}

fn main() {
    let matches = App::new("convert bitmap picture to vector picture")
        .arg(Arg::with_name("input")
            .short("i")
            .required(true)
            .takes_value(true))
        .arg(Arg::with_name("output")
            .short("o")
            .required(true)
            .takes_value(true))
        .get_matches();

    let input_path = PathBuf::from(matches.value_of("input").unwrap());
    let output_path = PathBuf::from(matches.value_of("output").unwrap());

    // first, try to run this
    let img = image::open(input_path).unwrap();
    let image_converted = convert_to_color_image(img);

    let (width, height) = (image_converted.width, image_converted.height);

    let runner = Runner::new(RunnerConfig {
        batch_size: 25600,
        good_min_area: 4,
        good_max_area: (width * height),
        is_same_color_a: 2,
        is_same_color_b: 1,
        deepen_diff: 4,
        hollow_neighbours: 1,
    }, image_converted);

    let mut clustering = runner.start();

    while !clustering.tick() {
        println!("clustering tick");
    };

    let clusters = clustering.result();

    let view = clusters.view();

    let mut svg = SvgBuilder::new(width, height);
    for this_cluster_output in view.clusters_output.iter().rev() {
        println!("vectorize tick");
        let cluster = view.get_cluster(*this_cluster_output);
        let svg_path = cluster.to_svg_path(
            &view,
            false,
            PathSimplifyMode::Polygon,
            10.0,
            4.0,
            32,
            15.0
        );
        svg.add_fill(svg_path, cluster.residue_color());
    };

    let mut out_file = File::create(output_path).unwrap();
    out_file.write_all(&svg.to_vector_file().as_bytes()).unwrap();
}


// gradient step: 32
