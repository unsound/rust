//! A work-in-progress futures library for Rust.
//!
//! This library is an **experimental** implementation of Futures in Rust, and
//! is very likely to change over time and break compatibility without notice.
//! Be warned!
//!
//! The documentation of this library is also very much a work in progress, but
//! if anything is unclear please open an issue and hopefully it'll be
//! documented quickly!
//!
//! For more information, see the [README for now][readme]
//!
//! [readme]: https://github.com/alexcrichton/futures-rs#futures-rs

#![deny(missing_docs)]

#[macro_use]
extern crate log;

mod lock;
mod slot;
mod util;

#[macro_use]
mod poll;
pub use poll::Poll;

mod task;
pub use task::{Task, TaskData, TaskHandle};

pub mod executor;

// Primitive futures
mod collect;
mod done;
mod empty;
mod failed;
mod finished;
mod lazy;
mod promise;
mod store;
pub use collect::{collect, Collect};
pub use done::{done, Done};
pub use empty::{empty, Empty};
pub use failed::{failed, Failed};
pub use finished::{finished, Finished};
pub use lazy::{lazy, Lazy};
pub use promise::{promise, Promise, Complete, Canceled};
pub use store::{store, Store};

// combinators
mod and_then;
mod flatten;
mod fuse;
mod join;
mod map;
mod map_err;
mod or_else;
mod select;
mod select_all;
mod then;
pub use and_then::AndThen;
pub use flatten::Flatten;
pub use fuse::Fuse;
pub use join::{Join, Join3, Join4, Join5};
pub use map::Map;
pub use map_err::MapErr;
pub use or_else::OrElse;
pub use select::{Select, SelectNext};
pub use select_all::{SelectAll, SelectAllNext, select_all};
pub use then::Then;

// streams
pub mod stream;

// impl details
mod chain;
mod impls;
mod forget;

macro_rules! join {
    ($(($join:ident, $Join:ident, $new:ident, <$($name:ident: $B:ident),*>),)*) => ($(
        /// Same as `join`, but with more futures.
        fn $join<$($B),*>(self, $($name: $B),*) -> $Join<Self, $($B::Future),*>
            where $($B: IntoFuture<Error=Self::Error>,)*
                  Self: Sized,
        {
            join::$new(self, $($name.into_future()),*)
        }
    )*)
}

/// Trait for types which represent a placeholder of a value that will become
/// available at possible some later point in time.
///
/// Futures are used to provide a sentinel through which a value can be
/// referenced. They crucially allow chaining operations through consumption
/// which allows expressing entire trees of computation as one sentinel value.
///
/// The ergonomics and implementation of the `Future` trait are very similar to
/// the `Iterator` trait in Rust which is where there is a small handful of
/// methods to implement and a load of default methods that consume a `Future`,
/// producing a new value.
///
/// # Core methods
///
/// The core methods of futures, currently `poll`, `schedule`, and `tailcall`,
/// are not intended to be called in general. These are used to drive an entire
/// task of many futures composed together only from the top level.
///
/// More documentation can be found on each method about what its purpose is,
/// but in general all of the combinators are the main methods that should be
/// used.
///
/// # Combinators
///
/// Like iterators, futures provide a large number of combinators to work with
/// futures to express computations in a much more natural method than
/// scheduling a number of callbacks. For example the `map` method can change
/// a `Future<Item=T>` to a `Future<Item=U>` or an `and_then` combinator could
/// create a future after the first one is done and only be resolved when the
/// second is done.
///
/// Combinators act very similarly to the methods on the `Iterator` trait itself
/// or those on `Option` and `Result`. Like with iterators, the combinators are
/// zero-cost and don't impose any extra layers of indirection you wouldn't
/// otherwise have to write down.
// TODO: expand this
pub trait Future: Send + 'static {

    /// The type of value that this future will resolved with if it is
    /// successful.
    type Item: Send + 'static;

    /// The type of error that this future will resolve with if it fails in a
    /// normal fashion.
    ///
    /// Futures may also fail due to panics or cancellation, but that is
    /// expressed through the `PollError` type, not this type.
    type Error: Send + 'static;

    /// Query this future to see if its value has become available.
    ///
    /// This function will check the internal state of the future and assess
    /// whether the value is ready to be produced. Implementors of this function
    /// should ensure that a call to this **never blocks** as event loops may
    /// not work properly otherwise.
    ///
    /// Callers of this function must provide the "task" in which the future is
    /// running through the `task` argument. This task contains information like
    /// task-local variables which the future may have stored references to
    /// internally.
    ///
    /// # Runtime characteristics
    ///
    /// This function, `poll`, is the primary method for 'making progress'
    /// within a tree of futures. For example this method will be called
    /// repeatedly as the internal state machine makes its various transitions.
    /// Additionally, this function may not necessarily have many guarantees
    /// about *where* it's run (e.g. always on an I/O thread or not). Unless it
    /// is otherwise arranged to be so, it should be ensured that
    /// **implementations of this function finish very quickly**.
    ///
    /// This prevents unnecessarily clogging up threads and/or event loops while
    /// a `poll` function call, for example, takes up compute resources to
    /// perform some expensive computation. If it is known ahead of time that a
    /// call to `poll` may end up taking awhile, the work should be offloaded to
    /// a thread pool (or something similar) to ensure that `poll` can return
    /// quickly.
    ///
    /// # Return value
    ///
    /// This function returns `Poll::NotReady` if the future is not ready yet,
    /// or `Poll::{Ok,Err}` with the result of this future if it's ready. Once
    /// a future has returned `Some` it is considered a contract error to
    /// continue polling it.
    ///
    /// # Panics
    ///
    /// Once a future has completed (returned `Poll::{Ok, Err}` from `poll`),
    /// then any future calls to `poll` may panic, block forever, or otherwise
    /// cause wrong behavior. The `Future` trait itself provides no guarantees
    /// about the behavior of `poll` after `Some` has been returned at least
    /// once.
    ///
    /// Callers who may call `poll` too many times may want to consider using
    /// the `fuse` adaptor which defines the behavior of `poll`, but comes with
    /// a little bit of extra cost.
    ///
    /// # Errors
    ///
    /// This future may have failed to finish the computation, in which case
    /// the `Poll::Err` variant will be returned with an appropriate payload of
    /// an error.
    fn poll(&mut self, task: &mut Task) -> Poll<Self::Item, Self::Error>;

    /// Schedule a task to be notified when this future is ready.
    ///
    /// Throughout the lifetime of a future it may frequently be `poll`'d on to
    /// test whether the value is ready yet. If `None` is returned, however, the
    /// caller may then register interest via this function to get a
    /// notification when the future can indeed make progress.
    ///
    /// The `task` argument provided is the same task as provided to `poll`, and
    /// it's the overall task which is driving this future. The task will be
    /// notified through the `TaskHandle` type generated from the `handle`
    /// method, and spurious notifications are allowed. That is, it's ok for a
    /// notification to be received which when the future is poll'd it still
    /// isn't complete.
    ///
    /// Implementors of the `Future` trait are recommended to just blindly pass
    /// around this task rather than attempt to manufacture new tasks.
    ///
    /// When the `task` is notified it will be provided a set of tokens that
    /// represent the set of events which have happened since it was last called
    /// (or the last call to `poll`). These events can then be used by the task
    /// to later inform `poll` calls to not poll too much.
    ///
    /// # Multiple calls to `schedule`
    ///
    /// This function cannot be used to queue up multiple tasks to be notified
    /// when a future is ready to make progress. Only the most recent call to
    /// `schedule` is guaranteed to have notifications received when `schedule`
    /// is called multiple times.
    ///
    /// If this function is called twice, it may be the case that the previous
    /// task is never notified. It is recommended that this function is called
    /// with the same task for the entire lifetime of this future.
    ///
    /// # Panics
    ///
    /// Once a future has returned `Some` (it's been completed) then future
    /// calls to either `poll` or this function, `schedule`, should not be
    /// expected to behave well. A call to `schedule` after a poll has succeeded
    /// may panic, block forever, or otherwise exhibit odd behavior.
    ///
    /// Callers who may call `schedule` after a future is finished may want to
    /// consider using the `fuse` adaptor which defines the behavior of
    /// `schedule` after a successful poll, but comes with a little bit of
    /// extra cost.
    fn schedule(&mut self, task: &mut Task);

    /// Perform tail-call optimization on this future.
    ///
    /// A particular future may actually represent a large tree of computation,
    /// the structure of which can be optimized periodically after some of the
    /// work has completed. This function is intended to be called after an
    /// unsuccessful `poll` to ensure that the computation graph of a future
    /// remains at a reasonable size.
    ///
    /// This function is intended to be idempotent. If `None` is returned then
    /// the internal structure may have been optimized, but this future itself
    /// must stick around to represent the computation at hand.
    ///
    /// If `Some` is returned then the returned future will be realized with the
    /// same value that this future *would* have been had this method not been
    /// called. Essentially, if `Some` is returned, then this future can be
    /// forgotten and instead the returned value is used.
    ///
    /// Note that this is a default method which returns `None`, but any future
    /// adaptor should implement it to flatten the underlying future, if any.
    fn tailcall(&mut self)
                -> Option<Box<Future<Item=Self::Item, Error=Self::Error>>> {
        None
    }

    /// Convenience function for turning this future into a trait object.
    ///
    /// This simply avoids the need to write `Box::new` and can often help with
    /// type inference as well by always returning a trait object.
    ///
    /// # Examples
    ///
    /// ```
    /// use futures::*;
    ///
    /// let a: Box<Future<Item=i32, Error=i32>> = done(Ok(1)).boxed();
    /// ```
    fn boxed(self) -> Box<Future<Item=Self::Item, Error=Self::Error>>
        where Self: Sized
    {
        Box::new(self)
    }

    /// Map this future's result to a different type, returning a new future of
    /// the resulting type.
    ///
    /// This function is similar to the `Option::map` or `Iterator::map` where
    /// it will change the type of the underlying future. This is useful to
    /// chain along a computation once a future has been resolved.
    ///
    /// The closure provided will only be called if this future is resolved
    /// successfully. If this future returns an error, panics, or is canceled,
    /// then the closure provided will never be invoked.
    ///
    /// Note that this function consumes the receiving future and returns a
    /// wrapped version of it, similar to the existing `map` methods in the
    /// standard library.
    ///
    /// # Examples
    ///
    /// ```
    /// use futures::*;
    ///
    /// let future_of_1 = finished::<u32, u32>(1);
    /// let future_of_4 = future_of_1.map(|x| x + 3);
    /// ```
    fn map<F, U>(self, f: F) -> Map<Self, F>
        where F: FnOnce(Self::Item) -> U + Send + 'static,
              U: Send + 'static,
              Self: Sized,
    {
        assert_future::<U, Self::Error, _>(map::new(self, f))
    }

    /// Map this future's error to a different error, returning a new future.
    ///
    /// This function is similar to the `Result::map_err` where it will change
    /// the error type of the underlying future. This is useful for example to
    /// ensure that futures have the same error type when used with combinators
    /// like `select` and `join`.
    ///
    /// The closure provided will only be called if this future is resolved
    /// with an error. If this future returns a success, panics, or is
    /// canceled, then the closure provided will never be invoked.
    ///
    /// Note that this function consumes the receiving future and returns a
    /// wrapped version of it.
    ///
    /// # Examples
    ///
    /// ```
    /// use futures::*;
    ///
    /// let future_of_err_1 = failed::<u32, u32>(1);
    /// let future_of_err_4 = future_of_err_1.map_err(|x| x + 3);
    /// ```
    fn map_err<F, E>(self, f: F) -> MapErr<Self, F>
        where F: FnOnce(Self::Error) -> E + Send + 'static,
              E: Send + 'static,
              Self: Sized,
    {
        assert_future::<Self::Item, E, _>(map_err::new(self, f))
    }

    /// Chain on a computation for when a future finished, passing the result of
    /// the future to the provided closure `f`.
    ///
    /// This function can be used to ensure a computation runs regardless of
    /// the conclusion of the future. The closure provided will be yielded a
    /// `Result` once the future is complete.
    ///
    /// The returned value of the closure must implement the `IntoFuture` trait
    /// and can represent some more work to be done before the composed future
    /// is finished. Note that the `Result` type implements the `IntoFuture`
    /// trait so it is possible to simply alter the `Result` yielded to the
    /// closure and return it.
    ///
    /// If this future is canceled or panics then the closure `f` will not be
    /// run.
    ///
    /// Note that this function consumes the receiving future and returns a
    /// wrapped version of it.
    ///
    /// # Examples
    ///
    /// ```
    /// use futures::*;
    ///
    /// let future_of_1 = finished::<u32, u32>(1);
    /// let future_of_4 = future_of_1.then(|x| {
    ///     x.map(|y| y + 3)
    /// });
    ///
    /// let future_of_err_1 = failed::<u32, u32>(1);
    /// let future_of_4 = future_of_err_1.then(|x| {
    ///     match x {
    ///         Ok(_) => panic!("expected an error"),
    ///         Err(y) => finished::<u32, u32>(y + 3),
    ///     }
    /// });
    /// ```
    fn then<F, B>(self, f: F) -> Then<Self, B, F>
        where F: FnOnce(Result<Self::Item, Self::Error>) -> B + Send + 'static,
              B: IntoFuture,
              Self: Sized,
    {
        assert_future::<B::Item, B::Error, _>(then::new(self, f))
    }

    /// Execute another future after this one has resolved successfully.
    ///
    /// This function can be used to chain two futures together and ensure that
    /// the final future isn't resolved until both have finished. The closure
    /// provided is yielded the successful result of this future and returns
    /// another value which can be converted into a future.
    ///
    /// Note that because `Result` implements the `IntoFuture` trait this method
    /// can also be useful for chaining fallible and serial computations onto
    /// the end of one future.
    ///
    /// If this future is canceled, panics, or completes with an error then the
    /// provided closure `f` is never called.
    ///
    /// Note that this function consumes the receiving future and returns a
    /// wrapped version of it.
    ///
    /// # Examples
    ///
    /// ```
    /// use futures::*;
    ///
    /// let future_of_1 = finished::<u32, u32>(1);
    /// let future_of_4 = future_of_1.and_then(|x| {
    ///     Ok(x + 3)
    /// });
    ///
    /// let future_of_err_1 = failed::<u32, u32>(1);
    /// future_of_err_1.and_then(|_| -> Done<u32, u32> {
    ///     panic!("should not be called in case of an error");
    /// });
    /// ```
    fn and_then<F, B>(self, f: F) -> AndThen<Self, B, F>
        where F: FnOnce(Self::Item) -> B + Send + 'static,
              B: IntoFuture<Error = Self::Error>,
              Self: Sized,
    {
        assert_future::<B::Item, Self::Error, _>(and_then::new(self, f))
    }

    /// Execute another future after this one has resolved with an error.
    ///
    /// This function can be used to chain two futures together and ensure that
    /// the final future isn't resolved until both have finished. The closure
    /// provided is yielded the error of this future and returns another value
    /// which can be converted into a future.
    ///
    /// Note that because `Result` implements the `IntoFuture` trait this method
    /// can also be useful for chaining fallible and serial computations onto
    /// the end of one future.
    ///
    /// If this future is canceled, panics, or completes successfully then the
    /// provided closure `f` is never called.
    ///
    /// Note that this function consumes the receiving future and returns a
    /// wrapped version of it.
    ///
    /// # Examples
    ///
    /// ```
    /// use futures::*;
    ///
    /// let future_of_err_1 = failed::<u32, u32>(1);
    /// let future_of_4 = future_of_err_1.or_else(|x| -> Result<u32, u32> {
    ///     Ok(x + 3)
    /// });
    ///
    /// let future_of_1 = finished::<u32, u32>(1);
    /// future_of_1.or_else(|_| -> Done<u32, u32> {
    ///     panic!("should not be called in case of success");
    /// });
    /// ```
    fn or_else<F, B>(self, f: F) -> OrElse<Self, B, F>
        where F: FnOnce(Self::Error) -> B + Send + 'static,
              B: IntoFuture<Item = Self::Item>,
              Self: Sized,
    {
        assert_future::<Self::Item, B::Error, _>(or_else::new(self, f))
    }

    /// Waits for either one of two futures to complete.
    ///
    /// This function will return a new future which awaits for either this or
    /// the `other` future to complete. The returned future will finish with
    /// both the value resolved and a future representing the completion of the
    /// other work. Both futures must have the same item and error type.
    ///
    /// If either future is canceled or panics, the other is canceled and the
    /// original error is propagated upwards.
    ///
    /// Note that this function consumes the receiving future and returns a
    /// wrapped version of it.
    ///
    /// # Examples
    ///
    /// ```
    /// use futures::*;
    ///
    /// // A poor-man's join implemented on top of select
    ///
    /// fn join<A>(a: A, b: A)
    ///            -> Box<Future<Item=(A::Item, A::Item), Error=A::Error>>
    ///     where A: Future,
    /// {
    ///     a.select(b).then(|res| {
    ///         match res {
    ///             Ok((a, b)) => b.map(|b| (a, b)).boxed(),
    ///             Err((a, _)) => failed(a).boxed(),
    ///         }
    ///     }).boxed()
    /// }
    /// ```
    fn select<B>(self, other: B) -> Select<Self, B::Future>
        where B: IntoFuture<Item=Self::Item, Error=Self::Error>,
              Self: Sized,
    {
        let f = select::new(self, other.into_future());
        assert_future::<(Self::Item, SelectNext<Self, B::Future>),
                        (Self::Error, SelectNext<Self, B::Future>), _>(f)
    }

    /// Joins the result of two futures, waiting for them both to complete.
    ///
    /// This function will return a new future which awaits both this and the
    /// `other` future to complete. The returned future will finish with a tuple
    /// of both results.
    ///
    /// Both futures must have the same error type, and if either finishes with
    /// an error then the other will be canceled and that error will be
    /// returned.
    ///
    /// If either future is canceled or panics, the other is canceled and the
    /// original error is propagated upwards.
    ///
    /// Note that this function consumes the receiving future and returns a
    /// wrapped version of it.
    ///
    /// # Examples
    ///
    /// ```
    /// use futures::*;
    ///
    /// let a = finished::<u32, u32>(1);
    /// let b = finished::<u32, u32>(2);
    /// let pair = a.join(b);
    ///
    /// pair.map(|(a, b)| {
    ///     assert_eq!(a, 1);
    ///     assert_eq!(b, 1);
    /// });
    /// ```
    fn join<B>(self, other: B) -> Join<Self, B::Future>
        where B: IntoFuture<Error=Self::Error>,
              Self: Sized,
    {
        let f = join::new(self, other.into_future());
        assert_future::<(Self::Item, B::Item), Self::Error, _>(f)
    }

    join! {
        (join3, Join3, new3, <b: B, c: C>),
        (join4, Join4, new4, <b: B, c: C, d: D>),
        (join5, Join5, new5, <b: B, c: C, d: D, e: E>),
    }

    /// Flatten the execution of this future when the successful result of this
    /// future is itself another future.
    ///
    /// This can be useful when combining futures together to flatten the
    /// computation out the the final result. This method can only be called
    /// when the successful result of this future itself implements the
    /// `IntoFuture` trait and the error can be created from this future's error
    /// type.
    ///
    /// This method is equivalent to `self.then(|x| x)`.
    ///
    /// Note that this function consumes the receiving future and returns a
    /// wrapped version of it.
    ///
    /// # Examples
    ///
    /// ```
    /// use futures::*;
    ///
    /// let future_of_a_future = finished::<_, u32>(finished::<u32, u32>(1));
    /// let future_of_1 = future_of_a_future.flatten();
    /// ```
    fn flatten(self) -> Flatten<Self>
        where Self::Item: IntoFuture,
              <<Self as Future>::Item as IntoFuture>::Error:
                    From<<Self as Future>::Error>,
              Self: Sized
    {
        let f = flatten::new(self);
        assert_future::<<<Self as Future>::Item as IntoFuture>::Item,
                        <<Self as Future>::Item as IntoFuture>::Error,
                        _>(f)
    }

    /// Fuse a future such that `poll` will never again be called once it has
    /// returned a success.
    ///
    /// Currently once a future has returned `Some` from `poll` any further
    /// calls could exhibit bad behavior such as block forever, panic, never
    /// return, etc. If it is known that `poll` may be called too often then
    /// this method can be used to ensure that it has defined semantics.
    ///
    /// Once a future has been `fuse`d and it returns success from `poll`, then
    /// it will forever return `None` from `poll` again (never resolve). This,
    /// unlike the trait's `poll` method, is guaranteed.
    ///
    /// Additionally, once a future has completed, this `Fuse` combinator will
    /// ensure that all registered callbacks will not be registered with the
    /// underlying future.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use futures::*;
    ///
    /// let mut task = Task::new();
    /// let mut future = finished::<i32, u32>(2);
    /// assert!(future.poll(&mut task).is_ready());
    ///
    /// // Normally, a call such as this would panic:
    /// //future.poll(&mut task);
    ///
    /// // This, however, is guaranteed to not panic
    /// let mut future = finished::<i32, u32>(2).fuse();
    /// assert!(future.poll(&mut task).is_ready());
    /// assert!(future.poll(&mut task).is_not_ready());
    /// ```
    fn fuse(self) -> Fuse<Self>
        where Self: Sized
    {
        let f = fuse::new(self);
        assert_future::<Self::Item, Self::Error, _>(f)
    }

    /// Consume this future and allow it to execute without cancelling it.
    ///
    /// Normally whenever a future is dropped it signals that the underlying
    /// computation should be cancelled ASAP. This function, however, will
    /// consume the future and arrange for the future itself to get dropped only
    /// when the computation has completed.
    ///
    /// This function can be useful to ensure that futures with side effects can
    /// run "in the background", but it is discouraged as it doesn't allow any
    /// control over the future in terms of cancellation.
    ///
    /// Generally applications should retain handles on futures to ensure
    /// they're properly cleaned up if something unexpected happens.
    fn forget(self) where Self: Sized {
        forget::forget(self);
    }
}

// Just a helper function to ensure the futures we're returning all have the
// right implementations.
fn assert_future<A, B, F>(t: F) -> F
    where F: Future<Item=A, Error=B>,
          A: Send + 'static,
          B: Send + 'static,
{
    t
}

/// Class of types which can be converted themselves into a future.
///
/// This trait is very similar to the `IntoIterator` trait and is intended to be
/// used in a very similar fashion.
pub trait IntoFuture: Send + 'static {
    /// The future that this type can be converted into.
    type Future: Future<Item=Self::Item, Error=Self::Error>;

    /// The item that the future may resolve with.
    type Item: Send + 'static;
    /// The error that the future may resolve with.
    type Error: Send + 'static;

    /// Consumes this object and produces a future.
    fn into_future(self) -> Self::Future;
}

impl<F: Future> IntoFuture for F {
    type Future = F;
    type Item = F::Item;
    type Error = F::Error;

    fn into_future(self) -> F {
        self
    }
}

impl<T, E> IntoFuture for Result<T, E>
    where T: Send + 'static,
          E: Send + 'static,
{
    type Future = Done<T, E>;
    type Item = T;
    type Error = E;

    fn into_future(self) -> Done<T, E> {
        done(self)
    }
}

mod all;
