/**
 * Definition for singly-linked list.
 * function ListNode(val) {
 *     this.val = val;
 *     this.next = null;
 * }
 */

/**
 * @param {ListNode} headA
 * @param {ListNode} headB
 * @return {ListNode}
 */
var getIntersectionNode = function(headA, headB) {
  var ha = headA;
  var hb = headB;
  while(ha !== hb) {
    ha = ha ? ha.next : headB;
    hb = hb ? hb.next : headA;
  }
  return ha;
};
