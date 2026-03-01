//{"name":"Stock Prices","group":"Kattis","url":"https://open.kattis.com/problems/stockprices","interactive":false,"timeLimit":5000,"tests":[{"input":"2\n6\nbuy 10 shares at 100\nsell 1 shares at 120\nsell 20 shares at 110\nbuy 30 shares at 110\nsell 10 shares at 99\nbuy 1 shares at 120\n6\nsell 10 shares at 100\nbuy 1 shares at 80\nbuy 20 shares at 90\nsell 30 shares at 90\nbuy 10 shares at 101\nsell 1 shares at 80\n","output":"- 100 -\n120 100 -\n110 100 -\n120 110 110\n120 100 99\n- 100 120\n100 - -\n100 80 -\n100 90 -\n90 80 90\n100 80 90\n100 - 80\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"StockPrices"}}}

use std::cmp::Reverse;
use std::collections::BinaryHeap;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::io::output::Writable;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

#[derive(Clone, Debug)]
struct Stock {
    ask: BinaryHeap<Reverse<(u32, u32)>>,
    bid: BinaryHeap<(u32, u32)>,
    price: Option<u32>,
}

impl Writable for Stock {
    fn write(&self, output: &mut Output) {
        if let Some(Reverse((ask, _))) = self.ask.peek() {
            ask.write(output);
        } else {
            '-'.write(output);
        }
        ' '.write(output);

        if let Some((bid, _)) = self.bid.peek() {
            bid.write(output);
        } else {
            '-'.write(output);
        }
        ' '.write(output);

        if let Some(price) = self.price {
            price.write(output);
        } else {
            '-'.write(output);
        }
    }
}

impl Stock {
    fn new() -> Self {
        Stock {
            ask: BinaryHeap::new(),
            bid: BinaryHeap::new(),
            price: None,
        }
    }

    fn new_bid(&mut self, bid: u32, amount: u32) {
        self.bid.push((bid, amount));
    }

    fn new_ask(&mut self, ask: u32, amount: u32) {
        self.ask.push(Reverse((ask, amount)));
    }

    fn establish_deal(&mut self) {
        while let (Some(bid), Some(Reverse(ask))) = (self.bid.peek(), self.ask.peek()) {
            if bid.0 < ask.0 {
                break;
            }

            let (bid, mut offered) = self.bid.pop().unwrap();
            let Reverse((ask, mut wanted)) = self.ask.pop().unwrap();

            self.price = Some(ask);

            let to_trade = offered.min(wanted);
            offered -= to_trade;
            wanted -= to_trade;

            if offered > 0 {
                self.new_bid(bid, offered);
            }
            if wanted > 0 {
                self.new_ask(ask, wanted);
            }
        }
    }
}

/// Simulation with suitable data-structure.
/// Just apply what the problem says. Use a data-structure where you can
/// get the `min` and `max` value easily.
fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_ushort();
    let mut stock = Stock::new();

    for _ in 0..n {
        let order = input.read_line();
        let mut order = order.split_whitespace();

        let action = order.next().unwrap().as_bytes();
        let amount = order.next().unwrap().parse::<u32>().unwrap();
        let value = order
            .next_back()
            .and_then(|s| s.parse::<u32>().ok())
            .unwrap();

        if action[0] == b'b' {
            stock.new_bid(value, amount);
        } else {
            stock.new_ask(value, amount);
        }
        stock.establish_deal();

        out.print_line(stock.clone());
    }
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
