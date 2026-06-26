use std::cmp::Reverse;
use std::collections::BTreeMap;
use std::collections::HashMap;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::Str;
use algo_lib::string::str::StrReader;

const ARRIVE: u16 = 0;
const UPDATE: u16 = 1;
const TREAT: u16 = 2;
const QUERY: u16 = 3;

#[derive(Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
struct Info {
    level: Reverse<usize>,
    id: usize,
}

#[derive(Default)]
struct Map {
    entry: HashMap<Str, Info>,
    order: BTreeMap<Info, Str>,
}

impl Map {
    fn insert(&mut self, name: Str, id: usize, level: usize) {
        let info = Info {
            level: Reverse(level),
            id,
        };
        self.entry.insert(name.clone(), info);
        self.order.insert(info, name);
    }

    fn update(&mut self, name: &Str, by: usize) {
        if let Some(info) = self.entry.get_mut(name) {
            self.order.remove(info);
            info.level.0 += by;
            self.order.insert(*info, name.clone());
        }
    }

    fn remove(&mut self, name: &Str) {
        if let Some(info) = self.entry.remove(name) {
            self.order.remove(&info);
        }
    }

    fn query(&self, out: &mut Output) {
        if let Some(name) = self.order.values().next() {
            out.print_line(name);
        } else {
            out.print_line("The clinic is empty");
        }
    }
}

type CatData = Map;

/// Just pick the suitable data structure(s).
fn solve(input: &mut Input, out: &mut Output, id: usize, cats: &mut CatData) {
    match input.read_u16() {
        ARRIVE => {
            let name = input.read_str();
            let level = input.read_size();
            cats.insert(name, id, level);
        }
        UPDATE => {
            let name = input.read_str();
            let level = input.read_size();
            cats.update(&name, level);
        }
        TREAT => cats.remove(&input.read_str()),
        QUERY => cats.query(out),
        _ => unreachable!(),
    }
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");

    let mut cats = Map::default();

    match TEST_TYPE {
        TestType::Single => solve(&mut input, &mut output, 1, &mut cats),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 1..=t {
                solve(&mut input, &mut output, i, &mut cats);
            }
        }
        TestType::MultiEof => {
            let mut i = 1;
            while input.peek().is_some() {
                solve(&mut input, &mut output, i, &mut cats);
                i += 1;
            }
        }
        _ => {
            unreachable!();
        }
    }
    eprint!("\x1B[0m");
    output.flush();
    input.is_run_done()
}

#[cfg(feature = "local")]
mod tester;

#[cfg(feature = "local")]
fn main() {
    tester::run_tests();
}

#[cfg(not(feature = "local"))]
fn main() {
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
