mod line;
pub use self::line::*;

use error;
use std::str::FromStr;
use std::fmt;

pub struct Particle {
    pub unit:        usize,
    pub index:       usize,
    pub intra_index: usize,
}

pub type Pair = (Particle, Particle);
pub type Triple = (Particle, Particle, Particle);
pub type Quad = (Particle, Particle, Particle, Particle);

