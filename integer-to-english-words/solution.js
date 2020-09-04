/**
 * @param {number} num
 * @return {string}
 */

var terms = {
  1: "One",
  2: "Two",
  3: "Three",
  4: "Four",
  5: "Five",
  6: "Six",
  7: "Seven",
  8: "Eight",
  9: "Nine",
  10: "Ten",
  11: "Eleven",
  12: "Twelve",
  13: "Thirteen",
  14: "Fourteen",
  15: "Fifteen",
  16: "Sixteen",
  17: "Seventeen",
  18: "Eighteen",
  19: "Nineteen",
  20: "Twenty",
  30: "Thirty",
  40: "Forty",
  50: "Fifty",
  60: "Sixty",
  70: "Seventy",
  80: "Eighty",
  90: "Ninety",
  100: "Hundred",
};
var keta = {
  1: "Thousand",
  2: "Million",
  3: "Billion",
  4: "Trillion",
};

var numberToWords = function(num) {
  
  var a = Math.floor(num / 1000);
  var b = num % 1000;
  var result = [];
  var count = 0;
  while(a > 0 || b > 0) {
    var res = []
    if (count > 0 && b != 0) {
      result.unshift(keta[count]);
    }
    var hundred = Math.floor(b / 100);
    if (hundred) {
      res.push(terms[hundred]);
      res.push(terms[100]);
    }
    var ten = Math.floor(b%100/10)*10;
    var one = Math.floor(b%10);
    if (ten === 10) {
      let t = ten + one;
      res.push(terms[t]);
    } else if (ten >= 20) {
      res.push(terms[ten]);
      res.push(terms[one]);
    } else {
      res.push(terms[one]);
    }
    b = a % 1000;
    a = Math.floor(a / 1000);
    count++;
    
    result.unshift(...res);
  }
  if (result.length === 0) {
    return "Zero";
  }
  return result.filter(Boolean).join(" ");
};
