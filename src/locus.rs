use std::str::FromStr;
use regex::Regex;
use std::fmt;

#[derive(PartialEq, Eq, Ord, PartialOrd, Hash, Debug, RustcDecodable, Clone)]
pub struct Locus {
    pub chrom: String,
    pub start: u32,
    pub end: u32,
}

impl Locus {
    pub fn from_string(s: &str) -> Locus {
        FromStr::from_str(s).ok().unwrap()
    }
}

impl fmt::Display for Locus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}:{}-{}", self.chrom, self.start, self.end)
    }
}



#[derive(Debug)]
pub struct LocusParseError;

fn remove_commas(s: &str) -> String {
    let ss = s.to_string();
    ss.replace(",", "")
}


impl FromStr for Locus {
    type Err = LocusParseError;

    fn from_str(s: &str) -> Result<Locus, LocusParseError> {
        let re = Regex::new(r"^(.*):([0-9,]+)(-|..)([0-9,]+)$").unwrap();
        let cap = re.captures(s);

        if cap.is_none() {
            return Result::Err(LocusParseError {});
        }

        let cap = cap.unwrap();

        let start_s = remove_commas(cap.at(2).unwrap());
        let end_s = remove_commas(cap.at(4).unwrap());

        Ok(Locus {
            chrom: cap.at(1).unwrap().to_string(),
            start: FromStr::from_str(&start_s).unwrap(),
            end: FromStr::from_str(&end_s).unwrap(),
        })
    }
}