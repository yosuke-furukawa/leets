function findLongestPalindrome(text) { 
  var N = text.length; 
  if (N == 0) return; 
  N = 2 * N + 1; // Position count 
  var L = new Array(N + 1); // LPS Length Array 
  L[0] = 0; 
  L[1] = 1; 
  var C = 1; // centerPosition 
  var R = 2; // centerRightPosition 
  var i = 0; // currentRightPosition 
  var iMirror; // currentLeftPosition 
  var maxLPSLength = 0; 
  var maxLPSCenterPosition = 0; 
  var start = -1; 
  var end = -1; 
  var diff = -1; 

  for (i = 2; i < N; i++) { 
    iMirror = 2 * C - i; 
    L[i] = 0; 
    diff = R - i; 
    if (diff > 0) {
      L[i] = Math.min(L[iMirror], diff); 
    }
    while (((i + L[i]) + 1 < N && (i - L[i]) > 0) &&  
      (((i + L[i] + 1) % 2 == 0) || (text.charCodeAt((i + L[i] + 1) / 2) == text.charCodeAt((i - L[i] - 1) / 2)))) { 
      L[i]++; 
    } 

    if (L[i] > maxLPSLength) { 
      maxLPSLength = L[i]; 
      maxLPSCenterPosition = i; 
    } 

    if (i + L[i] > R)  { 
      C = i; 
      R = i + L[i]; 
    } 
  } 

  start = (maxLPSCenterPosition - maxLPSLength) / 2; 
  end = start + maxLPSLength - 1; 
  console.log(L);
} 

var a = findLongestPalindrome("babcbabcbaccba");
console.log(a);
a = findLongestPalindrome("aab");
console.log(a);
