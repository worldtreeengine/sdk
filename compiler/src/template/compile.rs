use crate::{Attribution, Problem};
use crate::expression::{BRANCH, ExpressionCompiler, JUMP};
use crate::string_table::StringTable;
use crate::template::TemplateParseNode;

pub const STRING_PUSH: u32 = 100;
pub const ITALIC_PUSH: u32 = 101;
pub const ITALIC_POP: u32 = 102;
pub const BOLD_PUSH: u32 = 103;
pub const BOLD_POP: u32 = 104;
pub const ANCHOR_PUSH: u32 = 105;
pub const ANCHOR_POP: u32 = 106;
pub const PARAGRAPH_PUSH: u32 = 107;

#[allow(dead_code)]
pub fn compile_template(parse: Vec<TemplateParseNode>, attribution: &Attribution, string_table: &mut StringTable, problems: &mut Vec<Problem>) -> Vec<u32> {
    let mut result = Vec::new();
    let expression_compiler = ExpressionCompiler::new();

    for parse in parse {
        match parse {
            TemplateParseNode::Text(s) => {
                result.push(STRING_PUSH);
                let address = string_table.put(&s);
                result.push(address.start as u32);
                result.push(address.end as u32);
            },
            TemplateParseNode::Italic(t) => {
                result.push(ITALIC_PUSH);
                result.extend(compile_template(t, attribution, string_table, problems));
                result.push(ITALIC_POP);
            },
            TemplateParseNode::Bold(t) => {
                result.push(BOLD_PUSH);
                result.extend(compile_template(t, attribution, string_table, problems));
                result.push(BOLD_POP);
            },
            TemplateParseNode::Anchor(href, t) => {
                result.push(ANCHOR_PUSH);
                result.extend(compile_template(t, attribution, string_table, problems));
                result.push(ANCHOR_POP);
                let address = string_table.put(&href);
                result.push(address.start as u32);
                result.push(address.end as u32);
            },
            TemplateParseNode::Branch(condition, then_branch, else_branch) => {
                let expression_result = expression_compiler.compile_logical(&condition, attribution, string_table);
                problems.extend(expression_result.problems);
                result.extend(expression_result.bytecode);
                result.push(BRANCH);
                let mut then_result = compile_template(then_branch, attribution, string_table, problems);
                if let Some(else_branch) = else_branch {
                    then_result.push(JUMP);
                    let else_result = compile_template(else_branch, attribution, string_table, problems);
                    then_result.push(else_result.len() as u32);
                    result.push(then_result.len() as u32);
                    result.extend(then_result);
                    result.extend(else_result);
                } else {
                    result.push(then_result.len() as u32);
                    result.extend(then_result);
                }
            },
            TemplateParseNode::Paragraph => result.push(PARAGRAPH_PUSH),
        }
    }

    result
}
