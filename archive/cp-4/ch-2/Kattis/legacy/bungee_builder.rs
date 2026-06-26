//{"name":"Bungee Builder","group":"Kattis","url":"https://open.kattis.com/problems/bungeebuilder","interactive":false,"timeLimit":1000,"tests":[{"input":"10\n3 5 1 4 2 3 7 2 2 5\n","output":"4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BungeeBuilder"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn _solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let heights = input.read_i32_vec(n);

    if n < 3 {
        out.print_line(0);
        return;
    }

    let mut left = heights[0];
    let mut max_so_far = 0;
    let mut min_so_far = heights[0];
    let mut result = 0;

    for &height in heights.iter().skip(1) {
        if left > height {
            if height < min_so_far {
                min_so_far = height;
                max_so_far = height;
            } else {
                result = result.max(height - min_so_far);
                max_so_far = max_so_far.max(height);
            }
        } else {
            result = result.max(left - min_so_far);
            left = height;
            min_so_far = height;
            max_so_far = 0;
        }
    }

    out.print_line(result.max(left.min(max_so_far) - min_so_far));
}

struct State {
    pub index: usize,
    pub depth: u32,
}

impl State {
    pub fn new(index: usize, depth: u32) -> Self {
        State { index, depth }
    }
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let heights = input.read_u32_vec(n);

    if n < 3 {
        out.print_line(0);
        return;
    }

    let mut stack: Vec<State> = vec![];
    let mut result = 0;

    for i in 0..n {
        let mut valley = heights[i];
        while let Some(state) = stack.last() {
            if heights[state.index] < heights[i] {
                valley = valley.min(state.depth);
                result = result.max(heights[state.index] - valley);
                stack.pop();
            } else {
                break;
            }
        }
        if !stack.is_empty() {
            stack.last_mut().unwrap().depth = stack.last().unwrap().depth.min(valley);
            result = result.max(heights[i] - stack.last().unwrap().depth);
        }
        stack.push(State::new(i, heights[i]));
    }

    out.print_line(result);
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
