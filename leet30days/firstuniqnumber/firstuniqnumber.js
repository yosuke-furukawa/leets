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
    if (this.head === entry) {
        this.head = this.head.next;
    }
    if (this.tail === entry) {
        this.tail = this.tail.prev;
    }
    if (prev !== null) {
      prev.next = next;
    }
    if (next !== null) {
      next.prev = prev;
    }
    this.length--;
    return this;
  }
}

/** 
 * Your LRUCache object will be instantiated and called as such:
 * var obj = new LRUCache(capacity)
 * var param_1 = obj.get(key)
 * obj.put(key,value)
 */

/**
 * @param {number[]} nums
 */
var FirstUnique = function(nums) {
    this.nums = nums;
    this.list = new LinkedList();
    this.map = new Map();
    for (var i=0;i<this.nums.length;i++) {
        var c = this.nums[i];
        var e = this.map.get(c);
        if (!e) {
            var e = this.list.unshift(c);
            this.map.set(c, e);
        } else if (e === -1) {
            continue;
        } else {
            this.list.remove(e);
            this.map.set(c, -1);
        }
    }
};

/**
 * @return {number}
 */
FirstUnique.prototype.showFirstUnique = function() {
    return this.list.tail ? this.list.tail.data : -1;
};

/** 
 * @param {number} value
 * @return {void}
 */
FirstUnique.prototype.add = function(value) {
    var e = this.map.get(value);
    if (!e) {
        const e = this.list.unshift(value);
        this.map.set(value, e);
    } else if (value === e.data) {
        this.list.remove(e);
        this.map.set(value, -1);
    }
};

/** 
 * Your FirstUnique object will be instantiated and called as such:
 * var obj = new FirstUnique(nums)
 * var param_1 = obj.showFirstUnique()
 * obj.add(value)
 */
