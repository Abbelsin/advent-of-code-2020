use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use regex::Regex;
use lazy_static::lazy_static;

//use std::io::BufRead;

lazy_static! {
    static ref HEIGHT: Regex = Regex::new(r"^(\d+)(in|cm)$").unwrap();
    static ref HAIRCOLORS: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    static ref PASSPORTID: Regex = Regex::new(r"^[0-9]{9}$").unwrap();
}

/**
 * byr (Birth Year)
 * iyr (Issue Year)
 * eyr (Expiration Year)
 * hgt (Height)
 * hcl (Hair Color)
 * ecl (Eye Color)
 * pid (Passport ID)
 * cid (Country ID)
*/

static TEST_INPUT: &str = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

const REQ_FIELDS: [&'static str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

fn parse_password(batch: &str) -> HashMap<&str,&str> {
    let b: Vec<&str> = batch.split_ascii_whitespace().collect();
    let mut map = HashMap::new();
    for field in &b {
        let vals: Vec<&str> = field.split(":").collect();
        
        if let Some(key) = vals.get(0) {
            if let Some(val) = vals.get(1) {
                map.entry(*key).or_insert(*val);
            } else {
                println!("Missing value for {:?}", key);
            }
        } else {
            println!("Missing key");
        }
    }
    return map;
    
}
fn validate(pwd_map: &HashMap<&str,&str>) -> bool {
    return REQ_FIELDS.iter().all(|field| pwd_map.contains_key(field));
}

fn validate_better(pwd_map: &HashMap<&str,&str>) -> bool {
    let has_valid_fields = REQ_FIELDS.iter().all(|field| pwd_map.contains_key(field));

    let valid = pwd_map.iter().all(|(&key, &val)| validate_field(key,val));
    if !valid {
        println!("{:?}", pwd_map);
    }
    return valid;
}

fn validate_field(key: &str, content: &str) -> bool {

    match key {
        "byr" => { //four digits; at least 1920 and at most 2002
            if let Ok(byr) = content.parse::<i32>() {
                return byr >= 1920 && byr <= 2002;
            } else {
                return false;
            }
        }
        "iyr" => { // four digits; at least 2010 and at most 2020
            return {
                content.chars().count() == 4 &&
                if let Ok(byr) = content.parse::<i32>() {
                    byr >= 2010 && byr <= 2020
                } else {
                    false
                }
            };
        }
        "eyr" => { // four digits; at least 2020 and at most 2030
            return {
                content.chars().count() == 4 &&
                if let Ok(byr) = content.parse::<i32>() {
                    byr >= 2020 && byr <= 2030
                } else {
                    false
                }
            };
        }
        "hgt" => { // a number followed by either cm or in:
            if content.ends_with("cm") {
                //content.chars().last()
                let mut chars = content.chars();
                chars.next_back();
                chars.next_back();
                if let Ok(_hgt) = chars.as_str().parse::<u32>() {
                    return  true;
                } else {
                    return false;
                }
            } else if content.ends_with("in") {
                true
            } else {
                return false;
            }
        }
        "hcl" => { // a # followed by exactly six characters 0-9 or a-f
            return content.chars().count() == 7 &&
            content.chars().all(|c| (c >= '0' && c <= '9') || (c >= 'a' && c <= 'f') || c == '#');
        }
        "ecl" => { // exactly one of: amb blu brn gry grn hzl oth
            match content {
                "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => return true,
                _ => return false,
            }
        }
        "pid" => { // a nine-digit number, including leading zeroes
            content.chars().count() == 9 &&
            content.chars().all(|c| c >= '0' && c <= '9' )
        }
        "cid" => { // ignored, missing or not
            true
        }
        _ => {
            panic!("unknown key {}", key);
        }
    }
}



fn main() {
    println!("~~~~ AoC 2020 day04 ~~~~");
    let input_string = include_str!("../input.txt");
    let inputs: Vec<&str> = input_string.split("\r\n\r\n").collect();
    
    let passwords: Vec<HashMap<&str,&str>> = inputs.iter().map(|pwd| parse_password(pwd)).collect();
    
    let n = passwords.iter().count();
    println!("{} passwords!", n);
    println!("{} valid passwords!", passwords.iter().filter(|pwd| validate(pwd)).count());

    //println!("{}",REQQ_FIELDS.iter().all(|item| timber_resources.contains(item)));
}

#[test]
fn part_2() {
    println!("~~~~ AoC 2020 day04 part 2 ~~~~");
    let input_string = include_str!("../input.txt");
    let inputs: Vec<&str> = input_string.split("\r\n\r\n").collect();
    
    let passwords: Vec<HashMap<&str,&str>> = inputs.iter().map(|pwd| parse_password(pwd)).collect();
    
    let n = passwords.iter().count();
    println!("{} passwords!", n);
    println!("{} valid passwords!", passwords.iter().filter(|pwd| validate_better(pwd)).count());

    //println!("{}",REQQ_FIELDS.iter().all(|item| timber_resources.contains(item)));
}

// sick solution from internet
#[test]
pub fn main2() {
    let input_string = include_str!("../input.txt");
    println!(
        "{}",
        input_string
            .split("\r\n\r\n")
            .map(|fields| fields
                .split_ascii_whitespace()
                .map(|field| field.split(':').next().unwrap())
                .collect::<HashSet<_>>())
            .filter(|passport| REQ_FIELDS.iter().all(|item| passport.contains(item)))
            .count(),
    );
}