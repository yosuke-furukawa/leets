/**
 * Definition for singly-linked list.
 * function ListNode(val, next) {
 *     this.val = (val===undefined ? 0 : val)
 *     this.next = (next===undefined ? null : next)
 * }
 */
/**
 * @param {ListNode} list1
 * @param {number} a
 * @param {number} b
 * @param {ListNode} list2
 * @return {ListNode}
 */
var mergeInBetween = function(list1, a, b, list2) {
  var head = list1;
  var n1f = list1?.next;
  var n1s = list1;
  var n2f = list2?.next;
  var n2s = list2;
  var replaceA = null;
  var replaceANext = null;
  var replaceB = null;
  var replaceBNext = null;
  while(n1f != null) {
    if (n1f.val === a) {
      replaceA = n1s;
      replaceANext = n2s;
      while (n2f != null) {
        n2f = n2f.next;
        n2s = n2s.next;
      }
    }
    if (replaceA && n1s.val === b) {
      replaceB = n2s;
      replaceBNext = n1f;
      break;
    }
    n1s = n1s.next;
    n1f = n1f.next;
  }
  if (replaceB == null) {
    return -1;
  }
  replaceA.next = replaceANext;
  replaceB.next = replaceBNext;
  return list1;
};
