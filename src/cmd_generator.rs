pub mod base {
    #[derive(Debug)]
    pub enum CmdType {
        All,
        Build,
        Jswire,
        JsTest
    }

    #[derive(Debug)]
    pub struct Cmd {
        base_path: String,
        dir: String,
        is_blaze: bool,
    }

    impl Cmd {
        pub fn new(base_path: String, dir: String, is_blaze: bool) -> Cmd {
            Cmd {base_path: base_path, is_blaze: is_blaze, dir: dir}
        }

        pub fn create_build_cmd(&self, cmd_type: CmdType) -> String {
            let mut param = String::new();
            let mut cmd = String::from("blaze build");
            match cmd_type {
                CmdType::All => {
                    param = format!(" {}/..", self.base_path);
                },
                CmdType::Build => {
                    param = format!(" {}/{}/..",self.base_path, self.dir);
                }
                CmdType::Jswire => {
                    param = format!(" {}/{}/{}/..", self.base_path, self.dir, "jswire");
                }
                CmdType::JsTest => {
                    param = format!(" {}/{}:{}", self.base_path, self.dir, "test_debug_1");
                }
                _ => {
                    println!("Command type not found.");
                }
            }
            cmd.push_str(&param);
            cmd
        }
    }
}
