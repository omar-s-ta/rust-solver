use algo_lib::collections::fenwick_tree::FenwickTree;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let m = input.read_size();
    let r = input.read_size();

    let mut ft = FenwickTree::<i32>::new(m + r);
    let mut at = vec![0usize; m];
    (0..m).for_each(|i| {
        ft.add(i + r, 1);
        at[i] = i + r;
    });

    let mut next_position = r - 1;
    for i in 0..r {
        let j = input.read_size() - 1;
        if i > 0 {
            out.print(' ');
        }
        out.print(ft.get_to(at[j]));
        ft.add(at[j], -1);
        at[j] = next_position;
        next_position = next_position.saturating_sub(1);
        ft.add(at[j], 1);
    }
    out.print_line(());
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
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
