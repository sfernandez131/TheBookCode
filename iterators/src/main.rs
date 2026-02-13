fn main() {
    ex3();
}

fn ex1() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {val}");
    }
}

fn ex2() {
    let v1 = vec![1, 2, 3];
    let v2: Vec<u32> = v1.iter().map(|x| x + 1).collect();

    v1.iter().for_each(|x| println!("{x}"));
    v2.iter().for_each(|x| println!("{x}"));
}

fn ex3() {
    #[derive(PartialEq, Debug)]
    struct Shoe {
        size: u32,
        style: String,
    }

    fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
        shoes.into_iter().filter(|s| s.size == shoe_size).collect()
    }

    fn filter_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 9,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];
        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                }
            ]
        );
    }

    filter_by_size();

    let v = vec![1, 2, 3, 4];
    let a: Vec<_> = v
        .iter()
        .filter(|x: &&i32| *x % 2 == 0)
        .map(|x: &i32| x * 2)
        .collect();
    let b: Vec<_> = v
        .iter()
        .map(|x: &i32| x * 2)
        .filter(|x: &i32| x % 2 == 0)
        .collect();
    println!("{} {}", a[0], b[0]);
}
