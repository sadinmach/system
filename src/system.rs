#[derive(Debug)]
pub struct Society {
    name: String,
    population: u32,
    income: i64,
}


impl Society {
    pub fn new() -> Self {
        Self {
            name: String::from("Martian Society"),
            population: 0,
            income: 0,
        }
    }
}


