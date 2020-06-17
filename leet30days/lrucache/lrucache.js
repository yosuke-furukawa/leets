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
    console.log({remove:"remove", prev, next, ishead: this.head === entry, istail: this.tail === entry});
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
    console.log("remove", this.list());
    entry.prev = null;
    entry.next = null;
    this.length--;
    return this;
  }
}



/**
 * @param {number} capacity
 */
var LRUCache = function(capacity) {
  this.capacity = capacity;
  this.lruCacheList = new LinkedList();
  this.lruCacheMap = {};
};

/** 
 * @param {number} key
 * @return {number}
 */
LRUCache.prototype.get = function(key) {
  const entry = this.lruCacheMap[key];
  console.log({get: "get", key, entry, next: entry.next, prev: entry.prev});
  if (!entry) {
    return -1;
  }
  if (entry.data.key === this.lruCacheList.head.data.key) {
    return entry.data.value;
  }
  this.lruCacheList.remove(entry);
  const newEntry = this.lruCacheList.unshift(entry.data);
  this.lruCacheMap[key] = newEntry;
  console.log({get: "get", value: entry.data.value});
  console.log(this.lruCacheList.list());
  return entry.data.value;
};

/** 
 * @param {number} key 
 * @param {number} value
 * @return {void}
 */
LRUCache.prototype.put = function(key, value) {
  const entry = this.lruCacheMap[key];
  console.log({put: "put", key, value});
  if (entry) {
    this.lruCacheList.remove(entry);
  }
  const newEntry = this.lruCacheList.unshift({ key, value });
  console.log({get: "put", key, newEntry, next: newEntry.next, prev: newEntry.prev});
  this.lruCacheMap[key] = newEntry;
  if (this.lruCacheList.length > this.capacity) {
    const entry = this.lruCacheList.pop();
    console.log("evict", entry);

    if (entry) {
      delete this.lruCacheMap[entry.data.key];
    }
  }
  console.log(this.lruCacheList.list());
};

/** 
 * Your LRUCache object will be instantiated and called as such:
 * var obj = new LRUCache(capacity)
 * var param_1 = obj.get(key)
 * obj.put(key,value)
 */

var lrucache = new LRUCache(10);
lrucache.put(1,2);
lrucache.put(2,3);
lrucache.get(1);
lrucache.get(2);
lrucache.get(1);
lrucache.get(2);
lrucache.get(1);
lrucache.get(1);
