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


/**
 * @param {string} s
 * @param {number[][]} shift
 * @return {string}
 */
var stringShift = function(s, shift) {
    var amount = 0;
    for (var i=0;i<shift.length;i++) {
        var [d, a] = shift[i];
        if (d === 0) {
            amount = amount - a;
        } else {
            amount = amount + a;
        }
    }
    amount %= s.length;
    if(amount < 0) {
        amount += s.length; 
    }
    return rotate(s.split(""), amount).join("");
};
