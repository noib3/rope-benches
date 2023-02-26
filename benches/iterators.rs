mod common;

use common::{LARGE, MEDIUM, SMALL, TINY};
use criterion::measurement::WallTime;
use criterion::{criterion_group, criterion_main, Bencher, BenchmarkGroup, Criterion};

trait Rope {
    type Chunks<'a>: Iterator<Item = &'a str> + Clone
    where
        Self: 'a;

    type Bytes<'a>: Iterator<Item = u8> + Clone
    where
        Self: 'a;

    type Chars<'a>: Iterator<Item = char> + Clone
    where
        Self: 'a;

    type Line<'a>
    where
        Self: 'a;

    type Lines<'a>: Iterator<Item = Self::Line<'a>> + Clone
    where
        Self: 'a;

    fn from_str(s: &str) -> Self;

    fn chunks(&self) -> Self::Chunks<'_>;

    fn bytes(&self) -> Self::Bytes<'_> {
        unimplemented!();
    }

    fn chars(&self) -> Self::Chars<'_> {
        unimplemented!();
    }

    fn lines(&self) -> Self::Lines<'_>;
}

impl Rope for crop::Rope {
    type Chunks<'a> = crop::iter::Chunks<'a>;
    type Bytes<'a> = crop::iter::Bytes<'a>;
    type Chars<'a> = crop::iter::Chars<'a>;
    type Line<'a> = crop::RopeSlice<'a>;
    type Lines<'a> = crop::iter::Lines<'a>;

    #[inline]
    fn from_str(s: &str) -> Self {
        s.into()
    }

    #[inline]
    fn chunks(&self) -> Self::Chunks<'_> {
        self.chunks()
    }

    #[inline]
    fn bytes(&self) -> Self::Bytes<'_> {
        self.bytes()
    }

    #[inline]
    fn chars(&self) -> Self::Chars<'_> {
        self.chars()
    }

    #[inline]
    fn lines(&self) -> Self::Lines<'_> {
        self.lines()
    }
}

impl Rope for ropey::Rope {
    type Chunks<'a> = ropey::iter::Chunks<'a>;
    type Bytes<'a> = ropey::iter::Bytes<'a>;
    type Chars<'a> = ropey::iter::Chars<'a>;
    type Line<'a> = ropey::RopeSlice<'a>;
    type Lines<'a> = ropey::iter::Lines<'a>;

    #[inline]
    fn from_str(s: &str) -> Self {
        Self::from_str(s)
    }

    #[inline]
    fn chunks(&self) -> Self::Chunks<'_> {
        self.chunks()
    }

    #[inline]
    fn bytes(&self) -> Self::Bytes<'_> {
        self.bytes()
    }

    #[inline]
    fn chars(&self) -> Self::Chars<'_> {
        self.chars()
    }

    #[inline]
    fn lines(&self) -> Self::Lines<'_> {
        self.lines()
    }
}

/// A wrapper around [`xi_rope::rope::Lines`] that implements `Clone`.
struct XiRopeLines<'a> {
    rope: &'a xi_rope::Rope,
    lines: xi_rope::rope::Lines<'a>,
}

impl Clone for XiRopeLines<'_> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            rope: self.rope,
            lines: self.rope.lines(..),
        }
    }
}

impl<'a> Iterator for XiRopeLines<'a> {
    type Item = std::borrow::Cow<'a, str>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.lines.next()
    }
}

/// A wrapper around [`xi_rope::rope::ChunkIter`] that implements `Clone`.
struct XiRopeChunks<'a> {
    rope: &'a xi_rope::Rope,
    chunks: xi_rope::rope::ChunkIter<'a>,
}

impl Clone for XiRopeChunks<'_> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            rope: self.rope,
            chunks: self.rope.iter_chunks(..),
        }
    }
}

impl<'a> Iterator for XiRopeChunks<'a> {
    type Item = &'a str;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.chunks.next()
    }
}

impl Rope for xi_rope::Rope {
    type Chunks<'a> = XiRopeChunks<'a>;
    type Bytes<'a> = std::str::Bytes<'a>;
    type Chars<'a> = std::str::Chars<'a>;
    type Line<'a> = std::borrow::Cow<'a, str>;
    type Lines<'a> = XiRopeLines<'a>;

    #[inline]
    fn from_str(s: &str) -> Self {
        s.into()
    }

    #[inline]
    fn chunks(&self) -> Self::Chunks<'_> {
        XiRopeChunks {
            rope: self,
            chunks: self.iter_chunks(..),
        }
    }

    #[inline]
    fn lines(&self) -> Self::Lines<'_> {
        XiRopeLines {
            rope: self,
            lines: self.lines(..),
        }
    }
}

fn bench_chunks<R: Rope>(group: &mut BenchmarkGroup<WallTime>) {
    #[inline]
    fn bench<R: Rope>(bench: &mut Bencher, s: &str) {
        let r = R::from_str(s);
        let mut chunks = r.chunks().cycle();
        bench.iter(|| {
            let _ = chunks.next();
        });
    }

    group.bench_function("create", |bench| {
        let rope = R::from_str(LARGE);
        bench.iter(|| {
            let _ = rope.chunks();
        });
    });

    group.bench_function("tiny", |b| bench::<R>(b, TINY));
    group.bench_function("small", |b| bench::<R>(b, SMALL));
    group.bench_function("medium", |b| bench::<R>(b, MEDIUM));
    group.bench_function("large", |b| bench::<R>(b, LARGE));
}

fn bench_bytes<R: Rope>(group: &mut BenchmarkGroup<WallTime>) {
    #[inline]
    fn bench<R: Rope>(bench: &mut Bencher, s: &str) {
        let r = R::from_str(s);
        let mut bytes = r.bytes().cycle();
        bench.iter(|| {
            let _ = bytes.next();
        });
    }

    group.bench_function("create", |bench| {
        let rope = R::from_str(LARGE);
        bench.iter(|| {
            let _ = rope.bytes();
        });
    });

    group.bench_function("tiny", |b| bench::<R>(b, TINY));
    group.bench_function("small", |b| bench::<R>(b, SMALL));
    group.bench_function("medium", |b| bench::<R>(b, MEDIUM));
    group.bench_function("large", |b| bench::<R>(b, LARGE));
}

fn bench_chars<R: Rope>(group: &mut BenchmarkGroup<WallTime>) {
    #[inline]
    fn bench<R: Rope>(bench: &mut Bencher, s: &str) {
        let r = R::from_str(s);
        let mut chars = r.chars().cycle();
        bench.iter(|| {
            let _ = chars.next();
        });
    }

    group.bench_function("create", |bench| {
        let rope = R::from_str(LARGE);
        bench.iter(|| {
            let _ = rope.chars();
        });
    });

    group.bench_function("tiny", |b| bench::<R>(b, TINY));
    group.bench_function("small", |b| bench::<R>(b, SMALL));
    group.bench_function("medium", |b| bench::<R>(b, MEDIUM));
    group.bench_function("large", |b| bench::<R>(b, LARGE));
}

fn bench_lines<R: Rope>(group: &mut BenchmarkGroup<WallTime>) {
    fn bench<R: Rope>(bench: &mut Bencher, s: &str) {
        let r = R::from_str(s);
        let mut lines = r.lines().cycle();
        bench.iter(|| {
            let _ = lines.next();
        });
    }

    group.bench_function("create", |bench| {
        let rope = R::from_str(LARGE);
        bench.iter(|| {
            let _ = rope.lines();
        });
    });

    group.bench_function("tiny", |b| bench::<R>(b, TINY));
    group.bench_function("small", |b| bench::<R>(b, SMALL));
    group.bench_function("medium", |b| bench::<R>(b, MEDIUM));
    group.bench_function("large", |b| bench::<R>(b, LARGE));
}

fn crop_iter_chunks(c: &mut Criterion) {
    let mut group = c.benchmark_group("crop_iter_chunks");
    bench_chunks::<crop::Rope>(&mut group);
}

fn crop_iter_bytes(c: &mut Criterion) {
    let mut group = c.benchmark_group("crop_iter_bytes");
    bench_bytes::<crop::Rope>(&mut group);
}

fn crop_iter_chars(c: &mut Criterion) {
    let mut group = c.benchmark_group("crop_iter_chars");
    bench_chars::<crop::Rope>(&mut group);
}

fn crop_iter_lines(c: &mut Criterion) {
    let mut group = c.benchmark_group("crop_iter_lines");
    bench_lines::<crop::Rope>(&mut group);
}

fn ropey_iter_chunks(c: &mut Criterion) {
    let mut group = c.benchmark_group("ropey_iter_chunks");
    bench_chunks::<ropey::Rope>(&mut group);
}

fn ropey_iter_bytes(c: &mut Criterion) {
    let mut group = c.benchmark_group("ropey_iter_bytes");
    bench_bytes::<ropey::Rope>(&mut group);
}

fn ropey_iter_chars(c: &mut Criterion) {
    let mut group = c.benchmark_group("ropey_iter_chars");
    bench_chars::<ropey::Rope>(&mut group);
}

fn ropey_iter_lines(c: &mut Criterion) {
    let mut group = c.benchmark_group("ropey_iter_lines");
    bench_lines::<ropey::Rope>(&mut group);
}

fn xi_rope_iter_chunks(c: &mut Criterion) {
    let mut group = c.benchmark_group("xi_rope_iter_chunks");
    bench_chunks::<xi_rope::Rope>(&mut group);
}

fn xi_rope_iter_lines(c: &mut Criterion) {
    let mut group = c.benchmark_group("xi_rope_iter_lines");
    bench_lines::<xi_rope::Rope>(&mut group);
}

criterion_group!(
    benches,
    crop_iter_chunks,
    crop_iter_bytes,
    crop_iter_chars,
    crop_iter_lines,
    ropey_iter_chunks,
    ropey_iter_bytes,
    ropey_iter_chars,
    ropey_iter_lines,
    xi_rope_iter_chunks,
    xi_rope_iter_lines,
);

criterion_main!(benches);
