# shareable
The purpose of this crate is to allow the the creation of objects that don't have the
synchronization overhead when used on one thread and the ability to run on multiple threads by
enabling the synchronization.

Once synchronization is enabled the "cheapest" method is chosen to share the data between
multiple threads.  This means atomic objects when it can and mutexes when it can't.  The 64
bit data objects (f64, i64, u64) are shared via atomics when on a 64 bit architecture, and via
mutexes on a 32 bit architecture.

## Examples

```
use shareable::SharedF32;

// Only 1 instance of value1 is created, so no syncrhonization is used.
let mut value1 = SharedF32::new(63.23);

println!("Value: {}", value1.get());

value1.set(78.3);

println!("Value: {}", value1.get());
```

```
use std::sync::mpsc;
use std::thread;
use shareable::SharedObject;

let mut value1 = SharedObject::new(String::from("abc"));

// Syncronization is enabled at this point,
// all access to the object is now done via a mutex.
let mut value2 = value1.dup();

let (tx, rx) = mpsc::channel();

let thread = thread::spawn(move || {
    rx.recv();
    assert_eq!(*value2.get(), "xyz");
});

value1.set(String::from("xyz"));

tx.send(());
thread.join().unwrap();
```
