use std::fmt; // Import `fmt`


#[derive(Debug, PartialEq, PartialOrd)]
struct BoardingPass {
    row: u8,
    col: u8,
    seat_id: u16,
}

impl BoardingPass {
    fn decode(code: &str) -> BoardingPass {
        // print!("code: {:?} ",code);
        let (row_code, col_code) = code.split_at(7);
        // print!("({},{}) ",row_code, col_code);
        let _row = u8::from_str_radix(row_code.replace("F", "0").replace("B", "1").as_str(),2).unwrap();
        let _col = u8::from_str_radix(col_code.replace("R", "1").replace("L", "0").as_str(),2).unwrap();
        // println!("->({:b},{:b})", _row, _col);
        let _seat_id: u16 = (_row as u16)*8 + (_col as u16);
        BoardingPass { row: _row, col: _col, seat_id: _seat_id}
    }
    fn print(&self) {
        println!("{}",self);
    }
}

impl fmt::Display for BoardingPass {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}(row:{}, col:{})", self.seat_id, self.row, self.col)
    }
}


#[test]
fn test_boarding_pass() {
    let bp_first = ("FFFFFFFLLL", BoardingPass{row: 0, col: 0, seat_id: 0});
    let bp_sample = ("FBFBBFFRLR", BoardingPass{row: 44, col: 5, seat_id: 357});
    let bp_sample0 = ("BFFFBBFRRR", BoardingPass{row: 70, col: 7, seat_id: 567});
    let bp_sample1 = ("FFFBBBFRRR", BoardingPass{row: 14, col: 7, seat_id: 119});
    let bp_sample2 = ("BBFFBBFRLL", BoardingPass{row: 102, col: 4, seat_id: 820});
    let bp_last = ("BBBBBBBRRR", BoardingPass{row: 127, col: 7, seat_id: 127*8+7});
    assert_eq!(BoardingPass::decode(bp_first.0), bp_first.1);
    assert_eq!(BoardingPass::decode(bp_sample.0), bp_sample.1);
    assert_eq!(BoardingPass::decode(bp_sample0.0), bp_sample0.1);
    assert_eq!(BoardingPass::decode(bp_sample1.0), bp_sample1.1);
    assert_eq!(BoardingPass::decode(bp_sample2.0), bp_sample2.1);
    assert_eq!(BoardingPass::decode(bp_last.0), bp_last.1);
}

fn main() {
    println!("--- Day 5: Binary Boarding ---");
    let puzzle_input = include_str!("../input.txt");
    for line in puzzle_input.lines() {
        let bp = BoardingPass::decode(line);
        bp.print();
    }
    let b_passes: Vec<BoardingPass> = puzzle_input.lines().map(|line|BoardingPass::decode(line)).collect();
    let maximum_seat_id = b_passes.iter().map(|bp| bp.seat_id).max().unwrap();
    println!("Largest seat_id: {}", maximum_seat_id);
}
