use std::rc::Rc;
//It's a smart pointer
// that enables multiple ownership of data by keeping track of the number of references to the data it owns.
// When the last reference to the data goes out of scope, the data is automatically deallocated.
//let a = Rc::new(5);
//let b = Rc::clone(&a);
// b is a clone of 'a', but they both point to the same data

//Rc is not thread-safe. If you need shared ownership across threads, you should use Arc<T>


use std::cell::RefCell;
use std::io;
use std::io::Write;
use crate::read_input;
//RefCell<T> is a type that provides interior mutability,
//meaning you can mutate the data inside even if the RefCell itself is immutable.
//This is achieved by enforcing Rust's borrowing rules at runtime rather than at compile time.

//let x = RefCell::new(5);
// * x.borrow_mut() += 1;  // Mutably borrow and modify the value inside RefCell
// println!("{}", x.borrow()); // Immutably borrow and read the value


//let a = Rc::new(RefCell::new(5));
// let b = Rc::clone(&a);
// * b.borrow_mut() += 10;
// println!("{}", a.borrow()); // 15

#[derive(Debug)]
struct Node<T> {
    value: T,
    next: Option<Rc<RefCell<Node<T>>>>,
    prev: Option<Rc<RefCell<Node<T>>>>
}


#[derive(Debug)]
struct LinkedList<T>{
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>
}

impl<T: std::fmt::Debug> LinkedList<T>{
    fn new() -> Self{
        LinkedList{
            head: None,
            tail: None
        }
    }

    fn append(&mut self, value: T){
        //Wraps a new Node in Rc and RefCell, making it possible to share and mutate it.
        let new_node = Rc::new(RefCell::new(Node{
            value,
            next:None,
            prev: self.tail.clone()
        }));
        //Checks if the list already has a tail. If so, it updates the next pointer of the old tail to the new node and sets self.tail to the new node.
        //If there is no tail (i.e., the list is empty), both head and tail are set to the new node
        match self.tail.take(){
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_node.clone());
                self.tail = Some(new_node)
            }
            None => {
                self.head = Some(new_node.clone());
                self.tail = Some(new_node);
            }
        }
    }

    fn delete(&mut self, value: T) where T: PartialEq {
        let mut current = self.head.clone();

        while let Some(node) = current {
            //Temporarily borrows the RefCell content of the node to access its value.
            if node.borrow().value == value {
                let prev = node.borrow().prev.clone();
                let next = node.borrow().next.clone();

                match (prev, next) {
                    (Some(prev_node), Some(next_node)) => {
                        //Mutably borrows the RefCell content of the node to modify next or prev.
                        prev_node.borrow_mut().next = Some(next_node.clone());
                        next_node.borrow_mut().prev = Some(prev_node.clone());
                    }
                    (Some(prev_node), None) => {
                        prev_node.borrow_mut().next = None;
                        self.tail = Some(prev_node);
                    }
                    (None, Some(next_node)) => {
                        next_node.borrow_mut().prev = None;
                        self.head = Some(next_node);
                    }
                    (None, None) => {
                        self.head = None;
                        self.tail = None;
                    }
                }
                return;
            }
            current = node.borrow().next.clone();
        }
    }

    fn print_list(&self){
        let mut current = self.head.clone();
        while let Some(node) = current {
            print!("{:?} -> ", node.borrow().value);
            current = node.borrow().next.clone();
        }
        println!("None");
    }
}
//Some and None: These are variants of the Option type in Rust,
// which is used to represent an optional value (i.e., a value that can be present or absent).
// Some(value) indicates that a value is present, while None indicates that it is absent.

//Methods provided by RefCell to borrow the contained value.
//borrow gives an immutable reference to the value, while borrow_mut gives a mutable reference.
//These methods enforce Rust's borrowing rules at runtime rather than at compile-time, allowing interior mutability.

pub(crate) fn linked_list_input(){
    let mut list = LinkedList::new();
    let mut option = 0;
    while option!=-1{
        println!("1. Append\n2. Delete\n3. Print\n-1. Exit");
        option = read_input().trim().parse().unwrap();
        match option{
            1 => {
                print!("Enter value to append: ");
                io::stdout().flush().unwrap();
                let value = read_input();
                list.append(value);
            }
            2 => {
                print!("Enter value to delete: ");
                io::stdout().flush().unwrap();
                let value = read_input();
                list.delete(value);
            }
            3 => {
                list.print_list();
            }
            -1 => {
                println!("Exiting...");
            }
            _ => {
                println!("Invalid option");
            }
        }
    }
}