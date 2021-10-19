use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

//use std::io::BufRead;

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
    let byr = pwd_map.get("byr").unwrap();
    let iyr = pwd_map.get("iyr").unwrap();
    let eyr = pwd_map.get("eyr").unwrap();
    let hgt = pwd_map.get("hgt").unwrap();
    let hcl = pwd_map.get("hcl").unwrap();
    let ecl = pwd_map.get("ecl").unwrap();
    let pid = pwd_map.get("pid").unwrap();
    let cid = pwd_map.get("cid").unwrap();
    byr >=1920 && byr <=2002
    
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