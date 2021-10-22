//#![feature(str_split_once)]


use std::collections::HashMap;
// use std::collections::HashSet;
// use std::fs;
use regex::Regex;
use lazy_static::lazy_static;
use std::env;

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

    let valid = pwd_map.iter().all(|(&key, &val)| validate_and_print(key,val));
    // if !valid {
    //     println!("{:?}", pwd_map);
    // }
    return valid;
}
fn validate_and_print(key: &str, content: &str) -> bool {
    if validate_field(key, content) {
        return true;
    } else {
        //println!("Failed at key:'{}'={}",key,content);
        return false;
    }
}
fn validate_field(key: &str, content: &str) -> bool {

    match key {
        "byr" => { //four digits; at least 1920 and at most 2002
            return {
                content.chars().count() == 4 &&
                if let Ok(byr) = content.parse::<i32>() {
                    byr >= 1920 && byr <= 2002
                } else {
                    false
                }
            };
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
            if content.ends_with("cm") { // the number must be at least 150 and at most 193
                let mut chars = content.chars();
                chars.next_back();
                chars.next_back();
                if let Ok(hgt) = chars.as_str().parse::<u32>() {
                    return hgt >= 150 && hgt <= 193;
                } else {
                    return false;
                }
            } else if content.ends_with("in") { // the number must be at least 59 and at most 76
                let mut chars = content.chars();
                chars.next_back();
                chars.next_back();
                if let Ok(hgt) = chars.as_str().parse::<u32>() {
                    return hgt >= 59 && hgt <= 76;
                } else {
                    return false;
                }
            } else {
                return false;
            }
        }
        "hcl" => { // a # followed by exactly six characters 0-9 or a-f
            if content.starts_with("#") {
                let mut chars = content.chars();
                chars.next();
                return content.chars().count() == 7 &&
                chars.all(|c| (c >= '0' && c <= '9') || (c >= 'a' && c <= 'f'));
            } else {
                return false;
            }
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

#[test]
fn test_validate_field() {
    // byr valid:   2002
    // byr invalid: 2003
    assert_eq!(true, validate_field("byr", "2002"), "byr valid:   2002");
    assert_eq!(false, validate_field("byr", "2003"), "byr invalid: 2003");
    
    // hgt valid:   60in
    // hgt valid:   190cm
    // hgt invalid: 190in
    // hgt invalid: 190
    assert_eq!(true, validate_field("hgt",  "60in"), "hgt valid:   60in");
    assert_eq!(true, validate_field("hgt",  "190cm"), "hgt valid:   190cm");
    assert_eq!(false, validate_field("hgt", "190in"), "hgt invalid: 190in");
    assert_eq!(false, validate_field("hgt", "190"), "hgt invalid: 190");

    // hcl valid:   #123abc
    // hcl invalid: #123abz
    // hcl invalid: 123a
    assert_eq!(true, validate_field("hcl", "#123abc"), "hcl valid:   #123abc");
    assert_eq!(false, validate_field("hcl","#123abz"), "hcl invalid: #123abz" );
    assert_eq!(false, validate_field("hcl","123abc"), "hcl invalid: 123abc");

    // ecl valid:   brn
    // ecl invalid: wat
    assert_eq!(true, validate_field("ecl", "brn"), "ecl valid:   brn");
    assert_eq!(false, validate_field("ecl", "wat"), "ecl invalid: wat");

    // pid valid:   000000001
    // pid invalid: 0123456789
    assert_eq!(true, validate_field("pid", "000000001" ), "pid valid:   000000001");
    assert_eq!(false, validate_field("pid", "0123456789" ), "pid invalid: 0123456789");
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let inp_arg = &args[1];
    let mut part1 = true;
    let mut part2 = true;
    if inp_arg.contains("2") && !inp_arg.contains("1") {
        part1 = false;
    }
    if inp_arg.contains("1") && !inp_arg.contains("2"){
        part2 = false;
    }
    println!("~~~~ AoC 2020 day04 ~~~~");

    let mut input_string = include_str!("../input.txt");
    if args.len() > 2 {
        input_string = &args[2];
    }
    let mut clean_inputs = String::from(input_string);
    clean_inputs = clean_inputs.replace("\r","");
    let inputs: Vec<&str> = clean_inputs.split("\n\n").collect();
    let passwords: Vec<HashMap<&str,&str>> = inputs.iter().map(|pwd| parse_password(pwd)).collect();
    let n = passwords.iter().count();
    print!("{} passwords!\n\n", n);
    if part1 {
        println!("--- Part 1 ---");
        print!("- {} valid passports!\n\n", passwords.iter().filter(|pwd| validate(pwd)).count());
    }
    if part2 {
        println!("--- Part 2 ---");
        print!("- {} valid passports!\n\n", passwords.iter().filter(|pwd| validate_better(pwd)).filter(|pwd| validate(pwd)).count());
    }
}

