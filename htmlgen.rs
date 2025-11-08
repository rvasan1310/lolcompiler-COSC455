#[derive(Default)]
pub struct Html {
    parts: Vec<String>,
    had_head: bool,
    in_body: bool,
    // When true: free text is emitted one word per line (matches prof's samples).
    // We disable this inside <p>…</p> so your paragraph output stays compact.
    word_per_line: bool,
}

impl Html {
    pub fn push<S: Into<String>>(&mut self, s: S) { self.parts.push(s.into()); }

    pub fn begin_html(&mut self) {
        self.push("<!doctype html>");
        self.push("<html>");
        // If the user never creates a head, we'll still allow body later.
    }
    pub fn end_html(&mut self) { self.push("</html>"); }

    pub fn begin_head(&mut self) {
        self.had_head = true;
        self.push("<head>");
    }
    pub fn end_head(&mut self) { self.push("</head>"); }

    pub fn title(&mut self, t: &str) {
        self.push(format!("<title>{}</title>", t.trim()));
    }

    pub fn begin_body(&mut self) {
        if !self.in_body {
            self.in_body = true;
            self.word_per_line = true; // default for plain body text
            self.push("<body>");
        }
    }
    pub fn end_body(&mut self) {
        if self.in_body {
            self.in_body = false;
            self.push("</body>");
        }
    }

    pub fn comment(&mut self, t: &str) {
        self.push(format!("<!-- {} -->", t.trim()));
    }

    pub fn begin_p(&mut self) {
        self.begin_body();              // ensure we're in <body>
        self.push("<p>");
        self.word_per_line = false;     // paragraphs should not be word-per-line
    }
    pub fn end_p(&mut self) {
        self.push("</p>");
        self.word_per_line = true;      // restore default for following free text
    }

    pub fn br(&mut self) {
        self.begin_body();
        self.push("<br>");
    }

    pub fn bold(&mut self, t: &str) {
        self.begin_body();
        self.push(format!("<b>{}</b>", t.trim()));
    }
    pub fn italics(&mut self, t: &str) {
        self.begin_body();
        self.push(format!("<i>{}</i>", t.trim()));
    }

    pub fn text(&mut self, t: &str) {
        self.begin_body();
        let trimmed = t.replace('\r', " ").replace('\n', " ");
        if trimmed.trim().is_empty() { return; }

        if self.word_per_line {
            // Emit each word on its own source line (browser still renders inline).
            for w in trimmed.split_whitespace() {
                self.push(w.to_string());
            }
        } else {
            // Inside <p>…</p> keep it compact
            self.push(trimmed.trim().to_string());
        }
    }

    pub fn finish(mut self) -> String {
        // If no explicit <body> was opened, open & close it so HTML is valid.
        if !self.in_body {
            // Only create an empty body if nothing else already did.
            // (If begin_body was called, in_body would be true.)
            self.in_body = true;
            self.push("<body>");
            self.push("</body>");
        } else {
            // Ensure body is closed if user code forgot.
            if !self.parts.last().map_or(false, |s| s == "</body>") {
                self.push("</body>");
            }
        }
        self.end_html();
        self.parts.join("\n")
    }
}
