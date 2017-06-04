#![feature(test)]

extern crate test;
extern crate binheap;
extern crate rand;

#[cfg(test)]
mod tests {
    use rand::{thread_rng, sample};
    use binheap::*;
    use test::Bencher;

    fn gen_sample() -> Vec<usize> {
        let mut rng = thread_rng();
        sample(&mut rng, 1..1000, 100)
    }

    #[bench]
    fn bench_extract_min(b: &mut Bencher) {
        let data = gen_sample();

        b.iter(|| {
            let mut heap = MinBinaryHeap::new();
            for x in &data {
                heap.insert(x);
            }
            heap.root();
        });
    }

    #[bench]
    fn bench_min_iter(b: &mut Bencher) {
        let data = gen_sample();

        b.iter(|| {
            let mut new_data = data.clone();
            new_data.sort();
            new_data[0];
        });
    }
}
