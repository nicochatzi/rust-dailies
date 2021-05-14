#[derive(Debug)]
struct Range {
    start: u32,
    end: u32,
}

impl Range {
    fn new(start: u32, end: u32) -> Self {
        Self { start, end }
    }

    fn with_end(end: u32) -> Self {
        Self { start: 0, end }
    }

    fn len(&self) -> u32 {
        self.end - self.start
    }

    fn trim_end(&mut self, n: u32) {
        self.end -= n;
    }
}

struct RangeBuilder {
    range: Range,
}

impl RangeBuilder {
    fn new() -> Self {
        Self {
            range: Range::new(0, 0),
        }
    }

    fn start(mut self, start: u32) -> Self {
        self.range.start = start;
        self
    }

    fn end(mut self, end: u32) -> Self {
        self.range.end = end;
        self
    }

    fn build(self) -> Range {
        self.range
    }
}

fn main() {
    let mut range = RangeBuilder::new().start(0).end(5).build();
    println!("{:#?}", range);

    println!("length: {}", range.len());

    println!("length: {}", range.len());

    range.trim_end(4);
    println!("length: {}", range.len());
}
