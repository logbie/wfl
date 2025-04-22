use std::fmt;

pub struct TestWriter {
    pub output: String,
}

impl fmt::Write for TestWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.output.push_str(s);
        Ok(())
    }
}

pub struct TestFormatter<'a> {
    writer: &'a mut TestWriter,
}

impl<'a> TestFormatter<'a> {
    pub fn new(writer: &'a mut TestWriter) -> Self {
        Self { writer }
    }
}

impl fmt::Write for TestFormatter<'_> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.writer.write_str(s)
    }
}
