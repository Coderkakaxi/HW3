use std::cell::RefCell;

macro_rules! hash_map {
    ($($key:expr => $val:expr),*) => {
        {
            let mut map = std::collections::HashMap::new();
            $(
                map.insert($key, $val);
            )*
            map
        }
    };
}

#[derive(Debug)]
struct SimpleStack<T> {
    stack: RefCell<Vec<T>>,
}

impl<T> SimpleStack<T> {
    fn new() -> SimpleStack<T> {
        SimpleStack {
            stack: RefCell::new(Vec::new()),
        }
    }

    fn push(&self, value: T) {
        self.stack.borrow_mut().push(value);
    }

    fn pop(&self) -> Option<T> {
        self.stack.borrow_mut().pop()
    }
}

struct MyRc<T> {
    data: *mut RcData<T>,
}

struct RcData<T> {
    value: UnsafeCell<T>,
    count: usize,
}

impl<T> MyRc<T> {
    fn new(value: T) -> Self {
        let data = Box::new(RcData {
            value: UnsafeCell::new(value),
            count: 1,
        });
        MyRc {
            data: Box::into_raw(data),
        }
    }

    fn clone(&self) -> Self {
        unsafe {
            (*self.data).count += 1;
        }
        MyRc { data: self.data }
    }

    fn strong_count(&self) -> usize {
        unsafe { (*self.data).count }
    }

    fn weak_count(&self) -> usize {
        // Weak count not implemented in this example
        0
    }

    fn get(&self) -> Option<&T> {
        unsafe { Some(&*(*self.data).value.get()) }
    }
}

impl<T> Drop for MyRc<T> {
    fn drop(&mut self) {
        unsafe {
            (*self.data).count -= 1;
            if (*self.data).count == 0 {
                Box::from_raw(self.data);
            }
        }
    }
}

fn main() {
    let map = hash_map! {
        "one" => 1,
        "two" => 2,
        "three" => 3
    };
    println!("{:?}", map);

    let stack = SimpleStack::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    println!("Popped value: {:?}", stack.pop());
    println!("Popped value: {:?}", stack.pop());

    let rc1 = MyRc::new(42);
    let rc2 = rc1.clone();

    println!("Strong count: {}", rc1.strong_count());
    println!("Weak count: {}", rc1.weak_count());

    println!("Value: {:?}", rc1.get());

    drop(rc1);

    println!("Strong count: {}", rc2.strong_count());
    println!("Weak count: {}", rc2.weak_count());

    println!("Value: {:?}", rc2.get());

    drop(rc2);
}