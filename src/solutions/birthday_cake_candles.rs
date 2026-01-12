pub fn run(input: Vec<i32>) {
    let output = birthday_cake_candles(&input);
    println!("Input: {:?}", input);
    println!("Output: {}", output)
}

/// You are in charge of the cake for a child's birthday. It will have one candle for each year of their total age. They will only be able to blow out the tallest of the candles. Your task is to count how many candles are the tallest.
/// # Example
/// ```
///  candles = [4, 4, 1, 3];
/// ```
/// The tallest candles are 4 units high. There are 2 candles with this height, so the function should return 2.
/// # Function Description
/// Complete the function *birthday_cake_candles*  with the following parameter(s):
/// - int *candles[\n]\*: the candle heights
/// ## Returns
/// - int: the number of candles that are tallest
/// ## Input Format
/// The first line contains a single integer, _n_, the size of _candles[]_.
/// The second line contains _n_ space-separated integers, where each integer _i_ describes the height of _candles\[i]\_.
/// ## Constraints
/// - 1 <= n <= 10^5
/// - i <= candles\[i] <= 10^7
/// ### Sample Input 0
/// ```
/// 4
/// 3 2 1 3
/// ```
/// ### Sample Output 0
/// ```
/// 2
/// ```
/// ### Explanation 0
/// Candle heights are [3, 2, 1, 3]. The tallest candles are 3 units, and there are *2* of them.
fn birthday_cake_candles(candles: &[i32]) -> i32 {
    let tallest_candle = candles.iter().max().unwrap_or(&0);

    candles
        .iter()
        .filter(|candle| candle == &tallest_candle)
        .count() as i32
}

#[cfg(test)]
mod test {
    use crate::solutions::birthday_cake_candles::birthday_cake_candles;

    #[test]
    fn test_birthday_cake_candles() {
        let candles = [3, 2, 1, 3];
        assert_eq!(birthday_cake_candles(&candles), 2)
    }
}
