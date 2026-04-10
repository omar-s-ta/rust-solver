//{"name":"Conformity","group":"Kattis","url":"https://open.kattis.com/problems/conformity","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n100 101 102 103 488\n100 200 300 101 102\n103 102 101 488 100\n","output":"2\n"},{"input":"3\n200 202 204 206 208\n123 234 345 456 321\n100 200 300 400 444\n","output":"3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"Conformity"}}}

use std::collections::HashMap;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();

    let mut combinations = HashMap::new();
    for _ in 0..n {
        let mut courses = input.read_ushort_vec(5);
        courses.sort();

        *combinations.entry(courses).or_insert(0_u32) += 1;
    }

    let popularity = *combinations.values().max().unwrap();
    let students = combinations
        .values()
        .filter(|&p| *p == popularity)
        .sum::<u32>();

    out.print_line(students);
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
