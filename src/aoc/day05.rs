use std::convert::TryInto;
use std::cmp::Ordering;

#[derive(Debug,Eq,Clone)]
struct Seat {
    code: [char;10],
    row: u32,
    col: u32,
    id: u32,
}

impl Ord for Seat {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}
impl PartialOrd for Seat {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for Seat {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Seat {
    fn new( code_str: &str ) -> Seat {
        let code_chars: Vec<char> = code_str.chars().collect();

        let mut s = Seat{
            code: code_chars.try_into().unwrap(),
            row: 0u32,
            col: 0u32,
            id: 0u32,
        };

        s.row = Seat::bsp( &s.code[0..7], 'B', 'F' );
        s.col = Seat::bsp( &s.code[7..], 'R', 'L' );
        s.id = s.row * 8 + s.col;

        return s;
    }

    fn bsp( code: &[char], up: char, low: char ) -> u32 {
        let mut min = 0u32;
        let mut max = 2u32.pow( code.len() as u32 ) - 1;

        for c in code {
            //println!("{} {} {}", c, min, max);
            if *c == low {
                max = ( max + min ) / 2;
            }
            if *c == up {
                min = 1 + ( max + min ) / 2;
            }
        }

        if min != max {
            panic!("Didn't Converge");
        }

        return min;
    }
}

fn parse_input(filename: &str) -> Vec<Seat> {
    let lines = super::read_lines(filename).unwrap();

    let mut seat: Vec<Seat> = Vec::new();

    for line_option in lines {
        seat.push(Seat::new( &line_option.unwrap()))
    }

    return seat;
}


pub fn solve( filename: &str ) -> super::Solution {
    let mut seat = parse_input( filename );
    seat.sort();

    //for s in &seat {
    //    println!("{:?}",s);
    //}


    let max_id = seat[seat.len()-1].id;

    let mut my_seat = 0u32;
    for s in (0..seat.len()-1) {
        if seat[s].id+2 == seat[s+1].id {
            my_seat = seat[s].id+1;
        }
    }

    Ok((max_id.to_string(),my_seat.to_string()))
}