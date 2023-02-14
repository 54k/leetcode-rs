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

// Напишите код для разбиения связного списка вокруг значения х,
// так чтобы все узлы, меньшие х, предшествовали узлам, большим или равным х.
// Если х содержится в списке, то значения х должны следовать строго после элементов, меньших х (см. далее).
// Элемент разбивки х может находиться где угодно в "правой части";
// он не обязан располагаться между левой и правой частью.
//
// Пример:
// Ввод: 3->5->8->5->10->2->1 [значение разбивки = 5]
// Вывод: 3->2->1->5->8->5->10
fn task_2_4(mut n: TNode, x: i32) -> TNode {
    let mut head = None;
    let mut head_tail = &mut head;
    let mut tail = None;
    let mut tail_tail = &mut tail;

    while let Some(mut node) = n.take() {
        n = node.next.take();
        if node.val < x {
            if head_tail.is_none() {
                head = Some(node);
                head_tail = &mut head;
            } else {
                let _ = head_tail.as_mut().unwrap().next.insert(node);
                head_tail = &mut head_tail.as_mut().unwrap().next;
            }
        } else if tail_tail.is_none() {
            tail = Some(node);
            tail_tail = &mut tail;
        } else {
            let _ = tail_tail.as_mut().unwrap().next.insert(node);
            tail_tail = &mut tail_tail.as_mut().unwrap().next;
        }
    }

    head_tail.as_mut().unwrap().next = tail;
    head
}

// Два числа хранятся в виде связных списков, в которых каждый узел представляет один разряд.
// Все цифры хранятся в обратном порядке, при этом малдший разряд (единицы) хранится в начале списка.
// Напишите функцию которая суммирует два числа и возвращает резульат в виде связного списка.
// Пример:
// Ввод: (7-1-6) + (5-9-2), то есть 617 + 295
// Вывод: 2-1-9, то есть 912
// Дополнительно: решите задачу предполагая, что цифры записаны в прямом порядке.
// Ввод: (6-1-7) + (2-9-5), то есть 617 + 295
// Вывод: (9-1-2), то есть 912
fn task_2_5(a: TNode, b: TNode) -> TNode {
    fn reverse_list(mut n: TNode) -> TNode {
        let mut prev = None;
        while let Some(mut current) = n.take() {
            let next = current.next.take();
            current.next = prev;
            prev = Some(current);
            n = next;
        }
        prev
    }

    fn reverse_order(mut a: TNode, mut b: TNode) -> TNode {
        const RADIX: i32 = 10;
        let mut ans = None;
        let mut ans_ptr = &mut ans;
        let mut carry = 0;
        loop {
            let ai = if let Some(v) = a.take() {
                a = v.next;
                v.val
            } else {
                0
            };
            let bj = if let Some(v) = b.take() {
                b = v.next;
                v.val
            } else {
                0
            };

            let mut sum = ai + bj + carry;
            carry = sum / RADIX;
            sum %= RADIX;
            let n = Some(Box::new(ListNode::new(sum)));

            if ans_ptr.is_none() {
                ans = n;
                ans_ptr = &mut ans;
            } else {
                ans_ptr.as_mut().unwrap().next = n;
                ans_ptr = &mut ans_ptr.as_mut().unwrap().next;
            }

            if a.is_none() && b.is_none() {
                if carry > 0 {
                    ans_ptr.as_mut().unwrap().next = Some(Box::new(ListNode::new(carry)));
                }
                break;
            }
        }
        ans
    }

    reverse_list(reverse_order(a, b))
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

    #[test]
    fn test_2_4() {
        // 3->5->8->5->10->2->1
        let head = Some(Box::new(ListNode {
            val: 3,
            next: Some(Box::new(ListNode {
                val: 5,
                next: Some(Box::new(ListNode {
                    val: 8,
                    next: Some(Box::new(ListNode {
                        val: 5,
                        next: Some(Box::new(ListNode {
                            val: 10,
                            next: Some(Box::new(ListNode {
                                val: 2,
                                next: Some(Box::new(ListNode { val: 1, next: None })),
                            })),
                        })),
                    })),
                })),
            })),
        }));
        println!("{:?}", task_2_4(head, 5));
    }

    #[test]
    fn test_2_5() {
        let a = Some(Box::new(ListNode {
            val: 7,
            next: Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode { val: 6, next: None })),
            })),
        }));
        let b = Some(Box::new(ListNode {
            val: 5,
            next: Some(Box::new(ListNode {
                val: 9,
                next: Some(Box::new(ListNode { val: 2, next: None })),
            })),
        }));
        println!("{:?}", task_2_5(a, b));

        let c = Some(Box::new(ListNode {
            val: 9,
            next: Some(Box::new(ListNode {
                val: 9,
                next: Some(Box::new(ListNode { val: 9, next: None })),
            })),
        }));
        let d = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 0,
                next: Some(Box::new(ListNode {
                    val: 0,
                    next: Some(Box::new(ListNode { val: 1, next: None })),
                })),
            })),
        }));
        println!("{:?}", task_2_5(c, d));
    }
}
