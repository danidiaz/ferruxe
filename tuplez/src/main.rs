fn main() {
    println!("Hello, world!");

    let foo : (u32,bool) = (3,false);
    let _uny : () = match foo {
        (n,b) => { println!("This is tuple: {} {}", n, b); },
    };
    let _num : u32 = match foo {
        (n,_) => n,
    };

    let mut arry : [u32;3] = [1,2,3];
    println!("This is array {} {} {}",arry[0],arry[1],arry[2]);
    arry = [1;3];
    println!("This is array {} {} {}",arry[0],arry[1],arry[2]);
    
    // cool, you can match on statically-sized arrays
    let _fst : u32 =
        match arry {
            [u,_,_] => u
        };
}
