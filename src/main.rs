use csv_validator::constraints;

fn main() {
    let field = "";
    match constraints::not_empty(field) {
        Ok(_) => println!("Good to go"),
        Err(e) => eprintln!("{e}"),
    }
}
