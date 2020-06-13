var isHappy = function(n) {
    var ns = {};
    var num = n;
    var o = num;
    while (!ns[num]) {
      var sum = 0;
      o = num;
      while (num > 0) {
        var a = num % 10;
        num = Math.floor(num / 10);
        sum += a**2;
      }
      if (sum === 1) {
          return true;
      }
      ns[o] = sum;
      num = sum;
    }
    return false;
};
