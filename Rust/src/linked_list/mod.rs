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

    pub struct Cursor<T> 
    where 
        T: std::clone::Clone
    {
        idx: usize,
        prev: Option<Box<Node<T>>>,
        succ: Option<Box<Node<T>>> 
    }

    impl<'a, T> Cursor<T> 
    where 
        T: std::clone::Clone
    {
        pub fn index(&self) -> usize {
            self.idx
        }

        pub fn move_next(&mut self) {
            self.idx += 1;
            self.ptr = self.ptr.unwrap().succ;
        }


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
            self.front.as_ref().map(|node| &node.as_ref().elem)
        }

        pub fn front_mut(&mut self) -> Option<&mut T> {
            self.front.as_mut().map(|node| &mut node.as_mut().elem) 
        }

        pub fn back(&self) -> Option<&T> {
            self.back.as_ref().map(|node| &node.as_ref().elem)
        }
        
        pub fn back_mut(&mut self) -> Option<&mut T> {
            self.back.as_mut().map(|node| &mut node.as_mut().elem) 
        }

        pub fn push_front(&mut self, x: T) {
            self.len += 1;
            match &(self.front) {
                None => {
                    let new_front_back = Node::new(None, x, None);
                    self.front = Some(Box::new(new_front_back));
                    self.back = Some(Box::new(new_front_back));
                },
                Some(_) => {
                    let new_front = Some(Box::new(Node::new(None, x, self.front.clone())));
                    self.front = new_front;
                    }
                };
        }

        pub fn pop_front(&mut self) -> Option<T> {
            self.len -= 1;
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
            self.len += 1;
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
            self.len -= 1;
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

        pub fn remove(&mut self, _at: i32) -> T {
            self.front.unwrap().elem.clone()
        }

    }
}
