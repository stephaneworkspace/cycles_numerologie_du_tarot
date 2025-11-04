use std::fs;
use cycles_numerologie_du_tarot::generate;

fn main() {
    // Cette date bug, à signaler pour le xls let calques = calcl(30,12,1986, 39);

    match generate(14,6,1946,79, "/Users/stephane/Code/rust/ref/cycles_numerologie_du_tarot/psd/cycles.psd".to_string() ) {
        Ok(ok) => {
            if let Err(e) = fs::create_dir_all("/Users/stephane/Code/rust/ref/cycles_numerologie_du_tarot/tmp") {
                eprintln!("Erreur création dossier tmp: {}", e);
                return;
            }
            if let Err(e) = fs::write("/Users/stephane/Code/rust/ref/cycles_numerologie_du_tarot/tmp/cycles.png", &ok) {
                eprintln!("Erreur écriture ./tmp/cycles.png: {}", e);
            }
        },
        Err(e) => println!("{:?}", e),
    }
}