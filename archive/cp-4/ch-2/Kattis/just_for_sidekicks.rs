use std::ops::Range;

use algo_lib::collections::fenwick_tree::FenwickTree;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::Str;
use algo_lib::string::str::StrReader;

type PreCalc = ();

const TYPES: usize = 6;

fn id(c: u8) -> usize {
    (c - b'1') as usize
}

struct NFenwickTree {
    fts: [FenwickTree<i32>; TYPES],
    values: [u64; TYPES],
    types: Str,
}

impl NFenwickTree {
    fn new(n: usize, values: [u64; TYPES], types: Str) -> Self {
        let mut ft = Self {
            fts: std::array::from_fn(|_| FenwickTree::new(n)),
            values,
            types,
        };
        for i in 0..ft.types.len() {
            ft.add(id(ft.types[i]), i);
        }
        ft
    }

    fn add(&mut self, gem_type: usize, at: usize) {
        self.fts[gem_type].add(at, 1)
    }

    fn remove(&mut self, gem_type: usize, at: usize) {
        self.fts[gem_type].add(at, -1)
    }

    fn get(&self, bounds: Range<usize>) -> u64 {
        self.fts
            .iter()
            .zip(&self.values)
            .map(|(ft, &v)| ft.get(bounds.clone()) as u64 * v)
            .sum()
    }

    fn revalue(&mut self, gem_type: usize, new_value: u64) {
        self.values[gem_type] = new_value;
    }

    fn change_type(&mut self, at: usize, gem_type: u8) {
        self.remove(id(self.types[at]), at);
        self.types[at] = gem_type;
        self.add(id(gem_type), at)
    }
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let q = input.read_size();
    let values: [u64; TYPES] = std::array::from_fn(|_| input.read_u64());
    let types = input.read_str();

    let mut ft = NFenwickTree::new(n, values, types);
    for _ in 0..q {
        match input.read_u16() {
            1 => {
                let at = input.read_size();
                let gem_type = input.read_char();
                ft.change_type(at - 1, gem_type);
            }
            2 => {
                let gem_type = input.read_size();
                let new_value = input.read_u64();
                ft.revalue(gem_type - 1, new_value);
            }
            3 => {
                let a = input.read_size();
                let b = input.read_size();
                out.print_line(ft.get(a - 1..b));
            }
            _ => unreachable!(),
        }
    }
}

pub static TEST_TYPE: TestType = TestType::Single;
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
            while !input.is_empty() {
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
