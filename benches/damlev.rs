use rand::Rng;
use rand::rngs::ThreadRng;
use eddie::damlev::DamLev;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use distance;
use strsim;

const GEN_SAMPLE_SIZE: usize = 100;


struct Generator {
    pub sample: Vec<(String, String, usize)>,
    dl: DamLev,
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
        let dl = DamLev::new();
        let sample = Vec::with_capacity(GEN_SAMPLE_SIZE);
        let i = 0;
        let mut gen = Generator { dl, rng, len, edits, chars, sample, i };
        gen.fill();
        gen
    }

    pub fn next_i(&mut self) -> usize {
        let i = self.i;
        if self.i >= self.sample.len() { self.i = 0; }
        i
    }

    fn fill(&mut self) -> &mut Self {
        for _ in 0..GEN_SAMPLE_SIZE {
            let w1 = self.gen_word();
            let w2 = self.edit(&w1, self.edits);
            let d = self.dl.dist(&w1, &w2);
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
            let case = rng.gen_range(0, 4);
            match case {
                0 => { word.insert(i, c); }
                1 => { word.remove(i); }
                2 => { word[i] = c; }
                3 => {
                    if word.len() < 2 { continue; }
                    let c1 = word.remove(clamp(i, &word));
                    let c2 = word.remove(clamp(i, &word));
                    word.insert(clamp(i, &word), c2);
                    word.insert(clamp(i, &word), c1);
                }
                x => { panic!("Invalid case: {}", x); }
            }
        }
        word.iter().collect()
    }
}


fn clamp(n: usize, word: &Vec<char>) -> usize {
    let len = word.len();
    if len == 0 { return 0; }
    if n > len - 1 { return len - 1; }
    n
}


pub fn criterion_benchmark(criterion: &mut Criterion) {
    for len in [3, 6, 9, 12].iter() {
        let mut gen = Generator::new(*len, 2);

        criterion.bench_function(&format!("eddie / damlev / len={}", len), |bench| {
            let dl = DamLev::new();
            bench.iter(|| {
                let i = gen.next_i();
                let (s1, s2, d_expected) = &gen.sample[i];
                let d_received = dl.dist(black_box(&s1), black_box(&s2));
                (d_expected, d_received);
            })
        });

        criterion.bench_function(&format!("strsim / damlev / len={}", len), |bench| {
            bench.iter(|| {
                let i = gen.next_i();
                let (s1, s2, d_expected) = &gen.sample[i];
                let d_received = strsim::damerau_levenshtein(black_box(&s1), black_box(&s2));
                (d_expected, d_received);
            })
        });

        criterion.bench_function(&format!("distance / damlev / len={}", len), |bench| {
            bench.iter(|| {
                let i = gen.next_i();
                let (s1, s2, d_expected) = &gen.sample[i];
                let d_received = distance::damerau_levenshtein(black_box(&s1), black_box(&s2));
                (d_expected, d_received);
            })
        });
    }
}


criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);