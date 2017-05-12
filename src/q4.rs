// 問題4
// 正の整数のリストを与えられたとき、数を並び替えて
// 可能な最大数を返す関数を記述せよ。例えば、[50, 2, 1, 9]が
// 与えられた時、95021が答えとなる

use std::cmp::Ordering;

/// 数値の桁数を求める
pub fn number_of_digits(n: u32) -> u32 {
    let s: String = n.to_string();
    s.len() as u32
}

/// 最上位の桁を求める
pub fn most_significant_digit(n: u32) -> u32 {
    let nn = n / 10;
    if nn == 0 {
        n
    } else {
        most_significant_digit(nn)
    }
}

// 最大になるように数値を組み合わせる
pub fn max(numbers: &mut Vec<u32>) -> u32 {
    numbers.sort_by(comparer);
    numbers
        .iter()
        .rev()
        .map(|x: &u32| format!("{}", x))
        .fold("".to_string(), |a, b| a + &b)
        .parse()
        .unwrap()
}

fn comparer(a: &u32, b: &u32) -> Ordering {
    // 最上位の桁の大きさを比べる
    let a_msd = most_significant_digit(*a);
    let b_msd = most_significant_digit(*b);
    let ordering = a_msd.cmp(&b_msd);

    match ordering {
        Ordering::Greater | Ordering::Less => ordering,
        Ordering::Equal => {
            // 勝負がつかなかった場合
            // aまたはbが一桁(つまり10より小さい)ならば
            let a_is_less_10 = a < &10;
            let b_is_less_10 = b < &10;
            match (a_is_less_10, b_is_less_10) {
                (true, true) => Ordering::Equal, // 引き分け
                (true, false) => Ordering::Greater, // aの勝ち
                (false, true) => Ordering::Less, // bの勝ち
                (false, false) => {
                    let a_without_msd = a % 10u32.pow(number_of_digits(*a) - 1);
                    let b_without_msd = b % 10u32.pow(number_of_digits(*b) - 1);
                    println!("{}, {}", a_without_msd, b_without_msd);
                    comparer(&a_without_msd, &b_without_msd)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max() {
        assert_eq!(95021, max(&mut vec![50, 2, 1, 9]));
        assert_eq!(929119, max(&mut vec![19, 91, 92]));
        assert_eq!(999999, max(&mut vec![999, 99, 9]));
    }

    #[test]
    fn test_number_of_digits() {
        assert_eq!(1, number_of_digits(9));
        assert_eq!(3, number_of_digits(321));
    }

    #[test]
    fn test_most_significant_digit() {
        assert_eq!(9, most_significant_digit(9));
        assert_eq!(3, most_significant_digit(321));
    }
}
