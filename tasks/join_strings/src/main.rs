//{"name":"Join Strings","group":"Kattis","url":"https://open.kattis.com/problems/joinstrings","interactive":false,"timeLimit":1000,"tests":[{"input":"4\ncute\ncat\nkattis\nis\n3 2\n4 1\n3 4\n","output":"kattiscatiscute\n"},{"input":"3\nhowis\nthis\npracticalexam\n1 2\n1 3\n","output":"howisthispracticalexam\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"JoinStrings"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::list::List;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let strs = input.read_vec::<String>(n);

    let mut lists = Vec::new();
    for i in 0..n {
        lists.push(List::with_elem(i));
    }

    let mut at = 0;
    let mut sizes = 0;

    for _ in 0..(n - 1) {
        let a = input.read_size() - 1;
        let b = input.read_size() - 1;

        if a < b {
            let (left, right) = lists.split_at_mut(b);
            left[a].append(&mut right[0]);
        } else {
            let (left, right) = lists.split_at_mut(a);
            right[0].append(&mut left[b]);
        }

        if lists[a].len() > sizes {
            sizes = lists[a].len();
            at = a;
        }
    }

    for index in lists[at].iter() {
        out.print(&strs[*index]);
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
