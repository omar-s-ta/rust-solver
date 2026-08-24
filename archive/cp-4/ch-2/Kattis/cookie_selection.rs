use std::cmp::Reverse;
use std::collections::BinaryHeap;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

const N: usize = 600_001;

type PreCalc = (BinaryHeap<usize>, BinaryHeap<Reverse<usize>>);

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, (lo, hi): &mut PreCalc) {
    if input.peek().is_some_and(|b| b == b'#') {
        let _ = input.read_char();
        let m = hi.pop().unwrap().0;
        if lo.len() > hi.len() {
            hi.push(Reverse(lo.pop().unwrap()));
        }
        out.print_line(m);
    } else {
        let n = input.read_size();
        if hi.peek().is_some_and(|&Reverse(top)| n > top) {
            hi.push(Reverse(n));
            if hi.len() > lo.len() + 1 {
                lo.push(hi.pop().unwrap().0);
            }
        } else {
            lo.push(n);
            if lo.len() > hi.len() {
                hi.push(Reverse(lo.pop().unwrap()));
            }
        }
    }
}

pub static TEST_TYPE: TestType = TestType::MultiEof;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");

    let mut pre_calc = (
        BinaryHeap::with_capacity(N / 2 + 1),
        BinaryHeap::with_capacity(N / 2 + 1),
    );

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
            while !input.is_empty() {
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
