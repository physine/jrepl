use crate::AppState;
use crate::Context;
use crate::command_interpreter::types::EvalValue;
use crate::command_interpreter::types::Expr;

pub fn eval(_state: &AppState, _ast: &Expr, _ctx: &Context) -> EvalValue {
    EvalValue::None
}
