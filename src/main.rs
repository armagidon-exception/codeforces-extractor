mod parser;

use std::{fs, path::Path};
use regex::Regex;
use xdg;

use clap::Parser;
use reqwest;
use parser::{parse_content, Problem};

#[derive(Debug, Parser)]
struct Args {
    /* /// Contest Name
    contest_name: String, */

    /// Save Path
    #[arg(short, long)]
    save_path: std::path::PathBuf,
    problem_id: String,
}


/* async fn get_content(contest_name: &str) -> String {
    let client = reqwest::Client::new();
    let url = format!("https://codeforces.com/contest/{}/problems", contest_name);
    let res = client
        .get(&url)
        .header("User-Agent", "Mozilla/5.0") // :D
        .send()
        .await
        .unwrap();
    res.text().await.unwrap()
} */

async fn get_problem_content(problem_id: &str) -> String {
    let client = reqwest::Client::new();
    let re = Regex::new(r"^([0-9]+)([A-Z]+)$").unwrap();
    assert!(re.is_match(problem_id));
    let caps = re.captures(problem_id).unwrap();

    let url = format!("https://codeforces.com/problemset/problem/{}/{}", &caps[1], &caps[2]);
    let res = client.get(&url).header("User-Agent", "Mozilla/5.0").send().await.unwrap();
    res.text().await.unwrap()
}

/// Save given `problem` to the given directory `save_path`
fn save_problem(problem: &Problem, save_path: &std::path::Path) {
    let problem_label = problem.name.to_string();
    let problem_path = save_path.join(problem_label);

    fs::create_dir_all(&problem_path).unwrap();

    for (i, input) in problem.inputs.iter().enumerate() {
        let input_path = problem_path.join(format!("{}.in", i));
        std::fs::write(input_path, input).unwrap();
    }
    for (i, output) in problem.outputs.iter().enumerate() {
        let output_path = problem_path.join(format!("{}.out", i));
        std::fs::write(output_path, output).unwrap();
    }
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let xdg_dirs = xdg::BaseDirectories::with_prefix("crookforce");
    let template_dir = xdg_dirs.create_config_directory("template").expect("could not create configuration directory");


    // let content = get_content(&args.contest_name).await;
    let content = get_problem_content(&args.problem_id).await;
    let problems: Vec<Problem> = parse_content(&content);

    save_problem(&problems[0], &args.save_path);

    for p in fs::read_dir(template_dir).unwrap() {
        let path = p.unwrap().path();
        let fname = Path::new(path.file_name().unwrap());
        let new_file = args.save_path.join(&problems[0].name).join(fname);
        let _ = std::fs::copy(&path, &new_file);
        println!("Copied {} to {}", path.display(), new_file.display())
    }
}

/* #[tokio::test]
async fn test_parser() {
    let content = get_content("1790").await;
    let problems: Vec<Problem> = parse_content(&content);
    assert_eq!(problems.iter().map(|p: &Problem| p.name.clone()).collect::<Vec<String>>(), vec![
        "A. Polycarp and the Day of Pi".to_string(),
        "B. Taisia and Dice".to_string(),
        "C. Premutation".to_string(),
        "D. Matryoshkas".to_string(),
        "E. Vlad and a Pair of Numbers".to_string(),
        "F. Timofey and Black-White Tree".to_string(),
        "G. Tokens on Graph".to_string(),
    ]);
} */
