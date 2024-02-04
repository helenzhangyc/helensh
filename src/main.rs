use chrono::{Datelike, Utc, Weekday};
use shrs::prelude::{cursor_buffer::CursorBuffer, styled_buf::StyledBuf, *};

struct MyPrompt {}

impl MyPrompt {
    pub fn new() -> Self {
        Self {}
    }
}

impl Prompt for MyPrompt {
    fn prompt_left(&self, line_ctx: &LineCtx) -> styled_buf::StyledBuf {
        let now = Utc::now();
        let days_till_fri = (7 + 4 - now.weekday().num_days_from_monday()) % 7;
        styled_buf!("Friday in ", days_till_fri.to_string(), " days > ")
    }
    fn prompt_right(&self, line_ctx: &LineCtx) -> styled_buf::StyledBuf {
        styled_buf!(" ")
    }
}

fn main() {
    let prompt = MyPrompt::new();

    let readline = LineBuilder::default()
        .with_prompt(prompt)
        .build()
        .expect("Could not construct readline");
    let myshell = ShellBuilder::default()
        .with_readline(readline)
        .build()
        .unwrap();

    myshell.run();
}
