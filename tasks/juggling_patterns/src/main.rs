//{"name":"Juggling Patterns","group":"Kattis","url":"https://open.kattis.com/problems/jugglingpatterns","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n33333\n345\n542\n543\n55550\n","output":"3: valid with 3 balls\n33333: valid with 3 balls\n345: valid with 4 balls\n542: invalid # of balls\n543: invalid pattern\n55550: valid with 4 balls \n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"JugglingPatterns"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

/// -------------------------------------------------------------
/// pattern of length n, contains throws t0..t(n-1) (0..9) each
/// Check all values (i + ti) % n are distinct. i <- beat number
///
/// Do not care about the hand or swaping hands in a throw
/// the 'hand' is determined by beat 'i' parity (i % 2).
/// If two balls land on the same beat, they are automatically
/// in the same hand for that beat.
/// -------------------------------------------------------------
fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let pattern = input.read_string();
    let bytes = pattern.as_bytes();
    let len = bytes.len();
    let sum = bytes.iter().map(|b| usize::from(b - b'0')).sum::<usize>();

    if sum % len != 0 {
        out.print_line(format!("{}: invalid # of balls", pattern));
        return;
    }

    let mut seen = vec![false; len];
    let invalid = bytes.iter().enumerate().any(|(i, &b)| {
        let throw = usize::from(b - b'0');
        let hand = (i + throw) % len;
        let visited = seen[hand];
        seen[hand] = true;
        visited
    });

    if invalid {
        out.print_line(format!("{}: invalid pattern", pattern));
    } else {
        out.print_line(format!("{}: valid with {} balls", pattern, sum / len));
    }
}

pub static TEST_TYPE: TestType = TestType::MultiEof;
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
