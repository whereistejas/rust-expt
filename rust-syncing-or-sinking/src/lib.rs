#[cfg(test)]
mod tests {
    use std::rc::Rc;
    use std::sync::mpsc;
    use std::thread;

    #[test]
    fn it_works() {
        let (tx, rx) = mpsc::channel::<u32>();
        let value: Rc<u32> = Rc::new(12);

        let _ = thread::spawn(|| {
            let value: u32 = unsafe { *Rc::<u32>::as_ptr(&value) };
            tx.send(value).unwrap()
        });

        assert_eq!(rx.recv(), Ok(12));
    }
}
