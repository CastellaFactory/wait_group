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
pub struct WaitGroup(Arc<WaitGroupImpl>);

struct WaitGroupImpl {
    cond: Condvar,
    count: Mutex<usize>,
}

impl WaitGroup {
    pub fn new() -> WaitGroup {
        WaitGroup(Arc::new(WaitGroupImpl {
            cond: Condvar::new(),
            count: Mutex::new(0),
        }))
    }

    pub fn add(&self, delta: usize) {
        let mut count = self.0.count.lock().unwrap();
        *count += delta;
        self.notify_if_empty(*count);
    }

    pub fn done(&self) {
        let mut count = self.0.count.lock().unwrap();
        assert!(*count > 0);
        *count -= 1;
        self.notify_if_empty(*count);
    }

    pub fn wait(&self) {
        let mut count = self.0.count.lock().unwrap();
        while *count > 0 {
            count = self.0.cond.wait(count).unwrap();
        }
    }

    fn notify_if_empty(&self, count: usize) {
        if count == 0 {
            self.0.cond.notify_all();
        }
    }
}

impl fmt::Debug for WaitGroup {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let count = self.0.count.lock().unwrap();
        write!(f, "WaitGroup {{ count {:?} }}", *count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    #[test]
    fn it_work() {
        let wg = WaitGroup::new();
        let v = vec![1, 2, 3, 4, 5];
        wg.add(v.len());
        for _ in v {
            let wg = wg.clone();
            thread::spawn(move || wg.done());
        }
        wg.wait();
    }

    #[test]
    #[should_panic]
    fn inner_counter_error() {
        let wg = WaitGroup::new();
        wg.done();
    }
}
