use super::branch::Comparison;
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

    pub fn compare(&self, other: &Literal, cmp: Comparison) -> Result<bool, String> {
        use Comparison::*;

        match cmp {
            Unconditional => Ok(true),

            Equals => Ok(match (self, other) {
                (Literal::String(a), Literal::String(b)) => a == b,
                (Literal::Integer(a), Literal::Integer(b)) => a == b,
                (Literal::Decimal(a), Literal::Decimal(b)) => a == b,
                (Literal::Integer(a), Literal::Decimal(b)) => (*a as f64) == *b,
                (Literal::Decimal(a), Literal::Integer(b)) => *a == (*b as f64),
                _ => false,
            }),

            NotEquals => Ok(!self.compare(other, Equals)?),

            Less | Greater | LessOrEquals | GreaterOrEquals => {
                let ordering = match (self, other) {
                    (Literal::Integer(a), Literal::Integer(b)) => a.partial_cmp(b),
                    (Literal::Decimal(a), Literal::Decimal(b)) => a.partial_cmp(b),
                    (Literal::Integer(a), Literal::Decimal(b)) => (*a as f64).partial_cmp(b),
                    (Literal::Decimal(a), Literal::Integer(b)) => a.partial_cmp(&(*b as f64)),
                    (Literal::String(a), Literal::String(b)) => Some(a.cmp(b)),
                    _ => {
                        return Err(format!(
                            "Cannot compare {} with {}",
                            self.display(),
                            other.display()
                        ));
                    }
                };

                let ord = ordering.ok_or_else(|| {
                    format!(
                        "Comparison failed between {} and {}",
                        self.display(),
                        other.display()
                    )
                })?;

                Ok(match cmp {
                    Less => ord.is_lt(),
                    Greater => ord.is_gt(),
                    LessOrEquals => ord.is_le(),
                    GreaterOrEquals => ord.is_ge(),
                    _ => unreachable!(),
                })
            }
        }
    }

    pub fn evaluate(&self, ctx: &Context) -> Option<String> {
        let lit = self.evaluate_to_lit(ctx);
        lit.map(|l| l.to_string())
    }

    pub fn evaluate_to_lit(&self, ctx: &Context) -> Option<Literal> {
        match self {
            Literal::String(s) => {
                let mut failed = false;

                let result = INTERPOLATE_DIRECTIVE_RE.replace_all(s, |caps: &Captures| {
                    let replacement = (|| {
                        let key = &caps[1];
                        let lit = ctx.call_stack.get_current_frame()?.resolve_to_lit(key)?;
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
                    Some(Literal::String(result.to_string()))
                }
            }

            Literal::Integer(i) => Some(Literal::Integer(*i)),
            Literal::Decimal(d) => Some(Literal::Decimal(*d)),
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
