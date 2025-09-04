use std::collections::HashMap;

use crate::solutions;

fn get_problem_hashmap() -> std::collections::HashMap<String, fn()> {
    let mut map: HashMap<String, fn()> = HashMap::new();

    map.insert("sum".to_string(), solutions::sum::run);
    map.insert("sum_array".to_string(), solutions::sum_array::run);
    map.insert("compare_triplets".to_string(), solutions::compare_triplets::run);

    map
}

pub fn run_solution(problem: &str) {
    match get_problem_hashmap().get(problem) {
        Some(solution) => solution(),
        None => {
            eprintln!("Invalid problem.");
            println!("These are the available problems:");

            for key in get_problem_hashmap().keys() {
                println!("{key}")
            }
        }
    };
}
