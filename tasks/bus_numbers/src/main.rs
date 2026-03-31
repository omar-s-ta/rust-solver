//{"name":"Bus Numbers","group":"Kattis","url":"https://open.kattis.com/problems/busnumbers","interactive":false,"timeLimit":1000,"tests":[{"input":"6\n180 141 174 143 142 175\n","output":"141-143 174 175 180\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BusNumbers"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

const M: usize = 1001;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();

    let mut visited = vec![false; M];
    for _ in 0..n {
        visited[input.read_size()] = true;
    }

    let mut first = true;
    let mut i = 1;
    while i < M {
        if visited[i] {
            let mut j = i;
            while j + 1 < M && visited[j + 1] {
                j += 1;
            }
            if !first {
                out.print(' ');
            }
            first = false;
            if j - i > 1 {
                out.print(i);
                out.print('-');
                out.print(j);
            } else {
                out.print(i);
                if j > i {
                    out.print(' ');
                    out.print(j);
                }
            }
            i = j;
        }
        i += 1;
    }
    out.print_empty_line();
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
