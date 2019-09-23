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
    let mut p_v = Vec::<Vec<char>>::new();

    for c in p.chars() {
        if c == '*' {
            if !p_v.is_empty() {
                let c_front_v = p_v.last_mut().unwrap();
                let c_f = c_front_v[0];
                if (c_front_v.len() == 1 && c_f == '*') || c_front_v.len() == 2 {
                    continue;
                } else {
                    c_front_v.push(c);
                }

                if p_v.len() == 1 {
                    continue;
                }

                if c_f == '.' {
                    for i in (0..p_v.len()-1).rev() {
                        if p_v.get(i).unwrap().len() == 2 {
                            p_v.remove(i);
                        } else {
                            break;
                        }
                    }
                } else {
                    let c_valid = p_v.get(p_v.len()-2).unwrap();
                    let c_v = c_valid[0];
                    if c_valid.len() == 2 && (c_v == '.' || c_f == c_v) {
                        p_v.pop();
                    }
                }
            } else {
                let c_v = vec![c];
                p_v.push(c_v);
            }
        } else {
            let c_v = vec![c];
            p_v.push(c_v);
        }
    }

    let mut c_ignore: Option<char> = None;
    let mut pv_index = 0_usize;
    let mut is_fuzzy = false;
    let mut fuzzy_index = 0_usize;
    let mut fuzzy_jud = false;
    for c in s.chars() {
        if c_ignore.is_some() && c_ignore.unwrap() == c && !is_fuzzy {
            continue;
        }

        if pv_index == p_v.len() {
            return false;
        }

        while pv_index < p_v.len() {
            let cs_valid = p_v.get(pv_index).unwrap();
            let c_f = cs_valid.first().unwrap();

            println!("c_f: {}    c: {}", c_f, c);

            if c_f == &'*' {
                break;
            } else if c_f == &'.' {
                if is_fuzzy && !fuzzy_jud {
                    return false;
                }
                if cs_valid.len() == 2 {
                    is_fuzzy = true;
                    fuzzy_index = pv_index + 1;
                }
                if pv_index + 1 == p_v.len() && cs_valid.len() == 2 {
                    return true;
                }
                pv_index = pv_index + 1;
                break;
            } else if c_f == &c {
                if is_fuzzy {
                    fuzzy_jud = true;
                }
                pv_index = pv_index + 1;
                if cs_valid.len() == 2 {
                    c_ignore = Some(c);
                } else {
                    c_ignore = None;
                }
                break;
            } else {
                if is_fuzzy {
                    let c_fuzzy = p_v.get(fuzzy_index).unwrap().first().unwrap();
                    if c_fuzzy == &c {
                        pv_index = fuzzy_index + 1;
                    } else {
                        pv_index = fuzzy_index;
                    }
                    fuzzy_jud = true;
                    break;
                } else {
                    return false;
                }
            }
        }
    }

    for i in pv_index..p_v.len() {
        let cs_valid = p_v.get(i).unwrap();
        if cs_valid.len() < 2 {
            return false;
        }
    }

    true
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