pub mod command;
pub mod eval;
pub mod interpreter;
pub mod lexer;
pub mod parser;
pub mod syntax_validation;
pub mod types;

use eval::eval;
use lexer::lexer;
