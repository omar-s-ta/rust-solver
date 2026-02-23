//{"name":"Knigs of the Forest","group":"Kattis","url":"https://open.kattis.com/problems/knigsoftheforest","interactive":false,"timeLimit":1000,"tests":[{"input":"2 4\n2013 2\n2011 1\n2011 3\n2014 4\n2012 6\n","output":"2013\n"},{"input":"2 4\n2011 1\n2013 2\n2012 4\n2011 5\n2014 3\n","output":"unknown\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"KnigsOfTheForest"}}}

use std::collections::BinaryHeap;

use algo_lib::io::input::Input;
use algo_lib::io::input::Readable;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

struct Turn {
    year: u32,
    power: u32,
}

impl Readable for Turn {
    fn read(input: &mut Input) -> Self {
        let year = input.read_unsigned();
        let power = input.read_unsigned();
        Turn { year, power }
    }
}

/// Simulation.
/// A priority_queue with 'k' initial competitors
/// Remove and add a competitor on each iteration
/// Do not forget the 'nth' step (I know I did)
fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let k = input.read_size();
    let n = input.read_size();

    let mut data = input.read_vec::<Turn>(n + k - 1);
    let karl = data[0].power;

    data.sort_unstable_by_key(|t| t.year);

    let mut heap = BinaryHeap::from(data.iter().take(k).map(|t| t.power).collect::<Vec<_>>());
    let mut year = 2011;

    for new_player in data.iter().skip(k) {
        if heap.pop().unwrap() == karl {
            out.print_line(year);
            return;
        }
        heap.push(new_player.power);
        year += 1;
    }

    if heap.pop().unwrap() == karl {
        out.print_line(year);
    } else {
        out.print_line("unknown");
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
