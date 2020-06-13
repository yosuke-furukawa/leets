/**
 * @param {ListNode} head
 * @return {ListNode}
 */
var middleNode = function(head) {
    var h = head;
    var count = 0;
    while (h != null) {
        h = h.next;
        count++;
    }
    var mid = Math.floor(count / 2);
    var middle = head;
    while (mid--) {
        middle = middle.next;
    }
    return middle;
};
