//{"name":"Shiritori","group":"Kattis","url":"https://open.kattis.com/problems/shiritori","interactive":false,"timeLimit":2000,"tests":[{"input":"5\napple\near\nreal\nletters\nstyle\n","output":"Fair Game\n"},{"input":"3\napple\nextra\napple\n","output":"Player 1 lost\n"},{"input":"2\napple\nneat\n","output":"Player 2 lost\n"},{"input":"5\napple\neast\nteam\nmeat\nteam\n","output":"Player 1 lost\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"Shiritori"}}}

use std::collections::HashSet;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();

    let mut set = HashSet::new();
    let mut prv = input.read_string().into_bytes();
    set.insert(prv.clone());

    for i in 1..n {
        let nxt = input.read_string().into_bytes();
        if nxt[0] != prv[prv.len() - 1] || set.contains(&nxt) {
            out.print_line(format!("Player {} lost", if i % 2 == 0 { 1 } else { 2 }));
            return;
        }
        set.insert(nxt.clone());
        prv = nxt;
    }
    out.print_line("Fair Game");
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
