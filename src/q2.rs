// 問題2
// 交互に要素を取ることで、2つのリストを結合する関数を記述せよ。例えば [a, b, c]と[1, 2, 3]という2つのリストを与えると、関数は [a, 1, b, 2, c, 3]を返す

pub fn concat(a: &[String], b: &[String]) -> Vec<String> {
    match a.len() {
        0 => vec![],
        _ => {
            let mut v: Vec<String> = vec![a[0].clone(), b[0].clone()];
            let mut rest = concat(&a[1..], &b[1..]);
            v.append(&mut rest);
            v
        }
    }
}

// #[cfg(test)]を付けると，テストの時だけビルドされるようになる
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let a = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let b = vec!["1".to_string(), "2".to_string(), "3".to_string()];

        assert_eq!(vec!["a".to_string(),
                        "1".to_string(),
                        "b".to_string(),
                        "2".to_string(),
                        "c".to_string(),
                        "3".to_string()],
                   concat(&a, &b));
    }
}
