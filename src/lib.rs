// pub struct Argument {
//     pub arg: String,
//     pub required: bool,
// }

pub struct Cmd {
    pub cmd: String,
    pub description: String,
    pub operation: Box<dyn Fn(i32)>,
}

impl Cmd {
    pub fn exec(&self, args: &[&str]) {
        println!("Executing Command... {}", self.cmd);
        (self.operation)(args[0].parse().unwrap_or(0));

        // Do the thing that is expected of the command
        // by passing the arguments
    }
}

pub fn run_command() {
    unimplemented!()
}
