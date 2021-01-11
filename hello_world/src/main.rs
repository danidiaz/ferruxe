fn main() {
    println!("Hello, world!");

    let foo : u32 = 1;
    println!("This foo: {}",foo);
    // foo = 2; // immutable by default
    
    let foo2 : u32 = 2;

    let bar : &u32 = &foo;
    println!("This bar: {}",*bar); 
    // it seems that automatic dereferencing is sometimes performed
    // https://stackoverflow.com/questions/28519997/what-are-rusts-exact-auto-dereferencing-rules
    // https://www.reddit.com/r/rust/comments/6if67g/why_do_we_need_dereferencing_could_not_every_use/
    println!("This bar: {}",bar);

    let mut xoo : &u32 = &foo;
    println!("This xoo: {}",xoo);
    xoo = &foo2;
    println!("This xoo: {}",xoo);

    // so... a constant binding can point to a *mutable* refernce?
    let mut foo3 : u32 = 7;
    let xee: &mut u32 = &mut foo3;
    *xee = 4;
    // It seems that dereferncing is required here
    // "consider dereferencing here to assign to the mutable borrowed piece of memory"
    // xee = 3; // Seems like de-referencig *is* necessary here...
    println!("This xee: {}",xee);
}
