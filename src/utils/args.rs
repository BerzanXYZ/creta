// A struct for representing environment args
pub struct Args {
    args: Vec<String>,
}

impl Args {
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