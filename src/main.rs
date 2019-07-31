extern crate ansi_term;
use ansi_term::Colour;
use ansi_term::Style;

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
        "This is {} in colour, {} in colour and {} in colour. Here is something that is {}",
        Colour::Red.paint("red"),
        Colour::Blue.paint("blue"),
        Colour::Green.paint("green"),
        Style::new().bold().paint("this is bold")
    );

    // Printing shit in bold
    println!(
        "Here is something else {}",
        Style::new().bold().paint("this is bold")
    )
}
