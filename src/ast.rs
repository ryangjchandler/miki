pub type Block = Vec<Stmt>;

#[derive(Debug, Clone)]
pub enum Stmt {
    If { cond: Expr, then: Block },
    Expr { expr: Expr },
    Return { expr: Expr },
    Print { expr: Expr },
    Def { name: String, params: Vec<Expr>, body: Block }
}

#[derive(Debug, Clone)]
pub enum Expr {
    Str { s: String },
    Int { i: i64 },
    // Var { v: String },
    Call { n: Box<Expr>, args: Vec<Expr> },
    Ident { n: String },
    LessThan { lhs: Box<Expr>, rhs: Box<Expr> },
    Add { lhs: Box<Expr>, rhs: Box<Expr> },
    Sub { lhs: Box<Expr>, rhs: Box<Expr> },
}