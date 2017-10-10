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

    let x = 3 + 2;
    println!("{}", x);

    // let program = "(\\plus. \\3. \\2. plus 3 2) 
    // (\\m.\\n.\\f.\\x.m f (n f x)) 
    // (\\f.\\x.f (f (f x))) 
    // (\\f.\\x.f (f x))";

    let program = "(def.

    def (m.n.f.x.m f (n f x)) +.
    def (f.x.f x) 1.
    def (+ 1 1) 2.
    def (+ 2 1) 3.

    + 3 2
    
    ) (a.b.b a)";

    let mut string_table: Vec<String> = Vec::new();

    let mut program = parse_str(program, &mut string_table);
    beta_reduce2(&mut program, &mut string_table);


    println!("{}", to_simplified_string(&program, |i| &string_table[i as usize]));

}

fn print_info(lambda: &str) {
    let mut string_table: Vec<String> = Vec::new();

    let tokens: Vec<Token> = string_to_tokens(lambda.chars(), &mut string_table);

    let bytecode: Vec<u32> = parse(&tokens);

    println!("");
    println!("input string:      {}", lambda);
    println!("syntax tokens:     {:?}", tokens);
    println!("string table:      {:?}", string_table);
    println!("hex output:        {}", u32s_to_hex(&bytecode));
    println!("canonical output:  {}", to_canonical_string(&bytecode, |i| &string_table[i as usize]));
    println!("simplified output: {}", to_simplified_string(&bytecode, |i| &string_table[i as usize]));

    let r = replace_strs(&bytecode, "x", "j", &mut string_table);

    println!("[x := j]           {}", to_simplified_string(&r, |i| &string_table[i as usize]));

    let r = replace_strs(&bytecode, "y", "x", &mut string_table);

    println!("[y := x]           {}", to_simplified_string(&r, |i| &string_table[i as usize]));

    let r = replace_strs(&bytecode, "x", "b e d", &mut string_table);

    println!("[x := b e d]       {}", to_simplified_string(&r, |i| &string_table[i as usize]));

    println!("");
}


fn u32_to_hex(u: &u32) -> String {
    let mut f = format!("{:x}", u);
    while f.len() < 8 {
        f.insert(0, '0');
    }
    f
}

fn u32s_to_hex(u: &[u32]) -> String {
    let v: Vec<String> = u.iter().map(|i| u32_to_hex(i)).collect();
    v.join(" ")
}