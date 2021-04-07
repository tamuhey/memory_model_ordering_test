use rand::{thread_rng, Rng};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::{thread, time::Duration};

static mut v: bool = true;

fn sleep() {
    let mut rng = thread_rng();
    let t = rng.gen_range(100..1000);
    thread::sleep(Duration::from_nanos(t));
}

fn main() {
    let ntry = 100_000;
    let mut ret = [0usize, 0usize];
    for _ in 0..ntry {
        unsafe {
            v = true;
        }
        let x = Arc::new(AtomicBool::new(false));
        let y = Arc::new(AtomicBool::new(false));
        let th1 = {
            let x = Arc::clone(&x);
            let y = Arc::clone(&y);
            thread::spawn(move || {
                sleep();
                let r1 = y.load(Ordering::Relaxed);
                x.store(r1, Ordering::Relaxed);
                unsafe {
                    v &= r1;
                };
            })
        };
        let th2 = {
            let x = Arc::clone(&x);
            let y = Arc::clone(&y);
            thread::spawn(move || {
                sleep();
                let r2 = x.load(Ordering::Relaxed);
                y.store(true, Ordering::Relaxed);
                unsafe {
                    v &= r2;
                };
            })
        };
        th1.join().unwrap();
        th2.join().unwrap();
        unsafe {
            ret[v as usize] += 1;
        }
    }
    println!("{:?}", ret);
}
