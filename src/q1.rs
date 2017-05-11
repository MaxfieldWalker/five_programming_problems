// 問題1
// forループ、whileループ、および再帰を使用して、リスト内の数字の合計を計算する3つの関数を記述せよ。

fn for_sum(numbers: &Vec<i32>) -> i32 {
    let mut sum: i32 = 0;
    for n in numbers {
        sum += *n;
    }
    sum
}

fn while_sum(numbers: &Vec<i32>) -> i32 {
    let mut sum: i32 = 0;
    let mut i = 0;

    while i < numbers.len() {
        let n = numbers[i];
        sum += n;
        i += 1;
    }

    sum
}

fn recursive_sum(numbers: &[i32]) -> i32 {
    match numbers.len() {
        0 => 0,
        _ => numbers[0] + recursive_sum(&numbers[1..]),
    }
}

// #[cfg(test)]を付けると，テストの時だけビルドされるようになる
#[cfg(test)]
mod tests {
    use super::for_sum;
    use super::while_sum;
    use super::recursive_sum;

    #[test]
    fn for_sum_test() {
        let numbers = vec![1, 3, 5, 7, 9];
        let sum = for_sum(&numbers);
        assert_eq!(25, sum);
    }

    #[test]
    fn while_sum_test() {
        let numbers = vec![1, 3, 5, 7, 9];
        let sum = while_sum(&numbers);
        assert_eq!(25, sum);
    }

    #[test]
    fn recursive_sum_test() {
        let numbers = vec![1, 3, 5, 7, 9];
        let sum = recursive_sum(&numbers);
        assert_eq!(25, sum);
    }
}
