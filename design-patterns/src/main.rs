mod behavioral;
mod creational;
mod structural;

use behavioral::chain::*;

fn main() {
    // Chain pattern
    let mut h1 = ReqHandler { next: None };
    let mut h2 = ReqHandler { next: None };
    let mut h3 = ReqHandler { next: None };
    let mut h4 = ReqHandler { next: None };

    // They must be binded in reverse order or the compiler will be cross!
    // It's because you can't borrow it mutably and then immutably at the same time.
    h3.next_handler(&mut h4);
    h2.next_handler(&mut h3);
    h1.next_handler(&mut h2);

    let mut req = Request("Hello".to_owned());

    h1.handle(&mut req);

    println!("{}", req.0);
}
