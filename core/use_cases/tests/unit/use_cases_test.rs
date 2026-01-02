use core_entities::Greeting;
use core_ports::output_boundary::{GreetingOutput, GreetingOutputBoundary};
use core_ports::{BoxFuture, GreetingRepository, RepoError};
use core_use_cases::ListGreetingsInteractor;
use std::future::Future;
use std::pin::Pin;
use std::sync::Mutex;
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

struct FakeRepo;

impl GreetingRepository for FakeRepo {
    fn list_greetings<'a>(&'a self) -> BoxFuture<'a, Result<Vec<Greeting>, RepoError>> {
        Box::pin(async move { Ok(vec![Greeting::new(1, "Hello")]) })
    }
}

struct CapturingPresenter {
    output: Mutex<Option<GreetingOutput>>,
}

impl CapturingPresenter {
    fn new() -> Self {
        Self {
            output: Mutex::new(None),
        }
    }

    fn take(&self) -> GreetingOutput {
        self.output.lock().unwrap().take().unwrap()
    }
}

impl GreetingOutputBoundary for CapturingPresenter {
    fn present(&self, output: GreetingOutput) {
        *self.output.lock().unwrap() = Some(output);
    }
}

fn block_on<F: Future>(mut future: F) -> F::Output {
    fn raw_waker() -> RawWaker {
        fn no_op(_: *const ()) {}
        fn clone(_: *const ()) -> RawWaker {
            raw_waker()
        }
        static VTABLE: RawWakerVTable = RawWakerVTable::new(clone, no_op, no_op, no_op);
        RawWaker::new(std::ptr::null(), &VTABLE)
    }

    let waker = unsafe { Waker::from_raw(raw_waker()) };
    let mut context = Context::from_waker(&waker);
    let mut future = unsafe { Pin::new_unchecked(&mut future) };

    loop {
        match future.as_mut().poll(&mut context) {
            Poll::Ready(value) => return value,
            Poll::Pending => {}
        }
    }
}

#[test]
fn list_greetings_sends_output_to_presenter() {
    let repo = FakeRepo;
    let presenter = CapturingPresenter::new();
    let interactor = ListGreetingsInteractor::new(&repo, &presenter);

    block_on(interactor.execute()).expect("use case should succeed");

    let output = presenter.take();
    assert_eq!(output.greetings.len(), 1);
    assert_eq!(output.greetings[0].message, "Hello");
}
