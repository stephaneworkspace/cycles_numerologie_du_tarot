use cycles_numerologie_du_tarot::generate;

fn main() {
    match generate(3,4,1986,39, "/Users/stephane/Code/rust/ref/cycles_numerologie_du_tarot/psd/cycles.psd".to_string() ) {
        Ok(_) => println!("Ok"),
        Err(e) => println!("{:?}", e),
    }
}