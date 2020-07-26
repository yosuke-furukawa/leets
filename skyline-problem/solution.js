/**
 * @param {number[][]} buildings
 * @return {number[][]}
 */
var getSkyline = function(buildings) {
  // store all candidate x-positions
  let cp = new Set();
  for(let [l,r,h] of buildings){
    cp.add(l);
    cp.add(r);
  }
  let cp1 = [...cp];
  cp1.sort((a,b)=>a-b);

  // find the height for every candidate x-position
  let n = buildings.length;
  let res = [[-1,0]];
  for(let p of cp1){
    let i = 0;
    let height = 0;
    while(i<n && buildings[i][0]<=p){
      if(buildings[i][1]>p){
        height = Math.max(height, buildings[i][2]);
      }
      i++;
    }
    if(res[res.length-1][1] === height) continue;
    res.push([p, height]);
  }

  return res.slice(1);
};
