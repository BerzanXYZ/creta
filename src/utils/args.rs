// A struct for representing environment args
pub struct Args {
    args: Vec<String>,
}

impl Args {
    // Creates an Args object which holds environment args
    pub fn new() -> Args {
        // Declare temporary env_args vector
        let mut env_args = Vec::new();
        // Push env args into env_args vector
        for arg in std::env::args() {
            env_args.push(arg)
        }
        // Return object
        Args { args: env_args }
    }
    // Function that returns the count of args
    pub fn len(&self) -> usize {
        self.args.len()
    }
    // Function that returns an Option of the arg on given index
    pub fn get(&self, index: usize) -> Option<&str> {
        if index > self.args.len() {
            None
        } else {
            Some(&self.args[index])
        }
    }
}