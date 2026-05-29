//{"name":"Working at the Restaurant","group":"Kattis","url":"https://open.kattis.com/problems/restaurant","interactive":false,"timeLimit":1000,"tests":[{"input":"3\nDROP 100\nTAKE 50\nTAKE 20\n3\nDROP 3\nDROP 5\nTAKE 8\n0\n","output":"DROP 2 100\nMOVE 2->1 100\nTAKE 1 50\nTAKE 1 20\n\nDROP 2 3\nDROP 2 5\nMOVE 2->1 8\nTAKE 1 8\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"WorkingAtTheRestaurant"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

const P1: usize = 1;
const P2: usize = 0;

fn from_waiter(dishes: i32, stacks: &mut [i32]) {
    stacks[P2] += dishes;
    println!("DROP 2 {}", dishes);
}

fn to_dishwasher(dishes: i32, stacks: &mut [i32]) {
    let mut dishes = dishes;
    let to_take = dishes.min(stacks[P1]);
    if to_take != 0 {
        println!("TAKE 1 {}", to_take);
        stacks[P1] -= to_take;
        dishes -= to_take;
    }
    if dishes != 0 {
        stacks[P1] += stacks[P2] - dishes;
        println!("MOVE 2->1 {}", stacks[P2]);
        stacks[P2] = 0;
        println!("TAKE 1 {}", dishes);
    }
}

fn solve(input: &mut Input, _out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let mut first = true;
    loop {
        let n = input.read_i32();
        if n == 0 {
            break;
        }
        if !first {
            println!();
        }

        let mut stacks = vec![0; 2];
        for _ in 0..n {
            let cmd = input.read_string();
            let dishes = input.read_i32();
            if cmd.starts_with('D') {
                from_waiter(dishes, &mut stacks);
            } else {
                to_dishwasher(dishes, &mut stacks);
            }
        }

        first = false;
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
