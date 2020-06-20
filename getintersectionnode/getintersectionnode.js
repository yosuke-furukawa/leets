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
    ha = ha === null ? headB : ha.next;
    hb = hb === null ? headA : hb.next;
  }
  return ha;
};
