/**
 * @param {number[]} nums
 * @return {boolean}
 */
var judgePoint24 = function(nums) {
  return solve(nums);
};

function solve(nums) {
  if (nums.length === 0) return false;
  if (nums.length === 1) return Math.abs(nums[0] - 24) < 1e-6;
  
  for (var i=0;i<nums.length;i++) {
    for (var j=0;j<nums.length;j++) {
      if (i != j) {
        var nums2 = [];
        for (var k=0;k<nums.length;k++) {
          if (k!=i && k!=j) {
            nums2.push(nums[k]);
          }
        }
        
        for (var k=0;k<4;k++) {
          if (k < 2 && j > i) {
            continue;
          }
          if (k==0) nums2.push(nums[i] + nums[j]);
          if (k==1) nums2.push(nums[i] * nums[j]);
          if (k==2) nums2.push(nums[i] - nums[j]);
          if (k==3) nums2.push(nums[i] / nums[j]);
          if (solve(nums2)) return true;
          nums2.pop();
        }
      }
    }
  }
  return false;
}

function permutation({ result = [], pre = [], post, n = post.length }) {
  if (n > 0) {
    post.forEach((_, i) => {
      const rest = [...post];
      const elem = rest.splice(i, 1);
      permutation({ result, pre: [...pre, ...elem], post: rest, n: n - 1});
    });
  } else {
    result.push(pre);
  }
  return result;
};

function make24(a,b,c,d,ops) {
  for (var op1 of ops) {
    for (var op2 of ops) {
      for (var op3 of ops) {
        var ans = pattern1(a,b,c,d,op1,op2,op3);
        // console.log(ans, a,b,c,d,op1,op2,op3);
        if (round(ans,9) === 24) {
          // console.log(ans, a,b,c,d,op1,op2,op3);
          return true;
        }
        ans = pattern2(a,b,c,d,op1,op2,op3);
        // console.log(ans, a,b,c,d,op1,op2,op3);
        if (round(ans,9) === 24) {
          // console.log(ans, a,b,c,d,op1,op2,op3);
          return true;
        }
        ans = pattern3(a,b,c,d,op1,op2,op3);
        // console.log(ans, a,b,c,d,op1,op2,op3);
        if (round(ans,9) === 24) {
          // console.log(ans, a,b,c,d,op1,op2,op3);
          return true;
        }
        ans = pattern4(a,b,c,d,op1,op2,op3);
        // console.log(ans, a,b,c,d,op1,op2,op3);
        if (round(ans,9) === 24) {
          // console.log(ans, a,b,c,d,op1,op2,op3);
          return true;
        }
        ans = pattern5(a,b,c,d,op1,op2,op3);
        // console.log(ans, a,b,c,d,op1,op2,op3);
        if (round(ans,9) === 24) {
          // console.log(ans, a,b,c,d,op1,op2,op3);
          return true;
        }
      }
    }
  }
}

function round(ans, n) {
  return Math.round(ans* (10**n))/10**n;
}

// ((a op1 b) op2 c) op3 d
function pattern1(a,b,c,d,op1,op2,op3) {
  return calc(op3, calc(op2, calc(op1, a, b), c), d);
}

// (a op1 b) op2 (c op3 d)
function pattern2(a,b,c,d,op1,op2,op3) {
  return calc(op2, calc(op1, a, b), calc(op3, c, d));
}

// (a op1 (b op2 c)) op3 d
function pattern3(a,b,c,d,op1,op2,op3) {
  return calc(op3, calc(op1, calc(op2, b, c), a), d);
}

// a op1 ((b op2 c) op3 d)
function pattern4(a,b,c,d,op1,op2,op3) {
  return calc(op1, a, calc(op3, calc(op2, b, c), d));
}

// a op1 (b op2 (c op3 d))
function pattern5(a,b,c,d,op1,op2,op3) {
  return calc(op1, a, calc(op2, b, calc(op3, c, d)));
}

function calc(op, a, b) {
  switch(op) {
    case "+":
      return a + b;
    case "-":
      return a - b;
    case "*":
      return a * b;
    case "/":
      return a / b;
  }
}
