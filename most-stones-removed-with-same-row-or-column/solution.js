/**
 * @param {number[][]} stones
 * @return {number}
 */
var removeStones = function(stones) {
  var N = stones.length;
  var dsu = new DSU(20000);
  for (var stone of stones) {
    dsu.union(stone[0], stone[1] + 10000);
  }
  
  const seen = new Set();
  for (var stone of stones) {
    seen.add(dsu.find(stone[0]));
  }
  return N - seen.size;
};

class DSU {
    constructor(N) {
        this.parent = new Array(N);
        for (var i = 0; i < N; ++i)
            this.parent[i] = i;
    }
    find(x) {
        if (this.parent[x] != x) this.parent[x] = this.find(this.parent[x]);
        return this.parent[x];
    }
    union(x, y) {
        this.parent[this.find(x)] = this.find(y);
    }
}
