# wait\_group

golang [`sync.WaitGroup`](https://golang.org/pkg/sync/#WaitGroup) in Rust

## Example

```rust
extern crate wait_group;
use wait_group::WaitGroup;

use std::thread;

fn main() {
    let wg = WaitGroup::new();
    for i in 0..100 {
        // increment the WaitGroup counter
        wg.add(1);
        let wg2 = wg.clone();
        thread::spawn(move || {
            println!("{}", i);
            // and now decrement the counter
            wg2.done();
        });
    }
    // wait for all thread have finished
    wg.wait();
}
```

## License

MIT License

```
Copyright (c) 2016 CastellaFactory

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies
of the Software, and to permit persons to whom the Software is furnished to do so,
subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED,
INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR
PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,
TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR
THE USE OR OTHER DEALINGS IN THE SOFTWARE.
```
