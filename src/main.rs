//extern crate clap;
use clap::{App, Arg};
use std::process::{Command, Stdio};
use colored::*;

mod cmd_generator;

use cmd_generator::base::{Cmd, CmdType};

const BASE_PATH: &str = "boq/blogger";

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
        .get_matches();
    ///////////// END ////

    let dir = matches.value_of("dir").unwrap();
    let run_for = matches.value_of("run").unwrap_or("all");
    let cmd = Cmd::new(String::from(BASE_PATH), String::from(dir), true);
    let mut gen_command_for: Vec<String> = Vec::new();
    
    for run in run_for.split(",") {
        if run == "jswire" || run == "all" {
            gen_command_for.push(cmd.create_build_cmd(CmdType::Jswire));
        }
        if run == "build" || run == "all" {
            gen_command_for.push(cmd.create_build_cmd(CmdType::Build));
        }
        if run == "jstest" || run == "all" {
            gen_command_for.push(cmd.create_build_cmd(CmdType::JsTest));
        }
    }
    if matches.is_present("explain") {
        explain(gen_command_for);
        return;
    }
    
    let mut ghelp_cmd = Command::new(cmd.create_build_cmd(CmdType::JsTest));
    println!(" >>> Start {:?}", ghelp_cmd);
    ghelp_cmd
        .stdout(Stdio::inherit())
        .output()
        .expect("Process failed");
}

fn run_commands(cmds: Vec<String>) {
    for cmd in cmds {
        let cmd_array = cmd.split(" ");

    }
}

fn explain(cmds: Vec<String>) {
    for cmd in cmds {
        println!("{}", cmd.red());
    }
}
