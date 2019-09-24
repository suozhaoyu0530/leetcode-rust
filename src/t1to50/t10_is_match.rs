/// 给你一个字符串 s 和一个字符规律 p，请你来实现一个支持 '.' 和 '*' 的正则表达式匹配。
/// 
/// '.' 匹配任意单个字符
/// '*' 匹配零个或多个前面的那一个元素
/// 所谓匹配，是要涵盖 整个 字符串 s的，而不是部分字符串。
/// 
/// 说明:
/// s 可能为空，且只包含从 a-z 的小写字母。
/// p 可能为空，且只包含从 a-z 的小写字母，以及字符 . 和 *。
/// 
/// example 1:
/// 输入:
/// s = "aa"
/// p = "a"
/// 输出: false
/// 解释: "a" 无法匹配 "aa" 整个字符串。
///
/// example 2:
/// 输入:
/// s = "aa"
/// p = "a*"
/// 输出: true
/// 解释: 因为 '*' 代表可以匹配零个或多个前面的那一个元素, 在这里前面的元素就是 'a'。因此，字符串 "aa" 可被视为 'a' 重复了一次。
///
/// example 3:
/// 输入:
/// s = "ab"
/// p = ".*"
/// 输出: true
/// 解释: ".*" 表示可匹配零个或多个（'*'）任意字符（'.'）。
///
/// example 4:
/// 输入:
/// s = "aab"
/// p = "c*a*b"
/// 输出: true
/// 解释: 因为 '*' 表示零个或多个，这里 'c' 为 0 个, 'a' 被重复一次。因此可以匹配字符串 "aab"。
///
/// example 5:
/// 输入:
/// s = "mississippi"
/// p = "mis*is*p*."
/// 输出: false
///
/// 思路:
/// 先将p分组，*与前面一个非*的为一组，带有*的相邻的不能重复，重复的话则抛弃掉一个
/// 如 a*a*a*=a*  .*a*b*c*=.*  a*b*c*.*=.*
/// 然后分组往后推移字符的检测位置
fn is_match(s: String, p: String) -> bool {
    
}

#[test]
fn example1() {
    let s = String::from("aa");
    let p = String::from("a");

    assert!(!is_match(s, p));
}

#[test]
fn example2() {
    let s = String::from("aa");
    let p = String::from("a*a*a*a*");

    assert!(is_match(s, p));
}

#[test]
fn example3() {
    let s = String::from("ab");
    let p = String::from(".*");

    assert!(is_match(s, p));
}

#[test]
fn example4() {
    let s = String::from("aab");
    let p = String::from("c*a*b");

    assert!(is_match(s, p));
}

#[test]
fn example5() {
    let s = String::from("mississippi");
    let p = String::from("mis*is*p*.");

    assert!(!is_match(s, p));
}

#[test]
fn example6() {
    let s = String::from("aaa");
    let p = String::from("ab*a*c*a");

    assert!(is_match(s, p));
}

#[test]
fn example7() {
    let s = String::from("mississippi");
    let p = String::from("mis*is*ip*.");

    assert!(is_match(s, p));
}

#[test]
fn example8() {
    let s = String::from("ab");
    let p = String::from(".*ab");

    assert!(is_match(s, p));
}

#[test]
fn example9() {
    let s = String::from("ab");
    let p = String::from(".*..");

    assert!(is_match(s, p));
}