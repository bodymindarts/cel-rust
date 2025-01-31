use crate::context::Context;
use crate::objects::CelType;
use cel_parser::Expression;

pub fn size(target: Option<&CelType>, _args: &[Expression], _context: &Context) -> CelType {
    let target = target.unwrap();
    let result = match target {
        CelType::List(l) => l.len(),
        CelType::Map(m) => m.map.len(),
        CelType::String(s) => s.len(),
        CelType::Bytes(b) => b.len(),
        _ => unreachable!(),
    };
    CelType::Integer(result as i64)
}
