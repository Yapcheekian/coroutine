use crate::runtime::Waker;

pub trait Future {
    type Output;
    fn poll(&mut self, waker: &Waker) -> PollState<Self::Output>;
}

pub enum PollState<T> {
    Ready(T),
    NotReady,
}
pub struct JoinAll<F: Future> {
    futures: Vec<(bool, F)>,
    finished_count: usize,
}

impl<F: Future> Future for JoinAll<F> {
    type Output = String;
    fn poll(&mut self, waker: &Waker) -> PollState<Self::Output> {
        for (finished, fut) in self.futures.iter_mut() {
            if *finished {
                continue;
            }

            match fut.poll(waker) {
                PollState::Ready(_) => {
                    *finished = true;
                    self.finished_count += 1;
                }

                PollState::NotReady => continue,
            }
        }

        if self.finished_count == self.futures.len() {
            PollState::Ready(String::new())
        } else {
            PollState::NotReady
        }
    }
}

#[allow(dead_code)]
pub fn join_all<F: Future>(futures: Vec<F>) -> JoinAll<F> {
    let futures = futures.into_iter().map(|f| (false, f)).collect();
    JoinAll {
        futures,
        finished_count: 0,
    }
}
