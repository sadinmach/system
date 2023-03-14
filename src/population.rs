#[derive(Debug)]
pub struct Dob {
    year: i32,
    month: String,
    day: u8,
}

impl Dob {
    pub fn new(year: i32, month: String, day: u8) -> Self {
        Self {
            year,
            month,
            day,
        }
    }
}

#[derive(Debug)]
pub struct Citizen {
    first_name: String,
    last_name: String,
    username: String,
    dob: Dob,
    age: u8,
    income: i32,
    martian: bool,
}

impl Citizen {
    pub fn new(first_name: String, last_name: String, username: String, dob: Dob, age: u8, income: i32) -> Self {
        Self {
            first_name,
            last_name,
            username,
            dob,
            age,
            income,
            martian: true,
        }
    }
}
    


