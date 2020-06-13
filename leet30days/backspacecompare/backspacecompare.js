var backspacestr = function(S) {
  var sr = "";
  for (var s=0;s<S.length;s++) {
    if (S[s]==="#") {
      sr = sr.substring(0, sr.length-1);
    } else {
      sr += S[s];
    }
  }
  return sr;
}

var backspaceCompare = function(S, T) {
  var sr = backspacestr(S);
  var tr = backspacestr(T);
  console.log({sr, tr});
  return sr === tr;
};
console.log(backspaceCompare("ab##", "c#d#"));
