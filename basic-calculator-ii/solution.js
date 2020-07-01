/**
 * @param {string} s
 * @return {number}
 */
var calculate = function(s) {
  const stack = [];
  let num = '';
  let sign = null;

  for (var i = 0; i <= s.length; i++){
    const curr = s[i];
    if(curr === ' ') continue;
    if(!isNaN(curr)){
      num += curr;
    } else {
      switch(sign){
        case null:
        case '+':
          stack.push(Number(num))
          break;
        case '-':
          stack.push(-Number(num))
          break;
        case '*':
          stack.push(stack.pop()*Number(num))
          break;
        case '/':
          stack.push(stack.pop()/Number(num)|0)
          break;
        default:
          break;
      }
      sign = curr;
      num = ''
    }
  }
  var sum = stack.reduce((acc, num)=>{
    return acc+num
  })
  return sum
};
