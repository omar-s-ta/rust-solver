//{"name":"Game of Throwns","group":"Kattis","url":"https://open.kattis.com/problems/throwns","interactive":false,"timeLimit":1000,"tests":[{"input":"5 4\n8 -2 3 undo 2\n","output":"3\n"},{"input":"5 10\n7 -3 undo 1 4 3 -9 5 undo 2 undo 1 6\n","output":"2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GameOfThrowns"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_i32();
    let m = input.read_i32();

    let mut actions = Vec::new();
    for _ in 0..m {
        let action = input.read_str().to_string();
        if action.starts_with('u') {
            let undo = input.read_size();
            actions.truncate(actions.len() - undo);
        } else {
            actions.push(action.parse::<i32>().unwrap());
        }
    }

    out.print_line(actions.iter().sum::<i32>().rem_euclid(n));
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
