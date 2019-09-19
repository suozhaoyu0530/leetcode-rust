/// 请你来实现一个 atoi 函数，使其能将字符串转换成整数。
/// 首先，该函数会根据需要丢弃无用的开头空格字符，直到寻找到第一个非空格的字符为止。
/// 当我们寻找到的第一个非空字符为正或者负号时，则将该符号与之后面尽可能多的连续数字组合起来，
/// 作为该整数的正负号；假如第一个非空字符是数字，则直接将其与之后连续的数字字符组合起来，形成整数。
/// 该字符串除了有效的整数部分之后也可能会存在多余的字符，这些字符可以被忽略，
/// 它们对于函数不应该造成影响。
/// 注意：假如该字符串中的第一个非空格字符不是一个有效整数字符、字符串为空或字符串仅包含空白字符时，
/// 则你的函数不需要进行转换。
/// 在任何情况下，若函数不能进行有效的转换时，请返回 0。
/// 
/// 说明：
/// 假设我们的环境只能存储 32 位大小的有符号整数。如果数值超过这个范围，
/// 请返回 i32::max_value() 或 i32::min_value 。
/// 
/// example 1:
/// 输入: "42"
/// 输出: 42
///
/// example 2:
/// 输入: "   -42"
/// 输出: -42
/// 解释: 第一个非空白字符为 '-', 它是一个负号。
///      我们尽可能将负号与后面所有连续出现的数字组合起来，最后得到 -42 。
///
/// example 3:
/// 输入: "4193 with words"
/// 输出: 4193
/// 解释: 转换截止于数字 '3' ，因为它的下一个字符不为数字。
///
/// example 4:
/// 输入: "words and 987"
/// 输出: 0
/// 解释: 第一个非空字符是 'w', 但它不是数字或正、负号。
///      因此无法执行有效的转换。
///
/// example 5:
/// 输入: "-91283472332"
/// 输出: -2147483648
/// 解释: 数字 "-91283472332" 超过 32 位有符号整数范围。
///
///
/// 思路：
/// 从左到右一个个的比较
fn my_atoi(str: String) -> i32 {
    let mut r = 0;
    let mut is_minus = false;
    let mut has_no_space_char = false;

    for s in str.chars() {
        if s.is_ascii_whitespace() && !has_no_space_char  {
            continue;
        }

        if s == '-' && !has_no_space_char {
            is_minus = true;
            has_no_space_char = true;
            continue;
        }

        if s == '+' && !has_no_space_char {
            has_no_space_char = true;
            continue;
        }

        has_no_space_char = true;

        if !s.is_ascii_digit() {
            break;
        }

        let num = (s as i32) - 48;
        if is_minus && (r < i32::min_value() / 10 || (r == i32::min_value() / 10 && num > 8)) {
            return i32::min_value();
        }

        if !is_minus && (r > i32::max_value() / 10 || (r == i32::max_value() / 10 && num > 7)) {
            return i32::max_value();
        }

        if is_minus {
            r = r * 10 - num;
        } else {
            r = r * 10 + num;
        }
    }

    r
}

#[test]
fn example1() {
    let str = String::from("42");

    let num_calc = my_atoi(str);
    let num_excepted = 42;

    assert_eq!(num_calc, num_excepted);
}

#[test]
fn example2() {
    let str = String::from("  -42");

    let num_calc = my_atoi(str);
    let num_excepted = -42;

    assert_eq!(num_calc, num_excepted);
}

#[test]
fn example3() {
    let str = String::from("4193 with words");

    let num_calc = my_atoi(str);
    let num_excepted = 4193;

    assert_eq!(num_calc, num_excepted);
}

#[test]
fn example4() {
    let str = String::from("words and 987");

    let num_calc = my_atoi(str);
    let num_excepted = 0;

    assert_eq!(num_calc, num_excepted);
}

#[test]
fn example5() {
    let str = String::from("-91283472332");

    let num_calc = my_atoi(str);
    let num_excepted = i32::min_value();

    assert_eq!(num_calc, num_excepted);
}

#[test]
fn example6() {
    let str = String::from("-2147483649");

    let num_calc = my_atoi(str);
    let num_excepted = -2147483648;

    assert_eq!(num_calc, num_excepted);
}