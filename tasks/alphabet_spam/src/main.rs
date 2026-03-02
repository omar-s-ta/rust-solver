//{"name":"Alphabet Spam","group":"Kattis","url":"https://open.kattis.com/problems/alphabetspam","interactive":false,"timeLimit":1000,"tests":[{"input":"Welcome_NWERC_participants!\n","output":"0.0740740740740741\n0.666666666666667\n0.222222222222222\n0.0370370370370370\n"},{"input":"\\/\\/in_US$100000_in_our_Ca$h_Lo||ery!!!\n","output":"0.128205128205128\n0.333333333333333\n0.102564102564103\n0.435897435897436\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AlphabetSpam"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

/// Simulation. Just count and divide by length.
fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let string = input.read_string();
    let n = string.len() as f64;

    let (w, l, u, s) = string
        .chars()
        .fold((0.0, 0.0, 0.0, 0.0), |(w, l, u, s), ch| match ch {
            c if c.is_uppercase() => (w, l, u + 1_f64, s),
            c if c.is_lowercase() => (w, l + 1_f64, u, s),
            '_' => (w + 1_f64, l, u, s),
            _ => (w, l, u, s + 1_f64),
        });

    out.print_line(format!("{:.15}", w / n));
    out.print_line(format!("{:.15}", l / n));
    out.print_line(format!("{:.15}", u / n));
    out.print_line(format!("{:.15}", s / n));
}

pub static TEST_TYPE: TestType = TestType::Single;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
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
    }
    output.flush();
    match TASK_TYPE {
        TaskType::Classic => input.is_empty(),
        TaskType::Interactive => true,
    }
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
