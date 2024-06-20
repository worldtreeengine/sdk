mod source;

use std::process::exit;
use anyhow::{Context, Error, Result};
use worldtree_compiler::Model;

pub fn compile(context: &std::path::PathBuf) -> Result<Model> {
    let sources = source::gather_sources(context).with_context(|| "Failed to gather sources")?;
    let (model, problems) = match worldtree_compiler::compile(&sources) {
        Ok(result) => (result.model, result.problems),
        Err(e) => return Err(Error::msg(format!("Compilation failed: {}", e))),
    };

    if !problems.is_empty() {
        for problem in problems {
            eprintln!("{}", problem.message);
            eprintln!("    in {}", problem.attribution.source);
            eprintln!("    at {}", problem.attribution.path);
            eprintln!("        line {}, column {}", problem.attribution.start_mark.line + 1, problem.attribution.start_mark.column + 1);
            if let Some(context) = problem.context {
                eprintln!("{}", context.message);
                eprintln!("    in {}", context.attribution.source);
                eprintln!("    at {}", context.attribution.path);
                eprintln!("        line {}, column {}", context.attribution.start_mark.line + 1, context.attribution.start_mark.column + 1);
            }
        }
        exit(1);
    } else {
        Ok(model)
    }
}
