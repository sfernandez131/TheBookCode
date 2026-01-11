use std::vec;

fn main() {
    let mut vector1 = Vec::new();
    vector1.push(8);
    let count_v1 = vector1.len();
    let type_v1 = type_of(&vector1);
    println!("{type_v1} contains {count_v1} values:\n{vector1:#?}");

    let mut vector2 = vec!(1,2,3);
    println!("{} contains {} values:\n{vector2:#?}", 
        type_of(&vector2), vector2.len());

    //let doesnt_exist = &vector1[100];
    let also_doesnt_exist = vector1.get(100);
    match also_doesnt_exist {
        Some(value) => println!("The value is {value}"),
        None => println!("There is no such value at index.")
    }

    let formatted_value = format!("Formatted String:\n{vector2:#?}");
    println!("{formatted_value}");
    println!("{also_doesnt_exist:#?}");
    println!("{vector1:#?}");

    println!("\n");
    print_vals_simple(&vector2);

    println!("\n");
    print_vals_inline(&vector2);

    println!("");
    print_vals_inline_v2(&mut vector2);

    let mut v = vec![1,2,3];
    let mut v2: Vec<&mut i32> = Vec::new();
    for i in &mut v {
        v2.push(i);
    }

    *v2[0] = 5;

    let a = *v2[0];
    let b = v[0];

    println!("{a} {b}");

    let mut a = String::from("tic");
    let b = String::from("tac");
    let c = String::from("toe");

    //let ttt = a + &b;
    let tttt = a.push_str(&b);
}

fn type_of<T>(_: &T) -> &str {
    std::any::type_name::<T>()
}

fn print_vals_simple(collection: &Vec<i32>){
    for i in collection {
        if i == &collection[collection.len() - 1]{
            print!("{i}");
        }
        else{
            print!("{i} ");
        }
    }
}

fn print_vals_inline(collection: &Vec<i32>) {
    let mut vector_str = String::new();
    for i in collection {
        if i == &collection[0]{
            vector_str += &i.to_string();
        }
        else{
            let mut val = i.to_string();
            val.insert_str(0, " ");
            vector_str += val.as_str();
        }
    }
    println!("{vector_str}");
}

fn print_vals_inline_v2(collection: &mut Vec<i32>) {
    let mut vec_str = String::new();
    let first_val = collection[0];
    for i in collection {
        if *i == first_val {
            vec_str += &i.to_string();
        }
        else {
            vec_str += format!(" {}", &i.to_string().as_str()).as_str();
        }
    }
    println!("{vec_str}");
}