class Entry {
  constructor(data, prev, next) {
    this.data = data;
    this.prev = prev;
    this.next = next;
  }
}


/**
 * Initialize your data structure here.
 */
var MyLinkedList = function() {
  this.head = null;
  this.tail = null;
  this.size = 0;
};

/**
 * Get the value of the index-th node in the linked list. If the index is invalid, return -1. 
 * @param {number} index
 * @return {number}
 */
MyLinkedList.prototype.get = function(index) {
  let node = this.head;
  for (let i=0;i<index;i++) {
    if (node == null) {
      break;
    }
    node = node.next;
  }
  if (node == null) {
    return -1;
  }
  return node.data;
};

/**
 * Add a node of value val before the first element of the linked list. After the insertion, the new node will be the first node of the linked list. 
 * @param {number} val
 * @return {void}
 */
MyLinkedList.prototype.addAtHead = function(val) {
  const e = new Entry(val, null, this.head);
  if (this.head != null) {
    this.head.prev = e;
  }
  this.head = e;    
  if (this.tail == null) {
    this.tail = e;
  }
  this.size++;
  return;
};

/**
 * Append a node of value val to the last element of the linked list. 
 * @param {number} val
 * @return {void}
 */
MyLinkedList.prototype.addAtTail = function(val) {
  const e = new Entry(val, this.tail, null);
  if (this.tail != null) {
    this.tail.next = e;    
  }
  this.tail = e;
  if (this.head == null) {
    this.head = e;
  }
  this.size++;
  return;
};

/**
 * Add a node of value val before the index-th node in the linked list. If index equals to the length of linked list, the node will be appended to the end of linked list. If index is greater than the length, the node will not be inserted. 
 * @param {number} index 
 * @param {number} val
 * @return {void}
 */
MyLinkedList.prototype.addAtIndex = function(index, val) {
  if (index === 0) {
    return this.addAtHead(val);
  }
  if (index === this.size) {
    return this.addAtTail(val);
  }
  
  let node = this.head;
  
  for (let i=0;i<index;i++) {
    if (node == null) {
      break;
    }
    node = node.next;
  }
  if (node == null) {
    return;
  }
  const { prev } = node;
  const e = new Entry(val, prev, node);
  node.prev = e;
  prev.next = e;
  this.size++;
};

/**
 * Delete the index-th node in the linked list, if the index is valid. 
 * @param {number} index
 * @return {void}
 */
MyLinkedList.prototype.deleteAtIndex = function(index) {
  let node = this.head;
  for (let i=0;i<index;i++) {
    if (node == null) {
      break;
    }
    node = node.next;
  }
  if (node == null) {
    return;
  }
  var {prev, next} = node;
  if (prev) {
    prev.next = next;    
  }
  if (next) {
    next.prev = prev;
  }
  
  if (index === 0) {
    this.head = next;
  } else if (index === this.size-1) {
    this.tail = prev;
  }
  this.size--;
};

MyLinkedList.prototype.list = function () {
  console.log("list");
  var n = this.head;
  for (var i=0;i<this.size;i++) {
    console.log(n.data);
    n = n.next;
  }
};

MyLinkedList.prototype.reverse = function () {
  console.log("reverse");
  var n = this.tail;
  for (var i=0;i<this.size;i++) {
    console.log(n.data);
    n = n.prev;
  }
};

/** 
 * Your MyLinkedList object will be instantiated and called as such:
 * var obj = new MyLinkedList()
 * var param_1 = obj.get(index)
 * obj.addAtHead(val)
 * obj.addAtTail(val)
 * obj.addAtIndex(index,val)
 * obj.deleteAtIndex(index)
 */
