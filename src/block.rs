use std::io;

pub struct Block {
    pub label: String,
    pub lines: Vec<String>,
}

pub struct Blocks<R> {
    lines: io::Lines<R>,
}

impl<R: ReadBlockExt> Iterator for Blocks<R> {
    type Item = Block;
    fn next(&mut self) -> Option<Self::Item> {
        read_block(&mut self.lines).ok()
    }
}

pub trait ReadBlockExt: io::BufRead {
    fn blocks(self) -> Blocks<Self> where Self: Sized {
        Blocks { lines:self.lines() }
    }
}

impl<R: io::BufRead> ReadBlockExt for R {}

fn is_comment(line: &String) -> bool {
    &line[0..1] == "*"
}

fn is_end_of_block(line: &String) -> bool {
    line.len() >= 4 && &line[0..4] == ">>>>"
}

fn search_start_of_block<R: io::BufRead>(lines: &mut io::Lines<R>) -> io::Result<String> {
    for line in lines {
        let line = line?;
        if line.is_empty() || is_comment(&line) { continue; }
        if &line[0..4] == "<<<<" {
            return Ok(line[4..].trim().to_string());
        }
        // skip
    }
    Err(io::Error::new(io::ErrorKind::InvalidData, ""))
}

fn read_block<R: io::BufRead>(lines: &mut io::Lines<R>) -> io::Result<Block> {
    let label = search_start_of_block(lines)?;

    let mut contents = Vec::new();
    for line in lines {
        let line = line?;
        if line.is_empty() || is_comment(&line) { continue; }
        if is_end_of_block(&line) { break; }
        contents.push(line);
    }

    Ok(Block { label:label, lines:contents })
}
