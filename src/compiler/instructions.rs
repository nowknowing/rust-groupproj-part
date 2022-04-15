#[derive(Debug)]
pub enum Instruction {
    START,
    LDCI(i64),
    LDCB(bool),
    LDCU,
    LD(usize),
    // Binary operations.
    PLUS,
    MINUS,
    TIMES,
    DIV,
    EQUAL,
    GREATER,
    GEQ,
    LESS,
    LEQ,
    AND,
    OR,
    // Unary operations.
    NOT,
    UMINUS,
    // Others.
    POP,
    ASSIGN(usize),
    LDF(usize),
    CALL(usize),
    RTN,
    DONE
}

// TODO: Implement the following opcodes.
// Load String.
// ImmutableBorrow,
// MutableBorrow,
// Dereference,
// StringFrom,
// Drop,
// Len,
// AsStr,
// PushStr,
// Println,