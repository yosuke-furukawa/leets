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
var insertionSortList = function(head) {
  var result = new ListNode(-Infinity);
  while(head != null) {
    var r = result;
    var p;
    while(r != null && r.val < head.val) {
      p = r;
      r = r.next;
    }
    p.next = new ListNode(head.val, r);
    head = head.next;
  }
  return result.next;
};
