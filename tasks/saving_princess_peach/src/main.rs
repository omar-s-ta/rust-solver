//{"name":"Saving Princess Peach","group":"Kattis","url":"https://open.kattis.com/problems/princesspeach","interactive":false,"timeLimit":1000,"tests":[{"input":"20 4\n5\n10\n12\n16\n","output":"0\n1\n2\n3\n4\n6\n7\n8\n9\n11\n13\n14\n15\n17\n18\n19\nMario got 4 of the dangerous obstacles.\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"SavingPrincessPeach"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

///
///Use a array of bool as a 'Direct Access Table'
fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let obstacles = input.read_size_vec(m);

    let visited = obstacles.iter().fold(vec![false; n], |mut visited, &o| {
        visited[o] = true;
        visited
    });

    (0..n).for_each(|i| {
        if !visited[i] {
            out.print_line(i);
        }
    });

    let count = visited.iter().filter(|&&v| v).count();
    out.print_line(format!("Mario got {} of the dangerous obstacles.", count));
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
