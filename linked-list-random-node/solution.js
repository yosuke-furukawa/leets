/**
 * Definition for singly-linked list.
 * function ListNode(val, next) {
 *     this.val = (val===undefined ? 0 : val)
 *     this.next = (next===undefined ? null : next)
 * }
 */
/**
 * @param head The linked list's head.
        Note that the head is guaranteed to be not null, so it contains at least one node.
 * @param {ListNode} head
 */
var Solution = function(head) {
  var h = head;
  var length = 0;
  while (h != null) {
    length++;
    h = h.next;
  }
  this.head = head;
  this.length = length;
};

/**
 * Returns a random node's value.
 * @return {number}
 */
Solution.prototype.getRandom = function() {
  var r = Math.floor(Math.random() * this.length);
  
  var h = this.head;
  while(r > 0) {
    h = h.next;
    r--;
  }
  return h.val;
};

/** 
 * Your Solution object will be instantiated and called as such:
 * var obj = new Solution(head)
 * var param_1 = obj.getRandom()
 */
