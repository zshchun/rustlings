// Building on the last exercise, we want all of the threads to complete their
// work. But this time, the spawned threads need to be in charge of updating a
// shared value: `JobStatus.jobs_done`

//use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use std::{sync::Arc, thread, time::Duration};

struct JobStatus {
    //    jobs_done: AtomicUsize,
    jobs_done: u32,
}

fn main() {
    // TODO: `Arc` isn't enough if you want a **mutable** shared state.
    let status = Arc::new(Mutex::new(JobStatus {
        //jobs_done: AtomicUsize::new(0),
        jobs_done: 0,
    }));

    let mut handles = Vec::new();
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));

            // TODO: You must take an action before you update a shared value.
            //status_shared.jobs_done.fetch_add(1, Ordering::Relaxed);
            status_shared.lock().unwrap().jobs_done += 1;
        });
        handles.push(handle);
    }

    // Waiting for all jobs to complete.
    for handle in handles {
        handle.join().unwrap();
    }

    // TODO: Print the value of `JobStatus.jobs_done`.
    //    println!("Jobs done: {}", status.jobs_done.load(Ordering::Relaxed));
    println!("Jobs done: {}", status.lock().unwrap().jobs_done);
}
