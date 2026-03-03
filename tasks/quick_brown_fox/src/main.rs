//{"name":"Quick Brown Fox","group":"Kattis","url":"https://open.kattis.com/problems/quickbrownfox","interactive":false,"timeLimit":1000,"tests":[{"input":"3\nThe quick brown fox jumps over the lazy dog.\nZYXW, vu TSR Ponm lkj ihgfd CBA.\n.,?!'\" 92384 abcde FGHIJ\n","output":"pangram\nmissing eq\nmissing klmnopqrstuvwxyz\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"QuickBrownFox"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

/// Use a hash to mark seen letters
fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let line = input.read_line();
    let visited = line
        .as_bytes()
        .iter()
        .fold(vec![false; 26], |mut taken, b| {
            if b.is_ascii_alphabetic() {
                let at = usize::from(b.to_ascii_lowercase() - b'a');
                taken[at] = true;
            }
            taken
        });

    let missing = (0..26).fold(String::new(), |mut string, i| {
        if !visited[i] {
            string.push(char::from(i as u8 + b'a'));
        }
        string
    });

    if missing.is_empty() {
        out.print_line("pangram");
    } else {
        out.print_line(format!("missing {}", missing));
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
