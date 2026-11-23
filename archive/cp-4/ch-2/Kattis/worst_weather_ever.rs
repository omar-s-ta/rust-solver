use algo_lib::collections::segment_tree::SegmentTree;
use algo_lib::collections::slice_ext::sorted_ops::SortedOps;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::math::type_classes::Monoid;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

struct Max;

impl Monoid for Max {
    type T = i32;

    fn empty() -> Self::T {
        i32::MIN
    }

    fn combine(a: Self::T, b: Self::T) -> Self::T {
        a.max(b)
    }
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    for case in 0.. {
        let n = input.read_size();
        let mut years = Vec::with_capacity(n);
        let mut rain = Vec::with_capacity(n);
        for _ in 0..n {
            years.push(input.read_i32());
            rain.push(input.read_i32());
        }
        let m = input.read_size();
        if n == 0 && m == 0 {
            break;
        }
        if case > 0 {
            out.print_line(());
        }
        let mut tree = SegmentTree::<Max>::from(rain.as_slice());
        for _ in 0..m {
            let y = input.read_i32();
            let x = input.read_i32();
            let answer = match (years.binary_find(&y), years.binary_find(&x)) {
                (Some(yi), Some(xi)) => {
                    if rain[yi] < rain[xi] || tree.query(yi + 1..xi) >= rain[xi] {
                        "false"
                    } else if xi - yi < (x - y) as usize {
                        "maybe"
                    } else {
                        "true"
                    }
                }
                (None, Some(xi)) if tree.query(years.upper_bound(&y)..xi) >= rain[xi] => "false",
                (Some(yi), None) if tree.query(yi + 1..years.lower_bound(&x)) >= rain[yi] => {
                    "false"
                }
                _ => "maybe",
            };
            out.print_line(answer);
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
