
#[derive(Debug)]
struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

impl Passport {
    fn new() -> Passport {
        Passport {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: None,
            ecl: None,
            pid: None,
            cid: None,
        }
    }

    fn validate_field<F>(field: &Option<String>, predicate: F) -> bool
    where F: Fn(&String) ->bool {
        match field {
            Some(val) => predicate(&val),
            None => false,
        }
    }

    fn valid_basicly(&self) ->bool {
        return
            Passport::validate_field(&self.byr, |_| true) &&
            Passport::validate_field(&self.iyr, |_| true) &&
            Passport::validate_field(&self.eyr, |_| true) &&
            Passport::validate_field(&self.hgt, |_| true) &&
            Passport::validate_field(&self.hcl, |_| true) &&
            Passport::validate_field(&self.ecl, |_| true) &&
            Passport::validate_field(&self.pid, |_| true);
    }

    fn valid_extremely(&self) ->bool {
        return
            Passport::validate_field(&self.byr, |x| {
                let year = x.parse::<u32>().unwrap();
                return 4 == x.len() && 1920 <= year && 2002 >= year;
            }) &&
            Passport::validate_field(&self.iyr, |x| {
                let year = x.parse::<u32>().unwrap();
                return 4 == x.len() && 2010 <= year && 2020 >= year;
            }) &&
            Passport::validate_field(&self.eyr, |x| {
                let year = x.parse::<u32>().unwrap();
                return 4 == x.len() && 2020 <= year && 2030 >= year;
            }) &&
            Passport::validate_field(&self.hgt, |x| {
                let hstr: &str = &*x;
                if "cm" == &hstr[hstr.len()-2..] {
                    let height_cm = hstr[..hstr.len()-2].parse::<u32>().unwrap();
                    return 150 <= height_cm && 193 >= height_cm;
                }
                else if "in" == &hstr[hstr.len()-2..] {
                    let height_in = hstr[..hstr.len()-2].parse::<u32>().unwrap();
                    return 59 <= height_in && 76 >= height_in;
                }

                return false;
            }) &&
            Passport::validate_field(&self.hcl, |x| {
                let cstr: &str = &*x;
                if '#' != cstr.chars().nth(0).unwrap() || 7 != cstr.len() {
                    return false;
                }
                match u32::from_str_radix(&cstr[1usize..], 16) {
                    Ok(_) => true,
                    Err(_) => false,
                }
            }) &&
            Passport::validate_field(&self.ecl, |x| {
                match x.as_str() {
                    "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
                    _ => false,
                }
            }) &&
            Passport::validate_field(&self.pid, |x| {
                if 9 != x.len() {
                    return false;
                }
                match x.parse::<u32>() {
                    Ok(_) => true,
                    Err(_) => false,
                }
            });
    }
}

fn parse_input(filename: &str) -> Vec<Passport> {
    let lines = super::read_lines(filename).unwrap();

    let mut pp = Passport::new();
    let mut passport: Vec<Passport> = Vec::new();
    for line_option in lines {
        let line = line_option.unwrap();

        if line.len() == 0 {
            passport.push(pp);
            pp = Passport::new();
        }
        else {
            let pairs: Vec<&str> = line.split_whitespace().collect();
            for pair in pairs {
                let key = &pair[0..3];
                let val = pair[4..].to_string();

                match key {
                    "byr" => pp.byr = Some(val),
                    "iyr" => pp.iyr = Some(val),
                    "eyr" => pp.eyr = Some(val),
                    "hgt" => pp.hgt = Some(val),
                    "hcl" => pp.hcl = Some(val),
                    "ecl" => pp.ecl = Some(val),
                    "pid" => pp.pid = Some(val),
                    "cid" => pp.cid = Some(val),
                    _ => panic!("Unknown key"),
                };
            }
        }
    }
    passport.push(pp);

    return passport;
}


pub fn solve( filename: &str ) -> super::Solution {
    let passport = parse_input( filename );

    let mut valid_count_base = 0u32;
    let mut valid_count_extra = 0u32;
    for p in &passport {
        if p.valid_basicly() {
            valid_count_base += 1;
        }
        if p.valid_extremely() {
            valid_count_extra += 1;
        }
    }

    Ok((valid_count_base.to_string(),valid_count_extra.to_string()))
}