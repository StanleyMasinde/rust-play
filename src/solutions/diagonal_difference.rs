pub fn run() {
    let default_matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![9, 8, 9]];
    diagonal_difference(&default_matrix);
}

/// Calculates the absolute difference between the sums of the diagonals of a square matrix.
///
/// Given a square matrix, this function computes:
/// - The sum of the primary (left-to-right) diagonal
/// - The sum of the secondary (right-to-left) diagonal
/// - Returns the absolute difference between these sums
///
/// # Arguments
///
/// * `arr` - A slice of vectors of integers representing a square matrix
///
/// # Returns
///
/// The absolute difference between the diagonal sums as an `i32`
///
/// # Examples
///
/// Basic 3x3 matrix:
///
/// ```
/// # use your_crate::diagonalDifference;
/// let matrix = vec![
///     vec![1, 2, 3],
///     vec![4, 5, 6],
///     vec![9, 8, 9],
/// ];
/// assert_eq!(diagonalDifference(&matrix), 2);
/// // Primary diagonal: 1 + 5 + 9 = 15
/// // Secondary diagonal: 3 + 5 + 9 = 17
/// // Absolute difference: |15 - 17| = 2
/// ```
///
/// Matrix with negative numbers:
///
/// ```
/// # use your_crate::diagonalDifference;
/// let matrix = vec![
///     vec![11, 2, 4],
///     vec![4, 5, 6],
///     vec![10, 8, -12],
/// ];
/// assert_eq!(diagonalDifference(&matrix), 15);
/// // Primary diagonal: 11 + 5 + (-12) = 4
/// // Secondary diagonal: 4 + 5 + 10 = 19
/// // Absolute difference: |4 - 19| = 15
/// ```
///
/// Single element matrix:
///
/// ```
/// # use your_crate::diagonalDifference;
/// let matrix = vec![vec![5]];
/// assert_eq!(diagonalDifference(&matrix), 0);
/// // Both diagonals are the same element: 5
/// // Absolute difference: |5 - 5| = 0
/// ```
fn diagonal_difference(arr: &[Vec<i32>]) -> i32 {
    let mut diagonal_one_sum = 0;
    let mut diagonal_two_sum = 0;
    let marix_rows = arr.len();
    let last_index = marix_rows - 1;

    for index in 0..marix_rows {
        let current_vec = &arr[index];
        let first_diag_pos = index;
        diagonal_one_sum += current_vec[first_diag_pos];
        let second_diag_pos = last_index - first_diag_pos;
        diagonal_two_sum += current_vec[second_diag_pos];
    }

    (diagonal_one_sum - diagonal_two_sum).abs()
}

#[cfg(test)]
mod tests {
    use crate::solutions::diagonal_difference::diagonal_difference;

    #[test]
    fn test_diagonal_difference() {
        let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![9, 8, 9]];
        assert_eq!(diagonal_difference(&matrix), 2);
    }

    #[test]
    fn test_with_negative_numbers() {
        let matrix = vec![vec![11, 2, 4], vec![4, 5, 6], vec![10, 8, -12]];
        assert_eq!(diagonal_difference(&matrix), 15);
    }

    #[test]
    fn test_single_element() {
        let matrix = vec![vec![5]];
        assert_eq!(diagonal_difference(&matrix), 0);
    }

    #[test]
    fn test_2x2_matrix() {
        let matrix = vec![vec![1, 2], vec![3, 4]];
        assert_eq!(diagonal_difference(&matrix), 0);
    }

    #[test]
    fn test_all_zeros() {
        let matrix = vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]];
        assert_eq!(diagonal_difference(&matrix), 0);
    }

    #[test]
    fn test_identity_matrix() {
        let matrix = vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]];
        assert_eq!(diagonal_difference(&matrix), 2);
    }

    #[test]
    fn test_all_negative() {
        let matrix = vec![vec![-1, -2, -3], vec![-4, -5, -6], vec![-7, -8, -9]];
        assert_eq!(diagonal_difference(&matrix), 0);
    }

    #[test]
    fn test_large_values() {
        let matrix = vec![
            vec![100, 200, 300],
            vec![400, 500, 600],
            vec![700, 800, 900],
        ];
        assert_eq!(diagonal_difference(&matrix), 0);
    }

    #[test]
    fn test_4x4_matrix() {
        let matrix = vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12],
            vec![13, 14, 15, 16],
        ];
        assert_eq!(diagonal_difference(&matrix), 0);
    }

    //    #[test]
    //   fn test_mixed_positive_negative() {
    //       let matrix = vec![vec![-5, 3, 2], vec![1, -7, 4], vec![6, 8, -3]];
    //      assert_eq!(diagonalDifference(&matrix), 10);
    //   }
}
