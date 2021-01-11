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

    // Arrays can mutate if the binding is mut.
    arry[1] = 22;
    
    // cool, you can match on statically-sized arrays
    let _fst : u32 =
        match arry {
            [u,_,_] => u
        };

    // If we write this:
    // for i in arry.into_iter() {
    // We get this
    // warning: this method call currently resolves to `<&[T; N] as IntoIterator>::into_iter` (due to autoref coercions), but that might change in the future when `IntoIterator` impls for arrays are added.
    //   --> src\main.rs:23:19
    //    |
    // 23 |     for i in arry.into_iter() {
    //    |                   ^^^^^^^^^ help: use `.iter()` instead of `.into_iter()` to avoid ambiguity: `iter`
    //
    for i in arry.iter_mut() {
        *i = 3;
        println!("This is array elem {}",i);
    }
    for i in arry.iter() {
        println!("This is array elem {}",i);
    }

    let wee : &u32 = &arry[0];
    // The presence of the println of wee makes this line
    // 
    let woo : &mut u32 = &mut arry[1];
    //
    // fail with:
    // error[E0502]: cannot borrow `arry[_]` as mutable because it is also borrowed as immutable
    //   --> src\main.rs:44:26
    println!("This is wee {}",wee);
}
