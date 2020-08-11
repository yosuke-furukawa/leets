/**
 * @param {string} secret
 * @param {string} guess
 * @return {string}
 */
var getHint = function(secret, guess) {
  var a = 0;
  var b = 0;
  var temp = [];
  var guesstemp = [...guess];
  for (var i=0;i<secret.length;i++) {
    if (guess[i] === secret[i]) {
      a++;
      guesstemp.splice(i, 1, -1);
    } else {
      temp.push(secret[i]);      
    }
  }
  guesstemp = guesstemp.filter((a) => a !== -1);
  for (var i=0;i<guesstemp.length;i++) {
    if (temp.includes(guesstemp[i])) {
      temp.splice(temp.indexOf(guesstemp[i]), 1);
      b++;
    }
  }
  
  return `${a}A${b}B`;
};
