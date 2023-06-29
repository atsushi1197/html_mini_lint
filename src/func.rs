extern crate regex;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use regex::Regex;

pub struct Config {
    // query: String,
    filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.is_empty() {
            return Err("not enough arguments");
        }
        let filename = args[1].clone();

        Ok(Config { filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut f = File::open(config.filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    has_no_lint_error(&contents);

    // println!("{}", &contents);


    Ok(())
}

pub fn has_no_lint_error(contents: &str) -> bool {
    let regex = Regex::new(r"<+[|/]+*([a-z])+>").unwrap();
    let mut open_tags: Vec<&str> = Vec::new();
    let mut line_nums: Vec<i32> = Vec::new();
    let mut line_num = 1;
    for line in contents.lines() {
        // let match_result = regex.captures(line).unwrap();
        // println!("{}", &match_result[0]);
        for target in regex.find_iter(line) {
            if !is_html_void_tag(target.as_str()) {
                if is_html_open_tag(target.as_str()) {
                    open_tags.push(target.as_str());
                    line_nums.push(line_num);
                } else if is_html_close_tag(target.as_str()) && open_tags.last().unwrap() == &parse_close_to_open(target.as_str()) {
                    open_tags.pop();
                    line_nums.pop();
                }
            }
        }
        // println!("{}", line_num);
        line_num += 1;
    }
    println!("{}行目付近の{}に閉じタグがないようです。", line_nums.last().unwrap() ,open_tags.last().unwrap());
    true
}

pub fn is_html_open_tag(target: &str) -> bool {
    let html_open_tag: Vec<&str> = vec![
        "<html>", "<head>", "<body>", "<title>", "<div>", "<script>",
        "<p>", "<a>", "<b>", "<big>", "<small>", "<ul>", "<li>",
        "<dl>", "<dt>", "<dd>", "<table>", "<tr>", "<th>", "<td>", "<form>",
        "<select>", "<option>", "<form>", "<textarea>",
    ];
    return html_open_tag.iter().any(|&item| item == target);
}

pub fn is_html_close_tag(target: &str) -> bool {
    let html_close_tag: Vec<&str> = vec![
        "</html>", "</head>", "</body>", "</title>", "</div>", "</script>",
        "</p>", "</a>", "</b>", "</big>", "</small>", "</ul>", "</li>",
        "</dl>", "</dt>", "</dd>", "</table>", "</tr>", "</th>", "</td>", "</form>",
        "</select>", "</option>", "</form>", "</textarea>",
    ];
    return html_close_tag.iter().any(|&item| item == target);
}

pub fn is_html_void_tag(target: &str) -> bool {
    let html_void_tag: Vec<&str> = vec![
        "<br>", "<hr>", "<img>", "<input>", "<link>", "<meta>"
    ];
    return html_void_tag.iter().any(|&item| item == target);
}

pub fn parse_close_to_open(target: &str) -> String {
    let mut open_tag = String::new();
    for word in target.split('/') {
        open_tag += word;
    }
    return open_tag;
}