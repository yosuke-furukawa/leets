function isSquare(n) {
  var sq = Math.floor(Math.sqrt(n));
  return n == sq * sq;
}

var numSquares = function(n) {
  // four-square and three-square theorems.
  while (n % 4 == 0)
    n /= 4;
  
  if (n % 8 == 7)
    return 4;

  if (isSquare(n))
    return 1;
  
  for (var i = 1; i * i <= n; ++i) {
    if (isSquare(n - i * i))
      return 2;
  }
  return 3;
};
