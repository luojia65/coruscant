use coruscant_nbt;

fn main() {
    let a = 1;
    println!("{:?}", coruscant_nbt::value::to_value(a));
}
