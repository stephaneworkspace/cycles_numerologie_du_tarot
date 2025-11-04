use cycles_numerologie_du_tarot::generate;

fn main() {
    match generate(14,6,1946,79, "/Users/stephane/Code/rust/ref/cycles_numerologie_du_tarot/psd/cycles.psd".to_string() ) {
        Ok(ok) => println!("{}Ok", ok.len()),
        Err(e) => println!("{:?}", e),
    }
}