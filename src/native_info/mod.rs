mod line;
pub use self::line::*;

use error;
use block::ReadBlockExt;
use std::str::FromStr;
use std::fmt;
use std::io;

#[derive(Clone)]
pub struct Particle {
    pub unit:        usize,
    pub index:       usize,
    pub intra_index: usize,
}

pub type Pair = (Particle, Particle);
pub type Triple = (Particle, Particle, Particle);
pub type Quad = (Particle, Particle, Particle, Particle);

pub struct NativeInfo {
    pub bonds: Vec<Bond>,
    pub angles: Vec<Angle>,
    pub dihedral_angles: Vec<DihedralAngle>,
    pub contacts: Vec<Contact>,
    pub aicg_angles: Vec<AicgAngle>,
    pub aicg_dihedral_angles: Vec<AicgDihedralAngle>,
}

fn convert_all<T: FromStr>(lines: &Vec<String>) -> Vec<T> where
    <T as FromStr>::Err : fmt::Debug
{
    lines.iter()
         .map(|x| T::from_str(&x))
         .filter(|x| x.is_ok())
         .map(|x| x.unwrap())
         .collect()
}

impl NativeInfo {
    pub fn load<R: ReadBlockExt>(reader: R) -> io::Result<Self> {
        let mut bonds = Vec::new();
        let mut angles = Vec::new();
        let mut dihedral_angles = Vec::new();
        let mut contacts = Vec::new();
        let mut aicg_angles = Vec::new();
        let mut aicg_dihedral_angles = Vec::new();

        for block in reader.blocks() {
            match block.label.as_str() {
                "native bond length"     => bonds.extend(convert_all(&block.lines)),
                "native bond angles"     => angles.extend(convert_all(&block.lines)),
                "native dihedral angles" => dihedral_angles.extend(convert_all(&block.lines)),
                "native contact"         => contacts.extend(convert_all(&block.lines)),
                "1-3 contacts with L_AICG2 or L_AICG2_PLUS" =>
                    aicg_angles.extend(convert_all(&block.lines)),
                "<<<< 1-4 contacts with L_AICG2_PLUS" =>
                    aicg_dihedral_angles.extend(convert_all(&block.lines)),
                _ => {}
            }
        }

        Ok(NativeInfo {
            bonds: bonds,
            angles: angles,
            dihedral_angles: dihedral_angles,
            contacts: contacts,
            aicg_angles: aicg_angles,
            aicg_dihedral_angles: aicg_dihedral_angles,
        })
    }
}

impl fmt::Display for NativeInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if !self.contacts.is_empty() {
            writeln!(f, "<<<< native contact")?;
            writeln!(f, "** total_contact =   {}", self.contacts.len())?;
            writeln!(f, "** definition_of_contact =       6.50 A")?;
            writeln!(f, "** coef_go(kcal/mol) = factor_go * icon_dummy_mgo * cgo1210 * energy_unit_protein")?;
            writeln!(f, "")?;
            writeln!(f, "** contact between unit      1 and      1")?;
            writeln!(f, "** total_contact_unit =   {}", self.contacts.len())?;
            writeln!(f, "**        icon iunit1-iunit2   imp1 - imp2 imp1un-imp2un      go_nat   factor_go  dummy     coef_go")?;
            for contact in &self.contacts {
                writeln!(f, "{}", contact)?;
            }
            writeln!(f, ">>>>")?;
        }
        Ok(())
    }
}
