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
    prev: Option<Rc<RefCell<Node<T>>>>,
}

#[derive(Debug)]
struct LinkedList<T> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
}

impl<T: std::fmt::Debug + PartialEq + Clone> LinkedList<T> {
    fn new() -> Self {
        LinkedList {
            head: None,
            tail: None,
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


    fn find_index(&self, value: &T) -> Option<usize> {
        let mut current = self.head.clone();
        let mut index = 0;

        while let Some(node) = current {
            if node.borrow().value == *value {
                return Some(index);
            }
            index += 1;
            current = node.borrow().next.clone();
        }
        None
    }


    fn find_value(&self, index: usize) -> Option<T> {
        let mut current = self.head.clone();
        let mut i = 0;

        while let Some(node) = current {
            if i == index {
                return Some(node.borrow().value.clone());
            }
            i += 1;
            current = node.borrow().next.clone();
        }
        None
    }

    fn delete_val(&mut self, value: T) {
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


    fn delete_index(&mut self, index: usize) {
        let mut current = self.head.clone();
        let mut i = 0;

        while let Some(node) = current {
            if i == index {
                let prev = node.borrow().prev.clone();
                let next = node.borrow().next.clone();

                match (prev, next) {
                    (Some(prev_node), Some(next_node)) => {
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
            i += 1;
            current = node.borrow().next.clone();
        }
    }

    fn append_after_index(&mut self, index: usize, value: T) {
        let mut current = self.head.clone();
        let mut i = 0;

        while let Some(node) = current {
            if i == index {
                let new_node = Rc::new(RefCell::new(Node {
                    value,
                    next: node.borrow().next.clone(),
                    prev: Some(node.clone()),
                }));
                let next = node.borrow().next.clone();
                node.borrow_mut().next = Some(new_node.clone());
                if let Some(next_node) = next {
                    next_node.borrow_mut().prev = Some(new_node.clone());
                } else {
                    self.tail = Some(new_node.clone());
                }
                return;
            }
            i += 1;
            current = node.borrow().next.clone();
        }
    }


    fn append_before_index(&mut self, index: usize, value: T) {
        let mut current = self.head.clone();
        let mut i = 0;

        while let Some(node) = current {
            if i == index {
                let new_node = Rc::new(RefCell::new(Node {
                    value,
                    next: Some(node.clone()),
                    prev: node.borrow().prev.clone(),
                }));
                let prev = node.borrow().prev.clone();
                node.borrow_mut().prev = Some(new_node.clone());
                if let Some(prev_node) = prev {
                    prev_node.borrow_mut().next = Some(new_node.clone());
                } else {
                    self.head = Some(new_node.clone());
                }
                return;
            }
            i += 1;
            current = node.borrow().next.clone();
        }
    }

    fn print_list(&self){
        let mut current = self.head.clone();
        println!();
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
        println!("\n1. Append value\n2. Delete value\n\
        3. Delete index\n4. Find index\n5. Find value\
        \n6. Append after index\n7. Append before index\n\
        8. Print list\n-1. Exit");
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
                list.delete_val(value);
            }
            3 => {
                print!("Enter index to delete: ");
                io::stdout().flush().unwrap();
                let index = match read_input().trim().parse::<usize>() {
                    Ok(idx) => idx,
                    Err(_) => {
                        println!("Invalid input. Please enter a valid index.");
                        continue;
                    }
                };
                list.delete_index(index);
            }
            4 => {
                print!("Enter value to find index: ");
                io::stdout().flush().unwrap();
                let value = read_input();
                match list.find_index(&value) {
                    Some(index) => println!("Index of {} is {}", value, index),
                    None => println!("Value not found in list"),
                }
            }
            5 => {
                print!("Enter index to find value: ");
                io::stdout().flush().unwrap();
                let index = match read_input().trim().parse::<usize>() {
                    Ok(idx) => idx,
                    Err(_) => {
                        println!("Invalid input. Please enter a valid index.");
                        continue;
                    }
                };
                match list.find_value(index) {
                    Some(value) => println!("Value at index {} is {}", index, value),
                    None => println!("Index out of bounds"),
                }
            }
            6=>{
                print!("Enter index to append after: ");
                io::stdout().flush().unwrap();
                let index = match read_input().trim().parse::<usize>() {
                    Ok(idx) => idx,
                    Err(_) => {
                        println!("Invalid input. Please enter a valid index.");
                        continue;
                    }
                };
                print!("Enter value to append: ");
                io::stdout().flush().unwrap();
                let value = read_input();
                list.append_after_index(index, value);
            }
            7=>{
                print!("Enter index to append before: ");
                io::stdout().flush().unwrap();
                let index = match read_input().trim().parse::<usize>() {
                    Ok(idx) => idx,
                    Err(_) => {
                        println!("Invalid input. Please enter a valid index.");
                        continue;
                    }
                };
                print!("Enter value to append: ");
                io::stdout().flush().unwrap();
                let value = read_input();
                list.append_before_index(index, value);
            }
            8 => {
                list.print_list();
            }
            -1 => {
                break;
            }
            _ => {
                println!("Invalid option.");
            }
        }
    }
}