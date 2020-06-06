class ListNode {
  constructor(val) {
    this.val = val;
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

var deleteNode = function(node) {
  node.val = node.next.val;
  node.next = node.next.next;
};

const ln = new ListNode(4);
ln.push(5);
ln.push(1);
ln.push(9);
deleteNode(ln.next);
console.log(ln);
