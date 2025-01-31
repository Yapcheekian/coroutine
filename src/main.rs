mod future;
mod http;
mod runtime;

use future::{Future, PollState};
use runtime::Waker;
use std::fmt::Write;

fn main() {
    // let future = async_main();
    // let mut runtime = Runtime::new();
    // runtime.block_on(future);

    let mut executor = runtime::init();
    executor.block_on(async_main());
}

fn async_main() -> impl Future<Output = String> {
    Coroutine0::new()
}

enum State0 {
    Start,
    Wait1(Box<dyn Future<Output = String>>),
    Wait2(Box<dyn Future<Output = String>>),
    Resolved,
}

#[derive(Default)]
struct Stack0 {
    counter: Option<usize>,
    buffer: Option<String>,
    writer: Option<*mut String>,
}

struct Coroutine0 {
    state: State0,
    stack: Stack0,
}

impl Coroutine0 {
    fn new() -> Self {
        Self {
            state: State0::Start,
            stack: Stack0::default(),
        }
    }
}

impl Future for Coroutine0 {
    type Output = String;

    fn poll(&mut self, waker: &Waker) -> PollState<Self::Output> {
        loop {
            match self.state {
                State0::Start => {
                    // initialize stack (hoist variables)
                    self.stack.counter = Some(0);
                    self.stack.buffer = Some(String::from("\nBUFFER:\n----\n"));
                    self.stack.writer = Some(self.stack.buffer.as_mut().unwrap());

                    // ---- Code you actually wrote ----
                    println!("Program starting");

                    // ---------------------------------
                    let fut1 = Box::new(http::Http::get("/600/HelloAsyncAwait"));
                    self.state = State0::Wait1(fut1);
                }

                State0::Wait1(ref mut f1) => {
                    match f1.poll(waker) {
                        PollState::Ready(txt) => {
                            // Restore stack
                            let mut counter = self.stack.counter.take().unwrap();
                            let writer = unsafe { &mut *self.stack.writer.take().unwrap() };

                            // ---- Code you actually wrote ----
                            println!("{txt}");
                            counter += 1;
                            writeln!(writer, "{txt}").unwrap();

                            // ---------------------------------
                            let fut2 = Box::new(http::Http::get("/400/HelloAsyncAwait"));
                            self.state = State0::Wait2(fut2);
                            self.stack.counter = Some(counter);
                            self.stack.writer = Some(writer);
                        }
                        PollState::NotReady => break PollState::NotReady,
                    }
                }

                State0::Wait2(ref mut f2) => {
                    match f2.poll(waker) {
                        PollState::Ready(txt) => {
                            // Restore stack
                            let mut counter = self.stack.counter.take().unwrap();
                            let buffer = self.stack.buffer.as_ref().take().unwrap();
                            let writer = unsafe { &mut *self.stack.writer.take().unwrap() };

                            // ---- Code you actually wrote ----
                            println!("{txt}");
                            counter += 1;
                            println!("Received {} responses", counter);
                            writeln!(writer, "{txt}").unwrap();
                            println!("{}", buffer);

                            // ---------------------------------
                            self.state = State0::Resolved;
                            let _ = self.stack.buffer.take();
                            break PollState::Ready(String::new());
                        }
                        PollState::NotReady => break PollState::NotReady,
                    }
                }

                State0::Resolved => panic!("Polled a resolved future"),
            }
        }
    }
}
