#![allow(unused, unused_imports)]
use std::{
    collections::VecDeque,
    cell::Ref,
    thread,
    sync::mpsc::{self, Sender, Receiver, TryRecvError},
    time::Duration,
    error::Error,
};
use serde_json::{ Result as Serde_Result, Value, json, Map };
use rust_multi_json_benchmark::{search_tree::depth_first_search, test_json::pc_usage_exporter::{self, PcUsage}};

fn _how_to_insert_to_json_array() {
    let index = 5;
    let mut inner_array = vec![Value::Null; index];
    inner_array.push(Value::String(String::from("a")));
    let result: Value = Value::Array(inner_array);

    println!("{}", result);
}

fn is_eq(left: &Value, right: &Value) {
    if left == right {
        println!("left=right")
    } else {
        println!("left!=right")
    }
}

fn test_eq() {
    let str = String::from("Hello");
    let ref_str = &str;
    let x = json!(5);
    let y = json!(5);
    let z = json!(4);
    let v = json!("Hello");
    let ref_v = &v;

    assert_eq!(x, y);
    assert_ne!(x, z);
    assert_ne!(y, z);

    is_eq(&x,&x);
    is_eq(&x,&y);
    is_eq(&x,&z);

    if ref_str == ref_v {
        println!("str=v")
    } else {
        println!("str!=v")
    }

    println!("is string? {}", v.is_string());
}

fn test_systeminfo() {
    /* 
    use sysinfo::{ RefreshKind, CpuRefreshKind, System, SystemExt, CpuExt };
    
    println!("Is supported? {}", System::IS_SUPPORTED);

    let refresh_kind = RefreshKind::new()
        .with_cpu(
            CpuRefreshKind::new()
                .with_cpu_usage()
        )
        .with_memory();
    let mut system = System::new_with_specifics(refresh_kind);

    loop {
        for cpu in system.cpus() {
            println!("CPU = {}%", cpu.cpu_usage());
        }
        println!("Total Memory = {}", system.total_memory());
        println!("Free Memory = {}", system.free_memory());

        system.refresh_specifics(refresh_kind);
        std::thread::sleep(std::time::Duration::from_millis(500));
        break;
    }
    */
}

fn test_self_meter() {
    use std::time::Duration;
    use std::thread::sleep;
    use std::collections::BTreeMap;

    let mut meter = self_meter::Meter::new(Duration::new(1, 0)).unwrap();
    meter.track_current_thread("main");
    for _ in 0..=1 {
        meter.scan().map_err(|error| println!("Scan error: {}", error)).ok();
        println!("Report: {:#?}", meter.report());
        println!("Threads: {:#?}", meter.thread_report().map(|x| x.collect::<BTreeMap<_,_>>()));
        sleep(Duration::new(1, 0));
    }
}

fn foo() -> i32 {
    let mut x = 0;
    for _ in 0..1_000_000 {
        x += 1;
    }

    x
}

fn test_reporter() {
    use rust_multi_json_benchmark::test_json::reporter::Report;

    let mut reporter = Report::new();

    let x = String::from("a");
    let result = reporter.measure(&x, || foo());
    let durations = reporter.get_measures();
    println!("Result: {} ; times: {:?} ; specific: {:?}", result, durations, durations.get("a"));
}

fn test_multithreading() {
    let (main_sender, thread_receiver) = mpsc::channel::<()>();
    let (thread_sender, main_receiver) = mpsc::channel();

    let my_thread = thread::spawn(move || {
        let mut index = 0;
        loop {
            thread_sender.send(format!("Hello {}", index)).unwrap();
            index += 1;

            match thread_receiver.try_recv() {
                Ok(_) | Err(TryRecvError::Disconnected) => {
                    println!("I'M DYINGGGGGGGG, my index was {}", index);
                    break;
                },
                Err(TryRecvError::Empty) => {}
            }
        }
    });

    thread::sleep(Duration::from_millis(5000));
    println!("Killing my thread!");
    drop(main_sender); // Sends Err(TryRecvError::Disconnected)
    println!("Waiting for his funeral");
    my_thread.join();

    let mut count = 0;
    for received in main_receiver {
        count += 1;
    }
    println!("Recieved {} messages", count);
}

fn test_pc_usage_scan() -> Result<(), Box<dyn Error>> {
    use std::thread::sleep;
    use std::collections::BTreeMap;

    let mut meter = self_meter::Meter::new(Duration::new(1, 0)).unwrap();
    meter.track_current_thread("main");
    loop {
        meter.scan()?;
        match meter.report() {
            Some(report) => {
                println!("CPU: {}", report.process_cpu_usage);
                // println!("CPU2: {}", report.gross_cpu_usage);
                println!("RAM: {}", report.memory_rss);
                // println!("Report: {:?}", report);
            },
            None => {
                println!("Couldn't open report");
            }
        }
        // sleep(Duration::from_secs(1));
        let mut count = 0;
        for _ in 0..1_000_000_000 {
            count += 1;
        }
    }

    Ok(())
}

fn test_pc_usage_with_thread() -> Result<(), Box<dyn Error>> {
    use rust_multi_json_benchmark::test_json::{self, pc_usage_exporter::PcUsage};
    use self_meter::Meter;

    let (main_sender, thread_receiver) = mpsc::channel();
    let (thread_sender, main_receiver) = mpsc::channel();
    let sample_interval = 1000_u64;
    // let mut meter = Meter::new(Duration::from_secs(1)).expect("Couldn't create Meter");

    let my_thread = thread::spawn(move ||
        // pc_usage_exporter::main(thread_sender, thread_receiver, &sample_interval));
        // pc_usage_exporter::main(thread_sender, thread_receiver, &mut meter));
        {
            let mut scanner = Meter::new(Duration::from_millis(sample_interval))
            .expect("Couldn't create Meter instance");
    
        loop {
            match scanner.scan() {
                Ok(_) => {
                    match scanner.report() {
                        Some(report) => {
                            thread_sender.send(PcUsage {
                                cpu: report.process_cpu_usage,
                                ram: (report.memory_rss / 1024) / 1024
                            }).expect("Failed to send report to main thread");
                        },
                        None => {
                            continue;
                        }
                    }
                },
                Err(error) => {
                    eprintln!("Couldn't scan PC usage: {}", error);
                }
            }
    
            match thread_receiver.try_recv() {
                Ok(_) | Err(TryRecvError::Disconnected) => break,
                Err(TryRecvError::Empty) => {}
            }
        }
        });
    thread::sleep(Duration::from_secs(2));
    main_sender.send(());
    my_thread.join();

    for recevied in main_receiver {
        println!("Got: {:?}", recevied);
    }

    Ok(())
}

fn other_thread(thread_sender: Sender<PcUsage>, thread_recevier: Receiver<()>, sample_interval: u64) {
    use self_meter::Meter;

    let sample_interval = Duration::from_millis(sample_interval);
    let mut scanner = Meter::new(sample_interval).expect("can't create meter");
    loop {
        scanner.scan().expect("can't scan");
        match scanner.report() {
            Some(report) => thread_sender.send(PcUsage{cpu: report.process_cpu_usage, ram: (report.memory_rss / 1024) / 1024}).expect("can't send data to main"),
            None => println!("Can't open report")
        }
        match thread_recevier.try_recv() {
            Ok(_) | Err(TryRecvError::Disconnected) => break,
            Err(TryRecvError::Empty) => {}
        }
        thread::sleep(sample_interval);
    }
}

fn test_pc_usage_with_thread_2() -> Result<(), Box<dyn Error>> {
    use rust_multi_json_benchmark::test_json::{self, pc_usage_exporter::PcUsage};
    use self_meter::Meter;

    let sample_interval = 1000_u64;
    let sample_interval = Duration::from_millis(sample_interval);
    let (main_sender, thread_recevier) = mpsc::channel();
    let (thread_sender, main_recevier) = mpsc::channel();

    let my_thread = thread::spawn(move || pc_usage_exporter::main(thread_sender, thread_recevier, &sample_interval));

    // thread::sleep(Duration::from_secs(6));
    let mut count = 0;
    for _ in 0..1_000_000_000 {
        count += 1;
    }

    main_sender.send(()).expect("Can't send to thread terminate signal");
    my_thread.join();
    for recevied in main_recevier {
        // println!("CPU: {} ; RAM: {}", report.process_cpu_usage, report.memory_rss)
        println!("CPU/RAM: {:?}", recevied);
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    #[derive(Debug)]
    struct Storage {
        pub num1: u64,
        pub num2: u64,
    }

    let mut my_storage = Storage { num1: 8, num2: 16 };

    fn foo(mut x: u64) {
        x += 1;
        println!("Foo {}", x);
    }

    fn goo(x: u64) {
        println!("Goo {}", x);
    }

    fn hoo(x: &mut u64) {
        *x += 1;
        println!("hoo {}", x);
    }

    foo(my_storage.num1);
    goo(my_storage.num1);
    hoo(&mut my_storage.num2);

    println!("Final {:?}", my_storage);

    Ok(())
}