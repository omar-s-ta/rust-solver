//{"name":"Numbers On a Tree","group":"Kattis","url":"https://open.kattis.com/problems/numbertree","interactive":false,"timeLimit":1000,"tests":[{"input":"3 LR\n","output":"11\n"},{"input":"3 RRL\n","output":"2\n"},{"input":"2 \n","output":"7\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"NumbersOnATree"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

///
/// Simulate a `BinaryHeap` traversal.
/// Avoid creating lists since it can be of size 2^30.
/// Use `xor` to get the number in the reversed order.
///
fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let line = input.read_line();
    let line = line.split_whitespace().collect::<Vec<_>>();

    let mut it = line.iter();
    let n = it.next().and_then(|s| s.parse::<u32>().ok()).unwrap();
    let set = (1_u32 << (n + 1)) - 1;

    match it.next() {
        Some(command) => {
            let at = command.chars().fold(0, |i, ch| match ch {
                'L' => (i << 1) + 1,
                _ => (i << 1) + 2,
            });
            out.print_line(at ^ set);
        }
        None => out.print_line(set),
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
