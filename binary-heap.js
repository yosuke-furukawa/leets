class BinaryHeap {
  constructor(compare) {
    this.compare = compare;
    this.content = [];
  }

  push(element) {
    this.content.push(element);
    var n = this.bubbleUp(this.content.length - 1);
    if (this.content[n] - this.content[n-1] > 0) {
      console.log(n-Math.floor((n)/2));
    } else {
      console.log(n);
    }
  }

  size() {
    return this.content.length;
  }

  bubbleUp(n) {
    const element = this.content[n];
    const score = this.compare(element);

    while(n > 0) {
      const parentN = Math.floor((n+1)/2) - 1;
      const parent = this.content[parentN];

      if (score >= this.compare(parent)) {
        break;
      }

      this.content[parentN] = element;
      this.content[n] = parent;
      n = parentN;
    }
    return n;
  }
}

const bh = new BinaryHeap((a) => a);
const arr = [7,1,5,0,4,2,9,8,3]

for (var item of arr) {
  bh.push(item);
}
