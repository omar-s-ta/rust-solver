use std::collections::HashMap;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

/// 'b' is a median in an odd list.
/// therefore 'b' should be in the middle of 2k+1 numbers
/// k numbers less than b, then b, then k numbers greater than b
fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let b = input.read_size();
    let seq = input.read_size_vec(n);

    // B is always there as per problem description.
    let at = seq.iter().position(|&v| v == b).unwrap();

    let mut map = HashMap::from([(0, 1)]);
    let mut count = 0;
    (at + 1..n).for_each(|i| {
        if seq[i] > b {
            count += 1;
        } else {
            count -= 1;
        }
        *map.entry(count).or_insert(0) += 1;
    });

    // '0' key is always there as per map initialization
    let mut result = *map.get(&0).unwrap();
    let mut count = 0;
    (0..at).rev().for_each(|i| {
        if seq[i] > b {
            count -= 1;
        } else {
            count += 1;
        }
        if let Some(value) = map.get(&count) {
            result += value;
        }
    });
    out.print_line(result);
}

pub static TEST_TYPE: TestType = TestType::Single;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");

    let mut pre_calc = ();

    match TEST_TYPE {
        TestType::Single => solve(&mut input, &mut output, 1, &mut pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 1..=t {
                solve(&mut input, &mut output, i, &mut pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1;
            while input.peek().is_some() {
                solve(&mut input, &mut output, i, &mut pre_calc);
                i += 1;
            }
        }
        _ => {
            unreachable!();
        }
    }
    eprint!("\x1B[0m");
    output.flush();
    input.is_run_done()
}

#[cfg(feature = "local")]
mod tester;

#[cfg(feature = "local")]
fn main() {
    tester::run_tests();
}

#[cfg(not(feature = "local"))]
fn main() {
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
