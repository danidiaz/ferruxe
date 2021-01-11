fn main() {
    println!("Hello, world!");

    // https://doc.rust-lang.org/book/ch03-05-control-flow.html
    // https://doc.rust-lang.org/rust-by-example/flow_control.html
    let mut _foo : () = loop {
        break;
    };

    // https://doc.rust-lang.org/book/ch03-05-control-flow.html
    let mut _foo2 : u32 = loop {
        println!("Inside loop.");
        break 2
        // break 2; // also ok
    };

    // ifs: no parentheses on conditions, but blocks on branches!

    // The if doesn't need a break to be expression-like
    let _bar: u32  = if true { 3 } else { 4 };

    let mut _uny : () = if true { () } else { () };
    _uny = if true { println!("This is message"); }

}
