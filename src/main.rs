mod demo_log;

// use crate::demo_log::dump;

// mod mymod;

// use crate::mymod::submod::private;

use rand::prelude::*;

fn main()
{
    // dump();
    // private();

    if rand::random() {
        println!("char: {}", rand::random::<char>());
    }
}