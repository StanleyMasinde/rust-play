use std::collections::HashMap;

use crate::solutions;

fn get_problem_hashmap() -> std::collections::HashMap<String, fn()> {
    let mut map: HashMap<String, fn()> = HashMap::new();

    map.insert("sum".to_string(), solutions::sum::run);

    map
}

pub fn run_solution(problem: String) {
    match get_problem_hashmap().get(&problem) {
        Some(solution) => solution(),
        None => eprintln!("Invalid problem."),
    };
}
