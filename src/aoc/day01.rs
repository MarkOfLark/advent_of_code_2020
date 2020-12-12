
pub fn solve( filename: &str ) -> super::Solution {

    let lines = super::read_lines(filename).unwrap();

    let values: Vec<u32> = lines.map(|s| s.unwrap().parse::<u32>().unwrap() ).collect();

    let mut base = 0u32;
    for val1 in &values {
        for val2 in &values {
            if 2020 == val1 + val2 {
                base = val1*val2;
            }
        }
    }

    let mut extra = 0u32;
    for val1 in &values {
        for val2 in &values {
            for val3 in &values {
                if 2020 == val1 + val2 + val3 {
                    extra = val1*val2*val3;
                }
            }
        }
    }

    Ok((base.to_string(),extra.to_string()))
}