/// A single element in an HCL AST.
#[derive(Debug)]
pub enum Node {
    /// A boolean: `true` or `false`.
    Boolean(bool),
    /// A floating point number.
    Float(f64),
    /// An integer.
    Integer(i64),
    /// A heterogeneous list.
    List(Vec<Box<Node>>),
    /// A map of key-value pairs.
    Object(Vec<ObjectItem>),
    /// A string of characters.
    String(String),
}

/// HCL's special syntax for a possibly-nested key-value pair.
#[derive(Debug)]
pub enum ObjectItem {
    /// A single key-value pair inside an object.
    Assignment(Key, Node),
    /// A value nested within multiple objects.
    Nested(Vec<Key>, Node),
}

/// An [ObjectItem] key, which be either a bare identifier or a string.
#[derive(Debug)]
pub enum Key {
    /// A bare identifier.
    Ident(String),
    /// A double-quoted string.
    String(String),
}
