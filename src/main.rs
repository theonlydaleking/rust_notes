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
    );

    // Some stuff on threads. 
    let arr = &[1,25,-4,10];
    let max = find_max(arr);
    println!("The Max: {:?}", max );
    assert_eq!(max, Some(25));
}

fn find_max(arr: &[i32]) -> Option<i32> {
    const THRESHOLD: usize = 2;
    if arr.len() <= THRESHOLD {
        return arr.iter().cloned().max()
    }
    let mid = arr.len() / 2; 
    let (left, right) = arr.split_at(mid);

    crossbeam::scope(|s| {
        let thread_l = s.spawn(|_| find_max(left));
        let thread_r = s.spawn(|_| find_max(right));
        let min_l = thread_l.join().unwrap()?;
        let min_r = thread_r.join().unwrap()?;

        Some(min_l.max(min_r))


    }).unwrap()
}

