use std::collections::HashMap;

pub fn solve( filename: &str ) -> super::Solution {

    let lines = super::read_lines(filename).unwrap();

    let tree_map: Vec<Vec<char>> = lines.map(|s| {
        let line = s.unwrap();
        line.chars().collect()
    }).collect();

    let mut slopes: Vec<(usize,usize)> = vec!{
        (1usize,1usize),
        (3usize,1usize),
        (5usize,1usize),
        (7usize,1usize),
        (1usize,2usize),
    };

    let mut paths: HashMap<(usize,usize),u32> = HashMap::new();

    let map_width = tree_map[0].len();

    for (right,down) in &slopes {
        let mut c = 0usize;
        let mut num_trees = 0u32;
        for r in (0..tree_map.len()).step_by(*down) {
            if tree_map[r][c % map_width] == '#' {
                num_trees += 1;
            }
            c += *right;
        }

        paths.insert((*right,*down),num_trees);
    }

    let mut product = 1usize;
    for (_,num_trees) in &paths {
        product *= *num_trees as usize;
    }

    Ok((paths.get(&(3usize,1usize)).unwrap().to_string(),product.to_string()))
}