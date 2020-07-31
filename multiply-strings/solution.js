/**
 * @param {string} num1
 * @param {string} num2
 * @return {string}
 */
var multiply = function(num1, num2) {
  var result = "";
  var keta = "";
  if (num1 === "0" || num2 === "0") {
    return "0";
  }
  for (var i1=num1.length-1;i1>=0;i1--) {
    var n1 = +num1[i1];
    var temp = "";
    var digitMul = 0;
    for (var i2=num2.length-1;i2>=0;i2--) {
      var n2 = +num2[i2];
      var tmp = n2*n1 + digitMul;
      var ans = `${tmp%10}`;
      digitMul = (tmp/10)|0;
      temp = ans + temp;
    }
    if (digitMul != 0) {
      temp = `${digitMul}` + temp;
    }
    var digitSum = 0;
    var temp2 = "";
    temp += keta;
    for (var i3=temp.length-1,i4=result.length-1;i3>=0;i3--,i4--) {
      var n3 = +temp[i3];
      var r = result[i4] != null ? +result[i4] : 0;
      var tmp = r + n3 + digitSum;
      var ans = `${tmp % 10}`;
      digitSum = (tmp / 10)|0;
      temp2 = ans + temp2;
    }
    if (digitSum != 0) {
      temp2 = `${digitSum}` + temp2;
    }
    result = temp2;
    
    keta += "0";
  }
  return result;
};
