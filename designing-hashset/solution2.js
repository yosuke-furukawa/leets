/**
 * Initialize your data structure here.
 */
var MyHashSet = function() {
  this.data = new Array(987);
  this.capacity = 987;
};

/** 
 * @param {number} key
 * @return {void}
 */
MyHashSet.prototype.add = function(key) {
  var hash = key % this.capacity;
  if (!this.data[hash]) {
    this.data[hash] = new LinkedList();
    this.data[hash].push(key);
  } else {
    var len = this.data[hash].length;
    var found = false;
    for (var i=0;i<len;i++) {
      if (this.data[hash].get(i).data === key) {
        found = true;
        break;
      }
    }
    if (!found) {
      this.data[hash].push(key);
    }
  }
};

/** 
 * @param {number} key
 * @return {void}
 */
MyHashSet.prototype.remove = function(key) {
  var hash = key % this.capacity;
  if (this.data[hash]) {
    var len = this.data[hash].length;
    for (var i=0;i<len;i++) {
      const entry = this.data[hash].get(i);
      if (entry.data === key) {
        this.data[hash].remove(entry);
        break;
      }
    }
  }
};

/**
 * Returns true if this set contains the specified element 
 * @param {number} key
 * @return {boolean}
 */
MyHashSet.prototype.contains = function(key) {
  var hash = key % this.capacity;
  if (this.data[hash]) {
    var len = this.data[hash].length;
    for (var i=0;i<len;i++) {
      if (this.data[hash].get(i).data === key) {
        return true;
        break;
      }
    }
  }
  return false;
};

/** 
 * Your MyHashSet object will be instantiated and called as such:
 * var obj = new MyHashSet()
 * obj.add(key)
 * obj.remove(key)
 * var param_3 = obj.contains(key)
 */

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
    entries.forEach((e) => this.push(e));
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
      result.push({ k:h.data.key, v: h.data.value });
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
  
  removeById(index) {
    const { prev, next } = this.get(index);
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
