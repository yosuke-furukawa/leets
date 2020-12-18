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
var deleteNodes = function(head, m, n) {
  var h = head;
  var list = new ListNode(-1);
  var l = list;
  var count = 0;
  while (h != null) {
    if (count === m) {
      count = 0;
      var count2 = 0;
      var next = h;
      while(count2 < n) {
        next = next?.next;
        count2++;
      }
      h = next;
    }
    if (h != null) {
      l.next = new ListNode(h?.val);
    }
    count++;
    h = h?.next;
    l = l?.next;
  }
  return list.next;
};
