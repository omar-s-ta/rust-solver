//{"name":"Greeting Card","group":"Kattis","url":"https://open.kattis.com/problems/greetingcard","interactive":false,"timeLimit":3000,"tests":[{"input":"4\n20180000 20180000\n20180000 20182018\n20182018 20180000\n20182018 20182018\n","output":"4\n"},{"input":"6\n0 0\n1680 1118\n3360 0\n5040 1118\n6720 0\n8400 1118\n","output":"5\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GreetingCard"}}}

use std::collections::HashSet;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = HashSet<(i64, i64)>;

const D: i64 = 2018;

///
/// You can not do O(n^2) solution, so you have 2 options:
///
/// 1. This implemented solution.
///    Precompute all the offsets since you already know the wanted distance (2018),
///    then for each point count (p.x + offset.x, p.y + offset.y) that is already in
///    the given list of points.
///    Then return (count / 2), since you count point_a -> point_b and point_b -> point_a
///    complexity: O(n * number_of_offsets)
///    number_of_offsets = 12 in this case.
///
/// 2. Instead of precomputing the offsets, compute them on the fly for each point,
///    and count the point that is already in the list as solution 1.
///
fn solve(input: &mut Input, out: &mut Output, _test_case: usize, offsets: &PreCalc) {
    let n = input.read_size();
    let set = input
        .read_long_pair_vec(n)
        .into_iter()
        .collect::<HashSet<_>>();

    let result = set.iter().fold(0, |count, (x, y)| {
        let in_count = offsets
            .iter()
            .filter(|&(ox, oy)| set.contains(&(ox + x, oy + y)))
            .count();

        count + in_count
    });

    out.print_line(result / 2);
}

pub static TEST_TYPE: TestType = TestType::Single;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let offsets = (-D..=D)
        .flat_map(|dx| {
            let dy2 = D * D - dx * dx;
            let dy = (dy2 as f64).sqrt() as i64;
            [-dy, dy]
                .into_iter()
                .filter(move |y| y * y == dy2)
                .map(move |y| (dx, y))
        })
        .collect::<HashSet<_>>();

    match TEST_TYPE {
        TestType::Single => solve(&mut input, &mut output, 1, &offsets),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 1..=t {
                solve(&mut input, &mut output, i, &offsets);
            }
        }
        TestType::MultiEof => {
            let mut i = 1;
            while input.peek().is_some() {
                solve(&mut input, &mut output, i, &offsets);
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
