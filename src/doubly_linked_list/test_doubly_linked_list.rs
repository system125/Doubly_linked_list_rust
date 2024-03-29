use super::doubly_linked_list::{self, Doubly_Linked_List};


#[test]
pub fn test_push_value(){
    let mut d_link_list = Doubly_Linked_List::new();
    
    let arr: Vec<i32> = (0..10).collect();
    for i in arr.clone().into_iter() {
        d_link_list.push(i);
    }

    let d_link_list_vec:Vec<i32> = d_link_list.into_iter()
        .map(|val_ptr| 
            val_ptr.borrow_mut().val)
        .collect();
    assert_eq!(
        arr,
        d_link_list_vec
    );
}

#[test]
pub fn reverse_iterator_test () {
    let mut d_link_list = Doubly_Linked_List::new();

    for i in (0..10).into_iter() {
        d_link_list.push(i);
    }

    let reverse_iter: Vec<i32> = d_link_list.into_iter_reverse()
            .map(|val| val.borrow_mut().val)
            .collect();
    
    let reverse_array: Vec<i32> = (0..10).rev().collect();

    assert_eq!(reverse_array,reverse_iter);
}