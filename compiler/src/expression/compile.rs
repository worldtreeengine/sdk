use crate::expression::parser::{ExpressionParse};
use crate::{Attribution, Problem};
use crate::expression::token::{ExpressionAtom, ExpressionOperator};
use crate::string_table::StringTable;

pub const PUSH: u32 = 0;
pub const NOT: u32 = 1;
pub const AND: u32 = 2;
pub const OR: u32 = 3;
pub const ADD: u32 = 4;
pub const SUBTRACT: u32 = 5;
pub const MULTIPLY: u32 = 6;
pub const DIVIDE: u32 = 7;
pub const BRANCH: u32 = 8;
pub const JUMP: u32 = 9;
pub const PUSH_VALUE_OF: u32 = 11;
pub const IN_LOCATION: u32 = 12;
pub const EQ: u32 = 14;
pub const NEQ: u32 = 15;
pub const GT: u32 = 16;
pub const GTE: u32 = 17;
pub const LT: u32 = 18;
pub const LTE: u32 = 19;
pub const MAX: u32 = 20;
pub const MIN: u32 = 21;
pub const RAND: u32 = 22;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum TypeHint {
    Logical,
    Numeric,
}

pub struct ExpressionCompiler {}

#[derive(Debug)]
pub struct ExpressionCompilationResult {
    pub bytecode: Vec<u32>,
    pub problems: Vec<Problem>,
}

struct Compile<'a> {
    attribution: &'a Attribution,
    strings: &'a mut StringTable,
    problems: Vec<Problem>,
}

#[allow(dead_code)]
impl<'a> ExpressionCompiler {
    pub fn new() -> Self {
        ExpressionCompiler {}
    }

    pub fn compile_logical(&self, parse: &ExpressionParse, attribution: &Attribution, strings: &mut StringTable) -> ExpressionCompilationResult {
        Compile::new(attribution, strings).compile_logical(parse)
    }

    pub fn compile_numeric(&self, parse: &ExpressionParse, attribution: &Attribution, strings: &mut StringTable) -> ExpressionCompilationResult {
        Compile::new(attribution, strings).compile_numeric(parse)
    }
}

impl<'a> Compile<'a> {
    fn new(attribution: &'a Attribution, strings: &'a mut StringTable) -> Self {
        Self {
            attribution,
            strings,
            problems: Vec::new(),
        }
    }

    fn compile_logical(mut self, parse: &ExpressionParse) -> ExpressionCompilationResult {
        ExpressionCompilationResult {
            bytecode: self.compile_inner(parse, TypeHint::Logical),
            problems: self.problems,
        }
    }

    fn compile_numeric(mut self, parse: &ExpressionParse) -> ExpressionCompilationResult {
        ExpressionCompilationResult {
            bytecode: self.compile_inner(parse, TypeHint::Numeric),
            problems: self.problems,
        }
    }

    fn compile_inner(&mut self, parse: &ExpressionParse, type_hint: TypeHint) -> Vec<u32> {
        return match parse {
            ExpressionParse::Atom(atom) => {
                match atom {
                    ExpressionAtom::NumericLiteral(n) => vec!(PUSH, *n),
                    ExpressionAtom::LogicalLiteral(b) => if *b { vec!(PUSH, 1) } else { vec!(PUSH, 0) },
                    ExpressionAtom::Reference(s) => {
                        let address = self.strings.put(s);
                        vec!(PUSH_VALUE_OF, address.start as u32, address.end as u32)
                    },
                }
            },
            ExpressionParse::Operation(operator, operands) => {
                match operator {
                    ExpressionOperator::Not => {
                        let mut result = Vec::new();
                        let operands_len = operands.len();
                        for operand in operands {
                            result.extend(self.compile_inner(operand, TypeHint::Logical));
                            result.push(NOT)
                        }
                        for _ in 1..operands_len {
                            result.push(AND)
                        }
                        result
                    },
                    ExpressionOperator::And => {
                        let mut result = Vec::new();
                        let operands_len = operands.len();
                        for operand in operands {
                            result.extend(self.compile_inner(operand, TypeHint::Logical));
                        }
                        for _ in 1..operands_len {
                            result.push(AND)
                        }
                        result
                    },
                    ExpressionOperator::Or => {
                        let mut result = Vec::new();
                        let operands_len = operands.len();
                        for operand in operands {
                            result.extend(self.compile_inner(operand, TypeHint::Logical));
                        }
                        for _ in 1..operands_len {
                            result.push(OR)
                        }
                        result
                    },
                    ExpressionOperator::Plus => {
                        let mut result = Vec::new();
                        let operands_len = operands.len();
                        for operand in operands {
                            result.extend(self.compile_inner(operand, TypeHint::Numeric));
                        }
                        for _ in 1..operands_len {
                            result.push(ADD)
                        }
                        result
                    },
                    ExpressionOperator::Multiply => {
                        let mut result = Vec::new();
                        let operands_len = operands.len();
                        for operand in operands {
                            result.extend(self.compile_inner(operand, TypeHint::Numeric));
                        }
                        for _ in 1..operands_len {
                            result.push(MULTIPLY)
                        }
                        result
                    },
                    ExpressionOperator::Minus => {
                        let mut result = Vec::new();
                        let operands_len = operands.len();
                        for operand in operands.iter().rev() {
                            result.extend(self.compile_inner(operand, TypeHint::Numeric));
                        }
                        for _ in 1..operands_len {
                            result.push(SUBTRACT);
                        }
                        result
                    },
                    ExpressionOperator::Divide => {
                        let mut result = Vec::new();
                        let operands_len = operands.len();
                        for operand in operands.iter().rev() {
                            result.extend(self.compile_inner(operand, TypeHint::Numeric));
                        }
                        for _ in 1..operands_len {
                            result.push(DIVIDE);
                        }
                        result
                    },
                    ExpressionOperator::In => {
                        let mut result = Vec::new();
                        let operands_len = operands.len();
                        for operand in operands {
                            if let ExpressionParse::Atom(ExpressionAtom::Reference(s)) = operand {
                                let address = self.strings.put(s);
                                result.push(IN_LOCATION);
                                result.push(address.start as u32);
                                result.push(address.end as u32);
                            } else {
                                self.problems.push(Problem::fatal("Expected a location reference", self.attribution));
                            }
                        }
                        for _ in 1..operands_len {
                            result.push(OR);
                        }
                        result
                    },
                    ExpressionOperator::Then => {
                        if operands.len() == 3 {
                            let mut operand_iter = operands.iter();
                            let condition = self.compile_inner(operand_iter.next().unwrap(), TypeHint::Logical);
                            let branch = self.compile_inner(operand_iter.next().unwrap(), type_hint);
                            let fallback = self.compile_inner(operand_iter.next().unwrap(), type_hint);
                            let mut result = Vec::new();
                            result.extend(condition);
                            result.push(BRANCH);
                            result.push((branch.len() + 2) as u32);
                            result.extend(branch);
                            result.push(JUMP);
                            result.push((fallback.len() + 2) as u32);
                            result.extend(fallback);
                            result
                        } else {
                            self.problems.push(Problem::fatal("Expected 3 expressions", self.attribution));
                            vec!(PUSH, 0)
                        }
                    },
                    ExpressionOperator::Maximum => {
                        let mut result = Vec::new();
                        let operands_len = operands.len();
                        for operand in operands {
                            result.extend(self.compile_inner(operand, TypeHint::Numeric));
                        }
                        for _ in 1..operands_len {
                            result.push(MAX);
                        }
                        result
                    },
                    ExpressionOperator::Minimum => {
                        let mut result = Vec::new();
                        let operands_len = operands.len();
                        for operand in operands {
                            result.extend(self.compile_inner(operand, TypeHint::Numeric));
                        }
                        for _ in 1..operands_len {
                            result.push(MIN);
                        }
                        result
                    },
                    ExpressionOperator::Random => {
                        let mut result = Vec::new();
                        let operands_len = operands.len();
                        for operand in operands {
                            result.extend(self.compile_inner(operand, TypeHint::Numeric));
                        }
                        result.push(RAND);
                        result.push(operands_len as u32);
                        result
                    },
                    ExpressionOperator::Equal => {
                        let mut result = Vec::new();
                        let operands_len = operands.len();
                        for operand in operands {
                            result.extend(self.compile_inner(operand, TypeHint::Numeric));
                        }
                        for _ in 0..operands_len {
                            result.push(EQ);
                        }
                        result
                    },
                    ExpressionOperator::NotEqual => {
                        let mut result = Vec::new();
                        let operands_len = operands.len();
                        for operand in operands {
                            result.extend(self.compile_inner(operand, TypeHint::Numeric));
                        }
                        for _ in 0..operands_len {
                            result.push(NEQ);
                        }
                        result
                    },
                    ExpressionOperator::GreaterThan => {
                        let mut result = Vec::new();
                        let operands_len = operands.len();
                        for operand in operands.iter().rev() {
                            result.extend(self.compile_inner(operand, TypeHint::Numeric));
                        }
                        for _ in 0..operands_len {
                            result.push(GT);
                        }
                        result
                    },
                    ExpressionOperator::GreaterThanOrEqual => {
                        let mut result = Vec::new();
                        let operands_len = operands.len();
                        for operand in operands.iter().rev() {
                            result.extend(self.compile_inner(operand, TypeHint::Numeric));
                        }
                        for _ in 0..operands_len {
                            result.push(GTE);
                        }
                        result
                    },
                    ExpressionOperator::LessThan => {
                        let mut result = Vec::new();
                        let operands_len = operands.len();
                        for operand in operands.iter().rev() {
                            result.extend(self.compile_inner(operand, TypeHint::Numeric));
                        }
                        for _ in 0..operands_len {
                            result.push(LT);
                        }
                        result
                    },
                    ExpressionOperator::LessThanOrEqual => {
                        let mut result = Vec::new();
                        let operands_len = operands.len();
                        for operand in operands.iter().rev() {
                            result.extend(self.compile_inner(operand, TypeHint::Numeric));
                        }
                        for _ in 0..operands_len {
                            result.push(LTE);
                        }
                        result
                    },
                    _ => {
                        self.problems.push(Problem::fatal("Unimplemented operation", self.attribution));
                        vec!(PUSH, 0)
                    },
                }
            }
        }
    }
}
