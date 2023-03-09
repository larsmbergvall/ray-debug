use backtrace::{Backtrace, BacktraceSymbol};
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub struct Origin {
    pub function_name: String,
    pub file: String,
    pub line_number: usize,
    pub hostname: String,
}

impl Default for Origin {
    fn default() -> Self {
        let hostname = gethostname::gethostname()
            .to_str()
            .unwrap_or("Unknown")
            .to_string();

        Self {
            function_name: "Unknown".to_string(),
            file: "Unknown".to_string(),
            line_number: 0,
            hostname,
        }
    }
}

impl From<BacktraceSymbol> for Origin {
    fn from(value: BacktraceSymbol) -> Self {
        let function_name = match value.name() {
            Some(name) => name.to_string(),
            None => "Unknown".to_string(),
        };

        let filename = match value.filename() {
            Some(path) => path.to_str().unwrap_or("Unknown").to_string(),
            None => "No file name available".to_string(),
        };

        let line_number = value.lineno().unwrap_or(0);

        Self::new(function_name, filename, line_number as usize)
    }
}

impl Origin {
    pub fn capture(place_in_stack: Option<usize>) -> Self {
        if let Some(symbol) = Self::get_symbols(place_in_stack.unwrap_or(6)) {
            return Self::from(symbol);
        }

        Self::default()
    }

    pub fn test() -> Self {
        Self {
            function_name: "test_function".to_string(),
            file: "file_name.rs".to_string(),
            line_number: 0,
            hostname: "test_hostname".to_string(),
        }
    }

    pub fn new<T: Into<String>>(function_name: T, file: T, line_number: usize) -> Self {
        let hostname = gethostname::gethostname()
            .to_str()
            .unwrap_or("Unknown")
            .to_string();

        Self {
            function_name: function_name.into(),
            file: file.into(),
            line_number,
            hostname,
        }
    }

    fn get_symbols(place_in_stack: usize) -> Option<BacktraceSymbol> {
        let trace = Backtrace::new();

        // About place_in_stack:
        //
        // This is normally 4 because the first place in our code is this function (trace = Backtrace::new()),
        // the second is Self::capture(), and the third should be whatever called Self::capture(). Symbols
        // found before this is rusts internal stuff and external crates, and should be ignored.
        //
        // TODO: Make sure this is actually correct.

        let mut found_symbols = None;
        let mut found = 0;

        for f in trace.frames() {
            for symbol in f.symbols() {
                if let Some(file_path) = symbol.filename() {
                    let as_string = file_path.to_str().unwrap().to_string();

                    if !as_string.contains("/.cargo/registry") {
                        found += 1;

                        if found == place_in_stack {
                            found_symbols = Some(symbol);
                            break;
                        }
                    }
                }
            }
        }

        found_symbols.map(|v| v.to_owned())
    }
}
