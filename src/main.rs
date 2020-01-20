use clap::{App, Arg};
use std::process::{Command, Stdio};
use colored::*;

mod cmd_generator;

use cmd_generator::base::{Cmd, CmdType};

const BASE_PATH: &str = "java/com/google/social/boq/blogger";

fn main() {
    ///////////// CLAP START //////////////////////////
    let matches = App::new("rick")
        .version("0.1v")
        .about("Doing ricky things.")
        .author("Gaurav Tyagi")
        .arg(
            Arg::with_name("dir")
                .short("d")
                .long("directory")
                .takes_value(true)
                .required(true)
                .value_name("DIR, ex: [settingsweb|postweb/postsettings]"),
        )
        .arg(Arg::with_name("explain").short("e").long("explain"))
        .arg(
            Arg::with_name("run")
                .short("r")
                .long("run")
                .takes_value(true)
                .value_name("RUN, ex: [jswire,debug|build]"),
        )
        .arg(
            Arg::with_name("iblaze")
                .short("i")
                .long("iblaze")
                .value_name("Enable debug mode for jswire and jstest"),
        )
        .arg(
            Arg::with_name("command")
                .short("cmd")
                .long("command")
                .takes_value(true)
                .value_name("Command, give command to execute"),
        )
        .get_matches();
    ///////////// END ////

    let dir = matches.value_of("dir").unwrap();
    let run_for = matches.value_of("run").unwrap_or("all");
    let cmd = Cmd::new(String::from(BASE_PATH), String::from(dir), true);
    let mut generated_cmds: Vec<String> = Vec::new();
    
    for run in run_for.split(",") {
        if run == "jswire" || run == "all" {
            generated_cmds.push(cmd.create_build_cmd(CmdType::Jswire));
        }
        if run == "build" || run == "all" {
            generated_cmds.push(cmd.create_build_cmd(CmdType::Build));
        }
        if run == "jstest" || run == "all" {
            generated_cmds.push(cmd.create_build_cmd(CmdType::JsTest));
        }
    }
    // run a custom command only.
    if matches.is_present("command") {
        generated_cmds.clear();
        let mut command_list = matches.value_of("command").unwrap().split(",");
        while let Some(x) = command_list.next() {
            generated_cmds.push(x.to_string());
        }
    }
    if matches.is_present("explain") {
        explain(generated_cmds);
        return;
    }
    run_commands(generated_cmds);
}

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

fn explain(cmds: Vec<String>) {
    for cmd in cmds {
        println!(">>{}", cmd.yellow());
    }
}

