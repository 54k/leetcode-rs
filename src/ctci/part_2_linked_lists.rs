// #[derive(Debug)]
// struct ListNode {
//     val: i32,
//     next: *mut ListNode,
// }
//
// // Напишите код для удаления дубликатов из несортированного связного списка.
// // Дополнительно:
// // Как вы будете решать задачу, если использовать временный буфер запрещено?
// fn task_2_1(head: *mut ListNode) -> Box<ListNode> {
//     unsafe fn with_memory(mut head: *mut ListNode) {
//         use std::collections::*;
//         let mut set = HashSet::new();
//         let mut prev: *mut ListNode = std::ptr::null_mut();
//         while !head.is_null() {
//             if set.contains(&(*head).val) {
//                 (*prev).next = (*head).next;
//             } else {
//                 set.insert((*head).val);
//                 prev = head;
//             }
//             head = (*head).next;
//         }
//     }
//     unsafe {
//         with_memory(head);
//         Box::from_raw(head)
//     }
// }

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
type N = Option<Box<ListNode>>;

fn task_2_11(mut n: N) -> N {
    use std::collections::HashSet;
    let mut set = HashSet::new();
    let mut current = n.as_mut().unwrap();

    while let Some(i) = current.next.as_mut() {
        if set.contains(&i.val) {
            current.next = i.next.take();
        } else {
            set.insert(i.val);
        }
        current = current.next.as_mut().unwrap();
    }
    n
}

#[cfg(test)]
mod test {
    use super::*;

    // #[test]
    // fn test_2_1() {
    //     println!(
    //         "{:?}",
    //         task_2_1(Box::into_raw(Box::new(ListNode {
    //             val: 1,
    //             next: Box::into_raw(Box::new(ListNode {
    //                 val: 1,
    //                 next: Box::into_raw(Box::new(ListNode {
    //                     val: 2,
    //                     next: std::ptr::null_mut(),
    //                 })),
    //             })),
    //         })))
    //     );
    // }

    #[test]
    fn test_2_11() {
        let head = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode { val: 1, next: None })),
            })),
        }));
        println!("{:?}", task_2_11(head.clone()));
        println!("{:?}", head);
    }
}
