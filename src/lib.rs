#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}

use std::fmt;

#[derive(Debug)]
pub enum Token {
    OpenParen,
    CloseParen,
    Var(u32),
    Abs(u32),
    Compose
}

pub fn string_to_tokens<I: IntoIterator<Item=char>>(s: I, names: &mut Vec<String>) -> Vec<Token> { //TODO: make errors recoverable
    let mut tokens: Vec<Token> = Vec::new();

    let mut iter = s.into_iter();

    'outer: while let Some(c) = iter.next() {
        if c.is_whitespace() {
            continue;
        }

        match c {
            '(' => tokens.push(Token::OpenParen),
            ')' => tokens.push(Token::CloseParen),
            '\\' => {
                let mut string = String::new();
                while let Some(next) = iter.next() {
                    if next.is_whitespace() {
                        if string.len() == 0 {
                            panic!("the backslash must be immediately followed by a variable name")
                        } else {
                            panic!("abstraction parameters must be immediately followed by a dot")
                        }
                    }

                    match next {
                        '(' | ')' | '\\' => {
                            if string.len() == 0 {
                                panic!("the backslash must be immediately followed by a variable name")
                            } else {
                                panic!("abstraction parameters must be immediately followed by a dot")
                            }
                        },
                        '∀' => panic!("forall not allowed in a parameter name!"),
                        '.' => {
                            if string.len() == 0 {
                                panic!("variable names cannot be empty")
                            } else {
                                tokens.push(Token::Abs(lookup_string(string, names)));
                                continue 'outer
                            }
                        },
                        _ => {
                            string.push(next)
                        }
                    }
                }
                if string.len() == 0 {
                    panic!("the backslash must be immediately followed by a variable name")
                } else {
                    panic!("abstraction parameters must be immediately followed by a dot")
                }
            },
            '∀' => {
                let mut string = String::new();
                while let Some(next) = iter.next() {
                    if next.is_whitespace() {
                        if string.len() == 0 {
                            tokens.push(Token::Var(lookup_string(String::from("∀"), names)));
                            continue 'outer 
                        } else {
                            panic!("abstraction parameters must be immediately followed by a dot")
                        }
                    }
                    match next {
                        '(' => {
                            if string.len() == 0 {
                                tokens.push(Token::Var(lookup_string(String::from("∀"), names)));
                                tokens.push(Token::OpenParen);
                                continue 'outer
                            } else {
                                panic!("abstraction parameters must be immediately followed by a dot")
                            }
                        },
                        ')' => {
                            if string.len() == 0 {
                                tokens.push(Token::Var(lookup_string(String::from("∀"), names)));
                                tokens.push(Token::CloseParen);
                                continue 'outer
                            } else {
                                panic!("abstraction parameters must be immediately followed by a dot")
                            }
                        },
                        '\\' => panic!("backslash not allowed here!"),
                        '∀' => panic!("forall not allowed in a parameter name!"),
                        '.' => {
                            if string.len() == 0 {
                                panic!("variable names cannot be empty")
                            } else {
                                tokens.push(Token::Compose);
                                tokens.push(Token::Var(lookup_string(String::from("∀"), names)));
                                tokens.push(Token::Abs(lookup_string(string, names)));
                                continue 'outer
                            }
                        },
                        _ => string.push(next)
                    }
                }
                tokens.push(Token::Var(lookup_string(string, names)));
            },
            '.' => panic!("dot not allowed here!"),
            _ => {
                let mut string: String = c.to_string();
                while let Some(next) = iter.next() {
                    if next.is_whitespace() {
                        tokens.push(Token::Var(lookup_string(string, names)));
                        continue 'outer
                    }
                    match next {
                        '(' => {
                            tokens.push(Token::Var(lookup_string(string, names)));
                            tokens.push(Token::OpenParen);
                            continue 'outer
                        },
                        ')' => {
                            tokens.push(Token::Var(lookup_string(string, names)));
                            tokens.push(Token::CloseParen);
                            continue 'outer
                        },
                        '\\' => panic!("backslash not allowed here!"),
                        '.' => panic!("dot not allowed here!"),
                        '∀' => panic!("variables that contain forall symbols must contain no more than one character"),
                        _ => string.push(next)
                    }
                }
                tokens.push(Token::Var(lookup_string(string, names)));


            }
        }
    }

    tokens
}


fn lookup_string(s: String, strings: &mut Vec<String>) -> u32 {
    if let Some(i) = strings.iter().position(|t| *t == s) {
        i as u32
    } else {
        let size = strings.len() as u32;
        strings.push(s);
        size
    }
}


const ABSTRACTION_FLAG: u32 = 0b01000000_00000000_00000000_00000000;
const APPLICATION_FLAG: u32 = 0b10000000_00000000_00000000_00000000;
const MATCH_FLAG: u32 = APPLICATION_FLAG | ABSTRACTION_FLAG;
const CLEAR_FLAGS: u32 = !MATCH_FLAG;
const CLEAR_APPLICATION_FLAG: u32 = !APPLICATION_FLAG;



pub fn parse_str(string: &str, string_table: &mut Vec<String>) -> Vec<u32> {
    let tokens = string_to_tokens(string.chars(), string_table);
    parse(&tokens)
}

pub fn parse<'a, I: IntoIterator<Item=&'a Token>>(t: I) -> Vec<u32> { //TODO: make errors recoverable
    let mut firsts: Vec<Vec<u32>> = vec![Vec::new()];
    let mut seconds: Vec<Vec<u32>> = vec![Vec::new()];
    for token in t {
        match token {
            &Token::OpenParen => {
                firsts.push(Vec::new());
                seconds.push(Vec::new());
            },
            &Token::CloseParen => {
                let mut x = firsts.pop().expect("uh oh 0");
                let mut y = seconds.pop().expect("uh oh 1");

                let s = seconds.last_mut().expect("uh oh 3");
                let slen = s.len();
                if slen != 0 { //seconds is not empty
                    s.insert(0, slen as u32 | APPLICATION_FLAG);
                }
                s.append(&mut x);
                s.append(&mut y);
            },
            &Token::Var(x) => {
                let s = seconds.last_mut().expect("uh oh 5");
                let slen = s.len();
                if slen != 0 { //seconds is not empty
                    s.insert(0, slen as u32 | APPLICATION_FLAG);
                }
                s.push(x);
            },
            &Token::Abs(x) => {
                let f = firsts.last_mut().expect("uh oh 6");
                let s = seconds.last_mut().expect("uh oh 7");
                let slen = s.len();
                if slen != 0 { //seconds is not empty
                    f.push(slen as u32 | APPLICATION_FLAG);
                    f.append(s);
                }

                f.push(x | ABSTRACTION_FLAG);
            },
            &Token::Compose => {
                let s = seconds.last_mut().expect("uh oh 8");
                let slen = s.len();
                if slen != 0 { //seconds is not empty
                    let f = firsts.last_mut().expect("uh oh 9");
                    f.push(slen as u32 | APPLICATION_FLAG);
                    f.append(s);
                }
            }
        }
    }

    
    let mut f: Vec<u32> = firsts.pop().expect("uh oh 10");
    if firsts.len() != 0 {
         panic!("uh oh 11");
    }
    f.append(&mut seconds[0]);
    f
} 


fn contains(additional_vars: &[u32], lambda: &[u32], start: usize, var: u32) -> bool {
    if let Some(_) = additional_vars.iter().position(|i| *i == var) {
        return true;
    }
    enum T {
        Applicand,
        Argument,
        Body
    }
    let var_as_param = var | ABSTRACTION_FLAG;
    if var_as_param == var {
        panic!("invalid variable {}", var);
    }
    let mut vec: Vec<T> = Vec::new();
    let mut index = start;
    loop {
        let next = lambda[index];
        match next & MATCH_FLAG {
            0 => { //variable
                if var == next {
                    return true;
                }
                loop {
                    if let Some(b) = vec.pop() {
                        if let T::Applicand = b {
                            vec.push(T::Argument);
                            break;
                        }
                    } else {
                        return false;
                    }
                }
            },
            ABSTRACTION_FLAG => {//abstraction
                if var_as_param == next {
                    return true;
                }
                vec.push(T::Body);
            },
            _ => { //application
                vec.push(T::Applicand);
            }
        }

        index += 1;
    }
}

fn replace_var_unsafe(lambda: &mut [u32], start: usize, var: u32, replacement: u32) {
    enum T {
        Applicand,
        Argument,
        Body(bool)
    }
    let var_as_param = var | ABSTRACTION_FLAG;
    let mut vec: Vec<T> = Vec::new();
    let mut index = start;
    let mut var_is_bound = false;
    loop {
        let next = lambda[index];
        match next & MATCH_FLAG {
            0 => { //variable
                if next == replacement {
                    panic!("lambda contains variable {}", replacement);
                }
                
                loop {
                    if let Some(b) = vec.pop() {
                        if let T::Applicand = b {
                            vec.push(T::Argument);
                            break;
                        }
                    } else {
                        if !var_is_bound && next == var {
                            lambda[index] = replacement;
                        }
                        return;
                    }
                }

                if var_is_bound {
                    var_is_bound = false;
                    for t in vec.iter() {
                        if let &T::Body(true) = t {
                            var_is_bound = true;
                            break;
                        }
                    }
                } else if next == var {
                    lambda[index] = replacement;
                }
            },
            ABSTRACTION_FLAG => { //abstraction
                let binds_var = next == var_as_param;
                vec.push(T::Body(binds_var));
                var_is_bound |= binds_var;
            },
            _ => { //application
                vec.push(T::Applicand);
            }
        }

        index += 1;
    }
}

pub fn replace_strs(lambda: &[u32], var: &str, expression: &str, string_table: &mut Vec<String>) -> Vec<u32> {
    let expression = parse_str(expression, string_table);
    replace_str(lambda, var, &expression, string_table)
}

pub fn replace_str(lambda: &[u32], var: &str, expression: &[u32], string_table: &mut Vec<String>) -> Vec<u32> {
    let var = lookup_string(String::from(var), string_table);
    replace(lambda, var, expression, string_table)
}

pub fn replace(lambda: &[u32], var: u32, expression: &[u32], string_table: &mut Vec<String>) -> Vec<u32> {
    let free_vars = free_variables(expression, (0..(string_table.len() as u32)));
    let mut lambda = Vec::from(lambda);
    prepare_for_replace(var, &free_vars, &mut lambda, string_table);

    let mut output: Vec<u32> = Vec::new();

    enum T {
        Applicand,
        Argument,
        Body(bool)
    }
    let var_as_param = var | ABSTRACTION_FLAG;
    let mut vec: Vec<T> = Vec::new();
    let mut index = 0;
    let mut var_is_bound = false;
    loop {
        let next = lambda[index];
        match next & MATCH_FLAG {
            0 => { //variable
                
                loop {
                    if let Some(b) = vec.pop() {
                        if let T::Applicand = b {
                            vec.push(T::Argument);
                            break;
                        }
                    } else {
                        if !var_is_bound && next == var {
                            output.extend(expression.iter());
                        } else {
                            output.push(next);
                        }
                        return output;
                    }
                }

                if var_is_bound {
                    var_is_bound = false;
                    for t in vec.iter() {
                        if let &T::Body(true) = t {
                            var_is_bound = true;
                            break;
                        }
                    }
                    output.push(next);
                } else if next == var {
                    output.extend(expression.iter());
                } else {
                    output.push(next);
                }
            },
            ABSTRACTION_FLAG => { //abstraction
                let binds_var = next == var_as_param;
                vec.push(T::Body(binds_var));
                var_is_bound |= binds_var;
                output.push(next);
            },
            _ => { //application
                vec.push(T::Applicand);
                output.push(next);
            }
        }

        index += 1;
    }

}


fn prepare_for_replace(variable_to_replace: u32, free_vars: &[u32], lambda: &mut [u32], string_table: &mut Vec<String>) {
    let mut indexes_to_check: Vec<usize> = vec![0];

    while let Some(index) = indexes_to_check.pop() {
        let next = lambda[index];
        match next & MATCH_FLAG {
            0 => { //variable
                //do nothing
            },
            ABSTRACTION_FLAG => { //abstraction
                let var = next & CLEAR_FLAGS;
                if var == variable_to_replace {
                    //do nothing
                } else if !is_free(variable_to_replace, lambda, index + 1) {
                    //do nothing
                } else if let Some(_) = free_vars.iter().position(|i| *i == var) {
                    let next_index = index + 1;
                    let mut new_name = string_table[var as usize].clone();
                    new_name.push('\'');

                    let mut new_var = lookup_string(new_name, string_table);

                    while contains(free_vars, lambda, next_index, new_var) {
                        new_name = string_table[new_var as usize].clone();
                        new_name.push('\'');
                        new_var = lookup_string(new_name, string_table);
                    }

                    lambda[index] = new_var | ABSTRACTION_FLAG;
                    replace_var_unsafe(lambda, next_index, var, new_var);
                    indexes_to_check.push(next_index);
                } else {
                    indexes_to_check.push(index + 1);
                    //do nothing
                }
            }, 
            _ => { //application
                let applicand_index = index + 1;
                let argument_index = applicand_index + ((next & CLEAR_APPLICATION_FLAG) as usize);
                if argument_index <= applicand_index {
                    panic!("invalid application");
                }
                indexes_to_check.push(applicand_index);
                indexes_to_check.push(argument_index);
            }
        }
    }
}

// pub fn do_something(var: u32, free_vars: &[u32], lambda: &mut [u32]) {
//     enum T {
//         Applicand,
//         Argument,
//         CheckedBody,
//         UncheckedBody(usize)
//     }

//     let mut vec: Vec<T> = Vec::new();

//     for (i, next) in lambda.iter().enumerate() {
//         match next & MATCH_FLAG {
//             0 => { //variable
//                 if next == var {
//                     //1. get all unchecked bodies
//                     //2. if needs change of variables, change variables
//                     //3. replace with checkedbody
//                     for (j, next_token) in vec.iter().enumerate() {
//                         if let T::UncheckedBody(index) = next_token {
//                             let param = lambda[index] & CLEAR_FLAGS;
//                             if free_vars.iter().contains(param) {
//                                 let param2 = new_param_that_is_not_contained_inside_abstraction_nor_free_vars;
//                                 lambda[index] = param2 | ABSTRACTION_FLAG;
//                                 replace_var_unsafe(lambda, index + 1, param, param2);
//                             }
//                             vec[j] = T::CheckedBody;
//                         }
//                     }
//                 }
//                 loop {
//                     if let Some(b) = vec.pop() {
//                         if let T::Applicand = b {
//                             vec.push(T::Argument);
//                             break;
//                         }
//                     } else {
//                         return;
//                     }
//                 }
//             },
//             ABSTRACTION_FLAG => { //abstraction
//                 vec.push(T::UncheckedBody(i));
//             },
//             _ => { //application
//                 vec.push(T::Applicand);
//             }
//         }
//     }
// }

pub fn to_canonical_string<'a, S: fmt::Display, F: Fn(u32) -> S, I: IntoIterator<Item=&'a u32>>(lambda: I, string_table: F) -> String {

    let mut string = String::new();
    let mut vec: Vec<bool> = Vec::new();

    for next in lambda {
        match next & MATCH_FLAG {
            0 => { //variable
                string.push_str(&format!("{}", string_table(*next)));

                loop {
                    if let Some(b) = vec.pop() {
                        if b {
                            vec.push(false);
                            string.push_str(" ");
                            break;
                        } else {
                            string.push_str(")");
                        }
                    } else {
                        return string;
                    }
                }
            },
            ABSTRACTION_FLAG => { //abstraction
                vec.push(false);
                string.push_str(&format!("(λ{}.", string_table(next & CLEAR_FLAGS)))
            },
            _ => { //application
                vec.push(true);
                string.push_str("(")
            }
        }

    }
    panic!("iterator terminated too early");
}

#[derive(Debug)]
enum Tok {
    AppAsBody,
    AppAsApplicand,
    AppAsArgument,
    AbsAsBody,
    AbsAsApplicand,
    AbsAsArgument,
    FinishedApplicand
}

pub fn to_simplified_string<'a, S: fmt::Display, F: Fn(u32) -> S, I: IntoIterator<Item=&'a u32>>(lambda: I, string_table: F) -> String {

    let mut iter = lambda.into_iter();
    let next = *iter.next().expect("iterator ended too soon");

    let (mut vec, mut string) = match next & MATCH_FLAG {
        0 => return format!("{}", string_table(next)),
        ABSTRACTION_FLAG => (vec![Tok::AbsAsBody], format!("λ{}.", string_table(next & CLEAR_FLAGS))),
        _ => (vec![Tok::AppAsBody], String::new())
    };

    loop {
        let next = *iter.next().expect("iterator ended too soon");
        let tok = vec.pop().expect("uh oh");

        match next & MATCH_FLAG {
            0 => {
                if let Tok::FinishedApplicand = tok {
                    string.push_str(" ");
                }
                string.push_str(&format!("{}", string_table(next)));
                match tok {
                    Tok::FinishedApplicand | Tok::AbsAsBody | Tok::AbsAsArgument => {
                        if let Tok::AbsAsArgument = tok {
                            string.push_str(")");
                        }
                        loop {
                            if let Some(tok) = vec.pop() {
                                match tok {
                                    Tok::AppAsArgument | Tok::AbsAsArgument => string.push_str(")"),
                                    Tok::AppAsBody |  Tok::AbsAsBody => (),
                                    Tok::AppAsApplicand => {
                                        vec.push(Tok::FinishedApplicand);
                                        break
                                    },
                                    Tok::AbsAsApplicand => {
                                        vec.push(Tok::FinishedApplicand);
                                        string.push_str(")");
                                        break
                                    },
                                    Tok::FinishedApplicand => panic!("weird")
                                }
                            } else {
                                return string;
                            }
                        }
                    },
                    Tok::AbsAsApplicand => {
                        string.push_str(")");
                        vec.push(tok);
                        vec.push(Tok::FinishedApplicand);
                    },
                    Tok::AppAsBody | Tok::AppAsApplicand | Tok::AppAsArgument => {
                        vec.push(tok);
                        vec.push(Tok::FinishedApplicand);
                    }
                }
            },
            ABSTRACTION_FLAG => {
                match tok {
                    Tok::FinishedApplicand => {
                        string.push_str(&format!(" (λ{}.", string_table(next & CLEAR_FLAGS)));
                        vec.push(Tok::AbsAsArgument);
                    },
                    Tok::AppAsBody | Tok::AppAsApplicand => {
                        string.push_str(&format!("(λ{}.", string_table(next & CLEAR_FLAGS)));
                        vec.push(tok);
                        vec.push(Tok::AbsAsApplicand);
                    },
                    Tok::AppAsArgument => {
                        string.push_str(&format!("λ{}.", string_table(next & CLEAR_FLAGS)));
                        vec.push(tok);
                        vec.push(Tok::AbsAsApplicand);
                    },
                    Tok::AbsAsArgument | Tok::AbsAsApplicand | Tok::AbsAsBody => {
                        string.push_str(&format!("λ{}.", string_table(next & CLEAR_FLAGS)));
                        vec.push(tok);
                        vec.push(Tok::AbsAsBody);
                    }
                }
            },
            _ => {
                match tok {
                    Tok::AbsAsArgument | Tok::AbsAsApplicand | Tok::AbsAsBody => {
                        vec.push(tok);
                        vec.push(Tok::AppAsBody);
                    },
                    Tok::AppAsArgument | Tok::AppAsBody | Tok::AppAsApplicand => {
                        vec.push(tok);
                        vec.push(Tok::AppAsApplicand);
                    },
                    Tok::FinishedApplicand => {
                        string.push_str(" (");
                        vec.push(Tok::AppAsArgument);
                    }
                }
            }
        }

    }
    
}

pub fn is_free(var: u32, lambda: &[u32], start: usize) -> bool {
    let mut indexes_to_check: Vec<usize> = vec![start];

    let param = var | ABSTRACTION_FLAG;

    if param == var {
        panic!("invalid variable {}", var);
    }

    while let Some(next_ind) = indexes_to_check.pop() {
        let next = lambda[next_ind];
        match next & MATCH_FLAG {
            0 => { //variable
                if next == var {
                    return true;
                }
            },
            ABSTRACTION_FLAG => { //abstraction
                if next != param {
                    indexes_to_check.push(next_ind + 1);
                }
            },
            _ => { //application
                let applicand_index = next_ind + 1;
                let argument_index = applicand_index + ((next & CLEAR_APPLICATION_FLAG) as usize);
                if argument_index <= applicand_index {
                    panic!("invalid application");
                }
                indexes_to_check.push(applicand_index);
                indexes_to_check.push(argument_index);
            }
        }
    }

    false
}

pub fn free_variables<I: Iterator<Item=u32>> (lambda: &[u32], variables: I) -> Vec<u32> {
    let mut fvs: Vec<u32> = Vec::new();

    for var in variables {
        if is_free(var, lambda, 0) {
            fvs.push(var);
        }
    }
    
    fvs
}



pub fn to_hex_string(src: &[u32]) -> String {
    let mut src: Vec<u32> = src.iter().map(|i| i.to_be()).collect();

    let src: &mut [u32] = &mut src;

    let bytes: &mut [u8] = unsafe {
         std::slice::from_raw_parts_mut(src.as_mut_ptr() as *mut u8, src.len() * 4)
    };

    let mut string = String::new();
    for byte in bytes {
        let mut format = format!("{:x}", byte);
        if format.len() == 1 {
            format.insert(0, '0');
        }
        string.push_str(&format);
    }

    string

}


















///////////////////////////////////////////////////////////////////////
//DEPRECATED RECURSIVE STRUCTURES
///////////////////////////////////////////////////////////////////////

pub enum Expression {
    App(Box<Application>),
    Abs(Box<Abstraction>),
    Var(Variable) //todo: change to Var(usize)
}

pub struct Abstraction {
    pub param: Variable,
    pub body: Expression
}

pub struct Application {
    pub applicand: Expression,
    pub argument: Expression
}

pub struct Variable {
    name: String
}

pub trait Expressive {
    fn expression(self) -> Expression;
}

impl Expressive for Expression {
    fn expression(self) -> Expression { self }
}

impl Expressive for Abstraction {
    fn expression(self) -> Expression { Expression::Abs(Box::new(self)) }
}

impl Expressive for Application {
    fn expression(self) -> Expression { Expression::App(Box::new(self)) }
}

impl Expressive for Variable {
    fn expression(self) -> Expression { Expression::Var(self) }
}

pub fn var(name: String) -> Variable {
    Variable::new(name)
}

impl Abstraction {
    pub fn new<E: Expressive>(param: Variable, body: E) -> Abstraction {
        Abstraction {
            param,
            body: body.expression()
        }
    }
}

impl Application {
    pub fn new<E: Expressive, F: Expressive>(applicand: E, argument: F) -> Application {
        return Application {
            applicand: applicand.expression(),
            argument: argument.expression()
        }
    }
}

impl Variable {
    pub fn new(name: String) -> Variable {
        Variable { name }
    }
}


impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Expression::App(ref x) => x.fmt(f),
            &Expression::Abs(ref x) => x.fmt(f),
            &Expression::Var(ref x) => x.fmt(f)
        }
    }
}

impl fmt::Display for Abstraction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.body {
            Expression::Abs(ref x) => write!(f, "λ{}.{}", self.param, x),
            Expression::App(ref x) => write!(f, "λ{}. {}", self.param, x),
            Expression::Var(ref x) => write!(f, "λ{}. {}", self.param, x)
        }
    }
}





impl fmt::Display for Application {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.applicand {
            Expression::Abs(ref x) => match self.argument {
                Expression::Abs(ref y) => write!(f, "({}) ({})", x, y),
                Expression::App(ref y) => write!(f, "({}) ({})", x, y), 
                Expression::Var(ref y) => write!(f, "({}) {}", x, y)
            },
            Expression::App(ref x) => match self.argument {
                Expression::Abs(ref y) => write!(f, "{} ({})", x, y),
                Expression::App(ref y) => write!(f, "{} ({})", x, y), 
                Expression::Var(ref y) => write!(f, "{} {}", x, y)
            },
            Expression::Var(ref x) => match self.argument {
                Expression::Abs(ref y) => write!(f, "{} ({})", x, y),
                Expression::App(ref y) => write!(f, "{} ({})", x, y), 
                Expression::Var(ref y) => write!(f, "{} {}", x, y)
            }
        }
    }
}

impl fmt::Display for Variable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.name)
        //write!(f, "{}", self.name)
    }
}

//TODO: parser takes in (1) regular expression matching lambda symbol, 
//and (2) a string matching lambda symbols that actually become applicands 
//whose argument is the abstraction.
//1st case: \x.y   ->    Abstraction(x, y)
//2nd case: ∀x.y   ->    Application(∀, Abstraction(x, y))

//Should we get rid of the recursive parsing? Could cause a stack overflow if too big.
//TODO: variable should just store an int; (expression changes meaning in context of list of variable names)
//that would take up less space


