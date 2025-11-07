// Abstract Syntax Tree definitions

use std::fmt;

#[derive(Debug, Clone)]
pub struct CompilationUnit {
    pub namespace: Option<NamespaceDeclaration>,
    pub imports: Vec<ImportDirective>,
    pub types: Vec<TypeDeclaration>,
}

#[derive(Debug, Clone)]
pub struct NamespaceDeclaration {
    pub name: QualifiedIdentifier,
}

#[derive(Debug, Clone)]
pub enum ImportDirective {
    Simple { path: QualifiedIdentifier, alias: Option<String> },
    From { module: QualifiedIdentifier, items: Vec<String> },
}

#[derive(Debug, Clone)]
pub enum TypeDeclaration {
    Class(ClassDeclaration),
    Interface(InterfaceDeclaration),
    Enum(EnumDeclaration),
    Struct(StructDeclaration),
}

#[derive(Debug, Clone)]
pub struct ClassDeclaration {
    pub attributes: Vec<Attribute>,
    pub modifiers: Vec<Modifier>,
    pub name: String,
    pub type_parameters: Vec<TypeParameter>,
    pub base_classes: Vec<Type>,
    pub members: Vec<ClassMember>,
}

#[derive(Debug, Clone)]
pub struct InterfaceDeclaration {
    pub attributes: Vec<Attribute>,
    pub modifiers: Vec<Modifier>,
    pub name: String,
    pub type_parameters: Vec<TypeParameter>,
    pub base_interfaces: Vec<Type>,
    pub members: Vec<InterfaceMember>,
}

#[derive(Debug, Clone)]
pub struct EnumDeclaration {
    pub attributes: Vec<Attribute>,
    pub modifiers: Vec<Modifier>,
    pub name: String,
    pub members: Vec<EnumMember>,
}

#[derive(Debug, Clone)]
pub struct StructDeclaration {
    pub attributes: Vec<Attribute>,
    pub modifiers: Vec<Modifier>,
    pub name: String,
    pub members: Vec<StructMember>,
}

#[derive(Debug, Clone)]
pub enum ClassMember {
    Field(FieldDeclaration),
    Property(PropertyDeclaration),
    Method(MethodDeclaration),
    Constructor(ConstructorDeclaration),
}

#[derive(Debug, Clone)]
pub enum InterfaceMember {
    Method(MethodSignature),
    Property(PropertySignature),
}

#[derive(Debug, Clone)]
pub enum StructMember {
    Field(FieldDeclaration),
    Method(MethodDeclaration),
}

#[derive(Debug, Clone)]
pub struct EnumMember {
    pub name: String,
    pub value: Option<i64>,
}

#[derive(Debug, Clone)]
pub struct MethodDeclaration {
    pub attributes: Vec<Attribute>,
    pub modifiers: Vec<Modifier>,
    pub name: String,
    pub type_parameters: Vec<TypeParameter>,
    pub parameters: Vec<Parameter>,
    pub return_type: Option<Type>,
    pub body: Block,
}

#[derive(Debug, Clone)]
pub struct MethodSignature {
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub return_type: Option<Type>,
}

#[derive(Debug, Clone)]
pub struct ConstructorDeclaration {
    pub attributes: Vec<Attribute>,
    pub modifiers: Vec<Modifier>,
    pub parameters: Vec<Parameter>,
    pub body: Block,
}

#[derive(Debug, Clone)]
pub struct PropertyDeclaration {
    pub attributes: Vec<Attribute>,
    pub modifiers: Vec<Modifier>,
    pub name: String,
    pub property_type: Type,
    pub getter: Option<Block>,
    pub setter: Option<Block>,
}

#[derive(Debug, Clone)]
pub struct PropertySignature {
    pub name: String,
    pub property_type: Type,
}

#[derive(Debug, Clone)]
pub struct FieldDeclaration {
    pub attributes: Vec<Attribute>,
    pub modifiers: Vec<Modifier>,
    pub name: String,
    pub field_type: Type,
    pub initializer: Option<Expression>,
}

#[derive(Debug, Clone)]
pub struct Parameter {
    pub name: String,
    pub param_type: Type,
    pub default_value: Option<Expression>,
}

#[derive(Debug, Clone)]
pub struct Block {
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub enum Statement {
    Expression(Expression),
    VariableDeclaration { name: String, var_type: Option<Type>, initializer: Option<Expression> },
    Assignment { target: Expression, operator: AssignmentOperator, value: Expression },
    If { condition: Expression, then_block: Block, elif_clauses: Vec<(Expression, Block)>, else_block: Option<Block> },
    While { condition: Expression, body: Block },
    For { variable: String, iterable: Expression, body: Block },
    Try { body: Block, except_clauses: Vec<ExceptClause>, finally_block: Option<Block> },
    Return(Option<Expression>),
    Yield(Option<Expression>),
    Break,
    Continue,
    Raise(Option<Expression>),
}

#[derive(Debug, Clone)]
pub struct ExceptClause {
    pub variable: Option<String>,
    pub exception_type: Option<Type>,
    pub body: Block,
}

#[derive(Debug, Clone, Copy)]
pub enum AssignmentOperator {
    Assign,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
    ModAssign,
    AndAssign,
    OrAssign,
    XorAssign,
}

#[derive(Debug, Clone)]
pub enum Expression {
    Literal(Literal),
    Identifier(String),
    MemberAccess { object: Box<Expression>, member: String },
    MethodCall { method: Box<Expression>, arguments: Vec<Argument> },
    IndexAccess { array: Box<Expression>, index: Box<Expression> },
    Cast { expression: Box<Expression>, target_type: Type },
    Binary { left: Box<Expression>, operator: BinaryOperator, right: Box<Expression> },
    Unary { operator: UnaryOperator, operand: Box<Expression> },
    Lambda { parameters: Vec<Parameter>, body: Box<Expression> },
    Conditional { condition: Box<Expression>, then_expr: Box<Expression>, else_expr: Box<Expression> },
    ArrayLiteral(Vec<Expression>),
    HashLiteral(Vec<(Expression, Expression)>),
    This,
    Super,
}

#[derive(Debug, Clone)]
pub struct Argument {
    pub name: Option<String>,
    pub value: Expression,
}

#[derive(Debug, Clone)]
pub enum Literal {
    Integer(i64),
    Float(f64),
    String(String),
    Char(char),
    Boolean(bool),
    Null,
}

#[derive(Debug, Clone, Copy)]
pub enum BinaryOperator {
    Add, Sub, Mul, Div, Mod,
    Eq, Ne, Lt, Gt, Le, Ge,
    And, Or,
}

#[derive(Debug, Clone, Copy)]
pub enum UnaryOperator {
    Not, Neg, Pos, BitwiseNot,
}

#[derive(Debug, Clone)]
pub struct Type {
    pub base: BaseType,
    pub is_array: bool,
    pub is_nullable: bool,
}

#[derive(Debug, Clone)]
pub enum BaseType {
    Primitive(PrimitiveType),
    Named { name: QualifiedIdentifier, type_arguments: Vec<Type> },
}

#[derive(Debug, Clone, Copy)]
pub enum PrimitiveType {
    Int, Long, Short, Byte,
    Float, Double, Decimal,
    Bool, Char, String,
    Object, Void,
}

#[derive(Debug, Clone)]
pub struct TypeParameter {
    pub name: String,
    pub constraint: Option<Type>,
}

#[derive(Debug, Clone)]
pub struct Attribute {
    pub name: QualifiedIdentifier,
    pub arguments: Vec<Argument>,
}

#[derive(Debug, Clone, Copy)]
pub enum Modifier {
    Public, Private, Protected, Internal,
    Static, Final, Abstract, Virtual,
    Override, Async, Partial,
}

#[derive(Debug, Clone)]
pub struct QualifiedIdentifier {
    pub parts: Vec<String>,
}

impl fmt::Display for QualifiedIdentifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.parts.join("."))
    }
}
