fn main() {
    println!("Hello, world!");

    // https://doc.rust-lang.org/book/ch03-05-control-flow.html
    // https://doc.rust-lang.org/rust-by-example/flow_control.html
    // https://doc.rust-lang.org/reference/expressions/loop-expr.html
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
    _uny = if true { (); } else { (); };
    _uny = if true { println!("This is message") };
    _uny = if true { println!("This is message"); };

    // this work, but it tells you to use loop instead
    _uny = while true {
        break;
    };

    // This doesn't work
    // "error[E0571]: `break` with value from a `while` loop"
    // let inty: u32 = while true {
    //     break 3;
    // };
    
    let mut counter : u32 = 0;
    while counter < 3 {
        println!("Inside loop, counter is {}",counter);
        counter+=1;
    }

    _uny = for x in 1..3 {
        println!("this is x: {}",x);
    };

    // like with while, this doesn't fly
    // "`break` with value from a `for` loop"
    // let fory : u32 = for x in 1..3 {
    //     break 7;
    // };

}
