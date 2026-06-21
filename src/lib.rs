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
        if args.is_empty() {
            (self.operation)(0);
            return;
        }

        let code = args[0].parse().unwrap_or(0);
        println!("Parsed code: {}", code);
        (self.operation)(code);

        // Do the thing that is expected of the command
        // by passing the arguments
    }
}

pub fn run_command() {
    unimplemented!()
}
