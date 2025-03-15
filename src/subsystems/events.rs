use std::sync::Arc;
use crossbeam_queue::SegQueue;
use crate::events::types::AppEvent;
use iced::stream;
use iced::futures::{SinkExt, Stream};
use lazy_static::lazy_static;
use tokio::sync::Mutex;
use nanoid::nanoid;

pub struct EventManager {
    pub queue: SegQueue<Task>,
    pub response_queue: SegQueue<Task>
}

#[derive(Clone, Debug)]
pub enum TaskState {
    Pending,
    Processing,
    Complete,
}


#[derive(Clone, Debug)]
pub struct Task {
    pub id: TaskId,
    pub state: TaskState,
    pub data: AppEvent,
}

type TaskId = String;

impl Task {
    pub fn new(id: TaskId, data: AppEvent) -> Self {
        Self {
            id,
            state: TaskState::Pending,
            data,
        }
    }
}

impl EventManager {
    pub fn new() -> EventManager {
        EventManager {
            queue: SegQueue::new(),
            response_queue: SegQueue::new(),
        }
    }

    pub fn emit(&mut self, message: AppEvent) {
        let job_id = nanoid!(10);
        debug!("Enqueued job {} - {:?}", job_id, &message);

        let job = Task::new(job_id.clone(), message);

        self.queue.push(job);
    }


    pub fn poll(&mut self) -> Option<Task> {
        let task = self.queue.pop();
        if task.is_none() {
            return None;
        }

        let mut task = task.unwrap();
        task.state = TaskState::Processing;
        Some(task)
    }



    pub fn poll_response(&self) -> Option<Task> {
        self.response_queue.pop()
    }
}

lazy_static!(
    pub static ref EVENT_QUEUE: Arc<Mutex<EventManager>> = Arc::new(Mutex::new(EventManager::new()));
);


pub fn subscribe() -> impl Stream<Item=AppEvent> {
    let eq = EVENT_QUEUE.clone();
    stream::channel(100, |mut output| async move {
        loop {

            let eq2 = eq.lock().await;
            let response = eq2.poll_response();
            if let Some(mut response) = response {
                response.state = TaskState::Complete;

                let _ = output.send(AppEvent::EventQueue(Box::from(response))).await;
            } else {
                continue;
            }
        }
    })

}