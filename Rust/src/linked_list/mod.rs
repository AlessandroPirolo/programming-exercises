pub mod linked_list {

    struct Node<T>
    where
        T: std::clone::Clone
    {
        prev: Option<Box<Node<T>>>,
        elem: T,
        succ: Option<Box<Node<T>>>
    }

    pub struct LinkedList<T>
    where
        T: std::cmp::PartialOrd + std::clone::Clone
    {
        front : Option<Box<Node<T>>>,
        back : Option<Box<Node<T>>>,
        len : i32
    }

    impl<T> Node<T> 
    where
        T: std::cmp::PartialOrd + std::clone::Clone
    {
        fn new(prev: Option<Box<Node<T>>>, elem: T, succ: Option<Box<Node<T>>>) -> Node<T> {
            Node {
                prev,
                elem,
                succ
            }
        }
    }

    impl<'a, T: std::clone::Clone> Clone for Node<T> {
        fn clone(&self) -> Node<T> {
            Node {
                prev: self.prev.clone(),
                elem: self.elem.clone(),
                succ: self.succ.clone()
            }
        }

        fn clone_from(&mut self, source: &Node<T>) {
            self.elem = source.elem.clone();
            self.prev = source.prev.clone();
            self.succ = source.succ.clone();
        }
    }

    impl<T> LinkedList<T>
    where
        T: std::cmp::PartialOrd + std::clone::Clone
    {
        pub fn new() -> LinkedList<T> {
            LinkedList {
                front: None,
                back: None,
                len: 0
            }
        }

        pub fn is_empty(&self) -> bool {
            self.len == 0
        }

        pub fn len(&self) -> i32 {
            self.len
        }

        pub fn clear(&mut self) {
            self.front = None;
            self.len = 0;
        }

        pub fn contains(&self, x: T) -> bool
        where 
            T: std::cmp::PartialOrd,
        {
            let mut cond = self.front.clone();
            let mut find : bool = false;
            while let Some(node) = cond {
                find = node.elem == x;
                cond = node.succ; 
            }
            find
        }

        pub fn front(&self) -> Option<&T> {
            match &(self.front) {
                None => None, 
                Some(node) => Some(&(node.elem))
            }
        }

        pub fn back(&self) -> Option<&T> {
            match &(self.back) {
                None => None, 
                Some(node) => Some(&(node.elem))
            }
        }

        pub fn push_front(&mut self, x: T) {
            match &(self.front) {
                None => {
                    let new_front_back = Node::new(None, x, None);
                    self.front = Some(Box::new(new_front_back.clone()));
                    self.back = Some(Box::new(new_front_back.clone()));
                },
                Some(_) => {
                    let new_front = Some(Box::new(Node::new(None, x, self.front.clone())));
                    self.front = new_front;
                    }
                };
        }

        pub fn pop_front(&mut self) -> Option<T> {
            match &(self.front) {
                None => None,
                Some(el) => {
                    let elem = el.elem.clone();
                    let new_front = match &(el.succ) {
                        None => None,
                        Some(node) => Some(Box::new(Node::new(None, node.elem.clone(), node.succ.clone())))
                    };
                    self.front = new_front;
                    Some(elem)
                }
            }
        }

        pub fn push_back(&mut self, x: T) {
            match &(self.front) {
                None => {
                    let new_front_back = Node::new(None, x, None);
                    self.front = Some(Box::new(new_front_back.clone()));
                    self.back = Some(Box::new(new_front_back.clone()));
                },
                Some(_) => {
                    let new_back = Some(Box::new(Node::new(self.back.clone(), x, None)));
                    self.back = new_back;
                    }
                };
        } 

        pub fn pop_back(&mut self) -> Option<T> {
            match &(self.back) {
                None => None,
                Some(el) => {
                    let elem = el.elem.clone();
                    let new_back = match &(el.prev) {
                        None => None,
                        Some(node) => Some(Box::new(Node::new(node.prev.clone(), node.elem.clone(), None)))
                    };
                    self.back = new_back;
                    Some(elem)
                }
            }
        }

        pub fn split_off(&mut self, at: i32) -> LinkedList<T> {
            if at > self.len {
                panic!("List too short!")
            } else {
                let mut new_list = LinkedList::new();
                let mut _at = at.clone();
                while _at > 0 {
                    new_list.push_front(self.pop_back().unwrap());
                    _at = _at - 1;
                }
                new_list
            }
        }

    }
}
