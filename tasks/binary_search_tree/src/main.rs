//{"name":"Binary search tree","group":"Kattis","url":"https://open.kattis.com/problems/bst","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n1\n2\n3\n4\n","output":"0\n1\n3\n6\n"},{"input":"5\n3\n2\n4\n1\n5\n","output":"0\n1\n2\n4\n6\n"},{"input":"8\n3\n5\n1\n6\n8\n7\n2\n4\n","output":"0\n1\n2\n4\n7\n11\n13\n15\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BinarySearchTree"}}}

use std::collections::BTreeSet;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

/// Save the depth of each node in dist[e]
/// Build the cost from it, and accumulate to the total
fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();

    let mut set = BTreeSet::new();
    let mut dist = vec![0_u32; n + 1];
    let mut result = 0_u64;

    for _ in 0..n {
        let e = input.read_size();
        set.insert(e);

        let mut cost = 0;
        if let Some(&prev) = set.range(..e).next_back() {
            cost = cost.max(dist[prev] + 1);
        }
        if let Some(&next) = set.range((e + 1)..).next() {
            cost = cost.max(dist[next] + 1);
        }
        dist[e] = cost;
        result += cost as u64;

        out.print_line(result);
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
