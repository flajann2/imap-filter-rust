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

