use super::message_receiver::*;
use std::sync::Arc;
use std::sync::Mutex;

type Threadsafe<T> = Arc<Mutex<T>>;

fn threadsafe<T>(object: T) -> Arc<Mutex<T>> {
    Arc::new(Mutex::new(object))
}

pub struct Worker<Receiver>
where
    Receiver: MessageReceiver,
{
    id: usize,
    receiver: Option<Threadsafe<Receiver>>,
}

impl<Receiver> Worker<Receiver>
where
    Receiver: MessageReceiver,
{
    pub fn new(id: usize, receiver: Threadsafe<Receiver>) -> Self {
        Worker {
            id,
            receiver: Some(receiver),
        }
    }

    pub fn start(&self) {
        match &self.receiver {
            Some(receiver) => match receiver.lock() {
                Ok(receiver) => receiver.receive(),
                Err(_) => panic!(),
            },
            None => panic!(),
        };
    }
}

#[cfg(test)]
mod should {
    use super::*;

    #[test]
    fn have_a_working_constructor() {
        let id = 1234;
        let receiver = MockMessageReceiver::new();
        let receiver = threadsafe(receiver);
        let worker = Worker::new(id, receiver);
        assert_eq!(id, worker.id);
        assert!(worker.receiver.is_some());
    }

    mod when_started {
        use super::*;

        #[test]
        fn use_its_message_receiver() {
            let mut receiver = MockMessageReceiver::new();
            receiver.expect_receive().times(1).returning(|| {});
            let receiver = threadsafe(receiver);
            let worker = Worker::new(0, receiver);
            worker.start();
        }
    }
}
