fn main() {
    //test
    let file = std::fs::read_to_string("lines").unwrap();

    file
        .lines()
        .for_each(|line| println!("{}", line));
}
