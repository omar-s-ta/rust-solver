use std::collections::BTreeMap;
use std::collections::HashMap;

use algo_lib::io::input::Input;
use algo_lib::io::input::Readable;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::Str;
use algo_lib::string::str::StrReader;

type PreCalc = ();

struct Cost {
    catalog: u32,
    pick_up: u32,
    per_kms: u32,
}

impl Readable for Cost {
    fn read(input: &mut Input) -> Self {
        let catalog = input.read_u32();
        let pick_up = input.read_u32();
        let per_kms = input.read_u32();
        Cost {
            catalog,
            pick_up,
            per_kms,
        }
    }
}

enum State {
    Accident(u32),
    Pickup(Str),
    Return(u32),
}

impl Readable for State {
    fn read(input: &mut Input) -> Self {
        let c = input.read_char();
        match c {
            b'a' => State::Accident(input.read_u32()),
            b'p' => State::Pickup(input.read_str()),
            b'r' => State::Return(input.read_u32()),
            _ => unreachable!(),
        }
    }
}

fn bill_value(events: &[State], costs: &HashMap<Str, Cost>) -> Option<u32> {
    let mut car_cost = None;
    let mut bill = 0;
    for event in events {
        match event {
            State::Pickup(_) if car_cost.is_some() => return None,
            State::Pickup(car) => {
                car_cost = costs.get(car);
                bill += car_cost?.pick_up;
            }
            State::Accident(p) => bill += (car_cost?.catalog * p).div_ceil(100),
            State::Return(k) => bill += car_cost.take()?.per_kms * k,
        }
    }
    car_cost.is_none().then_some(bill)
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let cars = input.read_u32();
    let events = input.read_u32();

    let mut costs = HashMap::new();
    for _ in 0..cars {
        let car = input.read_str();
        costs.insert(car, input.read());
    }

    let mut spy: BTreeMap<Str, Vec<State>> = BTreeMap::new();
    for _ in 0..events {
        let _ = input.read_u32();
        let name = input.read_str();
        spy.entry(name).or_default().push(input.read());
    }

    for (name, events) in spy {
        match bill_value(&events, &costs) {
            Some(bill) => out.print_line((name, bill)),
            None => out.print_line((name, "INCONSISTENT")),
        }
    }
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");

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
