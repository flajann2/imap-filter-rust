%start Expr
%%
Stmt -> Result<, ()>:
   Keyword ';'
     | Keyword Params ';'
     | Keyword Curly
     | Keyword Params Curly
     ;

Curly -> Result<, ()>:
  '{' Stmts '}'
    ;

Params -> Result<, ()>:
  Param
    | Param ',' Params
    ;

Stmts -> Result<Vec<Stmt>, ()>:
  Stmt { Ok(vec![$1]) }
   | Stmts Stmt { recurse($1, $2) }
  ;

KeywordForAcc -> Result<Expr, ()>:
  Stmt 'LOGIN'
    | Stmt 'SERVER'
    | Stmt 'SSL'
    | Stmt 'AUTH'
    | Stmt 'PORT'
    ;
KeywordForFilt -> Result<Expr, ()>:
   Stmt 'MARK'
   | Stmt 'COPY'
   | Stmt 'MOVE'
   | Stmt 'SRCH'
   ;
KeywordForSrch -> Result<Expr, ()>:
   Stmt 'FROM'
   | Stmt 'TO'
   | Stmt 'CC'
   | Stmt 'BCC'
   | Stmt 'UNSEEN'
   | Stmt 'SEEN'
    ;

Keyword -> Result<, ()>:
   KeywrodForAcc
    | KeywordForFilt
    | KeywordForSrch
    ;

AccConfig -> Result<Expr, ()>:
  KeyAccStmt { $1 }
  | AccConfig {}
  ;
Expr -> Result<Expr, ()>:
  Term '+' Expr { Ok(Expr::Add{ span: $span, lhs: Box::new($1?), rhs: Box::new($3?) }) }
  | Term { $1 }
  ;

Term -> Result<Expr, ()>:
  Factor '*' Term { Ok(Expr::Mul{ span: $span, lhs: Box::new($1?), rhs: Box::new($3?) }) }
  | Factor { $1 }
  ;

Factor -> Result<Expr, ()>:
  '(' Expr ')' { $2 }
  | 'INT' { Ok(Expr::Number{ span: $span }) }
  ;
%%
use lrpar::Span;

fn recurse<T>(lhs: Result<Vec<T>, ()>, rhs: Result<T, ()>) -> Result<Vec<T>, ()> {
    let mut flt = lhs?;
    flt.push(rhs?);
    Ok(flt)
}

enum KeywordForCurly {
    Account(Box<Stmts>),
    Filter(Box<Stmts>),
}

#[derive(Debug)]
enum KeywordForAcc {
    Login {
        span: Span,
        name: String,
        passwd: String},
    Server {
        span: Span,
        uri: String},
    SSL {span: Span},
    TLS{span: Span},
    Auth {
        span: Span,
        auth: String},
    Port {
        span: Span,
        port: u32},
}

#[derive(Debug)]
enum KeywordForFilt {
    Mark{span: Span},
    Copy{span: Span},
    Move{span: Span},
    Srch{span: Span},
}

#[derive(Debug)]
enum KeywordForSrch {
    From{span: Span},
    To{span: Span},
    CC{span: Span},
    BCC{span: Span},
    Unseen{span: Span},
    Seen{span: Span},
}

#[derive(Debug)]
enum Keyword {
    Curly(KeywordForCurly),
    Acc(KeywordForAcc),
    Srch(KeywordForSrch),
    Filt(KeywordForFilt),
}

#[derive(Debug)]
pub enum Expr {
    Add {
        span: Span,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    Mul {
        span: Span,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    Number {
        span: Span
    }
}

