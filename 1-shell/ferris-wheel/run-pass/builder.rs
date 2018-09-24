// FIXME: Make me pass! Diff budget: 30 lines.

struct Builder {
    string: Option<String>,
    number: Option<usize>,
}

impl Builder {
    pub fn default() -> Builder {
        Builder {
            string: None,
            number: None
        }
    }

    pub fn string<T: Into<String>>(&mut self, string: T) -> &mut Self {
        self.string = Some(string.into());
        self
    }

    pub fn number(&mut self, number: usize) -> &mut Self {
        self.number = Some(number);
        self
    }

    pub fn to_string(&self) -> String {
        match (&self.string, &self.number) {
            (Some(string), Some(number)) => format!("{} {}", string, number),
            (Some(string), None)         => format!("{}", string),
            (None, Some(number))         => format!("{}", number),
            (None, None)                 => "".to_string()
        }
    }
}

// Do not modify this function.
fn main() {
    let empty = Builder::default().to_string();
    assert_eq!(empty, "");

    let just_str = Builder::default().string("hi").to_string();
    assert_eq!(just_str, "hi");

    let just_num = Builder::default().number(254).to_string();
    assert_eq!(just_num, "254");

    let a = Builder::default()
        .string("hello, world!")
        .number(200)
        .to_string();

    assert_eq!(a, "hello, world! 200");

    let b = Builder::default()
        .string("hello, world!")
        .number(200)
        .string("bye now!")
        .to_string();

    assert_eq!(b, "bye now! 200");

    let c = Builder::default()
        .string("heap!".to_owned())
        .to_string();

    assert_eq!(c, "heap!");
}
