//{"name":"Delimiter Soup","group":"Kattis","url":"https://open.kattis.com/problems/delimitersoup","interactive":false,"timeLimit":1000,"tests":[{"input":"8\n([] [] ]\n","output":"] 7\n"},{"input":"13\n(([] [[]] ())\n","output":"ok so far\n"},{"input":"21\n[ { { () () () () } ]\n","output":"] 20\n"},{"input":"27\n[ { [[()]] (({})) } ] () {}\n","output":"ok so far\n"},{"input":"19\n[[]] () ) [] {{}} {\n","output":") 8\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DelimiterSoup"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn is_match(a: char, b: char) -> bool {
    if a == '(' {
        b == ')'
    } else if a == '{' {
        b == '}'
    } else if a == '[' {
        b == ']'
    } else {
        false
    }
}

/// Just a normal expression parsing
/// `push` on open tokens
/// `pop`  on matching tokens, fail if tokens are not matching
fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let _ = input.read_size();
    let program = input.read_line();

    let is_open = |ch: char| ch == '(' || ch == '{' || ch == '[';
    let mut st = Vec::new();

    for (i, ch) in program.chars().enumerate() {
        if ch.is_whitespace() {
            continue;
        }
        if is_open(ch) {
            st.push(ch);
        } else if st.last().is_none_or(|&token| !is_match(token, ch)) {
            out.print_line(format!("{} {}", ch, i));
            return;
        } else {
            st.pop();
        }
    }

    out.print_line("ok so far");
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
