//{"name":"Conversation Log","group":"Kattis","url":"https://open.kattis.com/problems/conversationlog","interactive":false,"timeLimit":2000,"tests":[{"input":"8\nJepson no no no no nobody never\nAshley why ever not\nMarcus no not never nobody\nBazza no never know nobody\nHatty why no nobody\nHatty nobody never know why nobody\nJepson never no nobody\nAshley never never nobody no\n","output":"no\nnobody\nnever\n"},{"input":"2\nVillain avast\nScoundrel ahoy\n","output":"ALL CLEAR\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ConversationLog"}}}

use std::collections::BTreeMap;
use std::collections::HashSet;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::Str;
use algo_lib::string::str::StrReader;

type PreCalc = ();

#[derive(Default)]
struct Value {
    count: u32,
    callers: HashSet<Str>,
}

impl Value {
    fn inc(&mut self, caller: Str) {
        self.count += 1;
        self.callers.insert(caller);
    }
}

/// Just use the words as the map keys
fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_u32();

    let mut map: BTreeMap<Str, Value> = BTreeMap::new();
    let mut set: HashSet<Str> = HashSet::new();

    for _ in 0..n {
        let line = input.read_line();
        let mut words = line.split(|c| c.is_ascii_whitespace());
        let user = Str::from(words.next().unwrap());

        set.insert(user.clone());
        for word in words {
            map.entry(word.into()).or_default().inc(user.clone());
        }
    }

    let mut used_by_all = map
        .into_iter()
        .filter_map(|(u, v)| (v.callers.len() == set.len()).then_some((u, v.count)))
        .collect::<Vec<_>>();

    used_by_all.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));

    if used_by_all.is_empty() {
        out.print_line("ALL CLEAR");
    } else {
        out.print_iter_per_line(used_by_all.into_iter().map(|u| u.0));
    }
}

pub static TEST_TYPE: TestType = TestType::Single;
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
        _ => unreachable!(),
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
