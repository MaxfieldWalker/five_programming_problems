// 問題5
// 1, 2, ..., 9の数をこの順序で、"+"、"-"、または何もせず
// 結果が100となるあらゆる組合せを出力するプログラムを記述せよ。
// 例えば、1 + 2 + 34 – 5 + 67 – 8 + 9 = 100となる

use std::collections::LinkedList;

#[derive(Copy, Clone)]
enum Operation {
    Add,
    Sub,
}

struct Acc {
    // 今までの計算の履歴
    expression: String,
    // 今までの解
    answer: i32,
    // 一つ前の演算
    recent_operation: Operation,
    // 一つ前の数字
    recent_number: i32,
}

#[derive(Debug)]
pub struct Result {
    // 計算式
    expression: String,
    // 式の解
    answer: i32,
}

impl Acc {
    pub fn new(expression: String,
               answer: i32,
               recent_operation: Operation,
               recent_number: i32)
               -> Acc {
        Acc {
            expression,
            answer,
            recent_operation,
            recent_number,
        }
    }
}

pub fn solve(numbers: &Vec<i32>) -> LinkedList<Result> {
    if numbers.len() == 0 {
        panic!("No numbers given.")
    } else {
        let initial = Acc {
            expression: numbers[0].to_string(),
            answer: numbers[0],
            recent_operation: Operation::Add,
            recent_number: numbers[0],
        };
        inner_solve(&initial, &numbers[1..])
    }
}

fn inner_solve(acc: &Acc, rest_numbers: &[i32]) -> LinkedList<Result> {
    if rest_numbers.len() == 0 {
        let mut list = LinkedList::new();

        // これまでの結果をそのまま最終結果とする
        list.push_back(Result {
                           expression: acc.expression.clone(),
                           answer: acc.answer,
                       });
        list
    } else {
        let head = rest_numbers[0];
        let rest = &rest_numbers[1..];

        // 3通りそれぞれ計算する
        let mut a_result =
            inner_solve(&Acc::new(format!("{} + {}", acc.expression, head.to_string()),
                                  acc.answer + head,
                                  Operation::Add,
                                  head),
                        rest);

        let mut b_result =
            inner_solve(&Acc::new(format!("{} - {}", acc.expression, head.to_string()),
                                  acc.answer - head,
                                  Operation::Sub,
                                  head),
                        rest);

        let join = 10 * acc.recent_number + head;
        let mut c_result =
            inner_solve(&Acc::new(format!("{}{}", acc.expression, head.to_string()),
                                  match acc.recent_operation {
                                      Operation::Add => acc.answer - acc.recent_number + join,
                                      Operation::Sub => acc.answer + acc.recent_number - join,
                                  },
                                  acc.recent_operation,
                                  (10 * acc.recent_number + head)),
                        rest);

        // 3通りの結果をまとめる
        let mut results = LinkedList::new();
        results.append(&mut a_result);
        results.append(&mut b_result);
        results.append(&mut c_result);

        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let numbers = (1..10).collect::<Vec<i32>>();

        let expected_answer = 100;
        let results: Vec<String> = solve(&numbers)
            .iter()
            .filter(|x| x.answer == expected_answer)
            .map(|x| -> String { format!("{} = {}", x.expression, expected_answer) })
            .collect();

        let expected = vec!["1 + 2 + 3 - 4 + 5 + 6 + 78 + 9 = 100",
                            "1 + 2 + 34 - 5 + 67 - 8 + 9 = 100",
                            "1 + 23 - 4 + 5 + 6 + 78 - 9 = 100",
                            "1 + 23 - 4 + 56 + 7 + 8 + 9 = 100",
                            "12 + 3 + 4 + 5 - 6 - 7 + 89 = 100",
                            "12 + 3 - 4 + 5 + 67 + 8 + 9 = 100",
                            "12 - 3 - 4 + 5 - 6 + 7 + 89 = 100",
                            "123 + 4 - 5 + 67 - 89 = 100",
                            "123 + 45 - 67 + 8 - 9 = 100",
                            "123 - 4 - 5 - 6 - 7 + 8 - 9 = 100",
                            "123 - 45 - 67 + 89 = 100"];

        assert_eq!(expected, results);
    }
}
