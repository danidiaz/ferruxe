[How can I build multiple binaries with Cargo?](https://stackoverflow.com/questions/36604010/how-can-i-build-multiple-binaries-with-cargo)

    PS> cargo run --bin main1
        Finished dev [unoptimized + debuginfo] target(s) in 0.02s
         Running `target\debug\main1.exe`
    Hello, world 1!
    PS> cargo run --bin main2
        Finished dev [unoptimized + debuginfo] target(s) in 0.01s
         Running `target\debug\main2.exe`
    Hello, world 2!
