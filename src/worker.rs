use std::sync::mpsc;
use std::sync::mpsc::TryRecvError;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::thread::JoinHandle;

use log::info;

#[derive(Default)]
pub struct Worker {
    cancellation: Option<mpsc::Sender<()>>,
}

impl Worker {
    pub fn run<F, T>(&mut self, func: F) -> JoinHandle<T>
    where
        F: 'static + Send + FnOnce(Cancel) -> T,
        T: 'static + Send,
    {
        if let Some(cancellation) = self.cancellation.as_mut() {
            info!("asking existing worker to cancel");
            let _ = cancellation.send(());
        }
        let (cancellation, done) = mpsc::channel();
        self.cancellation = Some(cancellation);
        thread::spawn(move || func(Cancel::OnMessage(Arc::new(Mutex::new(done)))))
    }
}

pub enum Cancel {
    OnMessage(Arc<Mutex<mpsc::Receiver<()>>>),
    Never,
}

impl Cancel {
    pub fn check_done(&self) -> bool {
        match self {
            Cancel::OnMessage(done) => match done.lock().unwrap().try_recv() {
                Ok(()) | Err(TryRecvError::Disconnected) => true,
                Err(TryRecvError::Empty) => false,
            },
            Cancel::Never => false,
        }
    }

    pub fn maybe_abort(&self) {
        assert!(!self.check_done())
    }
}
