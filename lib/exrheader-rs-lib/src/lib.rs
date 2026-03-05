use std::fmt::Display;
use std::io::Write;
use std::path::Path;

use exr::meta::attribute::{AttributeValue, LevelMode, LineOrder, SampleType, TileDescription};
use exr::meta::MetaData;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParsingError {
    #[error("Could not read file on disk: '{0}'")]
    NotExist(String),

    #[error("Failed to parse metadata.")]
    EXRReadError(#[from] exr::error::Error),

    #[error("Failed to read attribute name as utf-8.")]
    FailedUTF8Conversion(#[from] std::string::FromUtf8Error),

    #[error("IO Error: {0}")]
    IoError(#[from] std::io::Error),
}

pub fn parse_metadata(p: impl AsRef<Path>) -> Result<MetaData, ParsingError> {
    let file_path = p.as_ref();
    log::debug!("Reading metadata of '{}'", file_path.display());

    if !file_path.exists() {
        return Err(ParsingError::NotExist(file_path.display().to_string()));
    }

    let pedantic = true;
    let metadata =
        MetaData::read_from_file(file_path, pedantic).map_err(|e| ParsingError::EXRReadError(e));

    metadata
}

pub fn format_metadata(meta: MetaData) -> Result<Vec<String>, ParsingError> {
    let mut lines = Vec::new();

    // Requirements
    lines.push(format!(
        "File format version: {}",
        meta.requirements.file_format_version
    ));
    lines.push(format!(
        "Has deep data: {}",
        meta.requirements.has_deep_data
    ));

    for header in meta.headers {
        for (name, value) in header.all_named_attributes() {
            let name = String::from_utf8(name.to_vec())
                .map_err(|e| ParsingError::FailedUTF8Conversion(e))?;

            let line = match value {
                AttributeValue::BlockType(bt) => {
                    let block_type = String::from_utf8(bt.to_text_bytes().to_vec())?;
                    format!(r#"{name}: "{block_type}""#,)
                }
                AttributeValue::ChannelList(list) => format_channels(list),
                AttributeValue::Compression(c) => format_compression(c),
                AttributeValue::LineOrder(lo) => {
                    let line_order = match lo {
                        LineOrder::Increasing => "increasing",
                        LineOrder::Decreasing => "decreasing",
                        LineOrder::Unspecified => "unspecified",
                    };
                    format!("lineOrder: {line_order}")
                }
                AttributeValue::I32(i) => {
                    format!("{name}: {i}")
                }
                AttributeValue::F32(f) => {
                    format!("{name}: {f}")
                }
                AttributeValue::F64(f) => {
                    format!("{name}: {f}")
                }
                AttributeValue::FloatVec2(fv) => {
                    format!("{name}: {}", format_vec2(fv))
                }
                AttributeValue::IntegerBounds(b) => {
                    let pos = b.position;
                    let max = b.max();
                    format!("{name}: {} - {}", format_vec2(pos), format_vec2(max))
                }
                AttributeValue::Chromaticities(chroma) => {
                    let red = format_vec2(chroma.red);
                    let green = format_vec2(chroma.green);
                    let blue = format_vec2(chroma.blue);
                    let white = format_vec2(chroma.white);
                    format!("chromaticies (rgbw): {red}, {green}, {blue}, {white}",)
                }
                AttributeValue::Text(t) => {
                    format!("{name}: {t}")
                }
                AttributeValue::TileDescription(td) => format_tile_description(td),
                _ => {
                    // FIXME: Keep implementing
                    log::warn!("Skipping unsupported attribute: {name}");
                    continue;
                }
            };
            lines.push(line);
        }
    }

    Ok(lines)
}

pub fn print_metadata(lines: Vec<String>) -> Result<(), std::io::Error> {
    let stdout = std::io::stdout();
    let mut lock = stdout.lock();

    for line in lines {
        writeln!(lock, "{}", line)?;
    }

    Ok(())
}

fn format_channels(channel_list: exr::meta::attribute::ChannelList) -> String {
    let mut s = Vec::new();
    s.push(format!("channels:"));

    for channel in channel_list.list {
        let channel_name = channel.name;
        let samples = format!("{} {}", channel.sampling.0, channel.sampling.1);
        let bitdepth = match channel.sample_type {
            SampleType::F16 => "16-bit floating-point",
            SampleType::F32 => "32-bit floating-point",
            SampleType::U32 => "32-bit unsigned",
        };
        let channel_info = format!("\t{channel_name}, {bitdepth}, sampling {samples}");
        s.push(channel_info);
    }

    s.join("\n")
}

fn format_compression(comp: exr::compression::Compression) -> String {
    let c = comp.to_string().replace(" compression", "");
    format!("compression: {c}",)
}

fn format_tile_description(td: TileDescription) -> String {
    let level = match td.level_mode {
        LevelMode::Singular => "single level",
        LevelMode::MipMap => "mipmap",
        LevelMode::RipMap => "ripmap",
    };

    let tile_size = format!("{} by {} pixels", td.tile_size.0, td.tile_size.1);
    format!("tiles:\n\t{level}\n\ttile size: {tile_size}")
}

fn format_vec2<T: Display>(v: exr::math::Vec2<T>) -> String {
    format!("({} {})", v.0, v.1)
}
