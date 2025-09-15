use cxx_qt::QObject;
use evalexpr::{eval, Value};

#[derive(QObject, Default)]
pub struct Qalculator;

impl Qalculator {
    /// Evaluate mathmatical expression. If `print_expr` is true,
    /// returns "expr = result", otherwise returns the result alone.
    #[qinvokable(cpp_name = "eval")]
    pub fn eval(&self, expr: &str, print_expr: bool) -> String {
        let trimmed = expr.trim();
        if trimmed.is_empty() {
            return String::new();
        }
        match eval(trimmed) {
            Ok(value) => {
                let result = match value {
                    Value::Float(f) => f.to_string(),
                    Value::Int(i) => i.to_string(),
                    Value::Boolean(b) => b.to_string(),
                    Value::String(s) => s,
                    _ => value.to_string(),
                };
                if print_expr {
                    format!("{} = {}", trimmed, result)
                } else {
                    result
                }
            }
            Err(e) => format!("error: {}", e),
        }
    }
}

pub fn register() {
    cxx_qt::qml_register_type::<Qalculator>("Vela", 1, 0, "Qalculator");
}
