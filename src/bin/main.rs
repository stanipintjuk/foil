extern crate foil;
extern crate nom;
use nom::IResult;

fn main() {
    /*let input = b"-2015";
    let result = foil::positive_year(input);
    println!("input: {:?}", input);
    println!("result: {:?}", result);

    let input = b"2015";
    let result = foil::take_4_digits(input);
    println!("input: {:?}", input);
    println!("result: {:?}", result);*/

    let input = b" }";
    let result = foil::valid_variable_name(input);
    println!("result: {:?}", result);

}
