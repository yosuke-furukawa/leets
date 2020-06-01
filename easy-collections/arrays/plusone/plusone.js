var plusOne = function(digits) {
  var r = BigInt(digits.join("")) + 1n;
  return (""+r).split("");
};

