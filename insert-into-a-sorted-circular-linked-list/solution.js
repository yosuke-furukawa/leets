/**
 * // Definition for a Node.
 * function Node(val, next) {
 *     this.val = val;
 *     this.next = next;
 * };
 */

/**
 * @param {Node} head
 * @param {number} insertVal
 * @return {Node}
 */
var insert = function(head, insertVal) {
  if (head == null) {
    var node = new Node(insertVal);
    node.next = node;
    return node;
  }
  
  if (head == head.next) {
    var node = new Node(insertVal);
    node.next = head;
    head.next = node;
    return head;
  }
  
  var slow = head;
  var fast = slow.next;
  var inserted = false;
  
  do {
    if (slow.val <= insertVal && fast.val >= insertVal) {
      var node = new Node(insertVal, fast);
      slow.next = node;
      inserted = true;
      break;
    } else {
      slow = slow.next;
      fast = fast.next;
    }
  } while(slow != head);
  
  if (!inserted) {
    do {
      if (slow.val > fast.val) {
        var node = new Node(insertVal, fast);
        slow.next = node;
        inserted = true;
        break;
      } else {
        slow = slow.next;
        fast = fast.next;
      }
    } while(fast != head);
  }
  
  if (!inserted) {
    var node = new Node(insertVal, fast);
    slow.next = node;
    inserted = true;
  }
  
  return head;
};
