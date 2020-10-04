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
var rotateRight = function(head, k) {
  var size = 0;
  var node = head;
  while(node != null) {
    size++;
    node = node.next;
  }
  
  if (size === 0) {
    return head;
  }
  
  var target = size - k;
  while (target < 0) {
    target = size + target;
  }
  // console.log(target)
  if (target === 0) {
    return head;
  }
  if (target === size) {
    return head;
  }
  
  var h = head;
  for (var i=0;i<target-1;i++) {
    h = h.next;
  }

  var newhead = h.next;
  var nh = newhead;
  var pre = h;
  h.next = null;
  while (nh != null) {
    pre = nh;
    nh = nh.next;
  }
  pre.next = head;
  return newhead;
};
