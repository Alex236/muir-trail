mod parameter_store;
mod map;

fn main() {
    for row in map::MAP.iter() {
        for item in row.iter() {
            print!("{}", item);
        }
        println!()
    }
}
