/**
 * @param {number[]} nums
 * @return {boolean}
 */
var judgePoint24 = function(nums) {
  var pems = permutation({post:nums});
  var ops = ["+", "-", "*", "/"];
  for (var [a,b,c,d] of pems) {
    var result = make24(a,b,c,d,ops);
    if (result) {
      return result;
    }    
  }
  return false;
};

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
