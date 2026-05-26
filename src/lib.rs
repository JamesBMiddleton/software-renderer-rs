use std::io::ErrorKind;

mod chunk;
mod render;

pub struct Config {
    pub viewport_height: u64,
    pub viewport_width: u64,
}

pub struct Frame {
    pub x: u64, // placeholder
}

pub fn get_frame(config: Config) -> Result<Frame, ErrorKind> {
    let y = chunk::addchunk(1, 1)
        + render::addrender(1, 1)
        + config.viewport_height
        + config.viewport_width;
    return Ok(Frame { x: y });
}
