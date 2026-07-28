#![allow(unexpected_cfgs)]
use algo_lib::collections::disjoint_set::CompressedDisjointSet;
use algo_lib::collections::disjoint_set::DisjointSet;
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let q = input.read_size();

    let mut uf = CompressedDisjointSet::new(n);
    for _ in 0..q {
        match input.read_char() {
            b'=' => {
                let a = input.read_size();
                let b = input.read_size();
                uf.union(a, b);
            }
            b'?' => {
                let a = input.read_size();
                let b = input.read_size();
                out.print_line(uf.find(a) == uf.find(b));
            }
            _ => unreachable!(),
        }
    }
}

pub static TEST_TYPE: TestType = TestType::Single;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");

    let mut pre_calc = ();

    output.set_bool_output(BoolOutput::YesNoLow);

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
mod tester {
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unused_imports)]

use crate::{TASK_TYPE, TEST_TYPE, TestType, run};
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::string::str::StrReader;
use std::io::{Read, Write};
use std::thread::yield_now;
use tester::Tester;
use tester::classic::EPS;
use tester::classic::default_checker;
use tester::interactive::SolutionRunner;
use tester::interactive::std_interactor;
use tester::test_set::GeneratedTestSet;

const PRINT_LIMIT: usize = 1000;

fn interact(
    mut input: Input,
    expected: Option<Input>,
    mut runner: SolutionRunner,
) -> Result<Option<i64>, String> {
    let (mut sol, mut out) = runner.run();
    Ok(None)
}

fn run_twice(
    mut input: Input,
    expected: Option<Input>,
    mut runner: SolutionRunner,
) -> Result<Option<i64>, String> {
    let (mut sol, mut out) = runner.run();
    input.read_line();
    out.print_line("first");
    let t = match TEST_TYPE {
        TestType::RunTwiceSingle => None,
        TestType::RunTwiceMultiNumber => {
            let t = input.read_size();
            out.print_line(t);
            Some(t)
        }
        _ => unreachable!(),
    };
    let mut input_vec = Vec::new();
    input.read_to_end(&mut input_vec).unwrap();
    out.write_all(&input_vec).unwrap();
    out.flush();
    while !runner.is_finished() {
        yield_now();
    }
    let mut first_output = Vec::new();
    sol.read_to_end(&mut first_output).unwrap();

    let (mut sol, mut out) = runner.run();
    out.print_line("second");
    if let Some(t) = t {
        out.print_line(t);
    }
    out.write_all(&first_output).unwrap();
    out.flush();
    let mut ans = Vec::new();
    sol.read_to_end(&mut ans).unwrap();
    default_checker(Input::slice(&input_vec), expected, Input::slice(&ans))
    // check(Input::slice(&input_vec), expected, Input::slice(&ans))
}

fn check(
    mut input: Input,
    expected: Option<Input>,
    mut output: Input,
) -> Result<Option<i64>, String> {
    Ok(None)
}

struct StressTest;

impl GeneratedTestSet for StressTest {
    type TestId = usize;

    fn tests(&self) -> impl Iterator<Item = Self::TestId> {
        1..
    }

    fn input(&self, test: &Self::TestId, out: &mut Output) {
        // let mut r = Random::new();
    }

    fn output(&self, test: &Self::TestId, input: &mut Input, out: &mut Output) -> bool {
        false
    }
}

struct MaxTest;

impl GeneratedTestSet for MaxTest {
    type TestId = usize;

    fn tests(&self) -> impl Iterator<Item = Self::TestId> {
        1..=1
    }

    fn input(&self, test: &Self::TestId, out: &mut Output) {
        // let mut r = Random::new_with_seed(239);
    }

    fn output(&self, test: &Self::TestId, input: &mut Input, out: &mut Output) -> bool {
        false
    }
}

pub(crate) fn run_tests() -> bool {
    let path = "./union_find";
    let tl = 1000;
    let tester = match TASK_TYPE {
        crate::TaskType::Interactive => {
            Tester::new_interactive(tl, PRINT_LIMIT, path.to_string(), run, std_interactor)
            // Tester::new_interactive(tl, PRINT_LIMIT, path.to_string(), run, interact)
            // Tester::new_interactive(tl, PRINT_LIMIT, path.to_string(), run, run_twice)
        }
        crate::TaskType::Classic => {
            Tester::new_classic(tl, PRINT_LIMIT, path.to_string(), run, default_checker)
            // Tester::new_classic(tl, PRINT_LIMIT, path.to_string(), run, check)
        }
    };
    tester.test_samples()
    // tester.test_generated("Max test", true, MaxTest);
    // tester.test_generated("Stress test", false, StressTest);
}
}
#[test]
fn union_find() {
    assert!(tester::run_tests());
}
