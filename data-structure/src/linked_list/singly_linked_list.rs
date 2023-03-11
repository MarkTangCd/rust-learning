mod LinkedList {
    enum ListNode<T> {
        Empty,
        NonEmpty(ListNodeValue<T>),
    }

    struct ListNodeValue<T> {
        item: T,
        next: Box<ListNode<T>>,
    }

    impl<T> ListNodeValue<T> {
        fn new(item: T, next: Box<ListNode<T>>) -> self {
            Self { item, next }
        }
    }

    impl<T> ListNode<T> {
        fn new(item: T, next: Box<ListNode<T>>) -> Self {
            ListNode::NonEmpty(ListNodeValue::new(item, next))
        }

        fn take(&mut self) -> Self {
            let mut cur = Self::Empty;
            std::mem::swap(&mut cur, self); // Swaps the values at two mutable locations, without deinitializing either one.
            cur
        }
    }

    impl<T> Clone for ListNodeValue<T>
    where
        T: Clone,
    {
        fn clone(&self) -> Self {
            Self {
                item: self.item.clone(),
                next: Box::clone(&self.next),
            }
        }
    }

    impl<T> Iterator for SinglyLinkedList<T> {
        type Item = T;

        fn next(&mut self) -> Option<Self::Item> {
            self.pop()
        }
    }

    pub struct SinglyLinkedList<T> {
        head: Box<ListNode<T>>,
        size: usize,
    }

    impl<T> SinglyLinkedList<T> {
        pub fn new() -> Self {
            Self {
                head: Box::new(ListNode::Empty),
                size: 0,
            }
        }

        pub fn len(&self) -> usize {
            self.size
        }

        pub fn push(&mut self, item: T) {
            let cur_head = self.head.take();
            let new_node = Box::new(ListNode::new(item, Box::new(cur_head)));

            self.head = new_node;
            self.size += 1;
        }

        pub fn pop(&mut self) -> Option<T> {
            let node = self.head.take();

            if let ListNode::NonEmpty(node) = node {
                self.head = node.next;
                self.size -= 1;
                Some(node.item)
            } else {
                None
            }
        }
    }
}
