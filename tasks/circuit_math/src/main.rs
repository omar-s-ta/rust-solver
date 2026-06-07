//{"name":"Circuit Math","group":"Kattis","url":"https://open.kattis.com/problems/circuitmath","interactive":false,"timeLimit":1000,"tests":[{"input":"4\nT F T F\nA B * C D + - +\n","output":"F\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CircuitMath"}}}

use std::ops::Not;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_i32();
    let mut gates = 0;

    for i in 0..n {
        let ch = input.read_char();
        if ch == b'T' {
            gates |= 1 << i;
        }
    }

    let mut expr = Vec::new();

    while !input.is_empty() {
        let ch = input.read_char();
        if ch.is_ascii_uppercase() {
            let i = (ch - b'A') as u32;
            expr.push(gates & (1 << i) != 0);
        } else if ch == b'-' {
            if let Some(b) = expr.last_mut() {
                *b = b.not()
            }
        } else {
            expr.pop().and_then(|a| {
                expr.last_mut()
                    .map(|b| if ch == b'*' { *b &= a } else { *b |= a })
            });
        }
    }

    expr.last().iter().for_each(|&gate| {
        if *gate {
            out.print_line('T');
        } else {
            out.print_line('F');
        }
    });
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
