use std::cell::Cell;

use algo_lib::collections::disjoint_set::CompressedDisjointSet;
use algo_lib::collections::disjoint_set::DisjointSet;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

struct MovableDisjointSet {
    inner: CompressedDisjointSet,
    at: Vec<usize>,
    sum: Vec<usize>,
    next: Cell<usize>,
}

impl MovableDisjointSet {
    fn new(n: usize, m: usize) -> Self {
        Self {
            inner: CompressedDisjointSet::new(n + m),
            at: (0..n + m).collect(),
            sum: (0..n + m).collect(),
            next: Cell::new(n),
        }
    }

    fn root(&self, i: usize) -> usize {
        self.inner.find(self.at[i])
    }

    fn get(&self, i: usize) -> (usize, usize) {
        let r = self.root(i);
        (self.inner.size(r), self.sum[r])
    }

    fn change(&mut self, f: usize, t: usize) {
        let p = self.root(f);
        let q = self.root(t);
        if p == q {
            return;
        }
        self.inner.modify_at(p, 1);
        self.sum[p] -= f;
        let k = self.next.get();
        self.sum[k] = f;
        self.at[f] = k;
        self.inner.direct_union(k, q);
        self.sum[k] += self.sum[q];
        self.next.set(self.next.get() + 1)
    }

    fn union(&mut self, i: usize, j: usize) {
        let i = self.root(i);
        let j = self.root(j);
        if i == j {
            return;
        }
        self.inner.direct_union(i, j);
        self.sum[i] += self.sum[j];
    }
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();

    let mut ds = MovableDisjointSet::new(n + 1, m);
    for _ in 0..m {
        match input.read_size() {
            1 => {
                let a = input.read_size();
                let b = input.read_size();
                ds.union(a, b);
            }
            2 => {
                let a = input.read_size();
                let b = input.read_size();
                ds.change(a, b);
            }
            3 => {
                let i = input.read_size();
                out.print_line(ds.get(i));
            }
            _ => unreachable!(),
        }
    }
}

pub static TEST_TYPE: TestType = TestType::MultiEof;
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
