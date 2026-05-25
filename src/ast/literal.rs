use crate::config::INTERPOLATE_DIRECTIVE_RE;
use crate::context::Context;
use regex::Captures;

#[derive(Clone, Debug)]
pub enum Literal {
    String(String),
    Integer(i64),
    Decimal(f64),
}

impl Literal {
    pub fn parse(s: &str) -> Self {
        if let Ok(i) = s.parse::<i64>() {
            Literal::Integer(i)
        } else if let Ok(f) = s.parse::<f64>() {
            Literal::Decimal(f)
        } else {
            Literal::String(s.to_string())
        }
    }

    pub fn evaluate(&self, ctx: &Context) -> Option<String> {
        match self {
            Literal::String(s) => {
                let mut failed = false;

                let result = INTERPOLATE_DIRECTIVE_RE.replace_all(s, |caps: &Captures| {
                    let replacement = (|| {
                        let key = &caps[1];
                        let lit = ctx.call_stack.get_current_scope()?.resolve_to_lit(key)?;
                        lit.evaluate(ctx)
                    })();

                    match replacement {
                        Some(v) => v,
                        None => {
                            failed = true;
                            String::new()
                        }
                    }
                });

                if failed {
                    None
                } else {
                    Some(result.into_owned())
                }
            }

            Literal::Integer(i) => Some(i.to_string()),
            Literal::Decimal(d) => Some(d.to_string()),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Literal::String(s) => s.to_string(),
            Literal::Integer(i) => i.to_string(),
            Literal::Decimal(d) => d.to_string(),
        }
    }

    pub fn display(&self) -> String {
        match self {
            Literal::String(s) => format!("StringLit({})", s),
            Literal::Integer(i) => format!("IntegerLit({})", i),
            Literal::Decimal(d) => format!("DecimalLit({})", d),
        }
    }
}
