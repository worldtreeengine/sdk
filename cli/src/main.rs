mod compile;
mod package;
mod error;

use std::io::Write;
use std::path::PathBuf;
use clap::{Parser, Subcommand, crate_version};
use anyhow::{Context, Error, Result};
use serde_derive::Deserialize;
use worldtree_compiler::{Model, Text, TextNode};
use crate::package::{add_game_icons_credits};

#[derive(Debug, Parser)]
#[command(name = "worldtree")]
#[command(about = "The Worldtree CLI")]
#[command(version = crate_version!())]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(about = "Compile and package a world")]
    Build {
        context: Option<PathBuf>,
        #[arg(short, long)]
        #[arg(help = "Directory to create output files in. This directory will be created if it does not exist.")]
        out_dir: Option<PathBuf>,
        #[arg(short, long)]
        config_file: Option<PathBuf>,
        #[arg(short = 'D', long, visible_alias = "dev", action = clap::ArgAction::SetTrue)]
        #[arg(help = "Create the package in development mode")]
        development: bool,
        #[arg(short, long, action = clap::ArgAction::SetTrue)]
        #[arg(help = "Collect output files into a ZIP archive")]
        zip: bool,
        #[arg(short, long, action = clap::ArgAction::SetTrue, conflicts_with = "verbose")]
        #[arg(help = "Suppress output other than fatal errors. Conflicts with --verbose")]
        quiet: bool,
        #[arg(short, long, action = clap::ArgAction::SetTrue, conflicts_with = "quiet")]
        #[arg(help = "Show debug level output. Conflicts with --quiet")]
        verbose: bool,
    },
}

#[derive(Deserialize)]
struct PackageConfig {
    state_key: Option<String>,
    background_color: Option<String>,
    foreground_color: Option<String>,
    important_foreground_color: Option<String>,
    highlight_background_color: Option<String>,
    highlight_foreground_color: Option<String>,
    omit_bundled_stylesheet: Option<bool>,
    stylesheet: Option<PathBuf>,
    body_font_family: Option<String>,
    label_font_family: Option<String>,
}

fn to_plain(text: &Text) -> String {
    let mut result = String::new();
    for node in text {
        match node {
            TextNode::Plain(s) => result.push_str(s),
            TextNode::Paragraph(t) => {
                result.push_str(&to_plain(t));
                result.push(' ');
            },
            TextNode::Italic(t) | TextNode::Bold(t) | TextNode::Anchor(_, t) =>
                result.push_str(&to_plain(t)),
        }
    }

    if result.ends_with(' ') {
        result.truncate(result.len() - 1);
    }

    result
}

fn template(content: &Model, config: PackageConfig, google_fonts_params: String) -> Result<String> {
    return {
        let template = liquid::ParserBuilder::with_stdlib().build()?.parse(include_str!("../resources/index.html.liquid"))?;
        let title = if let Some(title) = &content.meta.title { Some(to_plain(title)) } else { None };
        let description: String = if let Some(description) = &content.meta.description {
            to_plain(description)
        } else {
            "This world does not have a description.".to_string()
        };
        let lang = "en-us";
        let generator = format!("Worldtree {}", crate_version!());

        let user_stylesheet = if let Some(stylesheet) = config.stylesheet {
            std::fs::read_to_string(&stylesheet).with_context(|| format!("Failed to load user stylesheet {:?}", &stylesheet))?
        } else {
            String::new()
        };

        let model = liquid::object!({
            "config": liquid::object!({
                "stateKey": config.state_key.unwrap_or(String::new()),
                "backgroundColor": config.background_color.unwrap_or(String::new()),
                "foregroundColor": config.foreground_color.unwrap_or(String::new()),
                "importantForegroundColor": config.important_foreground_color.unwrap_or(String::new()),
                "highlightBackgroundColor": config.highlight_background_color.unwrap_or(String::new()),
                "highlightForegroundColor": config.highlight_foreground_color.unwrap_or(String::new()),
                "omitBundledStylesheet": config.omit_bundled_stylesheet.unwrap_or(false),
                "userStylesheet": user_stylesheet,
                "bodyFontFamily": config.body_font_family.unwrap_or(String::new()),
                "labelFontFamily": config.label_font_family.unwrap_or(String::new()),
                "googleFontsParams": google_fonts_params,
            }),
            "meta": liquid::object!({
                "title": title,
                "description": description,
                "lang": lang,
                "generator": generator,
            }),
            "content": serde_json::to_string(&content)?,
            "bundle": liquid::object!({
                "script": include_str!("../../engine/standalone/browser/dist/bundle.js"),
                "stylesheet": include_str!("../../engine/standalone/browser/dist/bundle.css"),
            }),
        });
        template.render(&model)
    }.with_context(|| "Failed to generate HTML")
}

fn main() -> Result<()> {
    let args = Cli::parse();
    match args.command {
        Commands::Build { context, out_dir, config_file, development: _, zip: _, quiet: _, verbose: _} => {
            let resolved_context = match context {
                Some(path) => Ok(path),
                None => std::env::current_dir().with_context(|| "Context not provided, and current directory not accessible")
            }?;

            let mut compiled = compile::compile(&resolved_context).with_context(|| "Failed to compile world")?;

            let resolved_out_dir = match out_dir {
                Some(path) => Ok(path),
                None => std::env::current_dir().with_context(|| "Our dir not provided, and current directory not accessible").and_then(|current| Ok(current.join("dist"))),
            }?;
            if resolved_out_dir.is_file() {
                return Err(Error::msg("Out dir is not a directory"));
            }

            std::fs::create_dir_all(&resolved_out_dir).with_context(|| format!("Could not create out dir {:?}", &resolved_out_dir))?;

            // package_game_icons(&mut compiled, &resolved_out_dir).with_context(|| "Failed to download Game Icons")?;
            add_game_icons_credits(&mut compiled);

            let mut config: PackageConfig = if let Some(config_file) = &config_file {
                let config_string = std::fs::read_to_string(config_file).with_context(|| format!("Failed to read config file {:?}", config_file))?;
                toml::from_str(config_string.as_str()).with_context(|| format!("Failed to parse config file {:?}", config_file))?
            } else {
                PackageConfig {
                    background_color: None,
                    foreground_color: None,
                    important_foreground_color: None,
                    highlight_background_color: None,
                    highlight_foreground_color: None,
                    omit_bundled_stylesheet: None,
                    state_key: None,
                    stylesheet: None,
                    body_font_family: None,
                    label_font_family: None,
                }
            };

            let mut google_fonts_params = String::new();

            if let Some(body_font_family) = &config.body_font_family {
                if body_font_family.starts_with("google-fonts:") {
                    let font_name = &body_font_family[13..];
                    google_fonts_params.push_str(&format!("family={}:ital,wght@0,400;0,700;1,400;1,700", font_name.replace(" ", "+")));
                    config.body_font_family = Some(format!("'{}', serif", font_name.replace("+", " ")));
                }
            }

            if let Some(label_font_family) = &config.label_font_family {
                if label_font_family.starts_with("google-fonts:") {
                    if !google_fonts_params.is_empty() {
                        google_fonts_params.push('&');
                    }
                    let font_name = &label_font_family[13..];
                    google_fonts_params.push_str(&format!("family={}:ital,wght@0,400;0,700;1,400;1,700", font_name.replace(" ", "+")));
                    config.label_font_family = Some(format!("'{}', sans-serif", font_name.replace("+", " ")));
                }
            }

            if let Some(stylesheet) = &config.stylesheet {
                config.stylesheet = Some(config_file.unwrap().parent().unwrap().join(stylesheet));
            }

            let html_file_path = resolved_out_dir.join("index.html");
            if html_file_path.exists() {
                std::fs::remove_file(&html_file_path).with_context(|| format!("Failed to delete existing index.html {:?}", &html_file_path))?;
            }
            let html_string = template(&compiled, config, google_fonts_params).with_context(|| "Failed to generate index.html")?;
            let html_bytes = html_string.as_bytes();
            std::fs::File::create(&html_file_path)
                .with_context(|| format!("Failed to create index.html {:?}", &html_file_path))?
                .write_all(html_bytes)
                .with_context(|| "Failed to write index.html")?;
            eprintln!("{} {:.1}kb", html_file_path.display(), html_bytes.len() as f32 / 1024.0);
        },
    }
    Ok(())
}
