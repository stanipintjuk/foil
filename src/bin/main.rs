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

    let input = b"tag=\"val\"";
    let result = foil::tag_name(input);
    println!("result: {:?}", result);

    let input = b"\"val\"";
    let result = foil::take_string(input);
    println!("result: {:?}", result);

    let input = b"tag=\"val\"";
    let result = foil::take_attribute(input);
    println!("result: {:?}", result);

    let input = b"html {   \n\t\t  head {} body attr=\"hej\" attrTwo=\"hejdo\" {}";
    let result = foil::take_DOM_node(input);

    match result {
        IResult::Done(_, node) => {
            println!("{}", node.to_html());
        },
        IResult::Error(err) => {
            println!("Error: {}", err);
        },
        IResult::Incomplete(needed) => {
            println!("Incomplete. Need {:?}", needed)
        }
    };
    
}
