extern crate lambda;

use lambda::*;

fn main() {
    // print_info("a b c \\e.\\f. g (h i \\j.j x y j (\\j'.j')) t (a b (\\a.b))\\k.m");
    // print_info("this is actually a valid lambda expression");
    // print_info("the I combinator: \\x.x");
    // print_info("the K combinator: \\x.\\y.x");
    // print_info("the S combinator: \\x.\\y.\\z. x z (y z)");
    // print_info("the Y combinator: \\f.(\\x.f (x x))(\\x. f (x x))");
    // print_info("My 'compose' token in action: ∀x.y");
    // print_info("same token string without 'compose': ∀ \\x.y");
    // print_info("\\a.\\b. c d x");

    let program = "(def.

    def (m.n.f.x.m f (n f x)) +.
    def (f.x.f x) 1.
    def (+ 1 1) 2.
    def (+ 2 1) 3.

    + 3 2
    
    ) (a.b.b a)";

    run(program);

}

