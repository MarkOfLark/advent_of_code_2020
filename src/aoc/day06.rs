use std::collections::HashSet;

#[derive(Debug)]
struct Group {
    answers: Vec<Vec<char>>,
    unique_answers: HashSet<char>,
    common_answers: Vec<char>,
}

impl Group {
    fn new() -> Group {
        return Group{
            answers: Vec::new(),
            unique_answers: HashSet::new(),
            common_answers: Vec::new(),
        };
    }

    fn update(&mut self) {
        for person in &self.answers {
            for answer in person {
                self.unique_answers.insert( *answer );
            }
        }

        self.common_answers = Vec::new();

        let possible_answers = vec![
            'a','b','c','d','e','f','g','h','i','j','k','l','m',
            'n','o','p','q','r','s','t','u','v','w','x','y','z'
        ];

        for a in possible_answers {
            let mut is_common = true;
            for person in &self.answers {
                if !person.contains( &a ) {
                    is_common = false;
                }
            }
            if is_common {
                self.common_answers.push(a);
            }
        }
    }
}

fn parse_input(filename: &str) -> Vec<Group> {
    let lines = super::read_lines(filename).unwrap();

    let mut gg = Group::new();
    let mut group: Vec<Group> = Vec::new();
    for line_option in lines {
        let line = line_option.unwrap();

        if line.len() == 0 {
            gg.update();
            group.push(gg);
            gg = Group::new();
        }
        else {
            let mut a: Vec<char> = line.chars().collect();
            a.sort();
            gg.answers.push(a);
        }
    }
    gg.update();
    group.push(gg);

    return group;
}


pub fn solve( filename: &str ) -> super::Solution {
    let group = parse_input( filename );

    //for g in &group {
    //    println!("{:?}",g);
    //}

    let mut answer_base = 0usize;
    let mut answer_extra = 0usize;
    for g in &group {
        answer_base += g.unique_answers.len();
        answer_extra += g.common_answers.len();
    }

    Ok((answer_base.to_string(),answer_extra.to_string()))
}