use algo_lib::collections::btree_ext::BoundedLookup;
use algo_lib::collections::multi_tree_set::MultiTreeSet;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_u32();

    // normal std BtreeSet
    // let mut set = BTreeSet::new();

    let mut set = MultiTreeSet::new();
    for _ in 0..n {
        let cmd = input.read_str();
        if cmd[0] == b'a' {
            set.insert((input.read_u32(), input.read_u32()));
        } else {
            let mut x = input.read_u32();
            let mut gold = 0_u64;
            while let Some((&(e, g), _)) = set.prev_inclusive(&(x, u32::MAX)) {
                gold += g as u64;
                x -= e;
                set.remove(&(e, g));
            }
            // normal std BtreeSet
            // loop {
            //     let next = set
            //         .range((Reverse(x), Reverse(u32::MAX), 0)..)
            //         .next()
            //         .copied();
            //
            //     match next {
            //         Some(elem) => {
            //             let (Reverse(e), Reverse(g), _) = elem;
            //             if let Some(r) = x.checked_sub(e) {
            //                 gold += g as u64;
            //                 x = r;
            //                 set.remove(&elem);
            //             } else {
            //                 break;
            //             }
            //         }
            //         None => break,
            //     }
            // }
            out.print_line(gold);
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
