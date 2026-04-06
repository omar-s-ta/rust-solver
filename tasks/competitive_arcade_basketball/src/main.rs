//{"name":"Competitive Arcade Basketball","group":"Kattis","url":"https://open.kattis.com/problems/competitivearcadebasketball","interactive":false,"timeLimit":3000,"tests":[{"input":"3 10 13\nJohn\nKelly\nGeorge\nKelly 1\nGeorge 2\nKelly 1\nJohn 2\nGeorge 1\nJohn 3\nKelly 3\nKelly 1\nGeorge 3\nGeorge 1\nJohn 3\nGeorge 3\nKelly 1\n","output":"George wins!\n"},{"input":"4 10 13\nBob\nNina\nJess\nTim\nNina 2\nBob 2\nNina 1\nJess 3\nBob 2\nJess 2\nNina 1\nJess 2\nNina 3\nBob 1\nNina 3\nJess 3\nBob 2\n","output":"Nina wins!\nJess wins!\n"},{"input":"4 15 13\nBob\nNina\nJess\nTim\nNina 2\nBob 2\nNina 1\nJess 3\nBob 2\nJess 2\nNina 1\nJess 2\nNina 3\nBob 1\nNina 3\nJess 3\nBob 2\n","output":"No winner!\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CompetitiveArcadeBasketball"}}}

use std::collections::HashMap;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let p = input.read_unsigned();
    let m = input.read_size();

    let mut scores = (0..n)
        .map(|_| (input.read_string(), 0))
        .collect::<HashMap<_, _>>();

    let mut winners = Vec::new();
    for _ in 0..m {
        let player = input.read_string();
        let points = input.read_unsigned();

        if let Some(score) = scores.get_mut(&player) {
            *score += points;
            if *score >= p && !winners.contains(&player) {
                winners.push(player);
            }
        }
    }

    if winners.is_empty() {
        out.print_line("No winner!");
    } else {
        out.print_iter_per_line(winners.iter().map(|w| format!("{} wins!", w)));
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
