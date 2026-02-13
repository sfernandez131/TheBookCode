use std::thread;

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }

        if num_red > num_blue {
            return ShirtColor::Red;
        }
        ShirtColor::Blue
    }
}

fn main() {
    ex6();
}

fn ex1() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user1 = Some(ShirtColor::Red);
    let user2 = None;

    let giveaway = store.giveaway(user1);
    println!("User1 with preference {user1:?} got: {giveaway:?}");

    let giveaway = store.giveaway(user2);
    println!("User2 with preference {user2:?} got: {giveaway:?}");
}

fn ex2() {
    let example_closure = || "Hello from a closure!";

    println!("{0}", example_closure());

    let list = vec![1, 2, 3];

    println!("Before defining closure: {list:?}");

    let only_borrow = || println!("From closure: {list:?}");

    println!("Before calling closure: {list:?}");
    only_borrow();
    println!("After calling closure: {list:?}");
}

fn ex3() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let mut borrows_mutably = || list.push(4);
    //println!("After defining closure: {list:?}");
    borrows_mutably();
    println!("After calling closure: {list:?}");
}

fn ex4() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    thread::spawn(move || println!("From thread: {list:?}"))
        .join()
        .unwrap();
}

fn ex5() {
    let mut unsorted_list = vec![2, 4, 0, 1, 2, 8];
    unsorted_list.sort();
    unsorted_list.reverse();
    println!("{unsorted_list:?}");
}

fn ex6() {
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    let mut list = [
        Rectangle {
            width: 3,
            height: 2,
        },
        Rectangle {
            width: 8,
            height: 3,
        },
        Rectangle {
            width: 40,
            height: 10,
        },
    ];

    let value = String::from("closure called");
    let mut num_sort_ops = 0;
    list.sort_by_key(|r| {
        num_sort_ops += 1;
        r.width
    });

    println!("Closure was called {num_sort_ops} times.");
}
