use std::{thread, time::Duration};

use libthrob::Throbber;

fn main() {
    println!("Showing braille throbber");
    let mut _test = Throbber::braille(0.1);
    _test.start(false);
    thread::sleep(Duration::from_secs(5));

    println!("Showing classic throbber");
    _test = Throbber::classic(0.1);
    _test.start(false);
    thread::sleep(Duration::from_secs(5));

    println!("Showing custom throbber");
    _test = Throbber::custom(['A', 'B', 'C', 'D'].to_vec(), 0.1);
    _test.start(false);
    thread::sleep(Duration::from_secs(5));
}

