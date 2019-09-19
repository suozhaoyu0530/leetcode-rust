/// 判断一个整数是否是回文数。回文数是指正序（从左向右）和倒序（从右向左）读都是一样的整数。
/// 
/// example 1:
/// 输入: 121
/// 输出: true
///
/// example 2:
/// 输入: -121
/// 输出: false
/// 解释: 从左向右读, 为 -121 。 从右向左读, 为 121- 。因此它不是一个回文数。
///
/// example 3:
/// 输入: 10
/// 输出: false
/// 解释: 从右向左读, 为 01 。因此它不是一个回文数。
///
///
/// 思路：
/// 先转化成字符串，然后从中间隔断，进行左右对称的对比，当len == 1时，恒成立
fn is_palindrome_solution1(x: i32) -> bool {
    if x < 0 {
        return false;
    }

    let s = x.to_string();

    if s.len() == 1 {
        return true;
    }

    let (mut point_left, mut point_right) = if s.len() % 2 == 0 {
        let mid = s.len() / 2;
        (mid - 1, mid)
    } else {
        let mid = s.len() / 2;
        (mid, mid)
    };

    let mut r = true;
    let mut jud = true;
    while jud {
        let left = &s[point_left..point_left+1];
        let right = &s[point_right..point_right+1];

        if left != right {
            r = false;
            break;
        }

        if point_left == 0 {
            jud = false;
        } else {
            point_left = point_left - 1;
            point_right = point_right + 1;
        }
    }

    r
}

#[test]
fn example1_solution1() {
    let x = 121;
    assert!(is_palindrome_solution1(x));
}

#[test]
fn example2_solution1() {
    let x = -121;
    assert!(!is_palindrome_solution1(x));
}

#[test]
fn example3_solution1() {
    let x = 10;
    assert!(!is_palindrome_solution1(x));
}