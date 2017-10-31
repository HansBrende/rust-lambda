extern crate lambda;

use lambda::*;

fn main() {

    //let program = "(x. y. x - y) 3 2";
    let program = 
    "(def. 
    def (t.f.t) true.
    def (t.f.f) false.
    def (a.a false true) not.
    def (a.b. a true b) or.
    def (a.b. a b false) and.
    
    def (f.x. x) 0.
    def (f.x. f x) 1.
    def (f.x. f (f x)) 2.
    def (n.f.x. f (n f x)) S.
    def (n.m. (n S) m) +.
    def (n.m. n (+ m) 0) *.
    def (n.n (x. false) true) is_zero.
    def (n.n (g.k. is_zero (g 1) k (+ (g k) 1)) (v.0) 0) P.
    def (r.n. is_zero n 1 (* n (r (P n)))) factorial.
    def (S 2) 3.
    def (S 3) 4.
    def (f. (x. f (x x)) (x. f (x x))) Y.
    
    Y factorial 3
    
    ) (a.b. b a)";

    run(program);

}

