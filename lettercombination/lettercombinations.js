/**
 * @param {string} digits
 * @return {string[]}
 */

var phone = {
  "0": [],
  "1": [],
  "2": "abc".split(""),
  "3": "def".split(""),
  "4": "ghi".split(""),
  "5": "jkl".split(""),
  "6": "mno".split(""),
  "7": "pqrs".split(""),
  "8": "tuv".split(""),
  "9": "wxyz".split(""),
};

var letterCombinations = function(digits) {
  var dgs = digits.split("");
  var alphas = [];
  for (var i=0;i<dgs.length;i++) {
    var n = dgs[i];
    var a = phone[n];
    if (alphas.length === 0) {
      alphas.push(...a);
    } else {
      var newarray = [];
      for (var p=0;p<alphas.length;p++) {
        for (var q=0;q<a.length;q++) {
          newarray.push(alphas[p] + a[q]);
        }
      }
      alphas = newarray;
    }
  }
  return alphas;
};
