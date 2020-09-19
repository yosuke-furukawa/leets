/**
 * @param {string} num
 * @param {number} target
 * @return {string[]}
 */
var addOperators = function(num, target) {
  var result = [];
  var backtrack = (index, prevOp, currentOp, value, ops) => {
    if (index === num.length) {
      if (value === target && currentOp === 0) {
        var sb = "";
        
        ops.slice(1, ops.length).forEach(v => sb += v);
        result.push(sb);
      }
      return;
    }
    
    currentOp = currentOp * 10 + Number(num.charAt(index));
    var currentVal = currentOp;
    
    var len = num.length;
    
    if (currentOp > 0) {
      backtrack(index+1, prevOp, currentOp, value, ops);
    }
    
    ops.push("+");
    ops.push(currentVal);
    backtrack(index+1, currentOp, 0, value + currentOp, ops);
    ops.pop();
    ops.pop();
    
    if (ops.length > 0) {
      ops.push("-");
      ops.push(currentVal);
      backtrack(index+1, -currentOp, 0, value - currentOp, ops);
      ops.pop();
      ops.pop();
      
      ops.push("*");
      ops.push(currentVal);
      backtrack(index+1, currentOp*prevOp, 0, value - prevOp + (currentOp * prevOp), ops);
      ops.pop();
      ops.pop();
    }
  };
  
  if (num.length === 0) {
    return [];
  }
  backtrack(0,0,0,0,[]);
  
  return result;
};
