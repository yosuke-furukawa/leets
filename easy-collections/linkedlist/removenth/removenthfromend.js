class ListNode {
  constructor(val) {
    this.val = val === undefined ? 0 : val;
    this.next = null;
  }

  push(val) {
    if (this.next === null) {
      var e = new ListNode(val);
      this.next = e;
      return e;
    }
    var next = this.next;
    while (next.next !== null) {
      next = next.next;
    }
    next.next = new ListNode(val);
  }
}

var removeNthFromEnd = function(head, n) {
  var dummy = new ListNode(-1);
  dummy.next = head;
  var slow = dummy;
  var fast = dummy;
  for (var i = 0; i< n+1; i++) {
    fast = fast.next;
  }
  while (fast !== null) {
    slow = slow.next;
    fast = fast.next;
  }
  slow.next = slow.next.next;
  return dummy.next;
};

const ln = new ListNode(1);
removeNthFromEnd(ln, 1);
console.log(ln);

