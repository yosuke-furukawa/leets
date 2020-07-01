/**
 * Definition for singly-linked list.
 * function ListNode(val, next) {
 *     this.val = (val===undefined ? 0 : val)
 *     this.next = (next===undefined ? null : next)
 * }
 */
/**
 * @param {ListNode[]} lists
 * @return {ListNode}
 */
var mergeKLists = function(lists) {
  var result = new ListNode(-1);
  var head = result;
  var min = Infinity;
  var minIndex = -1;
  var values = [];
  var i = 0;
  var counter = 0;
  while(lists.length > 0) {
    var node = lists[i];
    if (node == null) {
      lists.splice(i, 1);
      i--;
    }
    if (min > node?.val) {
      min = node.val;
      minIndex = i;
    }
    if (i === lists.length-1) {
      if (minIndex < 0) {
        break;
      }
      head.next = new ListNode(min);
      head = head.next;
      lists[minIndex] = lists[minIndex].next;
      min = Infinity;
      minIndex = -1;
      i=-1;
    }
    i = (i+1) % lists.length;
  }
  return result.next;
};
