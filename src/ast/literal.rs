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

    pub fn to_string(&self) -> String {
        match self {
            Literal::String(s) => s.to_string(),
            Literal::Integer(i) => i.to_string(),
            Literal::Decimal(d) => d.to_string(),
        }
    }

    // TODO: Add evaluate method that checks for interpolation wihin string literals

    pub fn display(&self) -> String {
        match self {
            Literal::String(s) => format!("StringLit({})", s),
            Literal::Integer(i) => format!("IntegerLit({})", i),
            Literal::Decimal(d) => format!("DecimalLit({})", d),
        }
    }
}
