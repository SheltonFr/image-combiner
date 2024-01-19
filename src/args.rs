#[derive(Debug)]
pub struct Args {
    pub image_one: String,
    pub image_two: String,
    pub output: String,
}

impl Args {
    pub fn new() -> Self {
        Args { 
            image_one: get_nth_env(1), 
            image_two: get_nth_env(2), 
            output: get_nth_env(3), 
        }
    }
}

fn get_nth_env(n: usize) -> String {
    std::env::args().nth(n).unwrap()
}
