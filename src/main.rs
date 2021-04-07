use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::{thread, time::Duration};

static mut ret: [usize; 2] = [0, 0];
fn main() {
    let ntry = 10_000;
    for _ in 0..ntry {
        let x = Arc::new(AtomicUsize::new(0));
        let y = Arc::new(AtomicUsize::new(0));
        let th1 = {
            let x = Arc::clone(&x);
            let y = Arc::clone(&y);
            thread::spawn(move || {
                let r1 = y.load(Ordering::Relaxed);
                x.store(r1, Ordering::Relaxed);
                unsafe {
                    ret[r1] += 1;
                };
            })
        };
        let th2 = {
            let x = Arc::clone(&x);
            let y = Arc::clone(&y);
            thread::spawn(move || {
                let r2 = x.load(Ordering::Relaxed);
                y.store(1, Ordering::Relaxed);
                unsafe {
                    ret[r2] += 1;
                };
            })
        };
        th1.join().unwrap();
        th2.join().unwrap();
    }
    unsafe {
        println!("{:?}", ret);
    }
}
