use error;
use std::str::FromStr;
use std::fmt;

pub struct Particle {
    pub unit:        usize,
    pub index:       usize,
    pub intra_index: usize,
}

pub struct NativeBond {
    pub index:       usize,
    pub pair:        [Particle; 2],
    pub length:      f64,
    pub factor:      f64,
    pub correct_mgo: f64,
    pub coefficient: f64,
    pub bond_type:      String,
}

fn fmt_particles(particles: &[Particle], f: &mut fmt::Formatter) -> fmt::Result {
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

fn extract_last_string(string: &str, pos: usize, size: usize) -> String {
    let end = pos + size;
    if string.len() < pos {
        ""
    } else if string.len() < end {
        &string[pos..string.len()]
    } else {
        &string[pos..end]
    }.trim().to_string()
}

impl FromStr for NativeBond {
    type Err = error::Error;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let first = Particle {
            unit:        line[12..18].trim().parse()?,
            index:       line[26..32].trim().parse()?,
            intra_index: line[40..46].trim().parse()?,
        };
        let second = Particle {
            unit:        line[19..25].trim().parse()?,
            index:       line[33..39].trim().parse()?,
            intra_index: line[47..53].trim().parse()?,
        };
        Ok(NativeBond {
            index:       line[ 5.. 11].trim().parse()?,
            pair:        [first, second],
            length:      line[54.. 66].trim().parse()?,
            factor:      line[67.. 79].trim().parse()?,
            correct_mgo: line[80.. 92].trim().parse()?,
            coefficient: line[93..105].trim().parse()?,
            bond_type:   extract_last_string(&line, 106, 3),
        })
    }
}

impl fmt::Display for NativeBond {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "bond")?;
        write!(f, " {:6}", self.index)?;
        fmt_particles(&self.pair, f)?;
        write!(f, " {:12.4}", self.length)?;
        write!(f, " {:12.4}", self.factor)?;
        write!(f, " {:12.4}", self.correct_mgo)?;
        write!(f, " {:12.4}", self.coefficient)?;
        write!(f, " {}", self.bond_type)?;
        Ok(())
    }
}

pub struct NativeAngle {
    pub index: usize,
    pub triple: [Particle; 3],
    pub angle: f64,
    pub factor: f64,
    pub correct_mgo: f64,
    pub coefficient: f64,
    pub angle_type: String
}

impl FromStr for NativeAngle {
    type Err = error::Error;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let unit: usize = line[12..18].trim().parse()?;
        // assert_eq!(unit, line[19..25].trim().parse()?);
        let first = Particle {
            unit:        unit,
            index:       line[26..32].trim().parse()?,
            intra_index: line[47..53].trim().parse()?,
        };
        let second = Particle {
            unit:        unit,
            index:       line[33..39].trim().parse()?,
            intra_index: line[54..60].trim().parse()?,
        };
        let third = Particle {
            unit:        unit,
            index:       line[40..46].trim().parse()?,
            intra_index: line[61..67].trim().parse()?,
        };
        Ok(NativeAngle {
            index: line[5..11].trim().parse()?,
            triple: [first, second, third],
            angle: line[68..80].trim().parse()?,
            factor: line[81..93].trim().parse()?,
            correct_mgo: line[94..106].trim().parse()?,
            coefficient: line[107..119].trim().parse()?,
            angle_type: extract_last_string(&line, 120, 4),
        })
    }
}

impl fmt::Display for NativeAngle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "angl")?;
        write!(f, " {:6}", self.index)?;
        fmt_particles(&self.triple, f)?;
        write!(f, " {:12.4}", self.angle)?;
        write!(f, " {:12.4}", self.factor)?;
        write!(f, " {:12.4}", self.correct_mgo)?;
        write!(f, " {:12.4}", self.coefficient)?;
        write!(f, " {}", self.angle_type)?;
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
}
