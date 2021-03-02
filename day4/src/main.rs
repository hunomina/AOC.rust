// see : https://adventofcode.com/2020/day/4

use regex::Regex;
use std::fs;

#[derive(Debug, Default)]
struct Passport {
    byr: Option<u16>,
    iyr: Option<u16>,
    eyr: Option<u16>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<u16>, // can be null
}

const PASSPORT_ALLOWED_EYE_COLORS: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

impl Passport {
    fn valid(&self) -> bool {
        self.valid_byr()
            && self.valid_iyr()
            && self.valid_eyr()
            && self.valid_hgt()
            && self.valid_hcl()
            && self.valid_ecl()
            && self.valid_pid()
            && self.valid_cid()
    }

    fn valid_byr(&self) -> bool {
        match self.byr {
            Some(v) => v >= 1920 && v <= 2002,
            None => false,
        }
    }

    fn valid_iyr(&self) -> bool {
        match self.iyr {
            Some(v) => v >= 2010 && v <= 2020,
            None => false,
        }
    }

    fn valid_eyr(&self) -> bool {
        match self.eyr {
            Some(v) => v >= 2020 && v <= 2030,
            None => false,
        }
    }

    fn valid_hgt(&self) -> bool {
        match &self.hgt {
            Some(h) => {
                let (v, unit) = h.split_at(h.len() - 2);
                if unit == "cm" {
                    let v_as_int = v.parse::<u16>().unwrap();
                    return v_as_int >= 150 && v_as_int <= 193;
                } else if unit == "in" {
                    let v_as_int = v.parse::<u16>().unwrap();
                    return v_as_int >= 59 && v_as_int <= 76;
                }
                false
            }
            None => false,
        }
    }

    fn valid_hcl(&self) -> bool {
        match &self.hcl {
            Some(v) => Regex::new("^#[a-f0-9]{6}$").unwrap().is_match(&v.as_str()),
            None => false,
        }
    }

    fn valid_ecl(&self) -> bool {
        match &self.ecl {
            Some(v) => PASSPORT_ALLOWED_EYE_COLORS.contains(&v.as_str()),
            None => false,
        }
    }

    fn valid_pid(&self) -> bool {
        match &self.pid {
            Some(v) => Regex::new("^[0-9]{9}$").unwrap().is_match(&v.as_str()),
            None => false,
        }
    }

    fn valid_cid(&self) -> bool {
        true
    }
}

fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();
    let passports = parse_input(input.as_str());
    println!(
        "{}",
        passports
            .iter()
            .fold(0, |acc, p| if p.valid() { acc + 1 } else { acc })
    );
}

fn parse_input(input: &str) -> Vec<Passport> {
    let mut passports: Vec<Passport> = vec![];

    let ps = input.split("\n\n").map(|p| {
        p.replace("\n", " ")
            .split(" ")
            .map(str::to_owned)
            .collect::<Vec<String>>()
    });

    for fields in ps {
        let mut p = Passport::default();

        for field in fields {
            let split = field.split(":").map(str::to_owned).collect::<Vec<String>>();
            match split[0].as_str() {
                "byr" => p.byr = Some(split[1].parse().unwrap()),
                "iyr" => p.iyr = Some(split[1].parse().unwrap()),
                "eyr" => p.eyr = Some(split[1].parse().unwrap()),
                "hgt" => p.hgt = Some(split[1].clone()),
                "hcl" => p.hcl = Some(split[1].clone()),
                "ecl" => p.ecl = Some(split[1].clone()),
                "pid" => p.pid = Some(split[1].parse().unwrap()),
                "cid" => p.cid = Some(split[1].parse().unwrap()),
                _ => {}
            }
        }
        // println!("{:?}", p);
        passports.push(p);
    }
    passports // number of valid passwords
}
