function primeDecompose(num) {
  const primeFactors = [];
  let factor = 2;
  while (num >= factor * factor) {
    if (num % factor === 0) {
      primeFactors.push(factor);
      num = num / factor;
    } else {
      factor++;
    }
  }
  primeFactors.push(num);
  return primeFactors;
}

/**
 * @param {number[]} A
 * @return {number}
 */
var largestComponentSize = function(A) {
  const maxValue = Math.max(...A);
  const dsu = new DisjointSetUnion(maxValue);
  
  const numFactorMap = new Map();
  
  for (const num of A) {
    const primeFactors = [...new Set(primeDecompose(num))];
    numFactorMap.set(num, primeFactors[0]);
    for (var i=0;i<primeFactors.length-1;i++) {
      dsu.union(primeFactors[i], primeFactors[i+1]);
    }
  }
  
  let maxGroupSize = 0;
  const groupCount = new Map();
  for (const num of A) {
    const groupId = dsu.find(numFactorMap.get(num));
    const count = groupCount.get(groupId) || 0;
    groupCount.set(groupId, count+1);
    maxGroupSize = Math.max(maxGroupSize, count+1);
  }
  
  return maxGroupSize;
};

class DisjointSetUnion {
  constructor(size) {
    this.parent = new Array(size + 1);
    this.size = new Array(size + 1);
    for (let i=0;i<size+1;i++) {
      this.parent[i] = i;
      this.size[i] = 1;
    }
  }
  
  find(x) {
    if (this.parent[x] !== x) {
      this.parent[x] = this.find(this.parent[x]);
    }
    return this.parent[x];
  }
  
  union(x, y) {
    let px = this.find(x);
    let py = this.find(y);
    
    if (px === py) {
      return px;
    }
    
    if (this.size[px] > this.size[py]) {
      [px, py] = [py, px];
    }
    
    this.parent[px] = [py];
    this.size[py] += this.size[px];
    return py;
  }
}
