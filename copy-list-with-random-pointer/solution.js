/**
 * // Definition for a Node.
 * function Node(val, next, random) {
 *    this.val = val;
 *    this.next = next;
 *    this.random = random;
 * };
 */
var list = function(head) {
  var h = head;
  var l = [];
  var r = [];
  while(h!=null) {
    l.push(h.val);
    r.push(h.random?.val);
    h = h.next;
  }
  console.log(l);
  console.log(r);
}

/**
 * @param {Node} head
 * @return {Node}
 */
var copyRandomList = function(head) {
  if (head == null) {
    return null;
  }
  var h = head;
  while(h != null) {
    var node = new Node(h.val, h.next, h.random);
    var next = h.next;
    h.next = node;
    h = next;
  }
  var copied = head.next;
  var c = copied;
  while(c != null) {
    var random = c.random;
    var next = c.next;
    c.random = random ? random.next : null; 
    c = next ? next.next : null;
  }
  var c2 = copied;
  var h2 = head;
  while(c2 != null || h2 != null) {
    var cnextnext = (c2 && c2.next && c2.next.next);
    var hnextnext = (h2 && h2.next && h2.next.next);
    c2.next = cnextnext;
    h2.next = hnextnext;
    c2 = c2.next;
    h2 = h2.next;
  }
  return copied;
};
