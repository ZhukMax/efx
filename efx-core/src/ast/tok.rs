#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Tok {
    LAngle,       // <
    RAngle,       // >
    Slash,        // /
    LBrace,       // {
    RBrace,       // }
    Text(String), // arbitrary text without special characters
}
