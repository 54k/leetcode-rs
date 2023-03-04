class ListNode {
    static of(val, next) {
        return new ListNode(val, next ?? null)
    }

    constructor(val, next) {
        this.val = val
        this.next = next
    }

    toString() {
        return JSON.stringify(this)
    }
}

// 2.1 Напишите код для удаления дубликатов из несортированного связного списка.
// Дополнительно:
// Как вы будете решать задачу, если использовать временный буфер запрещено?
const task_2_1 = () => {
    const memo = (n) => {
        let set = new Set()
        let previous = null
        while (n !== null) {
            if (set.has(n.val)) {
                previous.next = n.next
            } else {
                set.add(n.val)
                previous = n
            }
            n = n.next;
        }
    }

    const no_memo = (n) => {
        let current = n
        while (current != null) {
            let runner = current
            while (runner.next != null) {
                if (current.val === runner.next.val) {
                    runner.next = runner.next.next
                } else {
                    runner = runner.next
                }
            }
            current = current.next
        }
    }

    const test_2_1 = () => {
        let head1 = ListNode.of(1, ListNode.of(2, ListNode.of(1, ListNode.of(3,))))
        memo(head1)
        console.log(head1.toString())
        let head2 = ListNode.of(1, ListNode.of(2, ListNode.of(1, ListNode.of(3,))))
        no_memo(head2)
        console.log(head2.toString())
    }
    test_2_1()
}

// 2.2 Реализуйте алгоритм для нахождения в односвязном списке k-го элемента с конца
const task_2_2 = () => {
    const iterative_sliding_window = (n, k) => {
        let slow = n;
        while (n.next !== null) {
            if (k === 1) {
                slow = slow.next
            } else {
                k--
            }
            n = n.next
        }
        return slow
    }

    const recursive = (n, k) => {
        let idx = {cnt: 0}
        const rec = (n, k, idx) => {
            if (n === null) {
                return n
            }
            let node = rec(n.next, k, idx)
            idx.cnt += 1
            if (idx.cnt === k) {
                return n
            }
            return node
        }
        return rec(n, k, idx)
    }

    const iterative_ctci = (n, k) => {
        let p1 = n
        let p2 = n
        for (let i = 0; i < k; i++) {
            if (p1 !== null) p1 = p1.next
        }
        while (p1 !== null) {
            p1 = p1.next
            p2 = p2.next
        }
        return p2
    }

    const test_2_2 = () => {
        let head1 = ListNode.of(1, ListNode.of(2, ListNode.of(1, ListNode.of(3))))
        console.log(iterative_sliding_window(head1, 1).toString())

        let head2 = ListNode.of(1, ListNode.of(2, ListNode.of(1, ListNode.of(3))))
        console.log(recursive(head2, 1).toString())

        let head3 = ListNode.of(1, ListNode.of(2, ListNode.of(1, ListNode.of(3))))
        console.log(iterative_ctci(head3, 1).toString())
    }
    test_2_2()
}

// 2.3 Реализуйте алгоритм, удаляющий узел из середины односвязного списка
// (то есть узла, не являющегося ни начальным, ни конечным - не обязательно находящимся точно в середине).
// Доступ предоставляется только к этому узлу.
//
// Пример:
// Ввод: узел с из списка a->b->c->d->e->f
// Вывод: ничего не возвращается, но новый список имеет вид a->b->c->d->e->f
const task_2_3 = () => {
    const remove_node = (n) => {
        if (n === null || n.next === null) {
            return;
        }
        let next = n.next
        n.val = next.val
        n.next = next.next
    }

    const test_2_3 = () => {
        let mid = ListNode.of(2, ListNode.of(1, ListNode.of(3)))
        let head = ListNode.of(1, mid)
        remove_node(mid)
        console.log(head.toString())
    }
    test_2_3()
}

// 2.4 Напишите код для разбиения связного списка вокруг значения х,
// так чтобы все узлы, меньшие х, предшествовали узлам, большим или равным х.
// Если х содержится в списке, то значения х должны следовать строго после элементов, меньших х (см. далее).
// Элемент разбивки х может находиться где угодно в "правой части";
// он не обязан располагаться между левой и правой частью.
//
// Пример:
// Ввод: 3->5->8->5->10->2->1 [значение разбивки = 5]
// Вывод: 3->2->1->5->8->5->10
const task_2_4 = () => {
    const ordered = (n, x) => {
        let headStart = null
        let headEnd = null
        let tailStart = null
        let tailEnd = null

        while (n !== null) {
            let next = n.next
            n.next = null
            if (n.val < x) {
                if (headStart === null) {
                    headStart = n
                    headEnd = headStart
                } else {
                    headEnd.next = n
                    headEnd = n
                }
            } else {
                if (tailStart === null) {
                    tailStart = n
                    tailEnd = tailStart
                } else {
                    tailEnd.next = n
                    tailEnd = n
                }
            }
            n = next
        }

        if (tailStart === null) {
            return headStart
        }

        headEnd.next = tailStart
        return headStart
    }

    const unordered = (n, x) => {
        let head = n
        let tail = n
        while (n !== null) {
            let next = n.next
            if (n.val < x) {
                n.next = head
                head = n
            } else {
                tail.next = n
                tail = n
            }
            n = next
        }
        tail.next = null
        return head
    }

    const test_2_4 = () => {
        let head1 = ListNode.of(3, ListNode.of(5, ListNode.of(8, ListNode.of(5, ListNode.of(10, ListNode.of(2, ListNode.of(1,)))))))
        console.log(ordered(head1, 5).toString())
        let head2 = ListNode.of(3, ListNode.of(5, ListNode.of(8, ListNode.of(5, ListNode.of(10, ListNode.of(2, ListNode.of(1,)))))))
        console.log(unordered(head2, 5).toString())
    }
    test_2_4()
}

// 2.5 Два числа хранятся в виде связных списков, в которых каждый узел представляет один разряд.
// Все цифры хранятся в обратном порядке, при этом малдший разряд (единицы) хранится в начале списка.
// Напишите функцию которая суммирует два числа и возвращает резульат в виде связного списка.
// Пример:
// Ввод: (7-1-6) + (5-9-2), то есть 617 + 295
// Вывод: 2-1-9, то есть 912
// Дополнительно: решите задачу предполагая, что цифры записаны в прямом порядке.
// Ввод: (6-1-7) + (2-9-5), то есть 617 + 295
// Вывод: (9-1-2), то есть 912
const task_2_5 = () => {
    const reverse = (n) => {
        let prev = null
        while (n !== null) {
            let next = n.next
            n.next = prev
            prev = n
            n = next
        }
        return prev
    }

    const sum_reverse_order = (a, b) => {
        let head = null
        let tail = null
        let carry = 0

        while (true) {
            let a_val = 0
            if (a !== null) a_val = a.val
            let b_val = 0
            if (b !== null) b_val = b.val

            let sum = a_val + b_val + carry
            carry = sum >= 10 ? 1 : 0
            sum %= 10

            let node = ListNode.of(sum)
            if (head === null) {
                head = tail = node
            } else {
                tail.next = node
                tail = node
            }
            if (carry > 0) {
                tail.next = ListNode.of(carry)
            }
            if (!!a) a = a.next
            if (!!b) b = b.next
            if (a === null && b === null) {

                break
            }
        }

        return head
    }

    const sum_straight_order = (a, b) => {
        return reverse(sum_reverse_order(reverse(a), reverse(b)))
    }

    const test_2_5 = () => {
        let a1 = ListNode.of(7, ListNode.of(1, ListNode.of(6)))
        let b1 = ListNode.of(5, ListNode.of(9, ListNode.of(2)))
        console.log(sum_reverse_order(a1, b1).toString())

        let a2 = ListNode.of(6, ListNode.of(1, ListNode.of(7)))
        let b2 = ListNode.of(2, ListNode.of(9, ListNode.of(5)))
        console.log(sum_straight_order(a2, b2).toString())

        let a3 = ListNode.of(9, ListNode.of(0, ListNode.of(0, ListNode.of(1))))
        let b3 = ListNode.of(9, ListNode.of(9, ListNode.of(9, ListNode.of(9))))
        console.log(sum_straight_order(a3, b3).toString())
    }
    test_2_5()
}

// 2.6
const task_2_6 = () => {
    const solve = (n) => {
        const eq = (a, b) => {
            while (a !== null && b !== null) {
                if (a.val !== b.val) {
                    return false
                }
                a = a.next
                b = b.next
            }
            return a === b
        }
        let prev = null
        while (n !== null) {
            let next = n.next
            if (eq(prev, next)) {
                return true
            }
            n.next = prev
            prev = n
            n = next
            if (eq(prev, n)) {
                return true
            }
        }
        return false
    }
    const test_2_6 = () => {
        const a = ListNode.of(1,
            ListNode.of(2,
                ListNode.of(3,
                    ListNode.of(2,
                        ListNode.of(1)))))
        console.log(solve(a).toString())
        const b = ListNode.of(1,
            ListNode.of(2,
                ListNode.of(2,
                    ListNode.of(1))))
        console.log(solve(b).toString())
        const c = ListNode.of(1,
            ListNode.of(2,
                ListNode.of(1,
                    ListNode.of(1))))
        console.log(solve(c).toString())
    }
    test_2_6()
}

// 2.7 Проверьте пересекаются ли два заданных односвязных списка. Верните узел пересечения.
// Учтите, что пересечени определяется ссылкой, а не значением.
// Иначе говоря, если k-й узел первого связного списка точно совпадает с j-м узлом второго связного списка,
// то списки считаются пересекающимися.
const task_2_7 = () => {
    const solve = (a, b) => {
        const len = (n) => {
            let len = 0
            while (n !== null) {
                len++
                n = n.next
            }
            return len
        }
        let a_len = len(a);
        let b_len = len(b);
        if (a_len > b_len) {
            let [t, t_len] = [a, a_len]
            a = b
            b = t
            a_len = b_len
            b_len = t_len
        }
        while (b_len-- !== a_len) {
            b = b.next
        }
        while (a !== null && b !== null) {
            if (a === b) {
                return a
            }
            a = a.next
            b = b.next
        }
        return null
    }
    const test_2_7 = () => {
        let joint = ListNode.of(3,
            ListNode.of(4));
        let a = ListNode.of(1,
            ListNode.of(2,
                joint))
        let b = ListNode.of(1, ListNode.of(1, ListNode.of(1, joint)))
        let c = ListNode.of(1, joint)
        console.log(solve(a, b).toString())
        console.log(solve(a, c).toString())
    }
    test_2_7()
}

// 2.8 Для кольцевого связного осписка реализуйте алгоритм возвращающий начальный узел петли.
// Ввод: А->B->C->D->E->C (предыдущий узел С)
const task_2_8 = () => {
    const solve = (n) => {
        let slow = n
        let fast = n
        while (fast !== null && fast.next !== null) {
            slow = slow.next
            fast = fast.next.next
            if (slow === fast) {
                slow = n
                while (slow !== fast) {
                    slow = slow.next
                    fast = fast.next
                }
                return slow
            }
        }
        return null
    }
    const test_2_8 = () => {
        let tail = ListNode.of('D');
        let joint = tail.next = ListNode.of('B', ListNode.of('C',
            tail))
        let ans = solve(ListNode.of('A', joint));
        console.log(ans)
    }

    test_2_8()
}

/* main */
(() => {
    task_2_1()
    task_2_2()
    task_2_3()
    task_2_4()
    task_2_5()
    task_2_6()
    task_2_7()
    task_2_8()
})()