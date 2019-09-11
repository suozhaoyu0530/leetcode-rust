use std::collections::HashMap;
use std::cmp;

/// 给定一个字符串，请你找出其中不含有重复字符的 最长子串 的长度。
/// 
/// example 1:
/// 输入: "abcabcbb"
/// 输出: 3 
/// 解释: 因为无重复字符的最长子串是 "abc"，所以其长度为 3。
///  
/// example 2:
/// 输入: "bbbbb"
/// 输出: 1
/// 解释: 因为无重复字符的最长子串是 "b"，所以其长度为 1。
///
/// example 3:
/// 输入: "pwwkew"
/// 输出: 3
/// 解释: 因为无重复字符的最长子串是 "wke"，所以其长度为 3。
///      请注意，你的答案必须是 子串 的长度，"pwke" 是一个子序列，不是子串。


/// 思路：
/// 用一个map将字符出现的位置记录下来
/// 并用一个变量front记录下上次重复的位置，在front之前记录的字符位置不作数需要重新记录
/// 用来排除asdfghdqwea这种情况,d已经重复，所以a-a不作数
pub fn length_of_longest_substring(s: String) -> i32 {
    let mut front = 0;
    let mut max_length = 0;

    let mut map = HashMap::new();

    let mut i = 0;
    for c in s.chars() {
        if map.contains_key(&c) {
            let c_index = map.get_mut(&c).unwrap();

            if *c_index >= front {
                max_length = cmp::max(max_length, i-front);

                front = *c_index+1;
                max_length = cmp::max(max_length, i-*c_index);
                *c_index = i;
                i = i+1;
                continue;
            }

            *c_index = i;
        }

        map.insert(c, i);
        i = i+1;
    }

    max_length = cmp::max(max_length, i-front);

    max_length
}

#[test]
fn example1() {
    let s = String::from("abcabcbb");
    let max_length_expected = 3;
    let max_length = length_of_longest_substring(s);

    assert_eq!(max_length, max_length_expected);
}

#[test]
fn example2() {
    let s = String::from("bbbbb");
    let max_length_expected = 1;
    let max_length = length_of_longest_substring(s);

    assert_eq!(max_length, max_length_expected);
}

#[test]
fn example3() {
    let s = String::from("pwwkew");
    let max_length_expected = 3;
    let max_length = length_of_longest_substring(s);

    assert_eq!(max_length, max_length_expected);
}

#[test]
fn example4() {
    let s = String::from("cdd");
    let max_length_expected = 2;
    let max_length = length_of_longest_substring(s);

    assert_eq!(max_length, max_length_expected);
}

#[test]
fn example5() {
    let s = String::from("dvdf");
    let max_length_expected = 3;
    let max_length = length_of_longest_substring(s);

    assert_eq!(max_length, max_length_expected);
}

#[test]
fn example6() {
    let s = String::from(" ");
    let max_length_expected = 1;
    let max_length = length_of_longest_substring(s);

    assert_eq!(max_length, max_length_expected);
}

#[test]
fn example7() {
    let s = String::from("abba");
    let max_length_expected = 2;
    let max_length = length_of_longest_substring(s);

    assert_eq!(max_length, max_length_expected);
}

#[test]
fn example8() {
    let s = String::from("ohomm");
    let max_length_expected = 3;
    let max_length = length_of_longest_substring(s);

    assert_eq!(max_length, max_length_expected);
}

#[test]
fn example9() {
    let s = String::from("abcb");
    let max_length_expected = 3;
    let max_length = length_of_longest_substring(s);

    assert_eq!(max_length, max_length_expected);
}