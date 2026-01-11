use resturant::hosting;

fn main() {
    eat_at_restaurant();
}

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}