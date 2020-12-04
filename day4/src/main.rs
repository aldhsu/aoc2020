#[derive(Debug)]
struct Candidate {
    attrs: std::collections::HashMap<String, String>,
}

enum Validator {
    Byr,
    Iyr,
    Eyr,
    Hgt,
    Hcl,
    Ecl,
    Pid,
}

impl Validator {
    fn validate(&self, input: &std::collections::HashMap<String, String>) -> bool {
        use Validator::*;

        match self {
            Byr => ByrValidation::validate(input),
            Iyr => IyrValidation::validate(input),
            Eyr => EyrValidation::validate(input),
            Hgt => HgtValidation::validate(input),
            Hcl => HclValidation::validate(input),
            Ecl => EclValidation::validate(input),
            Pid => PidValidation::validate(input),
        }
    }
}

trait Validation {
    const KEY : &'static str;
    // fn key() -> &'static str;
    fn validate_input(input: &str) -> bool;
    fn validate(attrs: &std::collections::HashMap<String, String>) -> bool {
        match attrs.get(Self::KEY) {
            Some(value) => Self::validate_input(value),
            None => false,
        }
    }
}


struct ByrValidation;
impl Validation for ByrValidation {
    const KEY : &'static str = "byr";

    fn validate_input(input: &str) -> bool {
        let digits: i32 = match input.parse::<i32>() {
            Ok(digits) => digits,
            _ => return false,
        };
        (1920_i32..=2002).contains(&digits)
    }
}

struct IyrValidation;
impl Validation for IyrValidation {
    const KEY : &'static str = "iyr";

    fn validate_input(input: &str) -> bool {
        let digits: i32 = match input.parse::<i32>() {
            Ok(digits) => digits,
            _ => return false,
        };
        (2010_i32..=2020).contains(&digits)
    }
}

struct EyrValidation;
impl Validation for EyrValidation {
    const KEY : &'static str = "eyr";

    fn validate_input(input: &str) -> bool {
        let digits: i32 = match input.parse::<i32>() {
            Ok(digits) => digits,
            _ => return false,
        };
        (2020_i32..=2030).contains(&digits)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn byr_works() {
        assert_eq!(ByrValidation::validate_input("2002"), true);
        assert_eq!(ByrValidation::validate_input("2003"), false);
    }

    #[test]
    fn iyr_works() {
        assert_eq!(IyrValidation::validate_input("2020"), true);
        assert_eq!(IyrValidation::validate_input("2021"), false);
    }

    #[test]
    fn eyr_works() {
        assert_eq!(EyrValidation::validate_input("2030"), true);
        assert_eq!(EyrValidation::validate_input("2031"), false);
    }
}

struct HgtValidation;
impl Validation for HgtValidation {
    const KEY : &'static str = "hgt";

    fn validate_input(input: &str) -> bool {
        let digits: i32 = input
            .chars()
            .take_while(|c| c.is_digit(10))
            .collect::<String>()
            .parse::<i32>()
            .expect("couldn't get input");
        if input.ends_with("in") {
            (59_i32..=76).contains(&digits)
        } else if input.ends_with("cm"){
            (150_i32..=193).contains(&digits)
        } else {
            false
        }
    }
}

#[cfg(test)]
mod hgt_test {
    use super::*;
    #[test]
    fn hgt_works() {
        assert_eq!(HgtValidation::validate_input("60in"), true);
        assert_eq!(HgtValidation::validate_input("190cm"), true);
        assert_eq!(HgtValidation::validate_input("190in"), false);
        assert_eq!(HgtValidation::validate_input("190"), false);
    }
}

struct EclValidation;
impl Validation for EclValidation {
    const KEY : &'static str = "ecl";

    fn validate_input(input: &str) -> bool {
        const COLORS: [&'static str; 7] =
            ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

        COLORS.contains(&&input[..])
    }
}

#[cfg(test)]
mod ecl_test {
    use super::*;
    #[test]
    fn ecl_works() {
        assert_eq!(EclValidation::validate_input("brn"), true);
        assert_eq!(EclValidation::validate_input("wat"), false);
    }
}

struct PidValidation;
impl Validation for PidValidation {
    const KEY : &'static str = "pid";

    fn validate_input(input: &str) -> bool {
        if input.len() != 9 { return false}
        input.chars().all(|c| c.is_digit(10))
    }
}

#[cfg(test)]
mod pid_test {
    use super::*;
    #[test]
    fn pid_works() {
        assert_eq!(PidValidation::validate_input("123456789"), true);
        assert_eq!(PidValidation::validate_input("000000091"), true);
        assert_eq!(PidValidation::validate_input("abcdef09g"), false);
    }
}

struct HclValidation;
impl Validation for HclValidation {
    const KEY : &'static str = "hcl";

    fn validate_input(input: &str) -> bool {
        if !input.starts_with('#') {
            return false;
        }
        if input.len() != 7 {
            return false;
        }

        input.chars()
            .skip(1)
            .all(|c| ('0'..='9').contains(&c) || ('a'..='f').contains(&c))
    }
}

#[cfg(test)]
mod hcl_test {
    use super::*;
    #[test]
    fn hcl_works() {
        assert_eq!(HclValidation::validate_input("#123456"), true);
        assert_eq!(HclValidation::validate_input("#fffffa"), true);
        assert_eq!(HclValidation::validate_input("abcdef09g"), false);
        assert_eq!(HclValidation::validate_input("#abcd9g"), false);
        assert_eq!(HclValidation::validate_input("#a97842"), true);
    }
}

impl Candidate {
    const KEYS: [&'static str; 8] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid"];

    fn evaluate(&self) -> bool {
        let missing_keys: Vec<_> = Self::KEYS
            .iter()
            .filter(|&&key| !self.attrs.contains_key(key))
            .collect();

        missing_keys.is_empty() || (missing_keys.contains(&&"cid") && missing_keys.len() == 1)
    }

    fn evaluate_and_validate(&self) -> bool {
        let validations = [
            Validator::Byr,
            Validator::Iyr,
            Validator::Eyr,
            Validator::Hgt,
            Validator::Hcl,
            Validator::Ecl,
            Validator::Pid,
        ].iter().all(|v| v.validate(&self.attrs));
        self.evaluate() && validations
    }

}

fn parse(input: &str) -> Vec<Candidate> {
    let mut iter = input.lines();
    let mut candidates = vec![];
    while let Some(candidate) = take_candidate(&mut iter) {
        candidates.push(candidate);
    }
    candidates
}

fn take_candidate<'a>(iter: &mut impl Iterator<Item = &'a str>) -> Option<Candidate> {
    let mut attrs = std::collections::HashMap::new();

    loop {
        match iter.next() {
            Some(line) => {
                if line == "" {
                    return Some(Candidate { attrs });
                }
                line.split(" ").for_each(|key_value| {
                    let mut k_v = key_value.splitn(2, ":");
                    attrs.insert(
                        k_v.next().unwrap().to_string(),
                        k_v.next().unwrap().to_string(),
                    );
                });
            }
            None => {
                if attrs.len() > 0 {
                    return Some(Candidate { attrs });
                } else {
                    return None;
                }
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("input.txt")?;
    let candidates = parse(&input);
    let count = candidates.iter().filter(|c| c.evaluate()).count();
    println!("part1: {}", count);

    let count = candidates.iter().filter(|c| c.evaluate_and_validate()).count();
    println!("part2: {}", count);
    Ok(())
}

#[cfg(test)]
mod integration {
    use super::*;

    #[test]
    fn it_detects_all_invalid(){
        let input = r#"
eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007
"#;
       let candidates = parse(&input);
       let count = candidates.iter().filter(|c| c.evaluate_and_validate()).count();
       assert_eq!(0, count);
}

    #[test]
    fn it_detects_all_valid(){
        let input = r#"
pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719
"#;
       let candidates = parse(&input);
       let valid = candidates.iter().filter(|c| c.evaluate_and_validate()).count();
       assert_eq!(valid, 4);
    }
}
