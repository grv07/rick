pub mod base {
    #[derive(Debug)]
    pub enum CmdType {
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
            let param: String;
            match cmd_type {
                CmdType::Build => {
                    param = format!("blaze build {}/{}/...",self.base_path, self.dir);
                }
                CmdType::Jswire => {
                    param = format!("blaze build {}/{}/{}/...", self.base_path, self.dir, "jswire");
                }
                CmdType::JsTest => {
                    param = format!("blaze test {}/{}:{}", self.base_path, self.dir, "jstests_1_debug");
                }
            }
            param
        }
    }
}
