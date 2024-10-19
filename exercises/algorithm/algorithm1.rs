/*
	single linked list merge
	This problem requires you to merge two ordered singly linked lists into one ordered singly linked list
*/
//Rust 是否进行拷贝跟变量的值是否完全分配在栈上无关，而是取决于该变量是否实现了 Copy 这个 trait。当然，如果变量想实现 Copy，那么值完全分配在栈上是前提。
//String 的实际字符串内容存在堆上，当调用 Box::new(String) 时，整个 String（包括它的栈上元数据和堆上数据）都被转移到堆上，由 Box 来管理。
//调用 Box::new(String) 会转移 String 的所有权，包括栈上的元数据和指向堆上数据的指针，但堆上的实际数据并不会被“移动”，而是由 Box 接管对它的管理。
use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;//NonNull 是 std::ptr 模块中的一个类型。它表示一个非空的裸指针，专门用来解决在使用裸指针时，指针可能为 null 的问题
//NonNull 的设计其实不是表示一个“暂时是空的”裸指针，而是完全保证指针永远不为 null。它在类型层面上强制要求，指针必须指向一个有效的内存地址。
use std::vec::*;//表示从 Rust 标准库中的 std::vec 模块导入该模块下的所有公共类型、函数和特性（traits）。* 是一个通配符，表示“导入所有内容”


//Rust 中的链表操作通常采用“先创建空链表，再进行操作”的模式。通过 new() 创建一个空链表实例，然后通过链表的各种方法（如 push, pop 等）来管理链表的内容。
#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node {
            val: t,
            next: None,
        }
    }
}
#[derive(Debug)]
struct LinkedList<T> {
    length: u32,
    start: Option<NonNull<Node<T>>>,
    end: Option<NonNull<Node<T>>>,
}

//default() 方法的作用是创建并返回一个新的链表实例。
impl<T: Clone + PartialOrd> Default for LinkedList<T> {
//impl<T: Clone + PartialOrd>：这是泛型的限制条件。它表示： T 是一个泛型类型，要求 T 实现了 Clone 和 PartialOrd 这两个 trait。
//Clone：要求 T 支持复制功能。这样可以在某些方法中安全地复制元素，而不是转移所有权。
//PartialOrd：要求 T 支持部分比较（例如 <, <=, > 等操作）。这在链表操作中可能用于排序或比较节点值
//Default for LinkedList<T>：表示我们在为 LinkedList<T> 这个结构体实现 Default trait。Default trait 的主要作用是为类型提供一个默认值（通常是“空”或“零”状态）
    fn default() -> Self {   //这个函数不接受任何参数，但它必须返回一个 LinkedList<T> 类型的实例。
        Self::new() //调用了 LinkedList<T> 的 new() 方法，该方法返回一个新的、空的 LinkedList<T> 实例。
    }
}

//new() 是显式创建空链表的惯例和常用方法。
// Default::default() 是为了兼容性和灵活性，使链表能在更多场景下使用。

impl<T: Clone + PartialOrd> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            start: None,
            end: None,
        }
    }
//我们可以把这个过程比作一个“递归”的结构。你首先构造了最内层的节点 n1，然后逐步向外添加节点，每次新的节点都会包裹着之前的节点。最终，header 包含的是最外层的节点 n5。 // 因为每个节点都通过 Box 包裹指向下一个节点，最后的链表顺序是从 header（即 n5）开始的，往里一层层嵌套到 n1。
    // 所以，虽然代码看起来像是从最里层的 n1 开始构造，但在实际执行过程中，是把最外层的节点作为头节点逐层往里“套”进去，最终形成完整的链表。
    // 代码的书写顺序 是从里到外（从 n1 到 n5）。
    // 实际链表的构造 是从外到里（从 n5 到 n1），即最外层 n5 是头节点。

    pub fn add(&mut self, obj: T) {
        let mut node = Box::new(Node::new(obj));
        node.next = None;
//核心作用是将一个 Box（堆上分配的对象）转换为一个非空指针 NonNull<T>，并用 Option 包装起来表示指针存在。
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
//Box::into_raw(node) 会返回一个指向堆内存中 node 的裸指针 *mut Node，这个指针不能保证是非空的（尽管通常情况下，它是非空的，因为 Box 本身不能是空的）
//NonNull::new_unchecked(ptr)：这个函数将一个裸指针 ptr（类型为 *mut T）转换为 NonNull<T> 类型，不进行空指针检查。这意味着调用者需要保证这个指针是非空的，否则可能导致未定义行为（UB）
//NonNull::new(ptr) 是安全版本，它会检查传入的指针是否为 null，如果是 null，它会返回 None。然而，NonNull::new_unchecked(ptr) 跳过了这个检查，因为调用者确信传入的指针不会为 null。
 // unsafe 关键字表示此代码段包含不安全操作。在 Rust 中，使用裸指针和调用 new_unchecked() 这种跳过检查的函数都被认为是不安全的操作，因为它们可能导致未定义行为（如空指针解引用）。Rust 的 unsafe 块表明你知道这段代码是潜在危险的，并且你已经确保在这些操作下不会出错。
        match self.end {  //通过匹配 self.end 来检查当前链表是否为空，或者是否已经有节点，并采取不同的操作。
            None => self.start = node_ptr,
            Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr },
           //更新尾指针需要通过解引用尾指针的裸指针来修改它的 next 字段，指向新节点
        }
        self.end = node_ptr;
        self.length += 1;
    }
    pub fn add_ptr(&mut self, ptr: Option<NonNull<Node<T>>>) {
        if ptr.is_none() {
            return;
        }

        unsafe { (*ptr.unwrap().as_ptr()).next = None }
        match self.end {
            None => self.start = ptr,
            Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = ptr },
        }
        self.end = ptr;
        self.length += 1;
    }



    pub fn get(&mut self, index: i32) -> Option<&T> {
        self.get_ith_node(self.start, index)
    }


   //定义了一个名为 get_ith_node 的递归函数，用于从链表中查找第 index 个节点，并返回该节点存储的数据的引用。
    //&mut self：表明该方法需要一个链表的可变引用（允许修改链表
//node: Option<NonNull<Node<T>>>：表示当前节点，可以是链表的某个节点（使用 NonNull<Node<T>> 类型的指针表示）或 None，如果链表为空或遍历结束。
//index: i32：表示希望查找的节点索引。每次递归调用时，该索引递减，直到找到目标节点
   fn get_ith_node(&mut self, node: Option<NonNull<Node<T>>>, index: i32) -> Option<&T> {
        match node {
            None => None,
            Some(next_ptr) => match index {
                0 => Some(unsafe { &(*next_ptr.as_ptr()).val }),//.val 是链表节点（Node<T>）结构体中的一个字段，通常用于存储该节点的数据。
                _ => self.get_ith_node(unsafe { (*next_ptr.as_ptr()).next }, index - 1),//递归自身
            }
        }
    }
	pub fn merge(list_a:LinkedList<T>,list_b:LinkedList<T>) -> Self
        where
        T: PartialOrd + Ord,
        {
            let mut node_a = list_a.start;
            let mut node_b = list_b.start;

            let mut merged_list = Self::new();

            while node_a.is_some() && node_b.is_some() {
                let val_a = unsafe { &(*node_a.unwrap().as_ptr()).val };
                let val_b = unsafe { &(*node_b.unwrap().as_ptr()).val };

                let mut tmp: Option<NonNull<Node<T>>>;
                if val_a < val_b {
                    tmp = unsafe { (*node_a.unwrap().as_ptr()).next };
                    merged_list.add_ptr(node_a);
                    node_a = tmp;
                } else {
                    tmp = unsafe { (*node_b.unwrap().as_ptr()).next };
                    merged_list.add_ptr(node_b);
                    node_b = tmp;
                }
            }

            if node_a.is_some() {
                unsafe { (*merged_list.end.unwrap().as_ptr()).next = node_a }
            } else {
                unsafe { (*merged_list.end.unwrap().as_ptr()).next = node_b }
            }

            merged_list.length = list_a.length + list_b.length;

            merged_list
        }
	}

impl<T> Display for LinkedList<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.start {
            Some(node) => write!(f, "{}", unsafe { node.as_ref() }),
            None => Ok(()),
        }
    }
}

impl<T> Display for Node<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.next {
            Some(node) => write!(f, "{}, {}", self.val, unsafe { node.as_ref() }),
            None => write!(f, "{}", self.val),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn create_numeric_list() {
        let mut list = LinkedList::<i32>::new();
        list.add(1);
        list.add(2);
        list.add(3);
        println!("Linked List is {}", list);
        assert_eq!(3, list.length);
    }

    #[test]
    fn create_string_list() {
        let mut list_str = LinkedList::<String>::new();
        list_str.add("A".to_string());
        list_str.add("B".to_string());
        list_str.add("C".to_string());
        println!("Linked List is {}", list_str);
        assert_eq!(3, list_str.length);
    }

    #[test]
    fn test_merge_linked_list_1() {
        let mut list_a = LinkedList::<i32>::new();
        let mut list_b = LinkedList::<i32>::new();
        let vec_a = vec![1, 3, 5, 7];
        let vec_b = vec![2, 4, 6, 8];
        let target_vec = vec![1, 2, 3, 4, 5, 6, 7, 8];

        for i in 0..vec_a.len() {
            list_a.add(vec_a[i]);
        }
        for i in 0..vec_b.len() {
            list_b.add(vec_b[i]);
        }
        println!("list a {} list b {}", list_a, list_b);
        let mut list_c = LinkedList::<i32>::merge(list_a, list_b);
        println!("merged List is {}", list_c);
        for i in 0..target_vec.len() {
            assert_eq!(target_vec[i], *list_c.get(i as i32).unwrap());
        }
    }
    #[test]
    fn test_merge_linked_list_2() {
        let mut list_a = LinkedList::<i32>::new();
        let mut list_b = LinkedList::<i32>::new();
        let vec_a = vec![11, 33, 44, 88, 89, 90, 100];
        let vec_b = vec![1, 22, 30, 45];
        let target_vec = vec![1, 11, 22, 30, 33, 44, 45, 88, 89, 90, 100];

        for i in 0..vec_a.len() {
            list_a.add(vec_a[i]);
        }
        for i in 0..vec_b.len() {
            list_b.add(vec_b[i]);
        }
        println!("list a {} list b {}", list_a, list_b);
        let mut list_c = LinkedList::<i32>::merge(list_a, list_b);
        println!("merged List is {}", list_c);
        for i in 0..target_vec.len() {
            assert_eq!(target_vec[i], *list_c.get(i as i32).unwrap());
        }
    }
}