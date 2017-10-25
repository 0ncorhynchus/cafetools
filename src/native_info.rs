use error;
use std::str::FromStr;
use std::fmt;
use std::num::{ParseIntError, ParseFloatError};

struct LineParser<'a> {
    line: &'a str,
    pos: usize,
    pub ty: String,
}

impl<'a> LineParser<'a> {
    pub fn new(line: &str) -> LineParser {
        LineParser {
            line: &line[5..],
            pos: 0,
            ty: line[0..4].to_string(),
        }
    }

    pub fn parse_float(&mut self) -> Result<f64, ParseFloatError> {
        let start = self.pos;
        let end = start + 12;
        self.pos = end + 1;
        self.line[start..end].trim().parse()
    }

    pub fn parse_usize(&mut self) -> Result<usize, ParseIntError> {
        let start = self.pos;
        let end = start + 6;
        self.pos = end + 1;
        self.line[start..end].trim().parse()
    }

    pub fn parse_string(&mut self, size: usize) -> String {
        let start = self.pos;
        let end = start + size;
        self.pos = end + 1;
        self.line[start..end].trim().to_string()
    }

    pub fn parse_particles(&mut self, num: usize) -> Result<Vec<Particle>, ParseIntError> {
        let unit = self.parse_usize()?;
        assert_eq!(unit, self.parse_usize()?);

        let mut indexes = Vec::new();
        let mut intra_indexes = Vec::new();

        for _ in 0..num {
            indexes.push(self.parse_usize()?);
        }

        for _ in 0..num {
            intra_indexes.push(self.parse_usize()?);
        }

        let mut result = Vec::new();
        for i in 0..num {
            result.push(Particle {
                unit: unit,
                index: indexes[i],
                intra_index: intra_indexes[i],
            });
        }

        Ok(result)
    }
}

pub struct Particle {
    pub unit:        usize,
    pub index:       usize,
    pub intra_index: usize,
}

pub struct NativeBond {
    pub index:       usize,
    pub pair:        Vec<Particle>,
    pub length:      f64,
    pub factor:      f64,
    pub correct_mgo: f64,
    pub coefficient: f64,
    pub bond_type:      String,
}

fn write_particles(f: &mut fmt::Formatter, particles: &[Particle]) -> fmt::Result {
    for p in particles.iter().take(2) {
        write!(f, " {:6}", p.unit)?;
    }
    for p in particles {
        write!(f, " {:6}", p.index)?;
    }
    for p in particles {
        write!(f, " {:6}", p.intra_index)?;
    }
    Ok(())
}

fn write_float(f: &mut fmt::Formatter, value: f64) -> fmt::Result {
    write!(f, " {:12.4}", value)
}

fn write_usize(f: &mut fmt::Formatter, value: usize) -> fmt::Result {
    write!(f, " {:6}", value)
}

impl FromStr for NativeBond {
    type Err = error::Error;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut parser = LineParser::new(line);
        Ok(NativeBond {
            index:       parser.parse_usize()?,
            pair:        parser.parse_particles(2)?,
            length:      parser.parse_float()?,
            factor:      parser.parse_float()?,
            correct_mgo: parser.parse_float()?,
            coefficient: parser.parse_float()?,
            bond_type:   parser.parse_string(2),
        })
    }
}

impl fmt::Display for NativeBond {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "bond")?;
        write_usize(f, self.index)?;
        write_particles(f, &self.pair)?;
        write_float(f, self.length)?;
        write_float(f, self.factor)?;
        write_float(f, self.correct_mgo)?;
        write_float(f, self.coefficient)?;
        write!(f, " {}", self.bond_type)?;
        Ok(())
    }
}

pub struct NativeAngle {
    pub index: usize,
    pub triple: Vec<Particle>,
    pub angle: f64,
    pub factor: f64,
    pub correct_mgo: f64,
    pub coefficient: f64,
    pub angle_type: String
}

impl FromStr for NativeAngle {
    type Err = error::Error;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut parser = LineParser::new(line);
        Ok(NativeAngle {
            index:       parser.parse_usize()?,
            triple:      parser.parse_particles(3)?,
            angle:       parser.parse_float()?,
            factor:      parser.parse_float()?,
            correct_mgo: parser.parse_float()?,
            coefficient: parser.parse_float()?,
            angle_type:  parser.parse_string(3),
        })
    }
}

impl fmt::Display for NativeAngle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "angl")?;
        write_usize(f, self.index)?;
        write_particles(f, &self.triple)?;
        write_float(f, self.angle)?;
        write_float(f, self.factor)?;
        write_float(f, self.correct_mgo)?;
        write_float(f, self.coefficient)?;
        write!(f, " {}", self.angle_type)?;
        Ok(())
    }
}

struct NativeDihedralAngle {
    pub index: usize,
    pub particles: Vec<Particle>,
    pub angle: f64,
    pub factor: f64,
    pub correct_mgo: f64,
    pub coefficient1: f64,
    pub coefficient3: f64,
    pub dihedral_type: String
}

impl FromStr for NativeDihedralAngle {
    type Err = error::Error;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut parser = LineParser::new(line);
        Ok(NativeDihedralAngle {
            index:         parser.parse_usize()?,
            particles:     parser.parse_particles(4)?,
            angle:         parser.parse_float()?,
            factor:        parser.parse_float()?,
            correct_mgo:   parser.parse_float()?,
            coefficient1:  parser.parse_float()?,
            coefficient3:  parser.parse_float()?,
            dihedral_type: parser.parse_string(4),
        })
    }
}

impl fmt::Display for NativeDihedralAngle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "dihd")?;
        write_usize(f, self.index)?;
        write_particles(f, &self.particles)?;
        write_float(f, self.angle)?;
        write_float(f, self.factor)?;
        write_float(f, self.correct_mgo)?;
        write_float(f, self.coefficient1)?;
        write_float(f, self.coefficient3)?;
        write!(f, " {}", self.dihedral_type)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_native_bond() {
        let line = "bond      1      1      1      1      2      1      2       3.7629       1.0000       1.0000     110.4000 pp";
        let bond: NativeBond = line.parse().unwrap();
        assert_eq!(bond.index, 1);

        assert_eq!(bond.pair[0].unit, 1);
        assert_eq!(bond.pair[0].index, 1);
        assert_eq!(bond.pair[0].intra_index, 1);

        assert_eq!(bond.pair[1].unit, 1);
        assert_eq!(bond.pair[1].index, 2);
        assert_eq!(bond.pair[1].intra_index, 2);

        assert_eq!(bond.length, 3.7629);
        assert_eq!(bond.factor, 1.0);
        assert_eq!(bond.correct_mgo, 1.0);
        assert_eq!(bond.coefficient, 110.4);
        assert_eq!(bond.bond_type, "pp");

        assert_eq!(&bond.to_string(), line);
    }

    #[test]
    fn test_parse_native_angle() {
        let line = "angl      1      1      1      2      3      4      2      3      4     148.8728       1.0000       1.0000      20.0000 ppp";
        let angle: NativeAngle = line.parse().unwrap();
        assert_eq!(angle.index, 1);

        assert_eq!(angle.triple[0].unit, 1);
        assert_eq!(angle.triple[0].index, 2);
        assert_eq!(angle.triple[0].intra_index, 2);

        assert_eq!(angle.triple[1].unit, 1);
        assert_eq!(angle.triple[1].index, 3);
        assert_eq!(angle.triple[1].intra_index, 3);

        assert_eq!(angle.triple[2].unit, 1);
        assert_eq!(angle.triple[2].index, 4);
        assert_eq!(angle.triple[2].intra_index, 4);

        assert_eq!(angle.angle, 148.8728);
        assert_eq!(angle.factor, 1.0);
        assert_eq!(angle.correct_mgo, 1.0);
        assert_eq!(angle.coefficient, 20.0);
        assert_eq!(angle.angle_type, "ppp");

        assert_eq!(&angle.to_string(), line);
    }

    #[test]
    fn test_parse_native_dihedral_angle() {
        let line = "dihd      1      1      1      2      3      4      5      2      3      4      5    -124.4044       1.0000       1.0000       1.0000       0.5000 pppp";
        let dihedral: NativeDihedralAngle = line.parse().unwrap();
        assert_eq!(dihedral.index, 1);

        assert_eq!(dihedral.particles[0].unit, 1);
        assert_eq!(dihedral.particles[0].index, 2);
        assert_eq!(dihedral.particles[0].intra_index, 2);

        assert_eq!(dihedral.particles[1].unit, 1);
        assert_eq!(dihedral.particles[1].index, 3);
        assert_eq!(dihedral.particles[1].intra_index, 3);

        assert_eq!(dihedral.particles[2].unit, 1);
        assert_eq!(dihedral.particles[2].index, 4);
        assert_eq!(dihedral.particles[2].intra_index, 4);

        assert_eq!(dihedral.particles[3].unit, 1);
        assert_eq!(dihedral.particles[3].index, 5);
        assert_eq!(dihedral.particles[3].intra_index, 5);

        assert_eq!(dihedral.angle, -124.4044);
        assert_eq!(dihedral.factor, 1.0);
        assert_eq!(dihedral.correct_mgo, 1.0);
        assert_eq!(dihedral.coefficient1, 1.0);
        assert_eq!(dihedral.coefficient3, 0.5);
        assert_eq!(dihedral.dihedral_type, "pppp");

        assert_eq!(&dihedral.to_string(), line);
    }
}
