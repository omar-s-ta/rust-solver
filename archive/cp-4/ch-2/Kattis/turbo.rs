use algo_lib::collections::fenwick_tree::FenwickTree;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let ns = input.read_size_vec(n).dec();

    let mut pos = vec![0usize; n];
    for (i, &ni) in ns.iter().enumerate() {
        pos[ni] = i;
    }

    let mut ft = FenwickTree::<i32>::ones(n);
    let mut lo = 0;
    let mut hi = n - 1;

    for p in 1..=n {
        if p.is_multiple_of(2) {
            out.print_line(ft.get_to(n) - ft.get_to(pos[hi] + 1));
            ft.add(pos[hi], -1);
            hi -= 1;
        } else {
            out.print_line(ft.get_to(pos[lo]));
            ft.add(pos[lo], -1);
            lo += 1;
        }
    }
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
