use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub enum CompareType {
    Ne, // Not equal
    Eq, // Equal
    Ge, // Greater or equal
    Gt, // Greater than
    Le, // Less or equal
    Lt, // Less than
}

impl CompareType {
    pub fn cmp<T: PartialOrd + PartialEq>(self, left: &T, right: &T) -> bool {
        match self {
            CompareType::Ne => left.ne(right),
            CompareType::Eq => left.eq(right),
            CompareType::Ge => left.ge(right),
            CompareType::Gt => left.gt(right),
            CompareType::Le => left.le(right),
            CompareType::Lt => left.lt(right),
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub enum OpType {
    Add, // Addition
    Sub, // Subtraction
    Mut, // Multiplication ??
    Reduce, // Reduce ?? ?? ??
    Set, // Assign
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct ConstantWrapper<T> {
    pub value: T,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "Type")]
pub enum Constant {
    Boolean(ConstantWrapper<bool>),
    Int(ConstantWrapper<i32>),
    String(ConstantWrapper<String>),
    Float(ConstantWrapper<f32>),
}

#[derive(Deserialize, Debug, Clone)]
pub enum VarType {
    Boolean,
    Int,
    String,
    Float,
    Prefab,
    Transform
}

#[derive(Deserialize, Debug, Clone)]
pub enum RefType {
    Entity,
    Quest,
    LevelPlay,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct Other {
    pub r#type: VarType,
    pub ref_type: RefType,
    pub ref_id: i64,
    pub name: String,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct Global {
    pub r#type: VarType,
    pub keyword: String,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct SelfVar {
    pub r#type: VarType,
    pub name: String,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct System {
    pub r#type: VarType,
    // TODO: Add Var substruct
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "Source")]
pub enum Var {
    Constant(Constant),
    Other(Other),
    Global(Global),
    #[serde(rename = "Self")]
    SelfVar(SelfVar),
    System(System),
}