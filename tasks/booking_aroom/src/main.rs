//{"name":"Booking a Room","group":"Kattis","url":"https://open.kattis.com/problems/bookingaroom","interactive":false,"timeLimit":1000,"tests":[{"input":"100 5\n42\n3\n2\n99\n1\n","output":"23\n"},{"input":"3 3\n2\n3\n1\n","output":"too late\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BookingARoom"}}}

use std::collections::HashSet;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

/// Simulation. Direct Access Table.
fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let r = input.read_size();

    let visited = (0..r).map(|_| input.read_size()).collect::<HashSet<_>>();

    if visited.len() == n {
        out.print_line("too late");
    } else {
        for i in 1..=n {
            if !visited.contains(&i) {
                out.print_line(i);
                break;
            }
        }
    }
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
