use rand::Rng;
use eddie::Levenshtein;
use rand::rngs::ThreadRng;
use std::time::Duration;
use distance;
use strsim;
use edit_distance;
use natural;
use txtdist;

use criterion::{
    criterion_group,
    criterion_main,
    Criterion,
};


pub fn leven_benchmark(cr: &mut Criterion) {
    let mut group = cr.benchmark_group("leven");
    let leven = Levenshtein::new();

    for size in &[3, 6, 9, 12, 15] {
        let mut gen = Generator::new(*size, 2);

        group.bench_with_input(
            format!("eddie size={}", size),
            size,
            |bench, _| {
                bench.iter(|| {
                    let (s1, s2, _) = &gen.next();
                    leven.dist(s1, s2)
                });
            }
        );

        group.bench_with_input(
            format!("strsim size={}", size),
            size,
            |bench, _| {
                bench.iter(|| {
                    let (s1, s2, _) = &gen.next();
                    strsim::levenshtein(s1, s2)
                });
            }
        );

        group.bench_with_input(
            format!("distance size={}", size),
            size,
            |bench, _| {
                bench.iter(|| {
                    let (s1, s2, _) = &gen.next();
                    distance::levenshtein(s1, s2)
                });
            }
        );

        group.bench_with_input(
            format!("edit_distance size={}", size),
            size,
            |bench, _| {
                bench.iter(|| {
                    let (s1, s2, _) = &gen.next();
                    edit_distance::edit_distance(s1, s2)
                });
            }
        );

        group.bench_with_input(
            format!("natural size={}", size),
            size,
            |bench, _| {
                bench.iter(|| {
                    let (s1, s2, _) = &gen.next();
                    natural::distance::levenshtein_distance(s1, s2)
                });
            }
        );

        group.bench_with_input(
            format!("txtdist size={}", size),
            size,
            |bench, _| {
                bench.iter(|| {
                    let (s1, s2, _) = &gen.next();
                    txtdist::levenshtein(s1, s2)
                });
            }
        );
    }

    group.finish();
}


criterion_group!{
    name = benches;
    config = Criterion::default()
                .warm_up_time(Duration::from_millis(20))
                .measurement_time(Duration::from_millis(50));
    targets = leven_benchmark
}

criterion_main!(benches);


const GEN_SAMPLE_SIZE: usize = 100;


struct Generator {
    pub sample: Vec<(String, String, usize)>,
    lv: Levenshtein,
    rng: ThreadRng,
    len: usize,
    edits: usize,
    chars: Vec<char>,
    i: usize,
}


impl Generator {
    pub fn new(len: usize, edits: usize) -> Generator {
        let chars = "abcdefghijklmnopqrstuvwxyz".chars().collect();
        let rng = rand::thread_rng();
        let lv = Levenshtein::new();
        let sample = Vec::with_capacity(GEN_SAMPLE_SIZE);
        let i = 0;
        let mut gen = Generator { lv, rng, len, edits, chars, sample, i };
        gen.fill();
        gen
    }

    #[inline]
    pub fn next<'a>(&'a mut self) -> &'a (String, String, usize) {
        let i = self.i;
        if self.i >= self.sample.len() { self.i = 0; }
        &self.sample[i]
    }

    fn fill(&mut self) -> &mut Self {
        for _ in 0..GEN_SAMPLE_SIZE {
            let w1 = self.gen_word();
            let w2 = self.edit(&w1, self.edits);
            let d = self.lv.dist(&w1, &w2);
            self.sample.push((w1, w2, d));
        }
        self
    }

    fn gen_word(&mut self) -> String {
        let Generator { rng, chars, len, .. } = self;
        let mut word: Vec<char> = Vec::with_capacity(*len);
        for _ in 0..*len {
            let c = chars[rng.gen_range(0, chars.len()) as usize];
            word.push(c);
        }
        word.iter().collect()
    }

    fn edit(&mut self, _word: &str, edits: usize) -> String {
        let Generator { rng, chars, .. } = self;
        let mut word: Vec<char> = _word.chars().collect();
        for _ in 0..edits {
            if word.len() == 0 { break; }
            let i = rng.gen_range(0, word.len()) as usize;
            let c = chars[rng.gen_range(0, chars.len()) as usize];
            let case = rng.gen_range(0, 3);
            match case {
                0 => { word.insert(i, c); }
                1 => { word.remove(i); }
                2 => { word[i] = c; }
                _ => { panic!("Unreachable"); }
            }
        }
        word.iter().collect()
    }
}
