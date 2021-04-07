use rand::{thread_rng, Rng};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::{thread, time::Duration};

fn sleep() {
    let mut rng = thread_rng();
    let t = rng.gen_range(100..1000);
    thread::sleep(Duration::from_nanos(t));
}

fn main() {
    let ntry = 100_000;
    for _ in 0..ntry {
        let x = Arc::new(AtomicBool::new(false));
        let lock = Arc::new(AtomicBool::new(false));
        let th1 = {
            let x = Arc::clone(&x);
            let lock = Arc::clone(&lock);
            thread::spawn(move || {
                x.store(true, Ordering::Relaxed);
                lock.store(true, Ordering::Relaxed);
            })
        };
        let th2 = {
            let x = Arc::clone(&x);
            let lock = Arc::clone(&lock);
            thread::spawn(move || {
                while !lock.load(Ordering::Relaxed) {}
                assert!(x.load(Ordering::Relaxed));
            })
        };
        th1.join().unwrap();
        th2.join().unwrap();
    }
}
