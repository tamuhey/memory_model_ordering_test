use rand::{thread_rng, Rng};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::{thread, time::Duration};

fn sleep() {
    let mut rng = thread_rng();
    let t = rng.gen_range(100..120);
    thread::sleep(Duration::from_nanos(t));
}

fn main() {
    let ntry = 100_000;
    for _ in 0..ntry {
        let x = Arc::new(AtomicBool::new(false));
        let y = Arc::new(AtomicBool::new(false));
        let z = Arc::new(AtomicBool::new(false));

        let th1 = {
            let x = Arc::clone(&x);
            thread::spawn(move || {
                sleep();
                x.store(true, Ordering::Relaxed);
            })
        };
        let th2 = {
            let y = Arc::clone(&y);
            thread::spawn(move || {
                sleep();
                y.store(true, Ordering::Relaxed);
            })
        };
        let th3 = {
            let x = Arc::clone(&x);
            let y = Arc::clone(&y);
            let z = Arc::clone(&z);
            thread::spawn(move || {
                while !x.load(Ordering::Relaxed) {}
                if y.load(Ordering::Relaxed) {
                    z.store(true, Ordering::Relaxed);
                }
            })
        };
        let th4 = {
            let x = Arc::clone(&x);
            let y = Arc::clone(&y);
            let z = Arc::clone(&z);
            thread::spawn(move || {
                while !y.load(Ordering::Relaxed) {}
                if x.load(Ordering::Relaxed) {
                    z.store(true, Ordering::Relaxed);
                }
            })
        };
        th1.join().unwrap();
        th2.join().unwrap();
        th3.join().unwrap();
        th4.join().unwrap();
        assert!(z.load(Ordering::Relaxed));
    }
}
