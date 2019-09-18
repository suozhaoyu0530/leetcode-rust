/// / 给定一个字符串 s，找到 s 中最长的回文子串。你可以假设 s 的最大长度为 1000。
/// 
/// example 1：
/// 输入: "babad"
/// 输出: "bab"
/// 注意: "aba" 也是一个有效答案。
///
/// example 2：
/// 输入: "cbbd"
/// 输出: "bb"
///
/// 思路：
/// 回文有两种情况：
///     一种是以一个固定位置，左右对称  如：abcba bbb(以中间的b左右对称)
///     另一种是以两个相邻的位置左右对称  如：abcddcba
/// 设定两个值point_left与point_right分别代表对称中心点的左右，当一个中心点时left==right
/// 以中心点左右对称去比较
fn longest_palindrome(s: String) -> String {
    if s.len() == 0 {
        return String::from("");
    }

    let mut point_left = 0;
    let mut point_right = 0;
    let mut serial_char: Option<&str> = None;

    let mut start = 0;
    let mut end = 0;

    for i in 0..s.len(){
        if serial_char.is_some() && i + 1 < s.len() {
            let next_char = &s[i+1..i+2];
            if serial_char.unwrap() == next_char {
                if i + 1 == s.len() - 1 {
                    if s.len() - 1 - point_left > end - start {
                        start = point_left;
                        end = s.len() - 1;
                    }
                }
                continue;
            } else {
                point_right = i;
                serial_char = None;
            }
        } else {
            let cur_char = &s[i..i+1];
            if i + 1 == s.len() {
                if serial_char.is_some() && serial_char.unwrap() == cur_char {
                    if s.len() - 1 - point_left > end - start {
                        start = point_left;
                        end = s.len() - 1;
                    }
                }
                break;
            }

            let next_char = &s[i+1..i+2];

            if cur_char == next_char {
                point_left = i;
                serial_char = Some(cur_char);
                continue;
            } else {
                point_left = i;
                point_right = i;
            }
        }

        if i == 0 {
            continue;
        }

        let mut limit_left = point_left;
        let mut limit_right = point_right;

        for j in 1..s.len()-point_right {
            if point_left < j {
                break;
            }

            let behind_index = point_right + j;
            let front_index = point_left - j;

            let behind_char = &s[behind_index..behind_index+1];
            let front_char = &s[front_index..front_index+1];

            if behind_char == front_char {
                limit_left = front_index;
                limit_right = behind_index;
            } else {
                break;
            }
        }

        let new_len = limit_right - limit_left + 1;
        let old_len = end - start + 1;

        if new_len > old_len {
            start = limit_left;
            end = limit_right;
        }
    }

    let longest_str = &s[start..=end];
    String::from(longest_str)
}

#[test]
fn example1() {
    let s = String::from("babad");
    let expected_str = String::from("bab");

    let longest_str = longest_palindrome(s);

    assert_eq!(expected_str, longest_str);
}

#[test]
fn example2() {
    let s = String::from("cbbd");
    let expected_str = String::from("bb");

    let longest_str = longest_palindrome(s);

    assert_eq!(expected_str, longest_str);
}

#[test]
fn example3() {
    let s = String::from("aaaaaaa");
    let expected_str = String::from("aaaaaaa");

    let longest_str = longest_palindrome(s);

    assert_eq!(expected_str, longest_str);
}

#[test]
fn example4() {
    let s = String::from("abcde");
    let expected_str = String::from("a");

    let longest_str = longest_palindrome(s);

    assert_eq!(expected_str, longest_str);
}

#[test]
fn example5() {
    let s = String::from("");
    let expected_str = String::from("");

    let longest_str = longest_palindrome(s);

    assert_eq!(expected_str, longest_str);
}

#[test]
fn example6() {
    let s = String::from("bb");
    let expected_str = String::from("bb");

    let longest_str = longest_palindrome(s);

    assert_eq!(expected_str, longest_str);
}

#[test]
fn example7() {
    let s = String::from("aba");
    let expected_str = String::from("aba");

    let longest_str = longest_palindrome(s);

    assert_eq!(expected_str, longest_str);
}

#[test]
fn example8() {
    let s = String::from("abadd");
    let expected_str = String::from("aba");

    let longest_str = longest_palindrome(s);

    assert_eq!(expected_str, longest_str);
}