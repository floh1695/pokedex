use pokerust::{Berry,FromName};

fn main() {
    let berry = Berry::from_name("cheri").unwrap();
    println!("{}: {}", berry.name, berry.max_harvest);
}
