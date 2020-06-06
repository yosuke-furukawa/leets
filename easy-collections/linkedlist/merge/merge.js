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

var mergeTwoLists = function(l1, l2) {
  var list = new ListNode();
  var head = list;

  while(l1 !== null && l2 !== null) {
    if (l1.val < l2.val) {
      list.next = new ListNode(l1.val);
      l1 = l1.next;
    } else {
      list.next = new ListNode(l2.val);
      l2 = l2.next;
    }
    list = list.next;
  }
  if (l1 !== null) {
    list.next = l1
  }
  if (l2 !== null) {
    list.next = l2
  }
  return head.next;
};

var l1 = new ListNode(-9);
var l2 = new ListNode(-10);
l1.pushAll([-5,-3,-2,-2,3,7]);
l2.pushAll([-8,-4,-3,-1,3]);
console.log(mergeTwoLists(l1, l2).listup());
