pub fn run(input: &[i32]) {
    let solution = grading_students(input);

    println!("Input: {:?}", input);
    println!("Output: {:?}", solution)
}

/// HackerLand University has the following grading policy:
/// - Every student receives a *grade* in the inclusive range from *0*  to *100*.
/// - Any *40* less than *40* is a failing grade.
/// Sam is a professor at the university and likes to round each student's *grade* according to these rules:
/// - If the difference between the *grade* and the next multiple of *5* is less than *3*, round  up to the next multiple of *5*.
/// - If the value of *grade* is less than *38*, no rounding occurs as the result will still be a failing grade.
/// # Examples
/// - grade = 84: round to  (85 - 84 is less than 3)
/// - grade = 29: do not round (result is less than 38)
/// - grade = 57: do not round (60 - 57 is 3 or higher)
/// Given the initial value of *grade* for each of Sam's *n* students, write code to automate the rounding process.
/// # Function Description
/// Complete the function  with the following parameter(s):
/// - int grades\[n\]: the grades before rounding
/// # Returns
/// - int[n]: the grades after rounding
/// # Input Format
/// The first line contains a single integer, *n*, the number of students.
/// Each line *i* of the *n* subsequent lines contains a single integer, .
/// # Constraints
/// - 1 <= n <= 60
/// - 0 <= grades\[i\] <= 100
///
/// # Sample Input 0
/// ```
/// 4
/// 73
/// 67
/// 38
/// 33
/// ```
/// # Sample Output 0
/// ```
/// 75
/// 67
/// 40
/// 33
/// ```
/// # Explanation 0
/// | ID   | Original Grade  | Final Grade |
/// | ---- | --------------- | ----------- |
/// |  1   |  70             |  75         |
/// |  2   |  67             |  67         |
/// |  3   |  38             |  40         |
/// |  4   |  33             |  33         |
///
/// Student *1* received a *73*, and the next multiple of *5* from *73* is *75*. Since *75 - 73 < 3*, the student's grade is rounded to *75*.
/// Student *2* received a *67*, and the next multiple of *5* from *67* is *70*. Since *70 - 67 = 3*, the grade will not be modified and the student's final grade is *67*.
/// Student *3* received a *38*, and the next multiple of *5* from *38* is *40*. Since *40 - 38 < 3*, the student's grade will be rounded to 40.
/// Student *4* received a grade below *38*, so the grade will not be modified and the student's final grade is *33*.
fn grading_students(grades: &[i32]) -> Vec<i32> {
    grades[1..]
        .iter()
        .map(|grade| {
            let f32_grade = *grade as f32;
            let next_multiple_of_5 = (f32_grade / 5f32).ceil() * (5f32);
            let mut rounded_grade: i32 = *grade;

            if next_multiple_of_5 - (f32_grade) < 3f32 && f32_grade >= 38f32 {
                rounded_grade = next_multiple_of_5 as i32;
            }

            rounded_grade
        })
        .collect::<Vec<i32>>()
}

#[cfg(test)]
mod test {
    use crate::solutions::grading_students::grading_students;

    #[test]
    fn test_grading_students() {
        let grades = [4, 73, 67, 38, 33];
        let grades_results = grading_students(&grades);
        assert_eq!(grades_results, vec![75, 67, 40, 33])
    }
}
