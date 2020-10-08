/**
 * Definition for singly-linked list.
 * function ListNode(val, next) {
 *     this.val = (val===undefined ? 0 : val)
 *     this.next = (next===undefined ? null : next)
 * }
 */
/**
 * @param {ListNode} head
 * @param {number} k
 * @return {ListNode}
 */
var reverseKGroup = function(head, k) {
  var res = [];
  var h = head;
  while (h != null) {
    var stack = [];
    var n = h;
    for (var i=0;i<k;i++) {
      if (n == null) {
        break;
      }
      stack.unshift(n);
      n = n?.next;
    }
    if (stack.length === k) {
      res.push(...stack);      
    } else {
      res.push(...stack.reverse());
    }
    h = n;
  }
  
  var pre = new ListNode(-1);
  var p = pre;
  for (var s of res) {
    p.next = s;
    p = p.next;
  }
  p.next = null;

  return pre.next;
};
