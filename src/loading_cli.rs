use std::io::{self, Write};
use std::sync::mpsc::{Receiver, TryRecvError};
use std::thread;
use std::time::Duration;

static FRAMES: [&str; 10] = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];

pub fn loading_animation(rx: Receiver<String>, msg: String) {
    'outer: loop {
        for frame in FRAMES.iter() {
            match rx.try_recv() {
                Ok(succ_msg) => {
                    println!("\x1B[2K\r{}", succ_msg);
                    break 'outer;
                }
                Err(TryRecvError::Disconnected) => {
                    break 'outer;
                }
                Err(TryRecvError::Empty) => {}
            }

            print!("\x1B[2K\r");
            print!("{} {}", frame, msg);
            io::stdout().flush().unwrap();
            thread::sleep(Duration::from_millis(200));
        }
    }
}
