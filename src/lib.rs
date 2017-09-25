use std::io::prelude::*;

pub fn skip_lines<R: BufRead>(reader: &mut R, num_lines: usize) -> std::io::Result<()> {
    let mut buf = String::new();
    for _ in 0..num_lines {
        reader.read_line(&mut buf)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
