use concurrent_queue::ConcurrentQueue;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

struct Task(dyn Fn() + Send + Sync);

struct Engine {
    immediate_queue: ConcurrentQueue<Task>
}

impl Engine {
    pub fn new() -> Self {
        Engine {
            immediate_queue: ConcurrentQueue::unbounded(),
        }
    }

    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let engine = Engine::new();

    }
}
