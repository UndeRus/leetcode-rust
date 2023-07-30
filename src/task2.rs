/*
You are given two non-empty linked lists representing two non-negative integers. The digits are stored in reverse order, and each of their nodes contains a single digit. Add the two numbers and return the sum as a linked list.

You may assume the two numbers do not contain any leading zero, except the number 0 itself.
 */

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
}

struct Solution;

#[inline(always)]
fn reverse(list: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut c1 = list;
    let mut result: Option<Box<ListNode>> = None;
    while let Some(cc1) = c1.clone() {
        let mut new_item = ListNode::new(cc1.val);
        new_item.next = result.clone();
        result = Some(Box::new(new_item));
        c1 = cc1.next;
    }
    result
}

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut h1 = l1;
        let mut h2 = l2;

        let mut overflow = 0;

        let mut result = None;
        loop {
            let mut next_h1 = None;
            let mut next_h2 = None;

            let v1 = match h1.clone() {
                Some(boxval) => {
                    next_h1 = boxval.next;
                    boxval.val
                }
                None => {
                    0
                }
            };

            let v2 = match h2.clone() {
                Some(boxval) => {
                    next_h2 = boxval.next;
                    boxval.val
                }
                None => {
                    0
                }
            };


            if h1.clone().is_none() && h2.clone().is_none() && overflow == 0 {
                break;
            }

            let digit_sum = v1 + v2 + overflow;
            let digit = digit_sum % 10;

            let mut new_result = ListNode::new(digit);

            overflow = if digit_sum >= 10 {
                digit_sum / 10
            }  else {
                0
            };

            new_result.next = result.clone();
            result = Some(Box::new(new_result));


            h1 = next_h1;
            h2 = next_h2;
        }

        reverse(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_from_vec(ints: Vec<i32>) -> Option<Box<ListNode>> {
        let mut item = None;
        for value in ints.iter() {
            let mut item2 = ListNode::new(*value);
            item2.next = item.clone();
            let new_item = Some(Box::new(item2));
            item = new_item;
        }
        item
    }

    #[test]
    fn test() {
        /*
        let result = Solution::add_two_numbers(build_from_vec(vec![1, 2, 3]), build_from_vec(vec![4, 5, 6]));
        assert_eq!(result, build_from_vec(vec![5, 7, 9]));


        assert_eq!(Solution::add_two_numbers(
            build_from_vec(vec![9]),
            build_from_vec(vec![9, 9, 9, 9, 9, 9, 9, 9, 9, 1])),
                   build_from_vec(vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]));
        */


        /*
        assert_eq!(Solution::add_two_numbers(
            build_from_vec(vec![1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1]),
            build_from_vec(vec![4,6,5])),
            reverse(build_from_vec(vec![6,6,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1]))
        );
        */

        assert_eq!(Solution::add_two_numbers(
            build_from_vec(vec![9,9,9,9,9,9,9]),
            build_from_vec(vec![9,9,9,9])),
            reverse(build_from_vec(vec![8,9,9,9,0,0,0,1]))
        )
    }
}