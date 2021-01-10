/**
 * @param {number[]} instructions
 * @return {number}
 */
var createSortedArray = function(instructions) {
  let res = 0;
  let c = new Array(100001).fill(0);

  for(let i = 0; i< instructions.length; i++){
    res += Math.min(get(instructions[i]-1), i - get(instructions[i]));
    res %= 1000000007;
    update(instructions[i]);
  }
  return res;

  function update(x){
    for(; x < 100001; x += x & -x)
      c[x]++;
  }

  function get(x){
    let res = 0;

    for(;x >0; x-= x & -x)
      res += c[x];

    return res;
  }
};
