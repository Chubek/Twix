#[derive(Debug, Clone, PartialEq)]
pub enum SedCommand {
    Substitute {
        pattern: String,
        replacement: String,
        flags: String,
    },
    Delete(String),
    Print(String),
    Add(String),
    Append(String),
    Change(String),
    Next,
    Quit,
    Label(String),
    Branch(String),
    Hold,
    Exchange,
    Read(String),
    Write(String),
    Transform {
        set1: Vec<char>,
        set2: Vec<char>,
    },
    ClearPatternSpace,
    AppendNextLine,
    PrintLineNumber,
    Insert(String),
    ChangeNext(String)
}

#[derive(Debug, Clone, PartialEq)]
pub enum SedAddress {
    Pattern(String),
    LineNo(usize),
    LastLine
}



#[derive(Debug)]
pub struct SedProgram {
    pub addr1: Option<SedAddress>,
    pub addr2: Option<SedAddress>,
    pub commands: Vec<SedCommand>,
}
