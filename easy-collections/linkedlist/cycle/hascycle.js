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
  pushAll(val) {
    val.forEach((v) => this.push(v));
    console.log(this.listup());
  }
  listup() {
    var results = [];
    var next = this;
    while(next !== null) {
      results.push(next.val);
      next = next.next;
    }
  
    return results;
  }
}

var hasCycle = function(head) {
  while(head) {
    if (head.walked === true) {
      return true;
    }
    head.walked = true;
    head = head.next;
  }
  return false;
};


const ln = new ListNode(3);
var ln2 = new ListNode(2);
var ln3 = new ListNode(0);
var ln4 = new ListNode(-4);
ln.next = ln2;
ln2.next = ln3;
ln3.next = ln4;
ln4.next = ln;
console.log(hasCycle(ln));


