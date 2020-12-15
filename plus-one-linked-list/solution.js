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
var plusOne = function(head) {
  var num = "";
  var h = head;
  while(h != null) {
    num += h.val;
    h = h.next;
  }
  var bn = BigInt(num) + 1n;
  var str = bn.toString();
  
  var newone = new ListNode(-1);
  var n = newone;
  for (var i=0;i<str.length;i++) {
    n.next = new ListNode(+str[i]);
    n = n.next;
  }
  return newone.next;
};
