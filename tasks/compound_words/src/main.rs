//{"name":"Compound Words","group":"Kattis","url":"https://open.kattis.com/problems/compoundwords","interactive":false,"timeLimit":1000,"tests":[{"input":"a bb\nab b\n","output":"aab\nab\naba\nabb\nabbb\nba\nbab\nbba\nbbab\nbbb\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CompoundWords"}}}

use std::collections::BTreeSet;
use std::ops::Sub;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::Str;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let mut words = vec![];
    while input.peek().is_some() {
        words.push(input.read_str());
    }
    words.sort();
    words.dedup();

    let mut compounds = Vec::with_capacity(words.len() * words.len().sub(1));
    for i in 0..words.len() {
        for j in 0..words.len() {
            if i != j {
                let mut string: Str = Vec::with_capacity(words[i].len() + words[j].len()).into();
                string += &words[i];
                string += &words[j];
                compounds.push(string);
            }
        }
    }
    compounds.sort();
    compounds.dedup();

    out.print_iter_per_line(compounds.iter());
}

fn _solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let mut words = vec![];
    while input.peek().is_some() {
        words.push(input.read_str());
    }
    let mut set = BTreeSet::new();
    for i in 0..words.len() {
        for j in 0..words.len() {
            if i == j {
                continue;
            }
            set.insert(format!("{}{}", words[i], words[j]));
        }
    }
    out.print_iter_per_line(set.iter());
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
