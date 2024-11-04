pub mod linked_list {

    struct Node<'a, T> {
        prev: Option<&'a Node<'a, T>>,
        elem: T,
        succ: Option<&'a Node<'a, T>>
    }

    pub struct LinkedList<'a, T> {
        start : Option<Node<'a, T>>
    }

    impl<'a, T> Node<'a, T> {
        fn new(prev: Option<&'a Node<'a, T>>, elem: T, succ: Option<&'a Node<'a, T>>) -> Node<'a, T> {
            Node {
                elem,
                prev, 
                succ
            }
        }

    }

    impl<'a, T> LinkedList<'a, T> {
        pub fn new() -> LinkedList<'a, T> {
            LinkedList {
                start: None 
            }
        }

    }
}
