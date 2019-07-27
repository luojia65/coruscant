use coruscant_world::level_dat::*;

fn main() {
// 1. Construct non-exhaustive struct values
// You may do this:
    let version = Version::new(233, "23w23a".to_string(), true);
    println!("{:?}", version);
// You cannot do this, because we cannot create non-exhaustive struct 
// using struct expression:
    // let version = Version { 
    //     id: 233, 
    //     name: "23w23a".to_string(), 
    //     snapshot: true 
    // };
// 2. Get values from struct field directly
// This is okay:
    let id = version.id;
    println!("{}", id);
// 3. Extract values using pattern match 
// You may do this:
    let Version { id, name, snapshot, .. } = version;
    println!("{}; {}; {}", id, name, snapshot);
// You cannot do this, as `..` is required with struct marked as non-exhaustive:
    // let Version { id, name, snapshot } = version;
}
