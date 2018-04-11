extern crate clap;
use clap::{Arg, App};

use std::time::{Duration, Instant};
use std::thread;


fn run_load(time: u64, process: u32, delay: u64, memory: usize) {

    let mut list_threads = Vec::new();

    for _ in 0..process {

        list_threads.push(thread::spawn(move || {

            let mut arr: Vec<u64> = vec![1; 1*1024*memory];

            let now = Instant::now();

            loop {

                for i in 0..arr.len() {
                    arr[i] = arr[i] + 1;
                }

                if now.elapsed() >= Duration::new(time, 0) {
                    break
                }

                std::thread::sleep(std::time::Duration::from_millis(delay));
            }
        }));
    }

    for t in list_threads {
        let _ = t.join();
    }
}


fn main() {
    let matches = App::new("HLoad")
                        .version("1.0")
                        .author("Morozov Andrey")
                        .about("cpu load generator")
                        .arg(Arg::with_name("time")
                           .short("t")
                           .help("time execution (in sec, default: 30)")
                           .takes_value(true))
                        .arg(Arg::with_name("process")
                           .short("p")
                           .help("run simultaneous processes/threads (default: 1)")
                           .takes_value(true))
                        .arg(Arg::with_name("delay")
                           .short("d")
                           .help("delay in ms")
                           .takes_value(true))
                        .arg(Arg::with_name("memory")
                           .short("m")
                           .help("memory usage for each threads(in Kb, default: 1")
                           .takes_value(true))
                        .get_matches();


    let time: u64 = matches.value_of("time").unwrap().parse().unwrap();
    let process: u32 = matches.value_of("process").unwrap().parse().unwrap();
    let delay: u64 = matches.value_of("delay").unwrap().parse().unwrap();
    let memory: usize = matches.value_of("memory").unwrap().parse().unwrap();

    let memory = if memory <= 0 { 1 } else { memory };

    println!(" time(secs): {:?}", &time);
    println!("  delay(ms): {:?}", &delay);
    println!(" memory(kb): {:?}", &memory);
    println!("num threads: {:?}", &process);

    run_load(time, process, delay, memory);
}
