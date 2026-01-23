//{"name":"Teque","group":"Kattis","url":"https://open.kattis.com/problems/teque","interactive":false,"timeLimit":2000,"tests":[{"input":"9\npush_back 9\npush_front 3\npush_middle 5\nget 0\nget 1\nget 2\npush_middle 1\nget 1\nget 2\n","output":"3\n5\n9\n5\n1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"Teque"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::teque::Teque;

type PreCalc = Teque<usize>;

/*
 * Use two deques and preserve the state that `a.size() == b.size() || a.size() == b.size() + 1`
 */
fn solve(input: &mut Input, out: &mut Output, _test_case: usize, teque: &mut PreCalc) {
    let command = input.read_string().chars().collect::<Vec<_>>();
    let value = input.read_size();

    if command[0] == 'p' {
        match command[command.len() - 1] {
            'k' => teque.push_back(value),
            't' => teque.push_front(value),
            _ => teque.push_middle(value),
        }
    } else {
        out.print_line(teque[value]);
    }
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = Teque::new();

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
