/**
 * @param {string[]} emails
 * @return {number}
 */
var numUniqueEmails = function(emails) {
  var set = new Set();
  for (var email of emails) {
    var [local, domain] = email.split("@");
    local = local.replace(/\./g, "");
    local = local.replace(/\+.*$/g, "");
    set.add(`${local}@${domain}`);
  }
  return set.size;
};
