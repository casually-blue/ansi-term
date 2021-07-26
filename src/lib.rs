#[derive(PartialEq,Eq,Debug)]
pub enum AnsiGeneral {
    Bell,
    Backspace,
    HTab,
    Newline,
    VTab,
    Newpage,
    CarriageReturn,
    Escape,
    Delete,
}

impl std::fmt::Display for AnsiGeneral {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use AnsiGeneral::*;
        write!(f, "{}", match self {
            Bell => "\x07",
            Backspace => "\x08",
            HTab => "\x09",
            Newline => "\x0A",
            VTab => "\x0B",
            Newpage => "\x0C",
            CarriageReturn => "\x0D",
            Escape => "\x1B",
            Delete => "\x7F",
        })
    }
}

#[derive(PartialEq,Eq,Debug)]
pub enum CursorControls {
    Home,
    Location(u16,u16),
    Up(u16),
    Down(u16),
    Right(u16),
    Left(u16),
    BeginningNextN(u16),
    BeginningLastN(u16),
    Column(u16),
    RequestPos,
    SavePos,
    RestorePos,
}

impl std::fmt::Display for CursorControls {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use CursorControls::*;
        write!(f, "{}[{}", AnsiGeneral::Escape, match self {
            Home => format!("H"),
            Location(line,col) => format!("{};{}f", line, col),
            Up(n) => format!("{}A", n),
            Down(n) => format!("{}B", n),
            Left(n) => format!("{}C", n),
            Right(n) => format!("{}D", n),
            BeginningNextN(n) => format!("{}E", n),
            BeginningLastN(n) => format!("{}F", n),
            Column(n) => format!("{}G", n),
            RequestPos => format!("6n"),
            SavePos => format!("s"),
            RestorePos => format!("u"),
        })
    }
}

#[derive(PartialEq,Eq,Debug)]
pub enum EraseCodes {
    Clear,
    ClearToEOS,
    ClearFromBOS,
    ClearEntireScreen,
    ClearCurrentLine,
    ClearToEOL,
    ClearFromBOL,
    ClearEntireLine,
}

impl std::fmt::Display for EraseCodes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use EraseCodes::*;

        write!(f, "{}[{}", AnsiGeneral::Escape, match self {
            Clear => "J",
            ClearToEOS => "0J",
            ClearFromBOS => "1J",
            ClearEntireScreen => "2J",
            ClearCurrentLine => "K",
            ClearToEOL => "0K",
            ClearFromBOL => "1K",
            ClearEntireLine => "2K",
        })
    }
}




#[cfg(test)]
mod tests {
    #[test]
    fn test_term_general_codes() {
        use crate::AnsiGeneral::*;

        assert_eq!(Bell.to_string(), "\x07");
        assert_eq!(Backspace.to_string(), "\x08");
    }
}
