/**
 * @param {string} IP
 * @return {string}
 */
var validIPAddress = function(IP) {
  var ipv4s = IP.split(".");
  var ipv6s = IP.split(":");
  if (ipv4s.length === 4) {    
    for (var i=0;i<4;i++) {
      var part = ipv4s[i];
      if (/^0[0-9]+$/.test(part)) {
        return "Neither";
      }
      var npart = Number.parseInt(part);
      if (npart != part) {
        return "Neither";
      }
      if (npart > 255 || npart < 0) {
        return "Neither";
      }
    }
    return "IPv4";
  }
  
  if (ipv6s.length === 8) {
    for (var i=0;i<8;i++) {
      var part = ipv6s[i];
      if (part.length > 4) {
        return "Neither";
      }

      if (!/^[a-fA-F0-9]{1,4}$/.test(part)) {
        return "Neither";
      }

      var npart = Number.parseInt(part, 16);
      var rpart = part.replace(/^0+([a-fA-F0-9]+)$/, "$1");
      if (npart.toString(16).toUpperCase() !== rpart.toUpperCase()) {
        return "Neither";
      }
    }
    return "IPv6";
  }
  
  return "Neither";
};
