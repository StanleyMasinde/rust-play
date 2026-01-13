use std::process::exit;

pub fn run(time: &str) {
    let result = time_conversion(time);
    println!("Input: {}", time);
    println!("Output: {}", result);
}

/// Given a time in *12*-hour AM/PM format, convert it to military (24-hour) time.
/// Note: - 12:00:00AM on a 12-hour clock is 00:00:00 on a 24-hour clock.
/// - 12:00:00PM on a 12-hour clock is 12:00:00 on a 24-hour clock.
/// Example
/// _s = '12:01:00PM'_
/// Return '12:01:00'.
/// _s = '12:01:00AM'_
/// Return '00:01:00'.
/// Function Description
/// Complete the time_conversion function with the following parameter(s):
/// - string s: a time in 12 hour format
/// Returns
/// - string: the time in 24 hour format
/// Input Format
/// A single string _s_ that represents a time in _12_-hour clock format (i.e.:*hh:mm:ssAM*  or *hh:mm:ssPM*).
/// Constraints
/// All input times are valid
/// Sample Input 0
/// 07:05:45PM
/// Sample Output 0
/// 19:05:45
fn time_conversion(time: &str) -> String {
    let portions: Vec<&str> = time.split(":").collect();
    if let [hours, minutes, seconds_day_half] = portions.as_slice() {
        let hours_int: u32 = hours.parse().unwrap();
        let (seconds, day_half) = seconds_day_half.split_at(2);
        let twenty4_hours: String;
        if day_half == "AM" && hours_int == 12 {
            twenty4_hours = "00".to_string();
        } else if day_half == "PM" && hours_int != 12 {
            let h = hours_int + 12;
            twenty4_hours = h.to_string();
        } else {
            let mut h = hours_int.to_string();
            if h.len() == 1 {
                h = format!("0{}", h);
            }
            twenty4_hours = h
        }

        let military_time = format!("{}:{}:{}", twenty4_hours, minutes, seconds);
        military_time
    } else {
        eprintln!("Invalid time format!");
        exit(1)
    }
}

#[cfg(test)]
mod test {
    use crate::solutions::time_conversion::time_conversion;

    #[test]
    fn test_time_conversion() {
        assert_eq!(time_conversion("12:01:00PM"), "12:01:00");
        assert_eq!(time_conversion("12:01:00AM"), "00:01:00");
        assert_eq!(time_conversion("06:40:03AM"), "06:40:03");
    }
}
