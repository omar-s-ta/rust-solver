//{"name":"CD","group":"Kattis","url":"https://open.kattis.com/problems/cd","interactive":false,"timeLimit":2000,"tests":[{"input":"3 3\n1\n2\n3\n1\n2\n4\n0 0\n","output":"2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CD"}}}

use std::collections::HashSet;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = bool;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, input_ended: &mut PreCalc) {
    let n = input.read_unsigned();
    let m = input.read_unsigned();

    if n == 0 && m == 0 {
        *input_ended = true;
        return;
    }

    let jack = (0..n).fold(HashSet::new(), |mut set, _| {
        set.insert(input.read_unsigned());
        set
    });
    let jill = (0..m).fold(HashSet::new(), |mut set, _| {
        set.insert(input.read_unsigned());
        set
    });

    let result = jack.iter().fold(0, |mut count, elem| {
        if jill.contains(elem) {
            count += 1;
        }
        count
    });
    out.print_line(result);
}

pub static TEST_TYPE: TestType = TestType::MultiEof;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut input_ended = false;

    match TEST_TYPE {
        TestType::Single => solve(&mut input, &mut output, 1, &mut input_ended),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 1..=t {
                solve(&mut input, &mut output, i, &mut input_ended);
            }
        }
        TestType::MultiEof => {
            let mut i = 1;
            while input.peek().is_some() {
                solve(&mut input, &mut output, i, &mut input_ended);
                if input_ended {
                    break;
                }
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
