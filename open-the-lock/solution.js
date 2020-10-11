/**
 * @param {string[]} deadends
 * @param {string} target
 * @return {number}
 */
const mod10 = (x) => x - 10 * Math.floor(x / 10);


var openLock = function(deadends, target) {
  const queue = [];
  const visited = new Set();
  if (deadends.includes("0000")) {
    return -1;
  }
  queue.push([0,0,0,0,0]);
  visited.add("0000");
  
  while (queue.length > 0) {
    const [p1,p2,p3,p4,p5] = queue.pop();
    const vsize = visited.size;
    if (target === [p1,p2,p3,p4].join("")) {
      return p5;
    }
    const n1 = [mod10(p1-1),p2,p3,p4];
    const n2 = [mod10(p1+1),p2,p3,p4];
    const n3 = [p1,mod10(p2-1),p3,p4];
    const n4 = [p1,mod10(p2+1),p3,p4];
    const n5 = [p1,p2,mod10(p3-1),p4];
    const n6 = [p1,p2,mod10(p3+1),p4];
    const n7 = [p1,p2,p3,mod10(p4-1)];
    const n8 = [p1,p2,p3,mod10(p4+1)];
    const n1s = n1.join("");
    const n2s = n2.join("");
    const n3s = n3.join("");
    const n4s = n4.join("");
    const n5s = n5.join("");
    const n6s = n6.join("");
    const n7s = n7.join("");
    const n8s = n8.join("");
    

    if (!deadends.includes(n1s) && !visited.has(n1s)) {
      visited.add(n1s);
      n1.push(p5+1);
      queue.unshift(n1);
    }
    
    if (!deadends.includes(n2s) && !visited.has(n2s)) {
      visited.add(n2s);
      n2.push(p5+1);
      queue.unshift(n2);
    }
    
    if (!deadends.includes(n3s) && !visited.has(n3s)) {
      visited.add(n3s);
      n3.push(p5+1);
      queue.unshift(n3);
    }
    
    if (!deadends.includes(n4s) && !visited.has(n4s)) {
      visited.add(n4s);
      n4.push(p5+1);
      queue.unshift(n4);
    }
    
    if (!deadends.includes(n5s) && !visited.has(n5s)) {
      visited.add(n5s);
      n5.push(p5+1);
      queue.unshift(n5);
    }
    
    if (!deadends.includes(n6s) && !visited.has(n6s)) {
      visited.add(n6s);
      n6.push(p5+1);
      queue.unshift(n6);
    }
    
    if (!deadends.includes(n7s) && !visited.has(n7s)) {
      visited.add(n7s);
      n7.push(p5+1);
      queue.unshift(n7);
    }
    
    if (!deadends.includes(n8s) && !visited.has(n8s)) {
      visited.add(n8s);
      n8.push(p5+1);
      queue.unshift(n8);
    }
  }
  return -1
};
