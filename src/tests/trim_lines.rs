pub trait TrimLines {
    fn trim_lines(self) -> String;
}

impl TrimLines for &str {
    fn trim_lines(self) -> String {
        let lines: Vec<String> = self
            .trim()
            .lines()
            .map(|line| line.trim().to_string())
            .collect();
        lines.join("\n") + "\n"
    }
}
