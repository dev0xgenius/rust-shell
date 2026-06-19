struct Cmd {
    cmd: String,
    args: i32,
    arg_required: bool,
}

impl Cmd {
    pub fn exec(&self) {
        println!("Executing Command... {}", self.cmd);
        // Do the thing that is expected of the command
        // by passing the arguments
    }
}

pub fn run_command() {
    unimplemented!()
}
