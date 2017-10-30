use super::*;

struct LineCursor<'a> {
    line: &'a str,
    pos: usize,
}

impl<'a> LineCursor<'a> {
    pub fn new(line: &str) -> LineCursor {
        LineCursor {
            line: line,
            pos: 0,
        }
    }

    pub fn proceed(&mut self, len: usize) -> &str {
        let start = self.pos;
        self.pos += len;
        &self.line[start..self.pos].trim()
    }

    pub fn parse<T: Parsable>(&mut self) -> Result<T, T::Err> {
        T::parse_from(self)
    }

    pub fn parse_with_space<T: Parsable>(&mut self) -> Result<T, T::Err> {
        self.proceed(1);
        self.parse()
    }
}

trait Parsable: Sized {
    type Err;
    fn parse_from(cursor: &mut LineCursor) -> Result<Self, Self::Err>;
}

trait Formattable {
    fn format(&self, f: &mut fmt::Formatter) -> fmt::Result;
}

impl Parsable for f64 {
    type Err = <Self as FromStr>::Err;
    fn parse_from(cursor: &mut LineCursor) -> Result<Self, Self::Err> {
        cursor.proceed(12).parse()
    }
}

impl Formattable for f64 {
    fn format(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:12.4}", self)
    }
}

impl Parsable for usize {
    type Err = <Self as FromStr>::Err;
    fn parse_from(cursor: &mut LineCursor) -> Result<Self, Self::Err> {
        cursor.proceed(6).parse()
    }
}

impl Formattable for usize {
    fn format(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:6}", self)
    }
}

impl Parsable for Pair {
    type Err = <usize as Parsable>::Err;
    fn parse_from(cursor: &mut LineCursor) -> Result<Self, Self::Err> {
        let unit0 = cursor.parse()?;
        let unit1 = cursor.parse_with_space()?;
        let index0 = cursor.parse_with_space()?;
        let index1 = cursor.parse_with_space()?;
        let intra_index0 = cursor.parse_with_space()?;
        let intra_index1 = cursor.parse_with_space()?;
        Ok((
                Particle {
                    unit: unit0,
                    index: index0,
                    intra_index: intra_index0,
                },
                Particle {
                    unit: unit1,
                    index: index1,
                    intra_index: intra_index1,
                }))
    }
}

impl<'a> Formattable for &'a Pair {
    fn format(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write(f, self.0.unit)?;
        write_with_space(f, self.1.unit)?;
        write_with_space(f, self.0.index)?;
        write_with_space(f, self.1.index)?;
        write_with_space(f, self.0.intra_index)?;
        write_with_space(f, self.1.intra_index)?;
        Ok(())
    }
}

impl Parsable for Triple {
    type Err = <usize as Parsable>::Err;
    fn parse_from(cursor: &mut LineCursor) -> Result<Self, Self::Err> {
        let unit0 = cursor.parse()?;
        let unit1 = cursor.parse_with_space()?;
        let index0 = cursor.parse_with_space()?;
        let index1 = cursor.parse_with_space()?;
        let index2 = cursor.parse_with_space()?;
        let intra_index0 = cursor.parse_with_space()?;
        let intra_index1 = cursor.parse_with_space()?;
        let intra_index2 = cursor.parse_with_space()?;
        Ok((
                Particle {
                    unit: unit0,
                    index: index0,
                    intra_index: intra_index0,
                },
                Particle {
                    unit: unit1,
                    index: index1,
                    intra_index: intra_index1,
                },
                Particle {
                    unit: unit1,
                    index: index2,
                    intra_index: intra_index2,
                }))
    }
}

impl<'a> Formattable for &'a Triple {
    fn format(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write(f, self.0.unit)?;
        write_with_space(f, self.1.unit)?;
        write_with_space(f, self.0.index)?;
        write_with_space(f, self.1.index)?;
        write_with_space(f, self.2.index)?;
        write_with_space(f, self.0.intra_index)?;
        write_with_space(f, self.1.intra_index)?;
        write_with_space(f, self.2.intra_index)?;
        Ok(())
    }
}

impl Parsable for Quad {
    type Err = <usize as Parsable>::Err;
    fn parse_from(cursor: &mut LineCursor) -> Result<Self, Self::Err> {
        let unit0 = cursor.parse()?;
        let unit1 = cursor.parse_with_space()?;
        let index0 = cursor.parse_with_space()?;
        let index1 = cursor.parse_with_space()?;
        let index2 = cursor.parse_with_space()?;
        let index3 = cursor.parse_with_space()?;
        let intra_index0 = cursor.parse_with_space()?;
        let intra_index1 = cursor.parse_with_space()?;
        let intra_index2 = cursor.parse_with_space()?;
        let intra_index3 = cursor.parse_with_space()?;
        Ok((
                Particle {
                    unit: unit0,
                    index: index0,
                    intra_index: intra_index0,
                },
                Particle {
                    unit: unit0,
                    index: index1,
                    intra_index: intra_index1,
                },
                Particle {
                    unit: unit1,
                    index: index2,
                    intra_index: intra_index2,
                },
                Particle {
                    unit: unit1,
                    index: index3,
                    intra_index: intra_index3,
                }))
    }
}

impl<'a> Formattable for &'a Quad {
    fn format(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write(f, self.0.unit)?;
        write_with_space(f, self.2.unit)?;
        write_with_space(f, self.0.index)?;
        write_with_space(f, self.1.index)?;
        write_with_space(f, self.2.index)?;
        write_with_space(f, self.3.index)?;
        write_with_space(f, self.0.intra_index)?;
        write_with_space(f, self.1.intra_index)?;
        write_with_space(f, self.2.intra_index)?;
        write_with_space(f, self.3.intra_index)?;
        Ok(())
    }
}

impl<'a> Formattable for &'a String {
    fn format(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

fn write<T: Formattable>(f: &mut fmt::Formatter, value: T) -> fmt::Result {
    value.format(f)
}

fn write_with_space<T: Formattable>(f: &mut fmt::Formatter, value: T) -> fmt::Result {
    write!(f, " ")?;
    write(f, value)
}


pub struct Bond {
    pub index:       usize,
    pub pair:        Pair,
    pub length:      f64,
    pub factor:      f64,
    pub correct_mgo: f64,
    pub coefficient: f64,
    pub ty:          String,
}

impl FromStr for Bond {
    type Err = error::Error;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut cursor = LineCursor::new(&line[4..]);
        Ok(Bond {
            index:       cursor.parse_with_space()?,
            pair:        cursor.parse_with_space()?,
            length:      cursor.parse_with_space()?,
            factor:      cursor.parse_with_space()?,
            correct_mgo: cursor.parse_with_space()?,
            coefficient: cursor.parse_with_space()?,
            ty:          cursor.proceed(3).to_string(),
        })
    }
}

impl fmt::Display for Bond {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "bond")?;
        write_with_space(f, self.index)?;
        write_with_space(f, &self.pair)?;
        write_with_space(f, self.length)?;
        write_with_space(f, self.factor)?;
        write_with_space(f, self.correct_mgo)?;
        write_with_space(f, self.coefficient)?;
        write_with_space(f, &self.ty)?;
        Ok(())
    }
}

pub struct Angle {
    pub index:       usize,
    pub triple:      Triple,
    pub angle:       f64,
    pub factor:      f64,
    pub correct_mgo: f64,
    pub coefficient: f64,
    pub ty:          String
}

impl FromStr for Angle {
    type Err = error::Error;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut cursor = LineCursor::new(&line[4..]);
        Ok(Angle {
            index:       cursor.parse_with_space()?,
            triple:      cursor.parse_with_space()?,
            angle:       cursor.parse_with_space()?,
            factor:      cursor.parse_with_space()?,
            correct_mgo: cursor.parse_with_space()?,
            coefficient: cursor.parse_with_space()?,
            ty:          cursor.proceed(4).to_string(),
        })
    }
}

impl fmt::Display for Angle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "angl")?;
        write_with_space(f, self.index)?;
        write_with_space(f, &self.triple)?;
        write_with_space(f, self.angle)?;
        write_with_space(f, self.factor)?;
        write_with_space(f, self.correct_mgo)?;
        write_with_space(f, self.coefficient)?;
        write_with_space(f, &self.ty)?;
        Ok(())
    }
}

pub struct DihedralAngle {
    pub index:        usize,
    pub quad:         Quad,
    pub angle:        f64,
    pub factor:       f64,
    pub correct_mgo:  f64,
    pub coefficient1: f64,
    pub coefficient3: f64,
    pub ty:           String,
}

impl FromStr for DihedralAngle {
    type Err = error::Error;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut cursor = LineCursor::new(&line[4..]);
        Ok(DihedralAngle {
            index:         cursor.parse_with_space()?,
            quad:          cursor.parse_with_space()?,
            angle:         cursor.parse_with_space()?,
            factor:        cursor.parse_with_space()?,
            correct_mgo:   cursor.parse_with_space()?,
            coefficient1:  cursor.parse_with_space()?,
            coefficient3:  cursor.parse_with_space()?,
            ty:            cursor.proceed(5).to_string(),
        })
    }
}

impl fmt::Display for DihedralAngle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "dihd")?;
        write_with_space(f, self.index)?;
        write_with_space(f, &self.quad)?;
        write_with_space(f, self.angle)?;
        write_with_space(f, self.factor)?;
        write_with_space(f, self.correct_mgo)?;
        write_with_space(f, self.coefficient1)?;
        write_with_space(f, self.coefficient3)?;
        write_with_space(f, &self.ty)?;
        Ok(())
    }
}

pub struct Contact {
    pub index:       usize,
    pub pair:        Pair,
    pub length:      f64,
    pub factor:      f64,
    pub dummy:       usize,
    pub coefficient: f64,
    pub ty:          String,
}

impl FromStr for Contact {
    type Err = error::Error;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut cursor = LineCursor::new(&line[7..]);
        Ok(Contact {
            index:       cursor.parse_with_space()?,
            pair:        cursor.parse_with_space()?,
            length:      cursor.parse()?,
            factor:      cursor.parse()?,
            dummy:       cursor.parse_with_space()?,
            coefficient: cursor.parse()?,
            ty:          cursor.proceed(4).to_string(),
        })
    }
}

impl fmt::Display for Contact {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "contact")?;
        write_with_space(f, self.index)?;
        write_with_space(f, &self.pair)?;
        write(f, self.length)?;
        write(f, self.factor)?;
        write_with_space(f, self.dummy)?;
        write(f, self.coefficient)?;
        write_with_space(f, &self.ty)?;
        Ok(())
    }
}

pub struct AICGAngle {
    pub index:       usize,
    pub triple:      Triple,
    pub value:       f64,
    pub factor:      f64,
    pub correct_mgo: f64,
    pub coefficient: f64,
    pub width:       f64,
    pub ty:          String,
}

impl FromStr for AICGAngle {
    type Err = error::Error;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut cursor = LineCursor::new(&line[6..]);
        Ok(AICGAngle {
            index:       cursor.parse_with_space()?,
            triple:      cursor.parse_with_space()?,
            value:       cursor.parse_with_space()?,
            factor:      cursor.parse_with_space()?,
            correct_mgo: cursor.parse_with_space()?,
            coefficient: cursor.parse_with_space()?,
            width:       cursor.parse_with_space()?,
            ty:          cursor.proceed(4).to_string(),
        })
    }
}

impl fmt::Display for AICGAngle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "aicg13")?;
        write_with_space(f, self.index)?;
        write_with_space(f, &self.triple)?;
        write_with_space(f, self.value)?;
        write_with_space(f, self.factor)?;
        write_with_space(f, self.correct_mgo)?;
        write_with_space(f, self.coefficient)?;
        write_with_space(f, self.width)?;
        write_with_space(f, &self.ty)?;
        Ok(())
    }
}

pub struct AICGDihedralAngle {
    pub index:       usize,
    pub quad:        Quad,
    pub value:       f64,
    pub factor:      f64,
    pub correct_mgo: f64,
    pub coefficient: f64,
    pub width:       f64,
    pub ty:          String,
}

impl FromStr for AICGDihedralAngle {
    type Err = error::Error;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut cursor = LineCursor::new(&line[7..]);
        Ok(AICGDihedralAngle {
            index:       cursor.parse_with_space()?,
            quad:        cursor.parse_with_space()?,
            value:       cursor.parse_with_space()?,
            factor:      cursor.parse_with_space()?,
            correct_mgo: cursor.parse_with_space()?,
            coefficient: cursor.parse_with_space()?,
            width:       cursor.parse_with_space()?,
            ty:          cursor.proceed(5).to_string(),
        })
    }
}

impl fmt::Display for AICGDihedralAngle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "aicgdih")?;
        write_with_space(f, self.index)?;
        write_with_space(f, &self.quad)?;
        write_with_space(f, self.value)?;
        write_with_space(f, self.factor)?;
        write_with_space(f, self.correct_mgo)?;
        write_with_space(f, self.coefficient)?;
        write_with_space(f, self.width)?;
        write_with_space(f, &self.ty)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_bond() {
        let line = "bond      1      1      1      1      2      1      2       3.7629       1.0000       1.0000     110.4000 pp";
        let bond: Bond = line.parse().unwrap();
        assert_eq!(bond.index, 1);

        assert_eq!(bond.pair.0.unit, 1);
        assert_eq!(bond.pair.0.index, 1);
        assert_eq!(bond.pair.0.intra_index, 1);

        assert_eq!(bond.pair.1.unit, 1);
        assert_eq!(bond.pair.1.index, 2);
        assert_eq!(bond.pair.1.intra_index, 2);

        assert_eq!(bond.length, 3.7629);
        assert_eq!(bond.factor, 1.0);
        assert_eq!(bond.correct_mgo, 1.0);
        assert_eq!(bond.coefficient, 110.4);
        assert_eq!(bond.ty, "pp");

        assert_eq!(&bond.to_string(), line);
    }

    #[test]
    fn test_parse_angle() {
        let line = "angl      1      1      1      2      3      4      2      3      4     148.8728       1.0000       1.0000      20.0000 ppp";
        let angle: Angle = line.parse().unwrap();
        assert_eq!(angle.index, 1);

        assert_eq!(angle.triple.0.unit, 1);
        assert_eq!(angle.triple.0.index, 2);
        assert_eq!(angle.triple.0.intra_index, 2);

        assert_eq!(angle.triple.1.unit, 1);
        assert_eq!(angle.triple.1.index, 3);
        assert_eq!(angle.triple.1.intra_index, 3);

        assert_eq!(angle.triple.2.unit, 1);
        assert_eq!(angle.triple.2.index, 4);
        assert_eq!(angle.triple.2.intra_index, 4);

        assert_eq!(angle.angle, 148.8728);
        assert_eq!(angle.factor, 1.0);
        assert_eq!(angle.correct_mgo, 1.0);
        assert_eq!(angle.coefficient, 20.0);
        assert_eq!(angle.ty, "ppp");

        assert_eq!(&angle.to_string(), line);
    }

    #[test]
    fn test_parse_dihedral_angle() {
        let line = "dihd      1      1      1      2      3      4      5      2      3      4      5    -124.4044       1.0000       1.0000       1.0000       0.5000 pppp";
        let dihedral: DihedralAngle = line.parse().unwrap();
        assert_eq!(dihedral.index, 1);

        assert_eq!(dihedral.quad.0.unit, 1);
        assert_eq!(dihedral.quad.0.index, 2);
        assert_eq!(dihedral.quad.0.intra_index, 2);

        assert_eq!(dihedral.quad.1.unit, 1);
        assert_eq!(dihedral.quad.1.index, 3);
        assert_eq!(dihedral.quad.1.intra_index, 3);

        assert_eq!(dihedral.quad.2.unit, 1);
        assert_eq!(dihedral.quad.2.index, 4);
        assert_eq!(dihedral.quad.2.intra_index, 4);

        assert_eq!(dihedral.quad.3.unit, 1);
        assert_eq!(dihedral.quad.3.index, 5);
        assert_eq!(dihedral.quad.3.intra_index, 5);

        assert_eq!(dihedral.angle, -124.4044);
        assert_eq!(dihedral.factor, 1.0);
        assert_eq!(dihedral.correct_mgo, 1.0);
        assert_eq!(dihedral.coefficient1, 1.0);
        assert_eq!(dihedral.coefficient3, 0.5);
        assert_eq!(dihedral.ty, "pppp");

        assert_eq!(&dihedral.to_string(), line);
    }

    #[test]
    fn test_parse_contact() {
        let line ="contact      1      1      1      2     63      2     63      6.2398      1.0000      1      0.5986 p-p";
        let contact: Contact = line.parse().unwrap();

        assert_eq!(contact.index, 1);

        assert_eq!(contact.pair.0.unit, 1);
        assert_eq!(contact.pair.0.index, 2);
        assert_eq!(contact.pair.0.intra_index, 2);

        assert_eq!(contact.pair.1.unit, 1);
        assert_eq!(contact.pair.1.index, 63);
        assert_eq!(contact.pair.1.intra_index, 63);

        assert_eq!(contact.length, 6.2398);
        assert_eq!(contact.factor, 1.0);
        assert_eq!(contact.dummy, 1);
        assert_eq!(contact.coefficient, 0.5986);
        assert_eq!(contact.ty, "p-p");

        assert_eq!(&contact.to_string(), line);
    }

    #[test]
    fn test_parse_aicg_angle() {
        let line = "aicg13      1      1      1      2      3      4      2      3      4       7.3690       1.0000       1.0000       1.1928       0.1500 ppp";
        let angle: AICGAngle = line.parse().unwrap();

        assert_eq!(angle.index, 1);

        assert_eq!(angle.triple.0.unit, 1);
        assert_eq!(angle.triple.0.index, 2);
        assert_eq!(angle.triple.0.intra_index, 2);

        assert_eq!(angle.triple.1.unit, 1);
        assert_eq!(angle.triple.1.index, 3);
        assert_eq!(angle.triple.1.intra_index, 3);

        assert_eq!(angle.triple.2.unit, 1);
        assert_eq!(angle.triple.2.index, 4);
        assert_eq!(angle.triple.2.intra_index, 4);

        assert_eq!(angle.value, 7.3690);
        assert_eq!(angle.factor, 1.0);
        assert_eq!(angle.correct_mgo, 1.0);
        assert_eq!(angle.coefficient, 1.1928);
        assert_eq!(angle.width, 0.15);
        assert_eq!(angle.ty, "ppp");

        assert_eq!(&angle.to_string(), line);
    }

    #[test]
    fn test_parse_aicg_dihedral_angle() {
        let line = "aicgdih      1      1      1      2      3      4      5      2      3      4      5    -124.4044       1.0000       1.0000       0.4350       0.1500 pppp";
        let angle: AICGDihedralAngle = line.parse().unwrap();

        assert_eq!(angle.index, 1);

        assert_eq!(angle.quad.0.unit, 1);
        assert_eq!(angle.quad.0.index, 2);
        assert_eq!(angle.quad.0.intra_index, 2);

        assert_eq!(angle.quad.1.unit, 1);
        assert_eq!(angle.quad.1.index, 3);
        assert_eq!(angle.quad.1.intra_index, 3);

        assert_eq!(angle.quad.2.unit, 1);
        assert_eq!(angle.quad.2.index, 4);
        assert_eq!(angle.quad.2.intra_index, 4);

        assert_eq!(angle.quad.3.unit, 1);
        assert_eq!(angle.quad.3.index, 5);
        assert_eq!(angle.quad.3.intra_index, 5);

        assert_eq!(angle.value, -124.4044);
        assert_eq!(angle.factor, 1.0);
        assert_eq!(angle.correct_mgo, 1.0);
        assert_eq!(angle.coefficient, 0.4350);
        assert_eq!(angle.width, 0.15);
        assert_eq!(angle.ty, "pppp");

        assert_eq!(&angle.to_string(), line);
    }
}
