extern crate lambda;

use lambda::*;

fn main() {

    let program = "(def.

    def (m.n.f.x.m f (n f x)) +.
    def (f.x.f x) 1.
    def (+ 1 1) 2.
    def (+ 2 1) 3.

    + 3 2
    
    ) (a.b.b a)";

    run_verbose(program);

}

