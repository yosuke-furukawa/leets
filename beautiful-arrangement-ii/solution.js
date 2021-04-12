/**
 * @param {number} n
 * @param {number} k
 * @return {number[]}
 */
var constructArray = function(n, k) {
  let array = [];
  
  for (var num=1;num<=n;num++) {
    array.push(num);
  }
  let result = [array.shift()];
  let dir = 1;
  while (array.length > 0) {
    let peek = result[result.length - 1];
    let p1 = peek + k;
    let p2 = peek - k;
    let p3 = k - peek;
    if (dir < 0) {
      let prevK = k;
      for (const p of [p2, p3, p1]) {
        if (array.indexOf(p) >= 0) {
          let index = array.indexOf(p);
          let num = array.splice(index, 1);
          result.push(num[0]);
          k--;
          break;
        }
      }
      if (prevK === k) {
        let num = array.shift();
        result.push(num);
      }
    } else {
      let prevK = k;
      for (const p of [p1, p2, p3]) {
        if (array.indexOf(p) >= 0) {
          let index = array.indexOf(p);
          let num = array.splice(index, 1);
          result.push(num[0]);
          k--;
          break;
        }
      }
      if (prevK === k) {
        let num = array.shift();
        result.push(num);
      }
    }
    dir *= -1;
  }
  
  return result;
};
