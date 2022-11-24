#[cfg(test)]
mod tests {
    use std::rc::Rc;
    use std::sync::{mpsc, Arc};
    use std::thread;

    #[test]
    fn it_works() {
        let (tx, rx) = mpsc::channel::<u32>();
        let value_: Rc<u32> = Rc::new(12);

        // Even if you wrap an `Rc` in a `Arc`, you still cannot move it around.
        let value: Arc<Rc<u32>> = Arc::new(value_);

        let _ = thread::spawn(|| {
            let value = unsafe { *Arc::<Rc<u32>>::as_ptr(&value) };
            let inner: u32 = unsafe { *Rc::<u32>::as_ptr(&value) };
            tx.send(inner).unwrap()
        });

        assert_eq!(rx.recv(), Ok(12));
    }
}
