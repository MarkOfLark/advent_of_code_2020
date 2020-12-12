struct Password {
    min: usize,
    max: usize,
    val: char,
    password: Vec<char>,
}


pub fn solve( filename: &str ) -> super::Solution {

    let lines = super::read_lines(filename).unwrap();

    let passwords: Vec<Password> = lines.map(|s| {
        let line = s.unwrap();
        let tokens: Vec<&str> = line.split(&['-', ':', ' '][..]).collect();

        if 5 != tokens.len() {
            for token in tokens {
                println!("token {}",token);
            }
            println!("Had a problem parsing line: {}", line);
            panic!("Could not parse");
        }

        Password{
            min: tokens[0].parse::<usize>().unwrap(),
            max: tokens[1].parse::<usize>().unwrap(),
            val: tokens[2].chars().next().unwrap(),
            password: tokens[4].chars().collect()
        }
    }).collect();

    let mut valid_count_base = 0;
    for password in &passwords {
        let mut num_chars_that_match = 0;
        for c in &password.password {
            if *c == password.val {
                num_chars_that_match += 1;
            }
        }

        if password.min <= num_chars_that_match && password.max >= num_chars_that_match {
            valid_count_base += 1;
        }
    }

    let mut valid_count_extra = 0;
    for password in &passwords {
        if
            (password.password[password.min-1] == password.val) ^
            (password.password[password.max-1] == password.val)
        {
            valid_count_extra += 1;
        }
    }

    Ok((valid_count_base.to_string(),valid_count_extra.to_string()))
}