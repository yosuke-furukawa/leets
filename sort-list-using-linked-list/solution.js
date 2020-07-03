'use strict';


var BinaryHeap = function (customCompare) {
  this.list = [];

  if (customCompare) {
    this.compare = customCompare;
  }
};

BinaryHeap.prototype.buildHeap = function (keys, values) {
  if (typeof values !== 'undefined' && values.length !== keys.length) {
    throw new Error('Key array must be the same length as value array');
  }

  var nodeArray = [];

  for (var i = 0; i < keys.length; i++) {
    nodeArray.push(new Node(keys[i], values ? values[i] : undefined));
  }

  buildHeapFromNodeArray(this, nodeArray);
};

BinaryHeap.prototype.clear = function () {
  this.list.length = 0;
};

BinaryHeap.prototype.extractMinimum = function () {
  if (!this.list.length) {
    return undefined;
  }
  if (this.list.length === 1) {
    return this.list.shift();
  }
  var min = this.list[0];
  this.list[0] = this.list.pop();
  heapify(this, 0);
  return min;
};

BinaryHeap.prototype.findMinimum = function () {
  return this.isEmpty() ? undefined : this.list[0];
};

BinaryHeap.prototype.insert = function (key, value) {
  var i = this.list.length;
  var node = new Node(key, value);
  this.list.push(node);
  var parent = getParent(i);
  while (typeof parent !== 'undefined' &&
      this.compare(this.list[i], this.list[parent]) < 0) {
    swap(this.list, i, parent);
    i = parent;
    parent = getParent(i);
  }
  return node;
};

BinaryHeap.prototype.isEmpty = function () {
  return !this.list.length;
};

BinaryHeap.prototype.size = function () {
  return this.list.length;
};

BinaryHeap.prototype.union = function (otherHeap) {
  var array = this.list.concat(otherHeap.list);
  buildHeapFromNodeArray(this, array);
};

BinaryHeap.prototype.compare = function (a, b) {
  if (a.key > b.key) {
    return 1;
  }
  if (a.key < b.key) {
    return -1;
  }
  return 0;
};

function heapify(heap, i) {
  var l = getLeft(i);
  var r = getRight(i);
  var smallest = i;
  if (l < heap.list.length &&
      heap.compare(heap.list[l], heap.list[i]) < 0) {
    smallest = l;
  }
  if (r < heap.list.length &&
      heap.compare(heap.list[r], heap.list[smallest]) < 0) {
    smallest = r;
  }
  if (smallest !== i) {
    swap(heap.list, i, smallest);
    heapify(heap, smallest);
  }
}

function buildHeapFromNodeArray(heap, nodeArray) {
  heap.list = nodeArray;
  for (var i = Math.floor(heap.list.length / 2); i >= 0; i--) {
    heapify(heap, i);
  }
}

function swap(array, a, b) {
  var temp = array[a];
  array[a] = array[b];
  array[b] = temp;
}

function getParent(i) {
  if (i === 0) {
    return undefined;
  }
  return Math.floor((i - 1) / 2);
}

function getLeft(i) {
  return 2 * i + 1;
}

function getRight(i) {
  return 2 * i + 2;
}

function Node(key, value) {
  this.key = key;
  this.value = value;
}


/**
 * Definition for singly-linked list.
 * function ListNode(val, next) {
 *     this.val = (val===undefined ? 0 : val)
 *     this.next = (next===undefined ? null : next)
 * }
 */
/**
 * @param {ListNode} head
 * @return {ListNode}
 */
var sortList = function(head) {
  const heap = new BinaryHeap();
  var h = head;
  while(h != null) {
    heap.insert(h.val);
    h = h.next;
  }
  var result = new ListNode(-Infinity);
  var rh = result;
  while (!heap.isEmpty()) {
    var node = heap.extractMinimum();
    rh.next = new ListNode(node.key);
    rh = rh.next;
  }
  return result.next;
};
