mod generator;

pub fn print_random_number() {
    println!("Random u8: {}", generator::gen_ran());
}
