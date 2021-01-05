/**
 * Definition for singly-linked list.
 * function ListNode(val, next) {
 *     this.val = (val===undefined ? 0 : val)
 *     this.next = (next===undefined ? null : next)
 * }
 */
/**
 * @param {ListNode} head
 * @return {ListNode}
 */
var deleteDuplicates = function(head) {
  if (head == null) {
    return null;
  }
  var p = head;
  var n = p.next;
  var result = new ListNode(-1);
  var r = result;
  while (p != null) {
    var dup = false;
    while (p.val === n?.val) {
      p = p.next;
      n = n.next;  
      dup = true;
    }
    if (!dup && p.val !== n?.val) {
      r.next = new ListNode(p.val);
      r = r.next; 
    }
    p = p.next;
    n = n?.next;
  }
  return result.next;
};
