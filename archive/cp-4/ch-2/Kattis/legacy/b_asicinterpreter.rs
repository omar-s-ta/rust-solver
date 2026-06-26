//{"name":"BASIC Interpreter","group":"Kattis","url":"https://open.kattis.com/problems/basicinterpreter","interactive":false,"timeLimit":2000,"tests":[{"input":"10 LET A = 1\n20 PRINT \"HELLO THERE \"\n30 PRINTLN A\n40 LET A = A + 1\n50 IF A <= 5 THEN GOTO 20\n60 PRINTLN \"DONE\"\n","output":"HELLO THERE 1\nHELLO THERE 2\nHELLO THERE 3\nHELLO THERE 4\nHELLO THERE 5\nDONE\n"},{"input":"40 PRINT P\n180 PRINTLN \"DONE\"\n130 PRINTLN \" IS PRIME\"\n60 LET X = D * D\n80 LET R = P / D\n100 LET R = P - R\n20 LET D = 1\n140 IF 1 = 1 THEN GOTO 180\n30 LET P = 111\n150 PRINTLN \" IS NOT PRIME\"\n170 PRINTLN \" IS A DIVISOR\"\n50 LET D = D + 1\n70 IF P < X THEN GOTO 130\n120 IF 1 = 1 THEN GOTO 50\n90 LET R = R * D\n110 IF R = 0 THEN GOTO 150\n10 PRINTLN \"PRIME TESTER\"\n160 PRINT D\n","output":"PRIME TESTER\n111 IS NOT PRIME\n3 IS A DIVISOR\nDONE\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BASICInterpreter"}}}

use std::collections::HashMap;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type Variables = [i32; 26];

enum ArithmeticOp {
    Plus,
    Minus,
    Multiply,
    Divide,
}

impl ArithmeticOp {
    fn apply(&self, lhs: i32, rhs: i32) -> i32 {
        match self {
            ArithmeticOp::Plus => lhs + rhs,
            ArithmeticOp::Minus => lhs - rhs,
            ArithmeticOp::Multiply => lhs * rhs,
            ArithmeticOp::Divide => lhs / rhs,
        }
    }
}

impl From<&str> for ArithmeticOp {
    fn from(value: &str) -> Self {
        match value {
            "+" => ArithmeticOp::Plus,
            "-" => ArithmeticOp::Minus,
            "*" => ArithmeticOp::Multiply,
            "/" => ArithmeticOp::Divide,
            _ => unreachable!(),
        }
    }
}

enum ConditionOp {
    Eq,
    Lt,
    Gt,
    Lte,
    Gte,
    Neq,
}

impl ConditionOp {
    fn apply(&self, lhs: i32, rhs: i32) -> bool {
        match self {
            ConditionOp::Eq => lhs == rhs,
            ConditionOp::Lt => lhs < rhs,
            ConditionOp::Gt => lhs > rhs,
            ConditionOp::Lte => lhs <= rhs,
            ConditionOp::Gte => lhs >= rhs,
            ConditionOp::Neq => lhs != rhs,
        }
    }
}

impl From<&str> for ConditionOp {
    fn from(value: &str) -> Self {
        match value {
            "=" => ConditionOp::Eq,
            ">" => ConditionOp::Gt,
            "<" => ConditionOp::Lt,
            "<>" => ConditionOp::Neq,
            "<=" => ConditionOp::Lte,
            ">=" => ConditionOp::Gte,
            _ => unreachable!(),
        }
    }
}

enum Operand {
    Variable(u8),
    Value(i32),
}

impl Operand {
    fn eval(&self, vars: &Variables) -> i32 {
        match self {
            Operand::Variable(name) => vars[*name as usize],
            Operand::Value(val) => *val,
        }
    }
}

enum PrintArg {
    Operand(Operand),
    Literal(String),
}

enum Statement {
    Let(u8, Operand, Option<(ArithmeticOp, Operand)>),
    If(Operand, ConditionOp, Operand, usize),
    Print(PrintArg, bool),
}

impl Statement {
    fn apply(&self, out: &mut Output, vars: &mut Variables, next_stmt: usize) -> usize {
        match self {
            Statement::Let(var, lhs, rhs) => {
                let value = match rhs {
                    Some((op, rhs)) => op.apply(lhs.eval(vars), rhs.eval(vars)),
                    None => lhs.eval(vars),
                };
                vars[*var as usize] = value;
                next_stmt
            }
            Statement::If(lhs, op, rhs, target) => {
                if op.apply(lhs.eval(vars), rhs.eval(vars)) {
                    *target
                } else {
                    next_stmt
                }
            }
            Statement::Print(arg, new_line) => {
                match arg {
                    PrintArg::Operand(op) => out.print(op.eval(vars)),
                    PrintArg::Literal(s) => out.print(s),
                }
                if *new_line {
                    out.print_line(());
                }
                next_stmt
            }
        }
    }
}

struct RawStatement<'a, I>
where
    I: Iterator<Item = &'a str> + Clone,
{
    tokens: I,
}

impl<'a, I> RawStatement<'a, I>
where
    I: Iterator<Item = &'a str> + Clone,
{
    fn label(&mut self) -> usize {
        self.next().unwrap().parse().unwrap()
    }

    fn statement(&mut self) -> Statement {
        let token = self.next().unwrap();
        match token {
            "LET" => self.let_statement(),
            "IF" => self.if_statement(),
            "PRINT" => self.print_statement(false),
            "PRINTLN" => self.print_statement(true),
            _ => unreachable!(),
        }
    }

    fn let_statement(&mut self) -> Statement {
        let var = self.next().unwrap().as_bytes()[0] - b'A';
        let _equal = self.next();
        let lhs = self.operand();
        let rhs = if self.has_arithmetic_op() {
            let op = self.next().unwrap().into();
            Some((op, self.operand()))
        } else {
            None
        };
        Statement::Let(var, lhs, rhs)
    }

    fn if_statement(&mut self) -> Statement {
        let lhs = self.operand();
        let op = self.next().unwrap().into();
        let rhs = self.operand();
        let _then = self.next();
        let _goto = self.next();
        Statement::If(lhs, op, rhs, self.label())
    }

    fn print_statement(&mut self, new_line: bool) -> Statement {
        let arg = if self.has_string() {
            let string = self.next().unwrap().replace('"', "").replace('_', " ");
            PrintArg::Literal(string)
        } else {
            PrintArg::Operand(self.operand())
        };
        Statement::Print(arg, new_line)
    }

    fn operand(&mut self) -> Operand {
        let raw = self.next().unwrap();
        let bytes = raw.as_bytes();
        if bytes[0].is_ascii_alphabetic() {
            Operand::Variable(bytes[0] - b'A')
        } else {
            Operand::Value(raw.parse().unwrap())
        }
    }

    fn has_arithmetic_op(&self) -> bool {
        self.tokens
            .clone()
            .any(|s| s == "+" || s == "-" || s == "*" || s == "/")
    }

    fn has_string(&self) -> bool {
        self.tokens.clone().any(|s| s.contains('"'))
    }

    fn next(&mut self) -> Option<&'a str> {
        self.tokens.next()
    }
}

/// Build a small Abstract Syntax Tree (AST)
/// Then map each label to an AST
/// sort by labels, and apply the next AST statment
fn solve(input: &mut Input, out: &mut Output, _test_case: usize, vars: &mut Variables) {
    let mut entries = Vec::new();

    while input.peek().is_some() {
        let mut statement = input.read_line().to_string();
        if let (Some(s), Some(e)) = (statement.find('"'), statement.rfind('"'))
            && statement[s + 1..e].contains(' ')
        {
            let without = statement[s + 1..e].replace(' ', "_");
            statement.replace_range(s + 1..e, &without);
        }

        let tokens = statement.split_whitespace();
        let mut raw = RawStatement { tokens };
        entries.push((raw.label(), raw.statement()));
    }

    entries.sort_by_key(|&(l, _)| l);
    let label_index = entries
        .iter()
        .enumerate()
        .map(|(i, &(label, _))| (label, i))
        .collect::<HashMap<_, _>>();

    let mut program = entries
        .into_iter()
        .map(|(_, stmt)| stmt)
        .collect::<Vec<_>>();

    for stmt in &mut program {
        if let Statement::If(_, _, _, label) = stmt {
            *label = label_index[label];
        }
    }

    let mut at = 0;
    while at < program.len() {
        at = program[at].apply(out, vars, at + 1);
    }
}

pub static TEST_TYPE: TestType = TestType::Single;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut variables = [0; 26];

    match TEST_TYPE {
        TestType::Single => solve(&mut input, &mut output, 1, &mut variables),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 1..=t {
                solve(&mut input, &mut output, i, &mut variables);
            }
        }
        TestType::MultiEof => {
            let mut i = 1;
            while input.peek().is_some() {
                solve(&mut input, &mut output, i, &mut variables);
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
