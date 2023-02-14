// Lasciate ogne speranza, voi ch’entrate
// Оставь надежду, всяк сюда входящий

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
type TNode = Option<Box<ListNode>>;

// Напишите код для удаления дубликатов из несортированного связного списка.
// Дополнительно:
// Как вы будете решать задачу, если использовать временный буфер запрещено?
fn task_2_1(n: TNode) -> TNode {
    fn linear(n: TNode) -> TNode {
        use std::collections::HashSet;
        let mut set = HashSet::new();

        let mut head = None;
        let mut tail = &mut head;

        let mut next = n;
        while let Some(mut t) = next.take() {
            next = t.next.take();
            if !set.contains(&t.val) {
                set.insert(t.val);
                if tail.is_none() {
                    head = Some(t);
                    tail = &mut head;
                } else {
                    tail.as_mut().unwrap().next = Some(t);
                    tail = &mut tail.as_mut().unwrap().next;
                }
            }
        }

        head
    }

    fn quadratique(n: TNode) -> TNode {
        let mut head = None;
        let mut tail = &mut head;

        let mut next = n;
        while let Some(mut t) = next.take() {
            let mut runner_head = None;
            let mut runner_tail = &mut runner_head;

            let mut runner_next = t.next.take();
            while let Some(mut n) = runner_next.take() {
                runner_next = n.next.take();
                if n.val != t.val {
                    if runner_tail.is_none() {
                        runner_head = Some(n);
                        runner_tail = &mut runner_head;
                    } else {
                        runner_tail.as_mut().unwrap().next = Some(n);
                        runner_tail = &mut runner_tail.as_mut().unwrap().next;
                    }
                }
            }

            if tail.is_none() {
                head = Some(t);
                tail = &mut head;
            } else {
                tail.as_mut().unwrap().next = Some(t);
                tail = &mut tail.as_mut().unwrap().next;
            }

            next = runner_head
        }
        head
    }
    quadratique(n)
}

// Реализуйте алгоритм для нахождения в односвязном списке k-го элемента с конца
fn task_2_2(n: &TNode, k: i32) -> &TNode {
    fn non_req(n: &TNode, k: i32) -> &TNode {
        let mut p1 = n;
        let mut p2 = n;
        for _ in 0..k {
            p1 = &p1.as_ref().unwrap().next;
        }
        while let Some(p) = p1 {
            p1 = &p.next;
            p2 = &p2.as_ref().unwrap().next;
        }
        p2
    }
    non_req(n, k)
}

// Реализуйте алгоритм, удаляющий узел из середины односвязного списка
// (то есть узла, не являющегося ни начальным, ни конечным - не обязательно находящимся точно в середине).
// Доступ предоставляется только к этому узлу.
//
// Пример:
// Ввод: узел с из списка a->b->c->d->e->f
// Вывод: ничего не возвращается, но новый список имеет вид a->b->c->d->e->f
fn task_2_3(n: &mut TNode) {
    if n.is_none() {
        return;
    }
    let mut unwrapped = n;
    while let Some(u) = unwrapped {
        if let Some(mut n) = u.next.take() {
            u.val = n.val;
            u.next = n.next.take();
        }
        unwrapped = &mut u.next;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2_1() {
        let head = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 1,
                    next: Some(Box::new(ListNode {
                        val: 2,
                        next: Some(Box::new(ListNode {
                            val: 3,
                            next: Some(Box::new(ListNode { val: 3, next: None })),
                        })),
                    })),
                })),
            })),
        }));
        println!("{:?}", task_2_1(head));
    }

    #[test]
    fn test_2_2() {
        let head = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode { val: 5, next: None })),
                    })),
                })),
            })),
        }));
        println!("{:?}", task_2_2(&head, 3));
    }

    #[test]
    fn test_2_3() {
        let mut head = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode { val: 5, next: None })),
                    })),
                })),
            })),
        }));
        let mut mid = &mut head;
        for _ in 0..2 {
            mid = &mut mid.as_mut().unwrap().next;
        }
        task_2_3(mid);
        println!("{:?}", head);
    }
}
