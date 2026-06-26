//{"name":"Pairing Socks","group":"Kattis","url":"https://open.kattis.com/problems/pairingsocks","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n1 2 2 1\n","output":"4\n"},{"input":"1\n3 7\n","output":"impossible\n"},{"input":"3\n5 5 5 5 5 5\n","output":"6\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"PairingSocks"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let mut stk = input.read_vec::<i32>(2 * n);
    stk.reverse();

    let mut aux_stk = Vec::new();
    let mut result = 0;

    while let Some(ai) = stk.pop() {
        if let Some(&aai) = aux_stk.last() {
            if ai == aai {
                aux_stk.pop();
            } else {
                aux_stk.push(ai);
            }
        } else {
            aux_stk.push(ai);
        }
        result += 1;
    }

    if aux_stk.is_empty() {
        out.print_line(result);
    } else {
        out.print_line("impossible");
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
        _ => unreachable!(),
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
