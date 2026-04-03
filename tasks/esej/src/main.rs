//{"name":"Esej","group":"Kattis","url":"https://open.kattis.com/problems/esej","interactive":false,"timeLimit":1000,"tests":[{"input":"2 7\n","output":"dobar je ovaj marko marulic\n"},{"input":"26 30\n","output":"nama je profesor reko da to ne treba za lektiru al onda je bila buka i nisam ga cuo pa jel bi mi mogli dat bodove\n"},{"input":"19 19\n","output":"konzekvence pojmovnoga diskursa u predstavljenoj noveli naizgled ne odrazavaju paradigmatske tendencije tipoloske klasifikacije iako su stilski i didakticki opravdane\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"Esej"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::lcg::Lcg;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz";

type PreCalc = Lcg;

/// If you use the 'rng' make sure the size is big to avoid
/// multiple collisions so you can achieve the 'B/2' uniqueness constraint
fn random_string(len: usize, rng: &mut PreCalc) -> String {
    (0..len)
        .map(|_| {
            let at = (rng.generate() as usize) % CHARSET.len();
            CHARSET[at] as char
        })
        .collect()
}

/// Generate a deterministic guaranteed unique string
fn to_string(n: usize) -> String {
    (0..4)
        .rev()
        .map(|i| (b'a' + (n / 26_usize.pow(i) % 26) as u8) as char)
        .collect()
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, rng: &mut PreCalc) {
    let _ = input.read_size();
    let b = input.read_size();

    let mut first = true;
    for i in 0..b {
        if !first {
            out.print(' ');
        }
        first = false;
        // out.print(random_string(14, rng));
        out.print(to_string(i));
    }
    out.print_empty_line();
}

pub static TEST_TYPE: TestType = TestType::Single;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut rng = Lcg::new();

    match TEST_TYPE {
        TestType::Single => solve(&mut input, &mut output, 1, &mut rng),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 1..=t {
                solve(&mut input, &mut output, i, &mut rng);
            }
        }
        TestType::MultiEof => {
            let mut i = 1;
            while input.peek().is_some() {
                solve(&mut input, &mut output, i, &mut rng);
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
