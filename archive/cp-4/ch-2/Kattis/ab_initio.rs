use algo_lib::collections::md_array::MdArray;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::io::output::Writable;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

const MOD: usize = 10_usize.pow(9) + 7;
const V: usize = 4001;

struct AdjMatrix {
    matrix: MdArray<bool, 2>,
    is_transpose: bool,
    is_complement: bool,
    len: usize,
}

impl AdjMatrix {
    fn new(n: usize, max_n: usize) -> Self {
        Self {
            matrix: MdArray::new([max_n, max_n], false),
            is_transpose: false,
            is_complement: false,
            len: n,
        }
    }

    fn transpose(&mut self) {
        self.is_transpose = !self.is_transpose;
    }

    fn complement(&mut self) {
        self.is_complement = !self.is_complement;
    }

    fn add_edge(&mut self, v1: usize, v2: usize) {
        if self.is_transpose {
            self.matrix[(v2, v1)] = !self.is_complement;
        } else {
            self.matrix[(v1, v2)] = !self.is_complement;
        }
    }

    fn del_edge(&mut self, v1: usize, v2: usize) {
        if self.is_transpose {
            self.matrix[(v2, v1)] = self.is_complement;
        } else {
            self.matrix[(v1, v2)] = self.is_complement;
        }
    }

    fn add_vertex(&mut self) {
        self.matrix
            .row_mut(self.len)
            .for_each(|v| *v = self.is_complement);
        self.matrix
            .col_mut(self.len)
            .for_each(|v| *v = self.is_complement);
        self.len += 1;
    }

    fn reset(&mut self, vertex: usize) {
        self.matrix
            .col_mut(vertex)
            .for_each(|v| *v = self.is_complement);
        self.matrix
            .row_mut(vertex)
            .for_each(|v| *v = self.is_complement);
    }
}

impl Writable for AdjMatrix {
    fn write(&self, output: &mut Output) {
        let n = self.len;
        output.print_line(n);
        for i in 0..n {
            let mut degree = 0usize;
            let mut pow = 1usize;
            let mut hash = 0usize;
            for j in 0..n {
                if i == j {
                    continue;
                }
                let (a, b) = if self.is_transpose { (j, i) } else { (i, j) };
                if self.matrix[[a, b]] != self.is_complement {
                    degree += 1;
                    hash = (hash + pow * j) % MOD;
                    pow = pow * 7 % MOD;
                }
            }
            output.print_line((degree, hash));
        }
    }
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let v = input.read_size();
    let e = input.read_size();
    let q = input.read_size();

    let mut adj_m = AdjMatrix::new(v, V);
    for _ in 0..e {
        let a = input.read_size();
        let b = input.read_size();
        adj_m.add_edge(a, b);
    }

    for _ in 0..q {
        match input.read_size() {
            1 => adj_m.add_vertex(),
            2 => {
                let a = input.read_size();
                let b = input.read_size();
                adj_m.add_edge(a, b);
            }
            3 => {
                let a = input.read_size();
                adj_m.reset(a);
            }
            4 => {
                let a = input.read_size();
                let b = input.read_size();
                adj_m.del_edge(a, b);
            }
            5 => adj_m.transpose(),
            6 => adj_m.complement(),
            _ => unreachable!(),
        }
    }
    out.print_line(adj_m);
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
