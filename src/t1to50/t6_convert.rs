/// 将一个给定字符串根据给定的行数，以从上往下、从左到右进行 Z 字形排列。
/// 比如输入字符串为 "LEETCODEISHIRING" 行数为 3 时，排列如下：
/// L   C   I   R
/// E T O E S I I G
/// E   D   H   N
/// 之后，你的输出需要从左往右逐行读取，产生出一个新的字符串，比如："LCIRETOESIIGEDHN"。
/// 
/// example 1:
/// 输入: s = "LEETCODEISHIRING", numRows = 3
/// 输出: "LCIRETOESIIGEDHN"
///
/// example 2:
/// 输入: s = "LEETCODEISHIRING", numRows = 4
/// 输出: "LDREOEIIECIHNTSG"
/// 解释:
/// L     D     R
/// E   O E   I I
/// E C   I H   N
/// T     S     G
///
///
/// 思路：
/// Z字形就是将字符串从上到下然后再从下到上的排列，不需要形成完整的z字形，只需要按照顺序排列即可
/// 形成一个一维数组，此数组记录每一行上的当前的字符串
/// 然后根据折返判断到了哪一行，在那一行上追加字符串即可
fn convert(s: String, num_rows: i32) -> String {
    let rows = (num_rows as usize).min(s.len());
    let mut row_v = Vec::<String>::new();
    for _ in 0..rows {
        row_v.push(String::new());
    }

    let mut jud = false;

    let mut cur_row = 0;

    for c in s.chars() {
        let behind_str = row_v.get_mut(cur_row).unwrap();
        behind_str.push(c);

        if cur_row == 0 && cur_row == rows - 1 {
            cur_row = 0;
            continue
        } else if cur_row == 0 ||  cur_row == rows - 1{
            jud = !jud;
        }

        cur_row = if jud {
            cur_row + 1
        } else {
            cur_row - 1
        };
    }

    let mut result = String::new();
    for st in row_v {
        for c in st.chars() {
            result.push(c);
        }
    }

    result
}

#[test]
fn example1() {
    let st = String::from("LEETCODEISHIRING");
    let num_rows = 3;

    let str_calc = convert(st, num_rows);
    let str_excepted = String::from("LCIRETOESIIGEDHN");

    assert_eq!(str_calc, str_excepted);
}

#[test]
fn example2() {
    let st = String::from("LEETCODEISHIRING");
    let num_rows = 4;

    let str_calc = convert(st, num_rows);
    let str_excepted = String::from("LDREOEIIECIHNTSG");

    assert_eq!(str_calc, str_excepted);
}

#[test]
fn example3() {
    let st = String::from("abc");
    let num_rows = 4;

    let str_calc = convert(st, num_rows);
    let str_excepted = String::from("abc");

    assert_eq!(str_calc, str_excepted);
}

#[test]
fn example4() {
    let st = String::from("AB");
    let num_rows = 1;

    let str_calc = convert(st, num_rows);
    let str_excepted = String::from("AB");

    assert_eq!(str_calc, str_excepted);
}