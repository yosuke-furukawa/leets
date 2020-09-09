/**
 * @param {string[][]} accounts
 * @return {string[][]}
 */
var accountsMerge = function(accounts) {
  var map1 = new Map();
  var map2 = new Map();
  for (var i=0;i<accounts.length;i++) {
    var account = accounts[i];
    var name = account[0];
    var mails = account.slice(1);
    map2.set(i, mails);
    for (var j=0;j<mails.length;j++) {
      var mail = mails[j];
      if (map1.has(mail)) {
        map1.get(mail).push(i);
        continue;
      }
      map1.set(mail, [i]);
    }
  }
  
  // console.log(map1);
  // console.log(map2);
  var visited = new Set();
  var helper = (index, set) => {
    if (visited.has(index)) {
      return;
    }
    visited.add(index);
    var mails = map2.get(index);
    for (var mail of mails) {
      if (!set.has(mail)) {
        set.add(mail);
        var keys = map1.get(mail);
        for (var k of keys) {
          helper(k, set);
        }
      }
    }
    return set;
  }; 
  var result = [];
  for (var i=0;i<accounts.length;i++) {
    var s = helper(i, new Set());
    if (s == null) {
      continue;
    }
    result.push([accounts[i][0], ...Array.from(s).sort()])
  }
  return result;
};
