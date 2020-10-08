use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker{
    id: usize,
    thread: Option<thread::JoinHandle<()>>
}

enum Message{
    NewJob(Job),
    Terminate
}

impl Worker{
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Self{
        let thread = thread::Builder::new().stack_size(20 * 1024 * 1024).spawn(move || {
            loop{
                let message = receiver.lock().unwrap().recv().unwrap();
                match message{
                    Message::NewJob(job) => {
                        job();
                    }
                    Message::Terminate => {
                        break;
                    }
                }
            }
        }).unwrap();
        Worker{
            id,
            thread: Some(thread)
        }
    }
}

pub struct ThreadPool{
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>
}

impl ThreadPool{
    pub fn new(size: usize) -> Self{
        assert!(size > 0);
        let mut workers = Vec::with_capacity(size);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        for id in 0..size{
            workers.push(Worker::new(id, receiver.clone()));
        }

        ThreadPool{
            workers,
            sender
        }
    }

    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool{
    fn drop(&mut self){
        for _ in 0..self.workers.len(){
            self.sender.send(Message::Terminate)
                .unwrap();
        }
        for worker in &mut self.workers{
            if let Some(thread) = worker.thread.take(){
                thread.join()
                    .unwrap();
            }
        }
    }
}
