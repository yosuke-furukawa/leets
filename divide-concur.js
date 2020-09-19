var a = "1234567890";

function divide(a) {
  var result = [];
  for (var i=0;i<2**(a.length-1);i++) {
    var newA = a;
    var spl = [...Number(i).toString(2)];

    var revA = [...newA].reverse();
    var j = 0;
    while (spl.length > 0) {
      var sp = spl.pop();
      if (sp === "1") {
        revA[j] = revA[j+1] + revA[j];
        revA.splice(j+1,1);
        j--;
      }
      j++;
    }
    result.push(revA.reverse());
  }
  return result;
}

var r = divide(a);
console.log(r.length);
