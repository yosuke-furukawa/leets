
/**
 * @param {number} n
 * @param {number[][]} edges
 * @return {number[]}
 */
var findMinHeightTrees = function(n, edges) {
  let g={};
  for(let i=0;i<n;i++) g[i]=new Set;
  for(let [a,b] of edges){
    g[a].add(b);
    g[b].add(a);
  }

  while(Object.keys(g).length > 2){
    Object.keys(g).filter(v=>g[v].size<2).forEach(v=>{
      g[v].forEach(nextv=>g[nextv].delete(parseInt(v)));
      delete g[v];
    })
  }

  return Object.keys(g);
};
