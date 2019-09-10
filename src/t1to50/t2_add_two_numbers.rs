///给出两个 非空 的链表用来表示两个非负的整数。其中，它们各自的位数是按照 逆序 的方式存储的
/// 并且它们的每个节点只能存储 一位 数字。
///如果，我们将这两个数相加起来，则会返回一个新的链表来表示它们的和。
///您可以假设除了数字 0 之外，这两个数都不会以 0 开头。
///
///example：
///
///输入：(2 -> 4 -> 3) + (5 -> 6 -> 4)
///输出：7 -> 0 -> 8
///原因：342 + 465 = 807

#[derive(Debug, PartialEq, Clone)]
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>
}

impl ListNode {
    fn new(val: i32) -> Self {
        ListNode {
            val,
            next: None
        }
    }
}

struct LinkList {
    head: Option<Box<ListNode>>
}

impl LinkList {
    fn new() -> LinkList {
        LinkList {
            head: None
        }
    }

    fn push(&mut self, val: i32) {
        let mut cur = &mut self.head;

        match cur {
            Some(_) => {
                while let Some(node) = cur {
                    cur = &mut node.next;
                }
            },
            None => ()
        }

        *cur = Some(Box::new(ListNode::new(val)));
    }
}

fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut ln = LinkList::new();
    let mut jud = true;

    let mut carry: u8 = 0;

    let mut left = l1;
    let mut right = l2;

    while jud {
        let v1 = match &left {
            Some(node) => node.val,
            None => 0
        };
        let v2 = match &right {
            Some(node) => node.val,
            None => 0
        };

        let v = v1+v2+(carry as i32);
        carry = (v / 10) as u8;

        ln.push(v % 10);


        if left.is_some() {
            left = left.unwrap().next;
        }

        if right.is_some() {
            right = right.unwrap().next;
        }

        if left.is_none() && right.is_none() {
            jud = false;
        }
    }

    if carry != 0 {
        ln.push(carry as i32);
    }

    ln.head
}

#[test]
fn test() {
    let mut l1_1 = Box::new(ListNode::new(2));
    let mut l1_2 = Box::new(ListNode::new(4));
    let l1_3 = Box::new(ListNode::new(3));
    l1_2.next = Some(l1_3);
    l1_1.next = Some(l1_2);
    let l1 = Some(l1_1);

    let mut l2_1 = Box::new(ListNode::new(5));
    let mut l2_2 = Box::new(ListNode::new(6));
    let l2_3 = Box::new(ListNode::new(4));
    l2_2.next = Some(l2_3);
    l2_1.next = Some(l2_2);
    let l2 = Some(l2_1);

    let mut ne_1 = Box::new(ListNode::new(7));
    let mut ne_2 = Box::new(ListNode::new(0));
    let ne_3 = Box::new(ListNode::new(8));
    ne_2.next = Some(ne_3);
    ne_1.next = Some(ne_2);
    let ne = Some(ne_1);

    let ce = add_two_numbers(l1, l2);

    assert_eq!(ce,ne);
}