//{"name":"Integer Lists","group":"Kattis","url":"https://open.kattis.com/problems/integerlists","interactive":false,"timeLimit":1000,"tests":[{"input":"4\nRDD\n4\n[1,2,3,4]\nDD\n1\n[42]\nRRD\n6\n[1,1,2,3,5,8]\nD\n0\n[]\n","output":"[2,1]\nerror\n[1,2,3,5,8]\nerror\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"IntegerLists"}}}

use std::collections::VecDeque;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

///
/// Use a double ended queue to simulate reversing and deletion.
/// Only reverse once in the end if needed.
///
fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let program = input.read_string();
    let _ = input.read_size();
    let list = input.read_string();

    let list = list[1..list.len() - 1]
        .split(',')
        .map_while(|s| s.parse::<i32>().ok())
        .collect::<VecDeque<_>>();

    let result = program
        .chars()
        .try_fold((list, false), |(mut list, reverse), ch| match ch {
            'D' if list.is_empty() => None,
            'D' if reverse => {
                list.pop_back();
                Some((list, reverse))
            }
            'D' => {
                list.pop_front();
                Some((list, reverse))
            }
            _ => Some((list, !reverse)),
        });

    match result {
        None => out.print_line("error"),
        Some((mut list, reverse)) => {
            if reverse {
                list.make_contiguous().reverse();
            }
            out.print_line(list);
        }
    }
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
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
