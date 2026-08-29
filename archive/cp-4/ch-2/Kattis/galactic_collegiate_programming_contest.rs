use algo_lib::collections::fenwick_tree::FenwickTree;
use algo_lib::collections::slice_ext::sorted_ops::SortedOps;
use algo_lib::collections::vec_ext::dedup::Dedup;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::math::cast::Cast;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

const N: usize = 100002;

type PreCalc = [Team; N];

#[derive(Default, Clone, Copy)]
struct Team {
    count: usize,
    penalty: usize,
}

impl Team {
    /// monotonically increasing in solved problems count
    /// monotonically decreasing in penalty sum
    fn score(&self, offset: usize) -> usize {
        self.count * offset - self.penalty
    }

    fn solve_with(&mut self, penalty: usize) {
        self.count += 1;
        self.penalty += penalty;
    }
}

/// Solvable with a 2D fenwick tree, but it will `Memory Limit Exceed`.
/// Map the score into compressed ids and use a 1D fenwick tree on the compressed ids.
fn solve(input: &mut Input, out: &mut Output, _test_case: usize, state: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let events = input.read_size_pair_vec(m);

    let offset = 1000 * m + 1;
    let scores = events
        .iter()
        .map(|&(team, penalty)| {
            state[team].solve_with(penalty);
            state[team].score(offset)
        })
        .collect::<Vec<_>>();

    let keys = scores
        .iter()
        .copied()
        .chain([0])
        .collect::<Vec<_>>()
        .sorted_deduped();

    let teams: i32 = n.to();
    let mut set = FenwickTree::<i32>::new(keys.len());
    let mut at = vec![0usize; n + 1];

    set.add(0, teams);
    for (&(team, _), score) in events.iter().zip(&scores) {
        set.add(at[team], -1);
        at[team] = keys.lower_bound(score);
        set.add(at[team], 1);
        out.print_line(1 + teams - set.get_to(at[1] + 1));
    }
}

pub static TEST_TYPE: TestType = TestType::Single;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");

    let mut penalty = [Team::default(); N];

    match TEST_TYPE {
        TestType::Single => solve(&mut input, &mut output, 1, &mut penalty),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 1..=t {
                solve(&mut input, &mut output, i, &mut penalty);
            }
        }
        TestType::MultiEof => {
            let mut i = 1;
            while !input.is_empty() {
                solve(&mut input, &mut output, i, &mut penalty);
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
