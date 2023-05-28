/* #region Imports */
// Standard
use std::thread;
use std::time::Duration;
use std::sync::mpsc::{Receiver, Sender, TryRecvError};

// 3rd Party
use self_meter::Meter;
/* #endregion */

#[derive(Debug)]
pub struct PcUsage {
    pub cpu: f32,
    pub ram: u64
}

pub fn main(sender_to_main: Sender<PcUsage>, receive_from_main: Receiver<()>, sample_interval: &Duration) {
    let mut scanner = Meter::new(sample_interval.clone())
        .expect("Couldn't create Meter instance");

    loop {
        match receive_from_main.try_recv() {
            Ok(_) | Err(TryRecvError::Disconnected) => break,
            Err(TryRecvError::Empty) => {}
        }

        thread::sleep(sample_interval.clone());
        if let Err(error) = scanner.scan() {
            eprintln!("Couldn't scan PC usage: {}", error);
            continue;
        }


        match scanner.report() {
            Some(report) => {
                sender_to_main.send(PcUsage {
                    cpu: report.process_cpu_usage,
                    ram: (report.memory_rss / 1024) / 1024
                }).expect("Failed to send report to main thread");
            },
            None => continue
        }
    }
}
