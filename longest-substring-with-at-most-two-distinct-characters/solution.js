/**
 * @param {string} s
 * @return {number}
 */
var lengthOfLongestSubstringTwoDistinct = function(s) {
  const lrucache = new LRUCache(2);
  const arr = [...s];
  var count = 0;
  var pre = "";
  var max = count;
  while (arr.length > 0) {
    var c = arr.shift();
    if (lrucache.size() < 2 || lrucache.get(c)) {
      count++;
      max = Math.max(max, count);
    } else {
      count = lrucache.get(pre) + 1;
    }
    var n = lrucache.get(c) || 0;
    if (pre === c) {
      n++;
    } else {
      n = 1;
    }
    lrucache.put(c, n);
    pre = c;
  }
  return max;
};

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
  if (!entry) {
    return null;
  }
  if (entry.data.key === this.lruCacheList.head.data.key) {
    return entry.data.value;
  }
  this.lruCacheList.remove(entry);
  const newEntry = this.lruCacheList.unshift(entry.data);
  this.lruCacheMap[key] = newEntry;
  return entry.data.value;
};

/** 
 * @param {number} key 
 * @param {number} value
 * @return {void}
 */
LRUCache.prototype.put = function(key, value) {
  const entry = this.lruCacheMap[key];
  if (entry) {
    this.lruCacheList.remove(entry);
  }
  const newEntry = this.lruCacheList.unshift({ key, value });
  this.lruCacheMap[key] = newEntry;
  if (this.lruCacheList.length > this.capacity) {
    const entry = this.lruCacheList.pop();
  
    if (entry) {
      delete this.lruCacheMap[entry.data.key];
    }
  }
};

LRUCache.prototype.size = function() {
  return this.lruCacheList.length;
}
