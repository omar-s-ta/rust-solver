use algo_lib::collections::order_stat_set::OrderStatSet;
use algo_lib::collections::slice_ext::sorted_ops::SortedOps;
use algo_lib::collections::vec_ext::dedup::Dedup;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = Vec<Option<usize>>;

fn solve(input: &mut Input, _out: &mut Output, _test_case: usize, data: &mut PreCalc) {
    if input.peek().is_some_and(|b| b == b'#') {
        input.read_char();
        data.push(None);
    } else {
        data.push(Some(input.read_size()));
    }
}

pub static TEST_TYPE: TestType = TestType::MultiEof;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");

    let mut pre_calc = Vec::with_capacity(600_001);

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
            let data = pre_calc
                .iter()
                .flatten()
                .copied()
                .collect::<Vec<_>>()
                .sorted_deduped();

            let mut set = OrderStatSet::new(data.len());
            for op in pre_calc {
                match op {
                    Some(d) => set.insert(data.lower_bound(&d)),
                    None => {
                        let m = set.len() / 2 + 1;
                        let k = set.kth(m);
                        set.remove(k);
                        output.print_line(data[k]);
                    }
                }
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
