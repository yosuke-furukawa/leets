const net = require("net");
/**
 * @param {string} IP
 * @return {string}
 */
var validIPAddress = function(IP) {
  if (net.isIPv4(IP)) {
    return "IPv4";
  }
  
  if (net.isIPv6(IP)) {
    var ipv6s = IP.split(":");
    // invalid case avoid empty string
    if (ipv6s.filter(Boolean).length < 8) {
      return "Neither";
    }

    return "IPv6";
  }
  
  
  return "Neither";
};
