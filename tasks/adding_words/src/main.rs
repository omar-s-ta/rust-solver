//{"name":"Adding Words","group":"Kattis","url":"https://open.kattis.com/problems/addingwords","interactive":false,"timeLimit":1000,"tests":[{"input":"def foo 3\ncalc foo + bar =\ndef bar 7\ndef programming 10\ncalc foo + bar =\ndef is 4\ndef fun 8\ncalc programming - is + fun =\ndef fun 1\ncalc programming - is + fun =\nclear\n","output":"foo + bar = unknown\nfoo + bar = programming\nprogramming - is + fun = unknown\nprogramming - is + fun = bar\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AddingWords"}}}

use std::collections::HashMap;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

struct State {
    value: HashMap<String, i32>,
    name: HashMap<i32, String>,
}

impl State {
    fn new() -> Self {
        Self {
            value: HashMap::with_capacity(2000),
            name: HashMap::with_capacity(2000),
        }
    }

    fn insert(&mut self, name: &str, value: i32) {
        if let Some(old) = self.value.insert(name.into(), value)
            && self.name.get(&old).map(String::as_str) == Some(name)
        {
            self.name.remove(&old);
        }
        self.name.insert(value, name.into());
    }

    fn clear(&mut self) {
        self.value.clear();
        self.name.clear();
    }

    fn calc<'a>(&self, mut iter: impl Iterator<Item = &'a str>) -> Option<&str> {
        let mut value = *self.value.get(iter.next()?)?;
        while let Some(op) = iter.next() {
            if op == "=" {
                break;
            }
            let next = self.value.get(iter.next()?)?;
            value = match op {
                "+" => value + next,
                "-" => value - next,
                _ => return None,
            };
        }
        self.name.get(&value).map(String::as_str)
    }
}

type PreCalc = State;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, data: &mut PreCalc) {
    let command = input.read_line();
    let mut commands = command.split_whitespace();

    match commands.next().unwrap() {
        "def" => {
            if let (Some(name), Some(value)) = (commands.next(), commands.next()) {
                data.insert(name, value.parse().unwrap());
            }
        }
        "calc" => {
            let result = data.calc(commands.clone()).unwrap_or("unknown");
            out.print_iter_one_line(commands.chain(std::iter::once(result)))
        }
        _ => data.clear(),
    }
}

pub static TEST_TYPE: TestType = TestType::MultiEof;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = State::new();

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
