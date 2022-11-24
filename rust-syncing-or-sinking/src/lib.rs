#[cfg(test)]
mod tests {
    use std::rc::Rc;
    use std::sync::{mpsc, Mutex};
    use std::thread;

    #[test]
    fn it_works() {
        let (tx, rx) = mpsc::channel::<u32>();
        let value_: Rc<u32> = Rc::new(12);

        // Even if you wrap an `Rc` in a Mutex, you still cannot move it around.
        let value: Mutex<Rc<u32>> = Mutex::new(value_);

        let _ = thread::spawn(|| {
            let value = value.into_inner().unwrap();
            let inner: u32 = unsafe { *Rc::<u32>::as_ptr(&value) };
            tx.send(inner).unwrap()
        });

        assert_eq!(rx.recv(), Ok(12));
    }
}
