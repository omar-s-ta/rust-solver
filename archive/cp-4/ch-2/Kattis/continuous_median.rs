use std::cmp::Reverse;
use std::collections::BinaryHeap;

use algo_lib::collections::id_map::IdMap;
use algo_lib::collections::order_stat_set::OrderStatSet;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::collections::vec_ext::dedup::Dedup;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

/// `order statistics` solution
fn solve2(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let ns = input.read_size_vec(n);

    let sorted = ns.clone().sorted_deduped();
    let mut id = IdMap::default();
    for i in sorted.indices() {
        id.intern_owned(sorted[i]);
    }

    let (result, _) = ns
        .iter()
        .fold((0, OrderStatSet::new(n)), |(acc, mut set), &n| {
            set.insert(id.lookup(&n).unwrap());
            let m = set.len() / 2;
            let to_add = match set.len().is_multiple_of(2) {
                true => (id.value(set.kth(m)) + id.value(set.kth(m + 1))) / 2,
                false => *id.value(set.kth(m + 1)),
            };
            (acc + to_add, set)
        });

    out.print_line(result)
}

/// two heaps solution
fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = n / 2 + 1;

    let mut lo: BinaryHeap<usize> = BinaryHeap::with_capacity(m);
    let mut hi: BinaryHeap<Reverse<usize>> = BinaryHeap::with_capacity(m);

    let mut result = 0;
    for _ in 0..n {
        let e = input.read_size();
        if lo.peek().is_some_and(|&top| e < top) {
            lo.push(e);
            if lo.len() > hi.len() + 1 {
                let top = lo.pop().unwrap();
                hi.push(Reverse(top));
            }
        } else {
            hi.push(Reverse(e));
            if hi.len() > lo.len() {
                let top = hi.pop().unwrap().0;
                lo.push(top);
            }
        }
        result += match lo.len() == hi.len() {
            true => (lo.peek().unwrap() + hi.peek().unwrap().0) / 2,
            false => *lo.peek().unwrap(),
        };
    }
    out.print_line(result);
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
