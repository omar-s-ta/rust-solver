use algo_lib::collections::disjoint_set::CompressedDisjointSet;
use algo_lib::collections::disjoint_set::DisjointSet;
use algo_lib::collections::hash_set::FxHashSet;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

const N: usize = 500000 + 1;

type PreCalc = CompressedDisjointSet;

/// The problem statement is a bit confusing
///  - The ingredients are split into groups. An unused one is on its own, and
///    each potion you already made is a group holding exactly its ingredients.
///  - You can only pour in a whole cauldron, never part of one, so a recipe can
///    be concocted `iff` every group it touches is fully inside it.
///  - The groups don't overlap, so that check is just: their sizes sum up to M.
///  - You don't get to pick. If a recipe can be made it's made, so one pass
///    over them is enough.
fn solve(input: &mut Input, out: &mut Output, _test_case: usize, ds: &mut PreCalc) {
    let n = input.read_size();
    let mut taken_by = [0usize; N];
    let result = (1..=n).fold(0, |brewed, recipe| {
        let mut roots = Vec::new();
        let m = input.read_size();
        let total = (0..m).fold(0, |covered, _| {
            let root = ds.find(input.read_size());
            if taken_by[root] != recipe {
                taken_by[root] = recipe;
                roots.push(root);
                covered + ds.size(root)
            } else {
                covered
            }
        });
        if total == m {
            for &root in &roots[1..] {
                ds.union(roots[0], root);
            }
            brewed + 1
        } else {
            brewed
        }
    });
    out.print_line(result);
}

pub static TEST_TYPE: TestType = TestType::Single;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");

    let mut pre_calc = CompressedDisjointSet::new(N);

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
