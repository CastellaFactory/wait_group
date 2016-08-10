# wait\_group

golang [`sync.WaitGroup`](https://golang.org/pkg/sync/#WaitGroup) in Rust

## Example

```
use std::thread;

let wg = wait_group::WaitGroup::new();

for _ in 0..10 {
    wg.add(1);
    thread::spawn(move || {
        // do something

        // call done
        wg.done();
    });
}
// block until all threads have finished
wg.wait();
```

