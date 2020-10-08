/**
 * @param {string[]} logs
 * @return {string[]}
 */
var reorderLogFiles = function(logs) {
  const letters = [];
  const digits = [];
  for (const log of logs) {
    const ls = log.split(" ");
    const head = ls.splice(0, 1);
    if (ls.every((n) => n == +n)) {
      digits.push(log);
    } else {
      letters.push(log);
    }
  }
  
  letters.sort((a, b) => {
    const as = a.split(" ");
    const bs = b.split(" ");
    const ah = as.splice(0, 1)[0];
    const bh = bs.splice(0, 1)[0];
    const at = as.join(" ");
    const bt = bs.join(" ");
    const compare = at.localeCompare(bt);
    return compare === 0 ? ah.localeCompare(bh) : compare;
  });
  return [...letters, ...digits];
};
