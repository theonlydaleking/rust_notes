extern crate ansi_term;
use ansi_term::Colour;

fn main() {
    // Sort a array.
    // Arrays are Vec's
    let mut vec = vec![1, 5, 10, 2, 15];

    vec.sort();
    // using debug trait instead of display. doing this with {:?}
    println!("{:?}", vec);

    assert_eq!(vec, vec![1, 2, 5, 10, 15]);

    // Print colourful shit
    println!(
        "This is {} in colour, {} in colour and {} in colour",
        Colour::Red.paint("red"),
        Colour::Blue.paint("blue"),
        Colour::Green.paint("green")
    )
}
