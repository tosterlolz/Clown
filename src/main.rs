use colored::*;
use serde::Deserialize;
use std::{collections::{HashMap, HashSet}, env, fs, process::Command};

#[derive(Debug, Deserialize, Default)]
struct Target {
    desc: Option<String>,
    deps: Option<Vec<String>>,
    steps: Option<Vec<String>>,
}

type Vars = HashMap<String, String>;
type ClownConfig = HashMap<String, Target>;

fn replace_vars(s: &str, vars: &Vars) -> String {
    let mut out = s.to_string();
    for (k, v) in vars {
        let pat = format!("${{{}}}", k);
        out = out.replace(&pat, v);
    }
    out
}

fn run_target(
    target: &str,
    config: &ClownConfig,
    vars: &Vars,
    visited: &mut HashSet<String>,
    stack: &mut Vec<String>
) -> Result<(), String> {
    if !config.contains_key(target) {
        return Err(format!("Target '{}' not found", target));
    }
    if visited.contains(target) {
        return Ok(()); // already built
    }
    if stack.contains(&target.to_string()) {
        return Err(format!("Dependency cycle detected: {}", stack.join(" -> ")));
    }
    stack.push(target.to_string());
    let section = &config[target];
    if let Some(deps) = &section.deps {
        for dep in deps {
            run_target(dep, config, vars, visited, stack)?;
        }
    }
    if let Some(steps) = &section.steps {
        for step in steps {
            let cmd = replace_vars(step, vars);
            println!("{}", format!("$ {}", cmd).cyan().bold());
            let mut parts = cmd.split_whitespace();
            if let Some(cmd_name) = parts.next() {
                let args: Vec<&str> = parts.collect();
                let status = Command::new(cmd_name)
                    .args(&args)
                    .status()
                    .map_err(|e| format!("{}: {}", "Failed to execute".red(), e))?;
                if !status.success() {
                    return Err(format!(
                        "{} {}",
                        "Command failed:".red().bold(),
                        cmd
                    ));
                }
            }
        }
    }
    visited.insert(target.to_string());
    stack.pop();
    Ok(())
}

fn parse_vars(config: &toml::Value) -> Vars {
    config
        .get("vars")
        .and_then(|v| v.as_table())
        .map(|tbl| {
            tbl.iter()
                .filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_string())))
                .collect()
        })
        .unwrap_or_default()
}

fn parse_sections(config: &toml::Value) -> ClownConfig {
    let mut map = ClownConfig::new();
    if let Some(tbl) = config.as_table() {
        for (k, v) in tbl {
            if k == "vars" {
                continue;
            }
            if let Ok(section) = toml::from_str::<Target>(&toml::to_string(v).unwrap()) {
                map.insert(k.clone(), section);
            }
        }
    }
    map
}

fn print_targets(config: &ClownConfig) {
    println!("{}", "Available targets:".yellow().bold());
    for (key, val) in config {
        if let Some(desc) = &val.desc {
            println!("  {:10} - {}", key.green(), desc);
        } else {
            println!("  {}", key.green());
        }
    }
}

fn print_help() {
    println!(
        "{}",
        r#"
Clown: Simple build tool with TOML targets, deps, and colored output

Usage:
  clown [target]         Run target (default: all)
  clown --list           Show available targets & descriptions
  clown --help           Show this help

Clownfile example:
[vars]
profile = "release"

[all]
desc = "Build and run everything"
deps = ["build", "run"]

[build]
desc = "Build the project"
steps = [
    "cargo build --${profile}",
    "echo Build done!"
]

[run]
desc = "Run the project"
steps = ["cargo run --${profile}"]

[clean]
desc = "Clean the project"
steps = ["cargo clean"]

[test]
desc = "Run tests"
deps = ["build"]
steps = ["cargo test"]
"#
        .blue()
    );
}

fn main() {
    let clownfile = "Clownfile";
    let content = match fs::read_to_string(clownfile) {
        Ok(c) => c,
        Err(_) => {
            eprintln!("{}", format!("Could not read {}", clownfile).red().bold());
            std::process::exit(1);
        }
    };
    let config_val = match content.parse::<toml::Value>() {
        Ok(val) => val,
        Err(e) => {
            eprintln!("{}", format!("TOML parse error: {}", e).red().bold());
            std::process::exit(1);
        }
    };
    let vars = parse_vars(&config_val);
    let config = parse_sections(&config_val);

    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        match args[1].as_str() {
            "--list" | "-l" => {
                print_targets(&config);
                return;
            }
            "--help" | "-h" => {
                print_help();
                return;
            }
            target => {
                let mut visited = HashSet::new();
                let mut stack = Vec::new();
                match run_target(target, &config, &vars, &mut visited, &mut stack) {
                    Ok(_) => println!("{}", "Done!".green().bold()),
                    Err(e) => {
                        eprintln!("{}", e);
                        std::process::exit(1);
                    }
                }
            }
        }
    } else {
        let mut visited = HashSet::new();
        let mut stack = Vec::new();
        match run_target("all", &config, &vars, &mut visited, &mut stack) {
            Ok(_) => println!("{}", "Done!".green().bold()),
            Err(e) => {
                eprintln!("{}", e);
                std::process::exit(1);
            }
        }
    }
}