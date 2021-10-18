use std::collections::HashMap;
use std::collections::HashSet;
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
#[derive(Debug, PartialEq, Eq, Hash)]
enum PostE {
    BirthYear,
    IssueYear,
    ExpirationYear,
    Height,
    HairColor,
    EyeColor,
    PassportID,
    CountryID,
}

fn try_convert_post_e(input :&str) -> Option<PostE>{
    match input {
        "byr" => Some(PostE::BirthYear),
        "iyr" => Some(PostE::IssueYear),
        "eyr" => Some(PostE::ExpirationYear),
        "hgt" => Some(PostE::Height),
        "hcl" => Some(PostE::HairColor),
        "ecl" => Some(PostE::EyeColor),
        "pid" => Some(PostE::PassportID),
        "cid" => Some(PostE::CountryID),
        _ => None,
    }
}

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


fn parse_password(batch: &str) -> HashMap<PostE,&str> {
    let b: Vec<&str> = batch.split(|c| c == ' ' || c == '\n').collect();
    let mut map = HashMap::new();
    for field in &b {
        let vals: Vec<&str> = field.split(":").collect();
        map.entry(try_convert_post_e(vals[0]).unwrap()).or_insert(vals[1]);
    }
    println!("{:?}", b);
    println!("{:?}", map);
    let a = map.get(&PostE::PassportID);
    

    if let Some(f) = a {
        let num: u32 = f.parse().unwrap();
        println!("PID = {:?}", f);
        println!("PID = {}", num);
        
    }
    return map;
    
}

const REQ_FIELDS: [&'static str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

const REQQ_FIELDS: [&'static str; 3] = ["Norway", "Denmark", "Iceland"];

fn main2() {
    println!("~~~~ AoC 2020 day04 ~~~~");
    let inputs: Vec<&str> = TEST_INPUT.split("\n\n").collect();
    parse_password(inputs[1]);

    //let timber_resources: HashSet<&str, i32> = [("Norway", 100), ("Denmark", 50), ("Iceland", 10)].iter().cloned().collect();

    //println!("{}",REQQ_FIELDS.iter().all(|item| timber_resources.contains(item)));
}

// sick solution from internet
pub fn main() {
    println!(
        "{}",
        TEST_INPUT
            .split("\n\n")
            .map(|fields| fields
                .split_ascii_whitespace()
                .map(|field| field.split(':').next().unwrap())
                .collect::<HashSet<_>>())
            .filter(|passport| REQ_FIELDS.iter().all(|item| passport.contains(item)))
            .count(),
    );
}