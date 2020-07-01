class Entry {
  constructor(data, prev, next) {
    this.data = data;
    this.prev = prev;
    this.next = next;
  }
}

class LinkedList {
  constructor(...entries) {
    this.head = null;
    this.tail = null;
    this.length = 0;
    entries.forEach((e) => this.unshift(e));
  }

  // add to tail
  push(data) {
    if (this.tail === null) {
      const e = new Entry(data, null, null);
      this.head = e;
      this.tail = e;
      this.length++;
      return e;
    }

    const e = new Entry(data, this.tail, null);
    this.tail.next = e;
    this.tail = e;
    this.length++;
    return e;
  }

  list() {
    var result = [];
    var h = this.head;
    while(h!==null) {
      result.push(h.data);
      h = h.next;
    }
    return result;
  }

  // get and remove from tail
  pop() {
    const t = this.tail;
    this.tail = t?.prev || null;
    if (this.tail !== null) {
      this.tail.next = null;
    }
    if (this.head === t) {
      this.head = null;
    }
    this.length--;
    return t;
  }

  // insert at top
  unshift(data) {
    if (this.head === null) {
      const e = new Entry(data, null, null);
      this.head = e;
      this.tail = e;
      this.length++;
      return e;
    }

    const e = new Entry(data, null, this.head);
    this.head.prev = e;
    this.head = e;
    this.length++;
    return e;
  }

  get(index) {
    let h = this.head;
    for (let i = 0; i < index; i++) {
      h = h?.next || null;
    }
    return h;
  }

  remove(entry) {
    const { prev, next } = entry;
    if (prev !== null) {
      prev.next = next;
    }
    if (next !== null) {
      next.prev = prev;
    }
    if (this.head === entry) {
      this.head = next;
    }
    if (this.tail === entry) {
      this.tail = prev;
    }
    entry.prev = null;
    entry.next = null;
    this.length--;
    return this;
  }
}


/**
 * @param {number[]} nums
 * @param {number} k
 * @return {number[]}
 */
var maxSlidingWindow = function(nums, k) {
  var maxs = [];
  var inits = nums.slice(0, k);
  var ll = new LinkedList(...inits);
  var max = Math.max(...inits);
  maxs.push(max);
    for (var i=0;i+k<nums.length;i++) {
      var v = nums[i+k];
      ll.unshift(v);
      var val = ll.pop();
      if (val.data === max) {
        max = Math.max(...ll.list());
        maxs.push(max);
      } else {
        if (max < v) {
          maxs.push(v);
          max = v;
        } else {
          maxs.push(max);
        }
      }
    }
  return maxs;
};
