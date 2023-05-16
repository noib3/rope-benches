use std::ops::Range;

pub trait Rope: for<'a> From<&'a str> {
    const NAME: &'static str;
    const EDITS_USE_BYTE_OFFSETS: bool = false;

    fn insert(&mut self, at_offset: usize, text: &str);

    fn remove(&mut self, between_offsets: Range<usize>);

    #[inline(always)]
    fn replace(&mut self, between_offsets: Range<usize>, text: &str) {
        let Range { start, end } = between_offsets;

        if end > start {
            self.remove(start..end);
        }

        if !text.is_empty() {
            self.insert(start, text);
        }
    }

    /// The returned length is interpreted as either number of codepoints or
    /// the number of bytes depending on the value of
    /// [`EDITS_USE_BYTE_OFFSETS`](Self::EDITS_USE_BYTE_OFFSETS).
    fn len(&self) -> usize;
}

impl Rope for String {
    const NAME: &'static str = "String";
    const EDITS_USE_BYTE_OFFSETS: bool = true;

    #[inline(always)]
    fn insert(&mut self, at: usize, s: &str) {
        self.insert_str(at, s);
    }

    #[inline(always)]
    fn remove(&mut self, range: Range<usize>) {
        self.replace_range(range, "");
    }

    #[inline(always)]
    fn replace(&mut self, range: Range<usize>, s: &str) {
        self.replace_range(range, s);
    }

    #[inline(always)]
    fn len(&self) -> usize {
        self.len()
    }
}

impl Rope for crop::Rope {
    const NAME: &'static str = "crop";
    const EDITS_USE_BYTE_OFFSETS: bool = true;

    #[inline(always)]
    fn insert(&mut self, at: usize, s: &str) {
        self.insert(at, s);
    }

    #[inline(always)]
    fn remove(&mut self, range: Range<usize>) {
        self.delete(range);
    }

    #[inline(always)]
    fn replace(&mut self, range: Range<usize>, s: &str) {
        self.replace(range, s);
    }

    #[inline(always)]
    fn len(&self) -> usize {
        self.byte_len()
    }
}

impl Rope for jumprope::JumpRope {
    const NAME: &'static str = "JumpRope";

    #[inline(always)]
    fn insert(&mut self, at: usize, s: &str) {
        self.insert(at, s);
    }

    #[inline(always)]
    fn remove(&mut self, range: Range<usize>) {
        self.remove(range);
    }

    #[inline(always)]
    fn replace(&mut self, range: Range<usize>, s: &str) {
        self.replace(range, s);
    }

    #[inline(always)]
    fn len(&self) -> usize {
        self.len_chars()
    }
}

impl Rope for jumprope::JumpRopeBuf {
    // We put `Buf` before `Rope` to be able to only run the `JumpRope`
    // benchmarks by passing `JumpR***` to the CLI.
    const NAME: &'static str = "JumpBufRope";

    #[inline(always)]
    fn insert(&mut self, at: usize, s: &str) {
        self.insert(at, s);
    }

    #[inline(always)]
    fn remove(&mut self, range: Range<usize>) {
        self.remove(range);
    }

    #[inline(always)]
    fn len(&self) -> usize {
        self.len_chars()
    }
}

impl Rope for ropey::Rope {
    const NAME: &'static str = "Ropey";

    #[inline(always)]
    fn insert(&mut self, at: usize, s: &str) {
        self.insert(at, s);
    }

    #[inline(always)]
    fn remove(&mut self, range: Range<usize>) {
        self.remove(range);
    }

    #[inline(always)]
    fn len(&self) -> usize {
        self.len_chars()
    }
}

impl Rope for xi_rope::Rope {
    const NAME: &'static str = "xi_rope";
    const EDITS_USE_BYTE_OFFSETS: bool = true;

    #[inline(always)]
    fn insert(&mut self, at: usize, s: &str) {
        self.edit(at..at, s);
    }

    #[inline(always)]
    fn remove(&mut self, range: Range<usize>) {
        self.edit(range, "");
    }

    #[inline(always)]
    fn replace(&mut self, range: Range<usize>, s: &str) {
        self.edit(range, s);
    }

    #[inline(always)]
    fn len(&self) -> usize {
        self.len()
    }
}

pub struct DT {
    oplog: diamond_types::list::OpLog,
    agent: u32,
    encode_from: usize,
}

impl From<&str> for DT {
    #[inline(always)]
    fn from(s: &str) -> Self {
        let mut oplog = diamond_types::list::OpLog::new();
        let agent = oplog.get_or_create_agent_id("DT");
        let time = oplog.add_insert(agent, 0, s);
        Self {
            oplog,
            agent,
            encode_from: time,
        }
    }
}

impl Rope for DT {
    const NAME: &'static str = "DT";

    #[inline(always)]
    fn insert(&mut self, at: usize, s: &str) {
        let time = self.oplog.add_insert(self.agent, at, s);
        let _ = self.oplog.encode_from(
            diamond_types::list::encoding::EncodeOptions::default(),
            &[self.encode_from],
        );
        self.encode_from = time;
    }

    #[inline(always)]
    fn remove(&mut self, range: Range<usize>) {
        let time = self.oplog.add_delete_without_content(self.agent, range);
        let _ = self.oplog.encode_from(
            diamond_types::list::encoding::EncodeOptions::default(),
            &[self.encode_from],
        );
        self.encode_from = time;
    }

    #[inline(always)]
    fn len(&self) -> usize {
        self.oplog.checkout_tip().len()
    }
}
