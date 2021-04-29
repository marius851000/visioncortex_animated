use crate::{dynamicimage_to_colorimage, SvgBuilder};
use image::DynamicImage;
use visioncortex::color_clusters::{Runner, RunnerConfig};
use visioncortex::path::PathSimplifyMode;

pub fn image_to_svg(image: DynamicImage) -> SvgBuilder {
    let image_converted = dynamicimage_to_colorimage(image);

    let (width, height) = (image_converted.width, image_converted.height);

    let runner = Runner::new(
        RunnerConfig {
            batch_size: 25600,
            good_min_area: 4,
            good_max_area: (width * height),
            is_same_color_a: 2,
            is_same_color_b: 1,
            deepen_diff: 4,
            hollow_neighbours: 1,
        },
        image_converted,
    );

    let mut clustering = runner.start();

    while !clustering.tick() {}

    let clusters = clustering.result();

    let view = clusters.view();

    let mut svg = SvgBuilder::new(width, height);
    for this_cluster_output in view.clusters_output.iter().rev() {
        let cluster = view.get_cluster(*this_cluster_output);
        let svg_path =
            cluster.to_svg_path(&view, false, PathSimplifyMode::Polygon, 10.0, 4.0, 32, 15.0);
        svg.add_fill(svg_path, cluster.residue_color());
    }

    svg
}
