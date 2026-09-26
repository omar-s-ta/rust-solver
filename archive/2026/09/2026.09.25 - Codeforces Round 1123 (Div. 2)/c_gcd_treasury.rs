use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

const N: usize = 300000 + 2;

type PrimeFactors = Vec<Vec<usize>>;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, factors: &mut PrimeFactors) {
    let n = input.read_size();
    let x = input.read_size();
    let a = input.read_size_vec(n);

    let x_factors = &factors[x];
    let mut count = vec![0; x_factors.len()];

    for ai in a {
        for (i, &f) in x_factors.iter().enumerate() {
            if ai.is_multiple_of(f) {
                count[i] += ai;
            }
        }
    }
    out.print_line(count.into_iter().max().unwrap_or_default());
}

#[allow(unused_variables)]
fn solve2(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PrimeFactors) {}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");

    let mut visited = [false; N];
    let mut prime_factors = vec![Vec::new(); N];

    for i in 2..N {
        if !visited[i] {
            for j in (i..N).step_by(i) {
                visited[j] = true;
                prime_factors[j].push(i);
            }
        }
    }

    match TEST_TYPE {
        TestType::Single => solve(&mut input, &mut output, 1, &mut prime_factors),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 1..=t {
                solve(&mut input, &mut output, i, &mut prime_factors);
            }
        }
        TestType::MultiEof => {
            let mut i = 1;
            while !input.is_empty() {
                solve(&mut input, &mut output, i, &mut prime_factors);
                i += 1;
            }
        }
        TestType::RunTwiceSingle => {
            let mode = input.read_str();
            match mode.as_slice() {
                b"first" => solve(&mut input, &mut output, 1, &mut prime_factors),
                b"second" => solve2(&mut input, &mut output, 1, &mut prime_factors),
                _ => unreachable!(),
            }
        }
        TestType::RunTwiceMultiNumber => {
            let mode = input.read_str();
            let t = input.read();
            for i in 1..=t {
                match mode.as_slice() {
                    b"first" => solve(&mut input, &mut output, i, &mut prime_factors),
                    b"second" => solve2(&mut input, &mut output, i, &mut prime_factors),
                    _ => unreachable!(),
                }
            }
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
