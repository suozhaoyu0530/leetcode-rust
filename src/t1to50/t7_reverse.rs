/// 给出一个 32 位的有符号整数，你需要将这个整数中每位上的数字进行反转。
/// 
/// example 1:
/// 输入: 123
/// 输出: 321
///
/// example 2:
/// 输入: -123
/// 输出: -321
///
/// example 3:
/// 输入: 120
/// 输出: 21
///
///
/// 思路：
/// 利用x%10 和 x/10 对末尾数进行弹出
/// 但是需要注意反转后是否会超过i32的最大值或最小值
/// i32_max_value：2147483647
/// i32_min_value：-2147483648
fn reverse(x: i32) -> i32 {
    let mut r = 0;
    let mut t = x;

    while t != 0 {
        let p = t % 10;

        if r > i32::max_value() / 10 || (r == i32::max_value() / 10 && p > 7) {
            return 0;
        }

        if r < i32::min_value() / 10 || (r == i32::min_value() / 10 && p < -8) {
            return 0;
        }

        r = r * 10 + p;
        t = t / 10;
    }

    r
}

#[test]
fn example1() {
    let x = 123;

    let x_calc = reverse(x);
    let x_excepted = 321;

    assert_eq!(x_calc, x_excepted);
}

#[test]
fn example2() {
    let x = 120;

    let x_calc = reverse(x);
    let x_excepted = 21;

    assert_eq!(x_calc, x_excepted);
}