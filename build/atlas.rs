use std::{
    collections::HashMap,
    fs::{self, File},
    path::Path,
};

use image::{
    codecs::png::{CompressionType, FilterType, PngEncoder},
    DynamicImage, ExtendedColorType, ImageEncoder,
};
use serde::{Deserialize, Serialize};

use texture_packer::{
    exporter::ImageExporter, importer::ImageImporter, TexturePacker, TexturePackerConfig,
};

#[derive(Serialize, Deserialize)]
pub struct Frame {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub rotated: bool,
    pub trimmed: bool,
}

pub fn create_atlas(dir: &str, output: &str, max_width: u32, max_height: u32) {
    let config = TexturePackerConfig {
        max_width,
        max_height,
        ..Default::default()
    };

    let mut packer = TexturePacker::new_skyline(config);

    add_dir_to_atlas(&mut packer, dir);
    export_atlas(&packer, output);
}

pub fn create_atlas_joined(dirs: Vec<&str>, output: &str, max_width: u32, max_height: u32) {
    let config = TexturePackerConfig {
        max_width,
        max_height,
        ..Default::default()
    };

    let mut packer = TexturePacker::new_skyline(config);

    for dir in dirs {
        add_dir_to_atlas(&mut packer, dir);
    }

    export_atlas(&packer, output);
}

fn add_dir_to_atlas(packer: &mut TexturePacker<DynamicImage, String>, dir: &str) {
    let dir = fs::read_dir(dir).unwrap();

    for file in dir {
        let file = file.unwrap();

        if file.metadata().unwrap().is_dir() {
            continue;
        }

        let file_path = file.path();
        let path = Path::new(file_path.to_str().unwrap());
        let name = path.file_stem().unwrap().to_owned().into_string().unwrap();

        let texture = ImageImporter::import_from_file(path).unwrap();
        packer.pack_own(name, texture).unwrap();
    }
}

fn export_atlas(packer: &TexturePacker<DynamicImage, String>, output: &str) {
    let mut map = HashMap::new();

    for (name, frame) in packer.get_frames() {
        map.insert(
            name.to_owned(),
            Frame {
                x: frame.frame.x,
                y: frame.frame.y,
                width: frame.frame.w,
                height: frame.frame.h,
                rotated: frame.rotated,
                trimmed: frame.trimmed,
            },
        );
    }

    let exporter = ImageExporter::export(packer, None).unwrap();
    let file = File::create(output).unwrap();

    let encoder = PngEncoder::new_with_quality(&file, CompressionType::Best, FilterType::NoFilter);
    encoder
        .write_image(
            exporter.as_bytes(),
            exporter.width(),
            exporter.height(),
            ExtendedColorType::Rgba8,
        )
        .unwrap();

    let rmp = rmp_serde::to_vec(&map).unwrap();

    fs::write(Path::new(output).with_extension("dat"), rmp).unwrap();
}
