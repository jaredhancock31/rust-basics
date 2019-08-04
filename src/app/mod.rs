mod models;
use models::User;

pub fn run() {
    println!("Warming up");

    let mut next = models::User::new("butt", "hole");
    do_stuff(&next);
    println!("Okay cool, we're done.");
}


fn do_stuff(usr: &User)  {
    println!("{}", usr.username);
}