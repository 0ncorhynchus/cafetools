use error;
use std::str::FromStr;
use std::string::ToString;

/// A SnapShot contains the data for each time-step.
pub struct SnapShot {
    pub unit:   String,
    pub step:   i32,
    pub tempk:  f32,
    pub radg:   f32,
    pub etot:   f32,
    pub velet:  f32,
    pub qscore: f32,
    pub rmsd:   f32,
}

impl FromStr for SnapShot {
    type Err = error::Error;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        Ok(SnapShot {
            unit:   line[ 0.. 6].trim().to_string(),
            step:   line[ 6..16].trim().parse()?,
            tempk:  line[17..25].trim().parse()?,
            radg:   line[26..34].trim().parse()?,
            etot:   line[35..45].trim().parse()?,
            velet:  line[46..56].trim().parse()?,
            qscore: line[57..63].trim().parse()?,
            rmsd:   line[64..72].trim().parse()?,
        })
    }
}

impl ToString for SnapShot {
    fn to_string(&self) -> String {
        format!("{:5} {:10} {:8.2} {:8.2} {:10.2} {:10.2} {:6.3} {:8.2}",
                self.unit,
                self.step,
                self.tempk,
                self.radg,
                self.etot,
                self.velet,
                self.qscore,
                self.rmsd)
    }
}

// /// A TimeSeries file contains trajectory data of CafeMol
// pub struct TimeSeries {
//     pub snapshots: Vec<SnapShot>,
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_snapshot() {
        let line = "               0   360.00   366.38      33.93     377.23  0.000   732.77";
        let snapshot = line.parse::<SnapShot>().unwrap();
        assert_eq!(snapshot.unit,   "".to_string());
        assert_eq!(snapshot.step,   0);
        assert_eq!(snapshot.tempk,  360.0);
        assert_eq!(snapshot.radg,   366.38);
        assert_eq!(snapshot.etot,   33.93);
        assert_eq!(snapshot.velet,  377.23);
        assert_eq!(snapshot.qscore, 0.0);
        assert_eq!(snapshot.rmsd,   732.77);

        assert_eq!(&snapshot.to_string(), line);
    }
}
