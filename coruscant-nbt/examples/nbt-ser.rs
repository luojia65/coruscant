use serde::Serialize;
use coruscant_nbt::to_vec;

#[derive(Serialize)]
pub struct Person {
    name: String,
    age: i16,
    school: School,
}

#[derive(Serialize)]
pub struct School {
    city: String,
    #[serde(rename = "985")]
    nine_eight_five: bool,
}

fn main() {
    let hust = School {
        city: String::from("Wuhan"),
        nine_eight_five: true,
    };
    let wzk = Person {
        name: String::from("Wang Zhekai"),
        age: 18,
        school: hust
    };
    println!("{:?}", to_vec(&wzk));
}
