use crate::Tokens;

pub fn ass(mut expression: Vec<Tokens>) -> Vec<Tokens> {
    let mut stack = Vec::new();
    let mut output = Vec::new();
    while !expression.is_empty() {
        let token = expression.remove(0);
        match token {
            Tokens::NUMBER(k) => output.push(Tokens::NUMBER(k)),
            Tokens::OP(k) => {
                if !stack.is_empty() {
                    println!("stack: {:?}", stack[stack.len() - 1]);
                    while !stack.is_empty() && match stack[stack.len() - 1] {
                        Tokens::OP(_) => true,
                        _ => false,
                    } {
                        output.push(stack.remove(stack.len() - 1));
                    }
                }
                stack.push(Tokens::OP(k));
            }
        }
    }
    if expression.is_empty() {
        while !stack.is_empty() {
            output.push(stack.pop().unwrap());
        }
    }

    return output;
}
