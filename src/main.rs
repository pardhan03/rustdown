use clap::Parser;
use maud::{DOCTYPE, Markup, html};
use pulldown_cmark::{html, Options, Parser as MarkdownParser};
use std::{fs, path::PathBuf};

#[derive(Debug, Parser)]
struct Args {
    #[arg(long, short)]
    input: String,

    #[arg(long, short)]
    output: Option<PathBuf>,
}

fn render_html_page(content: &str) -> Markup {
    html!{
        (DOCTYPE)
        html{
            head {
                meta charset= "utf-8";
                title{ "Markdown to HTML Output"}
            }
            body {
                (maud::PreEscaped(content.to_string()))
            }
        }
    }
}

fn main() {
    let args = Args::parse();
    let markdown_input = fs::read_to_string(&args.input).expect("Fail to read the file");

    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);

    let parser = MarkdownParser::new_ext(&markdown_input, options);
    let mut html_output = String::new();

    html::push_html(&mut html_output, parser);
    let full_html_content = render_html_page(&html_output).into_string();

    match args.output {
        Some(path) => fs::write(path, full_html_content).expect("Failed to write the file"),
        None => println!("Path not provided")
    }
}
