//{"name":"Sim","group":"Kattis","url":"https://open.kattis.com/problems/sim","interactive":false,"timeLimit":1000,"tests":[{"input":"1\nmy ]]name]] is]] steva<en]<n halim]]]]]\n","output":"my name is steven halim\n"},{"input":"1\n<<hate<<<<loves[steva<en ] cs2040c< and also cs2040c\n","output":"steven loves cs2040 and also cs2040c\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"Sim"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::io::output::Writable;
use algo_lib::list::List;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

struct Buffer {
    left: Vec<char>,
    right: Vec<char>,
}

impl Buffer {
    fn new() -> Self {
        Buffer {
            left: Vec::new(),
            right: Vec::new(),
        }
    }

    fn insert(&mut self, c: char) {
        self.left.push(c);
    }

    fn backspace(&mut self) {
        self.left.pop();
    }

    fn home(&mut self) {
        while let Some(c) = self.left.pop() {
            self.right.push(c);
        }
    }

    fn end(&mut self) {
        while let Some(c) = self.right.pop() {
            self.left.push(c);
        }
    }
}

impl Writable for Buffer {
    fn write(&self, output: &mut Output) {
        for c in &self.left {
            c.write(output);
        }
        for c in self.right.iter().rev() {
            c.write(output);
        }
    }
}

///
/// The problem can be done use a LinkedList where you can insert/delete in O(1),
/// but Rust does not have that yet, the Cursor functionality of lists in still `nightly`.
/// I tried to simulate the buffer, but the 'home' and 'end' actions are O(n)
/// Passes 90/100 test cases
///
fn _solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let s = input.read_line();

    let mut buffer = Buffer::new();
    for c in s.chars() {
        match c {
            '[' => buffer.home(),
            ']' => buffer.end(),
            '<' => buffer.backspace(),
            _ => buffer.insert(c),
        }
    }
    out.print_line(buffer);
}

///
/// I simulate the C++ std::list<T> here, and implemented a tiny Cursor implementation
/// to simulate the std::list<T>::iterator, `https://github.com/omar-s-ta/rust-solver/blob/main/algo_lib/src/list.rs`
/// This passes all the tests
///
fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let s = input.read_line();

    let mut list = List::new();
    let mut it = list.begin();

    for c in s.chars() {
        match c {
            '[' => it = list.begin(),
            ']' => it = list.end(),
            '<' => {
                if it != list.begin() {
                    it.dec();
                    it = list.erase(&mut it);
                }
            }
            _ => {
                list.insert(&it, c);
            }
        }
    }
    out.print_line(list);
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
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
    }
    output.flush();
    match TASK_TYPE {
        TaskType::Classic => input.is_empty(),
        TaskType::Interactive => true,
    }
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
