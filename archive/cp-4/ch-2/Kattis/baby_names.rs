use algo_lib::collections::static_fenwick_tree::StaticFenwickTree;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::math::cast::Cast;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::Str;
use algo_lib::string::str::StrReader;

type PreCalc = BabyNames;

const CH_N: usize = 26;

struct Node {
    next: [i32; CH_N],
    set: [StaticFenwickTree<i32, CH_N>; 2],
    end: [bool; 2],
}

impl Default for Node {
    fn default() -> Self {
        Self {
            next: [-1; CH_N],
            set: std::array::from_fn(|_| StaticFenwickTree::default()),
            end: [false; 2],
        }
    }
}

struct BabyNames {
    inner: Vec<Node>,
}

impl BabyNames {
    fn new() -> Self {
        Self {
            inner: vec![Node::default()],
        }
    }

    fn add(&mut self, name: &Str, gender: usize) {
        let mut at = 0;
        for ch in name {
            let ch: usize = (ch - b'A').to();
            if self.inner[at].next[ch] == -1 {
                let next = self.inner.len();
                self.inner[at].next[ch] = next.to();
                self.inner.push(Node::default());
            }
            self.inner[at].set[gender].add(ch, 1);
            at = self.inner[at].next[ch].to();
        }
        self.inner[at].end[gender] = true;
    }

    fn remove(&mut self, name: &Str, gender: usize) {
        let mut at = 0;
        for ch in name {
            let ch: usize = (ch - b'A').to();
            self.inner[at].set[gender].add(ch, -1);
            at = self.inner[at].next[ch].to();
        }
        self.inner[at].end[gender] = false;
    }

    fn count_lt(&self, name: &Str, gender: usize) -> i32 {
        let mut at = 0;
        let mut result = 0;
        for ch in name {
            let ch: usize = (ch - b'A').to();
            result += self.inner[at].set[gender].get_to(ch);
            result += i32::from(self.inner[at].end[gender]);
            match self.inner[at].next[ch] {
                -1 => return result,
                next => at = next.to(),
            }
        }
        result
    }

    fn gender_of(&self, name: &Str) -> Option<usize> {
        let mut at = 0;
        for ch in name {
            let ch: usize = (ch - b'A').to();
            match self.inner[at].next[ch] {
                -1 => return None,
                next => at = next.to(),
            }
        }
        self.inner[at].end.iter().position(|&g| g)
    }

    fn count(&self, a: &Str, b: &Str, gender: usize) -> i32 {
        self.count_lt(b, gender) - self.count_lt(a, gender)
    }
}

/// Use a `Trie` with `FenwickTree`s to count the how many string strictly
/// less than a given string 'X'
fn solve(input: &mut Input, out: &mut Output, _test_case: usize, baby_names: &mut PreCalc) {
    let op = input.read_size();
    if op == 0 {
        return;
    }
    match op {
        1 => {
            let name = input.read_str();
            let gender = input.read_char() - b'1';
            baby_names.add(&name, gender.to())
        }
        2 => {
            let name = input.read_str();
            if let Some(gender) = baby_names.gender_of(&name) {
                baby_names.remove(&name, gender)
            }
        }
        3 => {
            let a = input.read_str();
            let b = input.read_str();
            let gender = input.read_char();
            match gender {
                b'0' => out.print_line((0..2).map(|g| baby_names.count(&a, &b, g)).sum::<i32>()),
                b'1' => out.print_line(baby_names.count(&a, &b, 0)),
                b'2' => out.print_line(baby_names.count(&a, &b, 1)),
                _ => unreachable!(),
            }
        }
        _ => unreachable!(),
    }
}

pub static TEST_TYPE: TestType = TestType::MultiEof;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");

    let mut baby_names = BabyNames::new();

    match TEST_TYPE {
        TestType::Single => solve(&mut input, &mut output, 1, &mut baby_names),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 1..=t {
                solve(&mut input, &mut output, i, &mut baby_names);
            }
        }
        TestType::MultiEof => {
            let mut i = 1;
            while input.peek().is_some() {
                solve(&mut input, &mut output, i, &mut baby_names);
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
