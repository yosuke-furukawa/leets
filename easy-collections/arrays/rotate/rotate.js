function gcd(a, b) {
  while(b > 0) {
    [a, b] = [b, a%b]
  }

  return a;
}

function diff(a, b, len) {
  let result = 0;
  for(result = a - b; result < 0; result += len) { }
  return result;
}

var rotate = function(nums, k) {
  const r = gcd(k, nums.length);
  for (let i=0; i<r; i++) {
    let p = i;
    let n = diff(i, k, nums.length);

    while(n !== i) {
      let temp = nums[n];
      nums[n] = nums[p];
      nums[p] = temp;
      let np = diff(p, k, nums.length);
      [p, n] = [n, np];
    }

  }
  return nums;
};

console.log(rotate([1,2,3,4,5,6,7], 3))
console.log(rotate([1,2,3,4,5,6], 3))
console.log(rotate([-1], 2))
console.log(rotate([1, 2], 3))
console.log(diff(0, 3, 6))
