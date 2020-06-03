var myAtoi = function(str) {
  var value = 0;
  var sign = +1;
  var i = 0;
  var max = (2**31) - 1;
  var min = (2**31) * -1;
  while(str[i]=== " ") {
    i++;
  }

  if (str[i] === "-") {
    sign = -1;
    i++;
  } else if (str[i] === "+") {
    i++;
  }

  var isNum = (str, i) => (str[i] !== " " && +str[i] === +str[i]);
  while (isNum(str, i)) {
    value = value * 10 + Number(str[i]);
    i++;
  }
  if (sign * value >= max) {
    return max;
  }
  if (sign * value <= min) {
    return min;
  }
  return sign * value;
};

console.log(myAtoi("42"));
console.log(myAtoi("   -42"));
console.log(myAtoi("4193 with words"));
console.log(myAtoi("words and 987"));
console.log(myAtoi("-91283472332"));
