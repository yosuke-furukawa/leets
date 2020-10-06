/**
 * @param {number} num
 * @return {string}
 */

var intToRoman = function(num) {
  var result = "";
  
  const roman10 = (divisor, alpha1, alpha2) => {
    var ms = num / divisor;
    if (ms >= 4) {
      result += alpha1
      num -= 4 * divisor;
    } else if (ms >= 1) {
      for (var i=0;i<parseInt(ms);i++) {
        result += alpha2;
      }
      num -= parseInt(ms) * divisor;
    }
  };
  const roman5 = (divisor, alpha1, alpha2) => {
    var ds = num / divisor;
    var n = divisor / 5;
    if (ds >= 1.8) {
      result += alpha1;
      num -= 9 * n;
    } else if (ds >= 1) {
      result += alpha2;
      num -= 5 * n;
    }
  };
  roman10(1000, null, "M");
  roman5(500, "CM", "D");
  roman10(100, "CD", "C");
  roman5(50, "XC", "L");
  roman10(10, "XL", "X");
  roman5(5, "IX", "V");
  roman10(1, "IV", "I");
  return result;
};
