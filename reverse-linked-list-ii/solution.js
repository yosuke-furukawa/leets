/**
 * Definition for singly-linked list.
 * function ListNode(val, next) {
 *     this.val = (val===undefined ? 0 : val)
 *     this.next = (next===undefined ? null : next)
 * }
 */
/**
 * @param {ListNode} head
 * @param {number} m
 * @param {number} n
 * @return {ListNode}
 */
var reverseBetween = function(head, m, n) {
  if (head == null) {
    return null;
  }

  var cur = head, prev = null;
  while (m > 1) {
    prev = cur;
    cur = cur.next;
    m--;
    n--;
  }

  var con = prev, tail = cur;

  var third = null;
  while (n > 0) {
    third = cur.next;
    cur.next = prev;
    prev = cur;
    cur = third;
    n--;
  }

  if (con != null) {
    con.next = prev;
  } else {
    head = prev;
  }

  tail.next = cur;
  return head;
};
