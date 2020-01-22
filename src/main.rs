use clap::{App, Arg};
use std::process::{Command, Stdio};
use colored::*;

mod cmd_generator;

use cmd_generator::base::{Cmd, CmdType};

// const Base path to the application.
const BASE_PATH: &str = "java/com/google/social/boq/blogger";

// Main function to start Application CLI.
fn main() {
    // Define what this CLI app required.
    let matches = App::new("rick")
        .version("0.1v")
        .about("Vaba laba dub dub.")
        .author("Gaurav Tyagi")
        .arg(
            Arg::with_name("dir")
                .short("d")
                .long("directory")
                .takes_value(true)
                .help("Set a dir to run."),
            )
        .arg(
            Arg::with_name("explain")
                .short("e")
                .long("explain")
                .help("Explain what command it going to run."),
            )
        .arg(
            Arg::with_name("run")
                .short("r")
                .long("run")
                .takes_value(true)
                .value_name("j: jswire, d: debug, t: testcases")
                .help("run define test cases you want to execute, ex: -r=j,d")
        )
        .arg(
            Arg::with_name("iblaze")
                .short("i")
                .long("iblaze")
                .value_name("Not Implemented :( WIP..."),
        )
        .arg(
            Arg::with_name("command")
                .short("cmd")
                .long("command")
                .takes_value(true)
                .value_name("give a command to execute")
                .help("Run your custom command"),
        )
        .get_matches();

    let dir = matches.value_of("dir").unwrap();
    let run_for = matches.value_of("run").unwrap_or("all");
    let cmd = Cmd::new(String::from(BASE_PATH), String::from(dir), true);
    let mut generated_cmds: Vec<String> = Vec::new();
    // Push commands obj to Vec.
    for run in run_for.split(",") {
        if run == "j" || run == "all" {
            generated_cmds.push(cmd.create_build_cmd(CmdType::Jswire));
        }
        if run == "b" || run == "all" {
            generated_cmds.push(cmd.create_build_cmd(CmdType::Build));
        }
        if run == "t" || run == "all" {
            generated_cmds.push(cmd.create_build_cmd(CmdType::JsTest));
        }
    }

    // Check/Run a custom command.
    if matches.is_present("command") {
        generated_cmds.clear();
        let mut command_list = matches.value_of("command").unwrap().split(",");
        while let Some(x) = command_list.next() {
            generated_cmds.push(x.to_string());
        }
    }
    // Print and exit.
    if matches.is_present("explain") {
        explain(generated_cmds);
        return;
    }
    run_commands(generated_cmds);
}

// Run a command as process and prints its stdio.
fn run_commands(cmds: Vec<String>) {
    for cmd in cmds {

        let mut cmd_array: Vec<&str> = cmd.split(" ").collect();
        let mut output = Command::new(cmd_array[0])
            .args(&mut cmd_array.split_off(1))
            .stdout(Stdio::inherit())
            .spawn()
            .unwrap();
        let result = output.wait();

        println!("Command: {} exit with: {}", cmd.green(), result.unwrap().to_string().green());
    }
}

// Prints all the commands from Vec.
fn explain(cmds: Vec<String>) {
    for cmd in cmds {
        println!(">>{}", cmd.yellow());
    }
}

