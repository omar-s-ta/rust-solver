use algo_lib::collections::disjoint_set::CompressedDisjointSet;
use algo_lib::collections::disjoint_set::DisjointSet;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

struct Drawers {
    inner: CompressedDisjointSet,
    full: Vec<bool>,
}

impl Drawers {
    fn new(n: usize) -> Self {
        Self {
            inner: CompressedDisjointSet::new(n),
            full: vec![false; n],
        }
    }

    fn try_store(&mut self, a: usize, b: usize) -> bool {
        let a = self.inner.find(a);
        let b = self.inner.find(b);
        if a == b {
            let can_store = !self.full[a];
            self.full[a] = true;
            can_store
        } else {
            let full = self.full[a] || self.full[b];
            let can_store = !(self.full[a] && self.full[b]);
            self.inner.union(a, b);
            let root = self.inner.find(a);
            self.full[root] = full;
            can_store
        }
    }
}

/// Either use:
/// - count[i] -> count of elemnts in drawers with root i.
/// - full[i]  -> is drawers with root i full or not.
///
/// to decide if you can store an element or not.
fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let d = input.read_size();

    let mut drawers = Drawers::new(d);
    for _ in 0..n {
        let (a, b) = input.read::<(usize, usize)>().dec();
        out.print_line(drawers.try_store(a, b));
    }
}

pub static TEST_TYPE: TestType = TestType::Single;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");

    let mut pre_calc = ();

    output.set_bool_output(BoolOutput::Custom("LADICA", "SMECE"));

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
