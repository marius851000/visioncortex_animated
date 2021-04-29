use clap::Clap;
use rayon::prelude::*;
use std::fs::read_dir;
use std::fs::DirEntry;
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use video2svg::image_to_svg;

#[derive(Clap)]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    SingleImage(SingleImageCommand),
    Batch(BatchCommand),
}

#[derive(Clap)]
pub struct SingleImageCommand {
    input: PathBuf,
    output: PathBuf,
}

#[derive(Clap)]
pub struct BatchCommand {
    input: PathBuf,
    output: PathBuf,
}

fn main() {
    let matches = Opts::parse();

    match matches.subcmd {
        SubCommand::SingleImage(subcommand) => {
            let img = image::open(subcommand.input).unwrap();

            let svg = image_to_svg(img);

            let mut out_file = File::create(subcommand.output).unwrap();
            out_file
                .write_all(&svg.to_vector_file().as_bytes())
                .unwrap();
        }
        SubCommand::Batch(subcommand) => {
            create_dir_all(&subcommand.output).unwrap();

            let mut list_of_file_in_inputs = vec![];

            for e in read_dir(&subcommand.input).unwrap() {
                list_of_file_in_inputs.push(e.unwrap());
            }

            let handle_direntry = |direntry: &DirEntry| {
                let source_path = direntry.path();
                let target_path = subcommand.output.clone().join(format!(
                    "{}.svg",
                    source_path.file_name().unwrap().to_str().unwrap()
                ));
                let target_path_png = target_path.parent().unwrap().join(format!(
                    "{}.png",
                    target_path.file_name().unwrap().to_str().unwrap()
                ));

                if !target_path_png.exists() {
                    let img = image::open(source_path).unwrap();
                    let svg = image_to_svg(img);
                    let mut out_file = File::create(&target_path).unwrap();
                    out_file
                        .write_all(&svg.to_vector_file().as_bytes())
                        .unwrap();
                    Command::new("inkscape")
                        .arg(target_path.to_str().unwrap())
                        .arg("-o")
                        .arg(target_path_png.to_str().unwrap())
                        .arg("-C")
                        .output()
                        .unwrap();
                }
            };

            list_of_file_in_inputs.par_iter().for_each(handle_direntry);
        }
    }
}
