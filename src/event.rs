pub struct Event<T> {
    handlers: Vec<Box<FnMut(&T) -> () + Send + 'static>>,
}

// TODO: Return a handle to the event handler that can be acted on to unsubscribe (?)

impl<T> Event<T> {
    pub fn new() -> Self {
        Self {
            handlers: Vec::new(),
        }
    }

    pub fn add(&mut self, handler: impl FnMut(&T) -> () + Send + 'static) {
        self.handlers.push(Box::new(handler));
    }

    pub(crate) fn dispatch(&mut self, event: T) {
        for handler in &mut self.handlers {
            handler(&event);
        }
    }
}
