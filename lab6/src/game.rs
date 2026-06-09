use std::thread::current;

use itertools::{Itertools, Permutations};

pub fn mk_ops(symbols: &[char], n: usize) -> Vec<String> {
    if n == 0 {
        return vec![String::new()];
    }

    let mut result = vec![];

    for &symbol in symbols {
        for perm in mk_ops(symbols, n - 1) {
            result.push(format!("{}{}", symbol, perm));
        }
    }

    result
}

pub fn prepare(s: &str) -> Vec<String> {
    let mut result = vec![];
    let ops = mk_ops(&['+', '-', '*', '/'], 4);

    for digit in s.chars().permutations(s.len()) {
        for op_seq in &ops {
            let mut s = String::new();
            let mut it_op = op_seq.chars();
            for d in digit.iter() {
                s.push(*d);
                if let Some(op) = it_op.next() {
                    s.push(op);
                }
            }
            result.push(s);
        }
    }
    result
}

#[test]
fn test_mk_ops() {
    let symbols = vec!['+', '-', '*', '/'];
    let n = 4;
    let result = mk_ops(&symbols, n);
    assert_eq!(result.len(), symbols.len().pow(n as u32));

    let res = prepare("23423");
    println!("{} {:?}", res.len(), res.iter().take(n).collect::<Vec<_>>());
}

pub fn verify(v: &[String]) -> Vec<String> {
    let mut vec = vec![];
    'outer: for s in v {
        let mut digits = s.chars().step_by(2);
        let operations = s.chars().skip(1).step_by(2);

        let mut current_term = digits.nth(0).unwrap().to_digit(10).unwrap() as i64;
        let mut result = 0;

        for (op, digit) in operations.zip(digits) {
            let num = digit.to_digit(10).unwrap() as i64;

            match op {
                '*' => current_term *= num,
                '/' if num == 0 || current_term % num != 0 => continue 'outer,
                '/' if (current_term % num == 0) => current_term /= num,
                '/' => continue 'outer,
                '-' => {
                    result += current_term;
                    current_term = -num;
                }
                '+' => {
                    result += current_term;
                    current_term = num;
                }
                _ => {}
            }
        }

        result += current_term;

        if result == 10 {
            vec.push(s.clone());
        }
    }

    vec
}
