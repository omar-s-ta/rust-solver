use algo_lib::collections::fenwick_tree::FenwickTree;
use algo_lib::collections::id_map::IdMap;
use algo_lib::collections::vec_ext::dedup::Dedup;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
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
    let n = input.read_i32();
    let m = input.read_size();
    let events = input.read_size_pair_vec(m);

    let offset = 1000 * m + 1;
    let keys = events
        .iter()
        .fold(Vec::from([0]), |mut ks, &(team, penalty)| {
            state[team].solve_with(penalty);
            ks.push(state[team].score(offset));
            ks
        })
        .sorted_deduped();

    let id = keys.into_iter().fold(IdMap::default(), |mut id_map, key| {
        id_map.intern_owned(key);
        id_map
    });

    state.fill(Team::default());
    let mut set = FenwickTree::<i32>::new(id.len());
    set.add(0, n);

    for (team, penalty) in events {
        id.lookup(&state[team].score(offset))
            .into_iter()
            .for_each(|i| set.add(i, -1));

        state[team].solve_with(penalty);

        id.lookup(&state[team].score(offset))
            .into_iter()
            .for_each(|i| set.add(i, 1));

        id.lookup(&state[1].score(offset))
            .into_iter()
            .for_each(|one| out.print_line(1 + n - set.get_to(one + 1)));
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
