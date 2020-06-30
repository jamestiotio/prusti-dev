use super::untyped;
use serde::{Deserialize, Serialize};
use super::preparser::Arg;

#[derive(Serialize, Deserialize)]
pub struct Expression {
    /// Identifier of the specification to which this expression belongs.
    pub spec_id: untyped::SpecificationId,
    /// Identifier of the expression within the specification.
    pub expr_id: untyped::ExpressionId,
}

#[derive(Serialize, Deserialize)]
pub enum AssertionKind {
    Expr(Expression),
    And(Vec<Assertion>),
    Implies(Assertion, Assertion),
    // ForAll(ForAllVars, TriggerSet, Assertion),
}

#[derive(Serialize, Deserialize)]
pub struct Assertion {
    pub kind: Box<AssertionKind>,
}

#[derive(Serialize, Deserialize)]
pub struct ForAllVars {

}

#[derive(Serialize, Deserialize)]
pub struct TriggerSet {

}

trait ToStructure<T> {
    fn to_structure(&self) -> T;
}

impl ToStructure<Expression> for untyped::Expression {
    fn to_structure(&self) -> Expression {
        Expression {
            spec_id: self.spec_id.clone(),
            expr_id: self.id.clone(),
        }
    }
}

impl ToStructure<AssertionKind> for untyped::AssertionKind {
    fn to_structure(&self) -> AssertionKind {
        use super::common::AssertionKind::*;
        match self {
            Expr(expr) => AssertionKind::Expr(expr.to_structure()),
            And(assertions) => {
                AssertionKind::And(
                    assertions.into_iter()
                              .map(|assertion| Assertion { kind: Box::new(assertion.kind.to_structure()) })
                              .collect()
                )
            }
            Implies(lhs, rhs) => AssertionKind::Implies(
                lhs.to_structure(),
                rhs.to_structure()
            ),
            // ForAll(vars, triggers, body) => AssertionKind::ForAll(
            //     vars.to_structure(),
            //     triggers.to_structure(),
            //     body.to_structure()
            // ),
            x => {
                unimplemented!("{:?}", x);
            }
        }
    }
}

impl ToStructure<Assertion> for untyped::Assertion {
    fn to_structure(&self) -> Assertion {
        Assertion {
            kind: box self.kind.to_structure(),
        }
    }
}

pub fn to_json_string(assertion: &untyped::Assertion) -> String {
    serde_json::to_string(&assertion.to_structure()).unwrap()
}

impl Assertion {
    pub fn from_json_string(json: &str) -> Self {
        serde_json::from_str(&json).unwrap()
    }
}
