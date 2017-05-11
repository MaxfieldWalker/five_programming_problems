// 問題3
// 最初の100個のフィボナッチ数のリストを計算する関数を記述せよ。定義では、フィボナッチ数列の最初の2つの数字は0と1で、次の数は前の2つの合計となる。例えば最初の10個のフィボナッチ数列は、0, 1, 1, 2, 3, 5, 8, 13, 21, 34となる。

// ※ u128がunstableなのでu64で計算する

fn fibo(until: u32) -> Vec<u64> {
    match until {
        0 => vec![],
        1 => vec![1],
        _ => {
            let first = 1;
            let second = 1;

            let mut v: Vec<u64> = vec![first, second];
            let mut rest: Vec<u64> = rest_fibo(first, second, until - 2);
            v.append(&mut rest);
            v
        }
    }
}

fn rest_fibo(a: u64, b: u64, count: u32) -> Vec<u64> {
    match count {
        0 => vec![],
        _ => {
            let c = a + b;
            let mut v: Vec<u64> = vec![c];
            let mut rest = rest_fibo(b, c, count - 1);
            v.append(&mut rest);
            v
        }
    }
}

#[cfg(test)]
mod tests {
    use super::fibo;

    #[test]
    fn test() {
        let expected: Vec<u64> = vec![1,
                                      1,
                                      2,
                                      3,
                                      5,
                                      8,
                                      13,
                                      21,
                                      34,
                                      55,
                                      89,
                                      144,
                                      233,
                                      377,
                                      610,
                                      987,
                                      1597,
                                      2584,
                                      4181,
                                      6765,
                                      10946,
                                      17711,
                                      28657,
                                      46368,
                                      75025,
                                      121393,
                                      196418,
                                      317811,
                                      514229,
                                      832040,
                                      1346269,
                                      2178309,
                                      3524578,
                                      5702887,
                                      9227465,
                                      14930352,
                                      24157817,
                                      39088169,
                                      63245986,
                                      102334155,
                                      165580141,
                                      267914296,
                                      433494437,
                                      701408733,
                                      1134903170,
                                      1836311903,
                                      2971215073,
                                      4807526976,
                                      7778742049,
                                      12586269025];

        assert_eq!(expected, fibo(50));
    }
}