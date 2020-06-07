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

var isPalindrome = function(head) {
  var count = 0;
  var h = head;
  if (h === null) {
    return true
  }
  if (h.next === null) {
    return true
  }
  while(h) {
    h = h.next;
    count++;
  }

  var half = count %2 ===0 ? count / 2 : (count/2-1);
  var fast = head;
  var stack = [];
  for (var i=0; i<half;i++){
    stack.unshift(fast.val);
    fast = fast.next;
  }
  if (count%2 ===1) {
    stack.unshift(fast.val);
  }
  while(stack.length > 0) {
    if (fast === null) {
      return false;
    }
    if (stack.shift() !== fast.val) {
      return false;
    }
    fast = fast.next;
  }
  return true;
};


const ln = new ListNode(1);
ln.pushAll([2,1]);
console.log(isPalindrome(ln));

