extern crate binheap;
extern crate rand;

use rand::{thread_rng, sample};

#[test]
fn it_works() {
    let mut rng = thread_rng();
    let data = sample(&mut rng, 1..1000, 100);

    let mut heap = binheap::MinBinaryHeap::new();

    // build up our heap
    for x in &data {
        heap.insert(*x);
    }

    let mut old: Option<usize> = None;
    loop {
        // extract each value
        let new = heap.extract();
        if new.is_none() {
            break;
        }

        // and ensure older value is smaller than newer one
        if let Some(old) = old {
            if let Some(new) = new {
                assert!(old < new);
            }
        }

        old = new;
    }
}
