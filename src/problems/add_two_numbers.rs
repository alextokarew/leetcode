//Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    iterate(l1, l2, 0)
}

fn iterate(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>, offset: i32) -> Option<Box<ListNode>> {
    if l1.is_some() && l2.is_some() {
        let node1 = l1.unwrap();
        let node2 = l2.unwrap();
        let value = node1.val + node2.val + offset;
        let result = ListNode {
            val: value % 10,
            next: iterate(node1.next, node2.next, value / 10)
        };
        Some(Box::new(result))
    } else if l1.is_some() || l2.is_some() {
        let node = l1.or(l2).unwrap();
        if offset > 0 {
            let value = node.val + offset;
            let result = ListNode {
                val: value % 10,
                next: iterate(node.next, None, value / 10)
            };
            Some(Box::new(result))
        } else {
            Some(node)
        }
    } else {
        if offset > 0 {
            Some(Box::new(ListNode::new(offset)))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn list_from_vec(v: &[i32]) -> Option<Box<ListNode>> {
        if v.is_empty() {
            None
        } else {
            Some(Box::new(ListNode {val: v[0], next: list_from_vec(&v[1..])}))
        }
    }

    #[test]
    fn should_return_sum() {
        let a = list_from_vec(&[2,4,3]);
        let b = list_from_vec(&[5,6,4]);
        let result = add_two_numbers(a, b);
        assert_eq!(result, list_from_vec(&[7,0,8]))
    }

    #[test]
    fn should_deal_with_different_sizes() {
        let a = list_from_vec(&[5,2,4,3]);
        let b = list_from_vec(&[6,4]);
        let result = add_two_numbers(a, b);
        assert_eq!(result, list_from_vec(&[1,7,4,3]))
    }
    
    #[test]
    fn should_add_extra_digit() {
        let a = list_from_vec(&[2,4,6]);
        let b = list_from_vec(&[5,6,7]);
        let result = add_two_numbers(a, b);
        assert_eq!(result, list_from_vec(&[7,0,4,1]))
    }

    #[test]
    fn should_process_offset_correctly() {
        let a = list_from_vec(&[2,4,6]);
        let b = list_from_vec(&[5,6,7,9,9,9,8,7,6,5]);
        let result = add_two_numbers(a, b);
        assert_eq!(result, list_from_vec(&[7,0,4,0,0,0,9,7,6,5]))
    }
}