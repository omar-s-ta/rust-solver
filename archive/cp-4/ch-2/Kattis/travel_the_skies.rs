use algo_lib::collections::md_array::MdArray;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

#[derive(Clone, Copy)]
struct Edge {
    to: usize,
    capacity: i32,
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let airports = input.read_size();
    let days = input.read_size();
    let flights = input.read_size();

    let mut day_graph = vec![vec![Vec::<Edge>::new(); airports]; days + 1];
    for _ in 0..flights {
        let u = input.read_size().dec();
        let v = input.read_size().dec();
        let d = input.read_size().dec();
        let c = input.read_i32();
        day_graph[d][u].push(Edge { to: v, capacity: c });
    }

    let mut day_cap = MdArray::new([days + 1, airports], 0_i32);
    for _ in 0..airports * days {
        let a = input.read_size().dec();
        let d = input.read_size().dec();
        let c = input.read_i32();
        day_cap[(d, a)] = c;
    }

    for d in 0..days {
        for (u, graph) in day_graph[d].iter().enumerate() {
            if graph.is_empty() {
                day_cap[(d + 1, u)] += day_cap[(d, u)];
                continue;
            }
            for edge in graph {
                if day_cap[(d, u)] < edge.capacity {
                    out.print_line("suboptimal");
                    return;
                }
                day_cap[(d + 1, edge.to)] += edge.capacity;
                day_cap[(d, u)] -= edge.capacity;
            }
            day_cap[(d + 1, u)] += day_cap[(d, u)];
        }
    }
    out.print_line("optimal");
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
