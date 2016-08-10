use std::fmt;
use std::sync::{Arc, Mutex, Condvar};

/// A WaitGroup waits for a collection of threads to finish.
/// Then each of the threads runs and calls Done when finished.
/// At the same time, Wait can be used to block until all threads have finished.
///
/// #Example
///
/// ```
/// use std::thread;
///
/// let wg = wait_group::WaitGroup::new();
///
/// for _ in 0..10 {
///     wg.add(1);
///     let wg2 = wg.clone();
///     thread::spawn(move || {
///         // do something
///
///         // call done
///         wg2.done();
///     });
/// }
/// // block until all threads have finished
/// wg.wait();
/// ```
#[derive(Clone)]
pub struct WaitGroup {
    c: Arc<WaitGroupImpl>,
}

struct WaitGroupImpl {
    cond: Condvar,
    count: Mutex<i32>,
}

impl WaitGroup {
    pub fn new() -> WaitGroup {
        WaitGroup {
            c: Arc::new(WaitGroupImpl {
                cond: Condvar::new(),
                count: Mutex::new(0),
            }),
        }
    }

    pub fn add(&self, delta: i32) {
        let mut count = self.c.count.lock().unwrap();
        *count += delta;
        assert!(*count >= 0);
        self.notify_if_empty(*count);
    }

    pub fn done(&self) {
        self.add(-1);
    }

    pub fn wait(&self) {
        let mut count = self.c.count.lock().unwrap();
        while *count > 0 {
            count = self.c.cond.wait(count).unwrap();
        }
    }

    fn notify_if_empty(&self, count: i32) {
        if count == 0 {
            self.c.cond.notify_all();
        }
    }
}

impl fmt::Debug for WaitGroup {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let count = self.c.count.lock().unwrap();
        write!(f, "WaitGroup {{ count {:?} }}", *count)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_work() {}
}
