use serde::Serialize;

#[derive(Serialize)]
pub struct Person {
    name: String,
    school: School,
}

#[derive(Serialize)]
pub struct School {
    city: String,
    gaokao_score: i32,
    #[serde(rename = "985")]
    nine_eight_five: bool,
}

fn main() {
    
}
