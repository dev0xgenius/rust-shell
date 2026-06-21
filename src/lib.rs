// pub struct Argument {
//     pub arg: String,
//     pub required: bool,
// }

pub struct Cmd {
    pub cmd: String,
    pub description: String,
    pub operation: Box<dyn Fn()>,
}

impl Cmd {
    pub fn exec(&self) {
        println!("Executing Command... {}", self.cmd);
        (self.operation)();

        // Do the thing that is expected of the command
        // by passing the arguments
    }
}

pub fn run_command() {
    unimplemented!()
}
