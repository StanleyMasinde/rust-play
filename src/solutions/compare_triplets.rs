pub fn run(alice: Option<Vec<i32>>, bob: Option<Vec<i32>>) {
    let a = match alice {
        Some(array) => array,
        None => [5, 6, 7].into(),
    };

    let b = match bob {
        Some(array) => array,
        None => [3, 6, 10].into(),
    };

    let output = compare_triplets(&a, &b);
    println!("Input: {a:?}\n{b:?}");
    println!("Output {output:?}")
}

/// Compares two triplets of scores and returns the comparison points for each participant.
///
/// Alice and Bob each created one problem for HackerRank. A reviewer rates the two
/// challenges, awarding points on a scale from 1 to 100 for three categories:
/// problem clarity, originality, and difficulty.
///
/// The rating for Alice's challenge is the triplet `a = (a[0], a[1], a[2])`,
/// and the rating for Bob's challenge is `b = (b[0], b[1], b[2])`.
///
/// The task is to calculate their comparison points by comparing each category:
/// - If a\[i] > b\[i], Alice gets 1 point.
/// - If a\[i] < b\[i], Bob gets 1 point.
/// - If a\[i] == b\[i], no one scores.
///
/// # Arguments
/// * `a` - Alice's scores: clarity, originality, difficulty
/// * `b` - Bob's scores: clarity, originality, difficulty
///
/// # Returns
///
/// A 2-element vector: `[alice_score, bob_score]`
///
/// # Examples
///
/// Basic comparison where Alice wins 2 categories:
///
/// ```
/// use solutions::compare_triplets::compare_triplets;
/// let a = [5, 6, 7];
/// let b = [3, 6, 10];
/// assert_eq!(compare_triplets(&a, &b), vec![1, 1]);
/// // Alice: 5 > 3 (1 point), 6 == 6 (0 points), 7 < 10 (0 points) = 1 point
/// // Bob: 3 < 5 (0 points), 6 == 6 (0 points), 10 > 7 (1 point) = 1 point
/// ```
///
/// Alice wins clarity and difficulty:
///
/// ```
/// use solutions::compare_triplets::compare_triplets;
/// let a = [17, 28, 30];
/// let b = [99, 16, 8];
/// assert_eq!(compare_triplets(&a, &b), vec![2, 1]);
/// // Alice: 17 < 99 (0 points), 28 > 16 (1 point), 30 > 8 (1 point) = 2 points
/// // Bob: 99 > 17 (1 point), 16 < 28 (0 points), 8 < 30 (0 points) = 1 point
/// ```
///
/// All scores are equal (tie):
///
/// ```
/// # use solutions::compare_triplets::compare_triplets;
/// let a = [10, 20, 30];
/// let b = [10, 20, 30];
/// assert_eq!(compare_triplets(&a, &b), vec![0, 0]);
/// // All categories tied, no points awarded
/// ```
///
/// Alice wins all categories:
///
/// ```
/// # use solutions::compare_triplets::compare_triplets;
/// let a = [100, 100, 100];
/// let b = [1, 1, 1];
/// assert_eq!(compare_triplets(&a, &b), vec![3, 0]);
/// ```
/// // Alice wins all three categories
fn compare_triplets(a: &[i32], b: &[i32]) -> Vec<i32> {
    let mut result = Vec::new();
    let mut alice = 0;
    let mut bob = 0;

    for index in 0..=2 {
        let alice_val = a[index];
        let bob_val = b[index];

        if alice_val > bob_val {
            alice += 1
        } else if bob_val > alice_val {
            bob += 1
        }
    }
    result.push(alice);
    result.push(bob);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compare_triplets() {
        let a = [5, 6, 7];
        let b = [3, 6, 10];
        let res = compare_triplets(&a, &b);
        assert_eq!(res, [1, 1])
    }

    #[test]
    fn test_compare_triplets_2() {
        let a = [17, 28, 30];
        let b = [99, 16, 8];

        let res = compare_triplets(&a, &b);
        assert_eq!(res, [2, 1])
    }
}
