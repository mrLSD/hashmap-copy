extern crate time;

use std::collections::HashMap;

use std::sync::{Arc, Mutex};
use std::thread;

type BoxHashMap = Box<HashMap<String, i32>>;

fn main() {
    let mut m: BoxHashMap = Box::new(HashMap::new());
    let start = time::PreciseTime::now();
    for i in 0..10_000 {
        m.insert(format!("key{:?}", i), i);
    }

    let t = start.to(time::PreciseTime::now()).num_microseconds();
    println!("map2: {:?} length | generated: {:?} mic.sec", (*m).len(), t);

    let start = time::PreciseTime::now();
    let mut m2 = m.clone();
    *m2.get_mut("key10").unwrap() = 300;
    let t = start.to(time::PreciseTime::now()).num_microseconds();
    println!("map2: {:?} length | generated: {:?} mic.sec", (*m2).len(), t);
    println!("map1[key10]: {:?} | map2[key10]: {:?}", *m.get("key10").unwrap(), *m2.get("key10").unwrap());

    let data = Arc::new(Mutex::new(m));

    for _ in 0..10 {
        let start = time::PreciseTime::now();
        let m4 = m2.clone();
        let t = start.to(time::PreciseTime::now()).num_microseconds().unwrap();
        println!("map3: {:?} length | generated: {:?} mic.sec", (*m4).len(), t);
    }

    let mut v = Vec::new();
    for i in 0..2 {
        let data = data.clone();
        let jh = thread::spawn(move || {
            println!("RUN: {:?}", i);
            let start = time::PreciseTime::now();

            let mut data = data.lock().unwrap();
            let mut m3 = data.clone();
            println!("map key10: {:?}", &data.get("key10").unwrap());
            *data.get_mut("key10").unwrap() += 100;
            *m3.get_mut("key10").unwrap() += 30;

            let t = start.to(time::PreciseTime::now()).num_microseconds().unwrap();
            println!("map1[key10]: {:?} | map3[key10]: {:?}", data.get("key10").unwrap(), m3.get("key10").unwrap());
            println!("map3: {:?} length | generated: {:?} mic.sec", (*data).len(), t);
            println!("DONE: {:?}", i);
        });
        v.push(jh);
    }

    for h in v {
        h.join().unwrap()
    }
}
