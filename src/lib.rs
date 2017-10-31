pub mod error;
pub mod time_series;
pub mod native_info;
mod block;

use std::io::prelude::*;

pub fn skip_lines<R: BufRead>(reader: &mut R, num_lines: usize) -> std::io::Result<()> {
    let mut buf = String::new();
    for _ in 0..num_lines {
        reader.read_line(&mut buf)?;
    }
    Ok(())
}
