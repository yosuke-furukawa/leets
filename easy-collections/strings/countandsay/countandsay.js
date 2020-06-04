var countAndSay = function(n) {
  if (n===1) { 
    return "1";
  }
  var str = "1";
  for (var i = 1;i<n; i++) {
    var result = "";
    for (var s=0;s<str.length;s++) {
      var count = 1;
      while (str[s] === str[s+1]) {
        count++;
        s++;
      }
      result += `${count}${str[s]}`
    }
    str = result;
  }
  return str;
};
