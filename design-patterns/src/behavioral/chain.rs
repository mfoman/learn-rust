/// Chain of responsibility
/// [Pattern](https://refactoring.guru/design-patterns/chain-of-responsibility)
///
/// # Problem
/// Authentication system with sequential checks.
/// Keep adding checks of sanitaztion and anti-ddos.
///
/// # Solution
/// Each check should be it's own class, a handler.
/// Request and data is passed to handler.
/// Each handler has a field for reference for next handler.
/// A handler can stop the chain prematurely.
/// Ex. used on servers with auth checks or in DOM event model, bubbling.
///
///
///
///

/// Simple request with a string field.
pub struct Request(pub String);

/// The handler interface, which states you have to have:
/// next_handler to set the next handler in the chain.
/// handle to actually handle the data.
///
/// The tricky part is next_handler, becaues we want to take any concrete handler
/// that implements this Handler trait, so using the *dyn* keyword, telling
/// it that it's dynamically dispatched.
///
/// However, how does it know the size and lifetime of the concrete class?
/// You have to tell it...
pub trait Handler<'a> {
    fn next_handler(&mut self, next: &'a (dyn Handler<'a> + 'a)); // Sometimes, else a big structure handles it.

    fn handle(&self, req: &mut Request) -> bool;
}

pub struct ReqHandler<'a> {
    /// Optional reference to object of dynamic trait handler with lifetime 'a that cannot
    /// outlive lifetime 'a of ReqHandler struct.
    pub next: Option<&'a (dyn Handler<'a> + 'a)>,
}

/// Introduce 'a lifetime for Handler and ReqHandler.
impl<'a> Handler<'a> for ReqHandler<'a> {
    /// The next variable has lifetime 'a as has the trait Handler and it cannot
    /// outlive the + 'a lifetime.
    fn next_handler(&mut self, next: &'a (dyn Handler<'a> + 'a)) {
        self.next = Some(next)
    }

    fn handle(&self, req: &mut Request) -> bool {
        let content = req.0.clone();
        req.0 = content.repeat(2) + "\n";

        if let Some(next) = self.next {
            next.handle(req);
            return true;
        }

        false
    }
}
