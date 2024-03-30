use crate::doubly_linked_list::doubly_linked_list::Doubly_Linked_List;

pub mod doubly_linked_list;
fn main() {
    println!("Hello, world!");

    let arrays: Vec<i32> = (0..10).into_iter().collect();
    let mut  d_lined_list = Doubly_Linked_List::new();

    for i in arrays {
        d_lined_list.push(i);
    }

    d_lined_list.insert_at(0, 54).unwrap();
    d_lined_list.insert_at(4, 10).unwrap();

    d_lined_list.into_iter()
        .for_each(|val|
            println!("Vals {}",val.borrow_mut().val)
        );
}
