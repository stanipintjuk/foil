extern crate foil;

fn main() {
    let result = foil::grammar::html::node("div\tclass=\"row col12\" {}");
    println!("Result: {:?}", result);
}
