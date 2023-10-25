#[derive(Debug, PartialEq, Eq)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

#[allow(dead_code)]
impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

#[allow(dead_code)]
type List = Option<Box<ListNode>>;

#[allow(dead_code)]
pub fn add_two_numbers(l1: List, l2: List) -> List {
    fn atn_impl(l1: List, l2: List, carry: i32) -> List {
        match (l1, l2, carry) {
            (None, None, 0) => None,
            (None, None, carry) => Some(Box::new(ListNode::new(carry))),
            (Some(l1), None, carry) | (None, Some(l1), carry) => {
                let sum = l1.val + carry;
                let carry = sum / 10;
                let val = sum % 10;
                Some(Box::new(ListNode {
                    val,
                    next: atn_impl(l1.next, None, carry),
                }))
            }
            (Some(l1), Some(l2), carry) => {
                let sum = l1.val + l2.val + carry;
                let carry = sum / 10;
                let val = sum % 10;
                Some(Box::new(ListNode {
                    val,
                    next: atn_impl(l1.next, l2.next, carry),
                }))
            }
        }
    }
    atn_impl(l1, l2, 0)
}

#[allow(unused_macros)]
macro_rules! make_list {
    ($($e:expr),*) => {
        {
            let mut head = None;
            $(
                let mut node = ListNode::new($e);
                node.next = head;
                head = Some(Box::new(node));
            )*
            head
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_add() {
        let l1 = make_list![3, 4, 2];
        let l2 = make_list![4, 6, 5];

        let result = add_two_numbers(l1, l2);

        assert_eq!(result.unwrap(), make_list!(8, 0, 7).unwrap());
    }

    #[test]
    fn test_add_with_no_carry() {
        let l1 = make_list![1, 1, 1];
        let l2 = make_list![2, 2, 2];

        let result = add_two_numbers(l1, l2);

        assert_eq!(result.unwrap(), make_list!(3, 3, 3).unwrap());
    }

    #[test]
    fn test_add_lists_of_different_length() {
        let l1 = make_list![1, 1, 1];
        let l2 = make_list![9];

        let result = add_two_numbers(l1, l2);

        assert_eq!(result.unwrap(), make_list!(1, 2, 0).unwrap());
    }
}
