pub fn run(input: &[Vec<i32>]) {
    let output = forming_magic_square(input);

    println!("Input: {:?}", input);
    println!("Output: {}", output)
}

/// We define a [magic square](https://en.wikipedia.org/wiki/Magic_square) to be an *n x n* matrix of distinct positive integers from *1* to *n POW 2* where the sum of any row, column, or diagonal of length *n* is always equal to the same number: the magic constant.
/// You will be given a *3 x 3* matrix *s* of integers in the inclusive range *\[1, 9\]*. We can convert any digit *a* to any other digit *b* in the range *\[1, 9\]* at cost of *|a - b|*. Given *s*, convert it into a magic square at minimal cost. Print this cost on a new line.
/// *Note:* The resulting magic square must contain distinct integers in the inclusive range *\[1, 9\]*.
/// # Example
/// ```text
/// $s = [[5, 3, 4], [1, 5, 8], [6, 4, 2]]
/// ```
/// The matrix looks like this:
/// ```text
/// 5 3 4
/// 1 5 8
/// 6 4 2
/// ```
/// We can convert it to the following magic square:
/// ```text
/// 8 3 4
/// 1 5 9
/// 6 7 2
/// ```
/// This took three replacements at a cost of *|5 - 8| + |8 - 9| + |4 - 7|*.
/// # Function Description
/// Complete the _forming_magic_square_ function in the editor below.
/// _forming_magic_square_ has the following parameter(s):
/// - int s[3][3]: a *3 x 3* array of integers
/// # Returns
/// - int: the minimal total cost of converting the input square to a magic square
/// # Input Format
/// Each of the  lines contains three space-separated integers of row .
/// # Constraints
/// *s\[i\]\[j\] ∈ \[1, 9\]*
/// # Sample Input 0
/// ```text
/// 4 9 2
/// 3 5 7
/// 8 1 5
/// ```
/// # Sample Output 0
/// *1*
/// # Explanation 0
/// If we change the bottom right value, *s\[2\]\[2\]*, from  to  at a cost of *|6 - 5| = 1*, *s* becomes a magic square at the minimum possible cost.
/// # Sample Input 1
/// ```text
/// 4 8 2
/// 4 5 7
/// 6 1 6
/// ```
/// # Sample Output 1
/// *4*
/// # Explanation 1
/// Using 0-based indexing, if we make
/// - *s[0][1] -> 9* at a cost of *|9 - 8 = 1|*
/// - *[1][0] -> 3* at a cost of *|3 - 4 = 1|*
/// - *[2][0] -> 8* at a cost of *|8 - 6 = 2|*,
///
/// then the total cost will be *1 + 1 + 2 = 4*.
fn forming_magic_square(s: &[Vec<i32>]) -> i32 {
    // All 8 possible 3x3 magic squares (flattened)
    let valid_squares = vec![
        vec![8, 1, 6, 3, 5, 7, 4, 9, 2],
        vec![4, 3, 8, 9, 5, 1, 2, 7, 6],
        vec![2, 9, 4, 7, 5, 3, 6, 1, 8],
        vec![6, 7, 2, 1, 5, 9, 8, 3, 4],
        vec![6, 1, 8, 7, 5, 3, 2, 9, 4],
        vec![8, 3, 4, 1, 5, 9, 6, 7, 2],
        vec![4, 9, 2, 3, 5, 7, 8, 1, 6],
        vec![2, 7, 6, 9, 5, 1, 4, 3, 8],
    ];

    // Flatten input 2D array to 1D
    let input: Vec<i32> = s.iter().flatten().copied().collect();

    // Find minimum cost across all 8 magic squares
    let mut min_cost = i32::MAX;

    for square in &valid_squares {
        let mut cost = 0;
        for i in 0..9 {
            cost += (input[i] - square[i]).abs();
        }
        if cost < min_cost {
            min_cost = cost;
        }
    }

    min_cost
}

#[cfg(test)]
mod test {
    use crate::solutions::forming_magic_square::forming_magic_square;

    #[test]
    fn test_forming_magic_square() {
        let s1 = [vec![4, 9, 2], vec![2, 5, 7], vec![8, 1, 5]];

        assert_eq!(forming_magic_square(&s1), 2);
    }
}
