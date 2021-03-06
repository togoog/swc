use super::{Expr, Function, PropName};
use swc_common::{ast_node, Fold, Span};

#[ast_node]
pub struct Class {
    pub span: Span,

    pub body: Vec<ClassMethod>,
    pub super_class: Option<Box<Expr>>,
}

#[ast_node]
pub struct ClassMethod {
    pub span: Span,
    pub key: PropName,

    pub function: Function,

    #[fold(ignore)]
    pub kind: ClassMethodKind,

    pub is_static: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Fold)]
pub enum ClassMethodKind {
    Constructor,
    Method,
    Getter,
    Setter,
}
