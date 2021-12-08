#![feature(box_syntax)]
#![allow(dead_code)]

// use std::mem::discriminant;
use std::collections::VecDeque;

#[derive(Clone, Debug)]
enum Node {
    Add(Box<Node>, Box<Node>),
    Multiply(Box<Node>, Box<Node>),
    Number(i64),
    Empty,
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            &Node::Add(l, r) => write!(f, "({} + {})", l, r),
            &Node::Multiply(l, r) => write!(f, "({} * {})", l, r),
            &Node::Number(x) => write!(f, "{}", x),
            &Node::Empty => panic!("empty display"),
        }
    }
}

impl Node {
    fn replace_right(self, new_right: Node) -> Node {
        match self {
            Node::Add(left, _) => Node::Add(left, box new_right),
            Node::Multiply(left, _) => Node::Multiply(left, box new_right),
            _ => panic!("invalid replace_right, called on {:?}", self),
        }
    }
    pub fn solve(&self) -> i64 {
        match self {
            Self::Number(x) => *x,
            Self::Add(left, right) => left.solve() + right.solve(),
            Self::Multiply(left, right) => left.solve() * right.solve(),
            Self::Empty => panic!("cannot solve empty equation"),
        }
    }
}

#[derive(Debug)]
enum Opr {
    Add,
    Multiply,
}

#[derive(Debug)]
enum Expr {
    Paren(Box<Expr>, Box<ExprPrime>),
    NumPrime(i64, Box<ExprPrime>),
}

#[derive(Debug)]
enum ExprPrime {
    Some(Opr, Expr, Box<ExprPrime>),
    None,
}

#[derive(Copy, Clone, Debug)]
enum Token {
    LParen,
    RParen,
    Add,
    Mul,
    Num(i64),
}

fn tokenize(s: &str) -> Vec<Token> {
    let s = s.chars().collect::<Vec<char>>();
    let mut result = vec![];
    let mut i = 0;
    while i < s.len() {
        match s[i] {
            c @ '0'..='9' => {
                let mut buf = String::new();
                buf.push(c);
                while i + 1 < s.len() && matches!(s[i + 1], '0'..='9') {
                    i += 1;
                    buf.push(s[i]);
                }
                result.push(Token::Num(buf.parse::<i64>().unwrap()));
            }
            '*' => {
                result.push(Token::Mul);
            }
            '+' => {
                result.push(Token::Add);
            }
            '(' => {
                result.push(Token::LParen);
            }
            ')' => {
                result.push(Token::RParen);
            }
            _ => {}
        }
        i += 1;
    }
    result
}

type ParseResult<T> = Result<T, String>;

struct Parser {
    tokens: Vec<Token>,
    index: usize,
    paren_stack: VecDeque<Node>,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            index: 0,
            paren_stack: VecDeque::new(),
        }
    }

    fn peek(&self) -> ParseResult<Token> {
        if self.index + 1 >= self.tokens.len() {
            return Err("peek error, index + 1 >= self.tokens.len()".to_string());
        }
        Ok(self.tokens[self.index + 1])
    }

    fn current(&self) -> ParseResult<Token> {
        if self.index >= self.tokens.len() {
            return Err("current error, index >= self.tokens.len()".to_string());
        }
        Ok(self.tokens[self.index])
    }

    fn consume(&mut self, token: Token) -> ParseResult<Token> {
        if self.index >= self.tokens.len() {
            return Err("consume error, index >= self.tokens.len()".to_string());
        }
        if std::mem::discriminant(&token) != std::mem::discriminant(&self.tokens[self.index]) {
            return Err(format!(
                "error in consume, looking for token={:?}, found token={:?}",
                token, self.tokens[self.index]
            ));
        }

        self.index += 1;
        Ok(self.tokens[self.index - 1])
    }

    fn parse(&mut self) -> ParseResult<Node> {
        let result = self.p_expr()?;
        println!("syntax_tree={:?}", result);
        let result = self.build_ast(result);
        Ok(result)
    }

    fn p_expr(&mut self) -> ParseResult<Expr> {
        if let Ok(Token::Num(x)) = self.consume(Token::Num(0)) {
            let expr_prime = self.p_expr_prime()?;
            return Ok(Expr::NumPrime(x, box expr_prime));
        }
        self.consume(Token::LParen)?;
        let expr = self.p_expr()?;
        self.consume(Token::RParen)?;
        let expr_prime = self.p_expr_prime()?;

        Ok(Expr::Paren(box expr, box expr_prime))
    }

    fn p_expr_prime(&mut self) -> ParseResult<ExprPrime> {
        if let Ok(opr) = self.p_opr() {
            let expr = self.p_expr()?;
            let expr_prime = self.p_expr_prime()?;
            return Ok(ExprPrime::Some(opr, expr, box expr_prime));
        }
        Ok(ExprPrime::None)
    }

    fn p_opr(&mut self) -> ParseResult<Opr> {
        if let Ok(_) = self.consume(Token::Mul) {
            return Ok(Opr::Multiply);
        };
        if let Ok(_) = self.consume(Token::Add) {
            return Ok(Opr::Add);
        }
        Err("error in p_opr".to_string())
    }

    pub fn build_ast(&mut self, expr: Expr) -> Node {
        self.paren_stack.push_back(Node::Empty);
        self.ast_expr(box expr);
        return self.paren_stack.pop_back().unwrap();
    }

    fn ast_expr(&mut self, expr: Box<Expr>) {
        match *expr {
            Expr::Paren(ex, prime) => {
                self.paren_stack.push_back(Node::Empty);
                self.ast_expr(ex);
                let node = self.paren_stack.pop_back().unwrap();
                let prev = self.paren_stack.pop_back().unwrap();
                self.paren_stack.push_back(match prev {
                    Node::Empty => node,
                    p @ Node::Add(_, _) | p @ Node::Multiply(_, _) => p.replace_right(node),
                    _ => panic!("invalid paren found"),
                });
                self.ast_expr_prime(prime);
            }
            Expr::NumPrime(x, prime) => {
                let node = self.paren_stack.pop_back().unwrap();
                self.paren_stack.push_back(match node {
                    Node::Empty => Node::Number(x),
                    n @ Node::Add(_, _) | n @ Node::Multiply(_, _) => {
                        n.replace_right(Node::Number(x))
                    }
                    _ => panic!("weird NumPrime found idk"),
                });

                self.ast_expr_prime(prime);
            }
        }
    }

    fn ast_expr_prime(&mut self, prime: Box<ExprPrime>) {
        match *prime {
            ExprPrime::None => {}
            ExprPrime::Some(opr, ex, p) => {
                let node = self.paren_stack.pop_back().unwrap();
                self.paren_stack.push_back(match opr {
                    Opr::Add => Node::Add(box node, box Node::Empty),
                    Opr::Multiply => Node::Multiply(box node, box Node::Empty),
                });
                self.ast_expr(box ex);
                self.ast_expr_prime(p);
                // prime = p;
                // let node_clone = node.clone();
                // let result = match opr {
                //     // Opr::Add => Node::Add(box node, box self.ast_expr(box ex)),
                //     // Opr::Multiply => Node::Multiply(box node, box self.ast_expr(box ex)),
                //     Opr::Add => {
                //         Node::Add(box node, )
                //         let other = self.ast_expr(box ex);
                //         let copy = node.0.clone();
                //         node.0 = box self.ast_expr(box ex)
                //     }
                // };
                // println!("processing {:?}", opr);
                // println!("node={:?}", &node_clone);
                // result
            }
        }
        // }
        // node
    }

    // fn invertNode(node: Node, right: Node) {
    //     match node {
    //         Node::Number(x) =>  Node::Number(x),
    //         Node::Add(x, y),
    //     }
    // }
}

const INPUT: &str = include_str!("input.txt");
const EXAMPLE: &str = include_str!("example.txt");

fn main() -> ParseResult<()> {
    for line in EXAMPLE.lines() {
        let tokens = tokenize(line);
        let mut parser = Parser::new(tokens);
        // println!("about to parse, tokens={:?}", parser.tokens);
        let result = parser.parse()?;
        println!("ast={}", result);
        println!("solution={}", result.solve());
    }
    println!("done with examples");

    let mut sum = 0;
    for line in INPUT.lines() {
        let tokens = tokenize(line);
        let mut parser = Parser::new(tokens);
        // println!("about to parse, tokens={:?}", parser.tokens);
        sum += parser.parse()?.solve();
    }
    println!("{}", sum);
    Ok(())
}

// 4 * ((8 * 6 + 3 * 3 * 9 * 8) + (2 * 3 * 4 + 9 * 2 * 3) + 4)
// 4 * ((153 * 9 * 8) + (33 * 2 * 3) + 4)
// 4 * (( 11016 ) + (198) + 4)
// 4 * 11218
// 44872
