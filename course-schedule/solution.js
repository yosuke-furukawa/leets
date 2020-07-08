var traverse = (course, set, map, root) => {
  //console.log(course, set, map);
  if (course == null) {
    set.clear();
    set.add(root);
    return true;
  }
  var requires = map.get(course);
  if (requires == null || requires.length == 0) {
    set.clear();
    set.add(root);
    return true;
  }
  var result = true;
  while (requires.length > 0) {
    var req = requires.shift();
    //console.log({req});
    if (set.has(req)) {
      return false;
    }
    set.add(req);
    result = result && traverse(req, set, map,root);
  }
  return result;
}

/**
 * @param {number} numCourses
 * @param {number[][]} prerequisites
 * @return {boolean}
 */
var canFinish = function(numCourses, prerequisites) {
  var map = new Map;
  for (var requires of prerequisites) {
    var course = requires[0];
    var require = requires[1];
    var courses = map.get(course);
    if (courses) {
      if (!courses.includes(require)) {
        courses.push(require);
        map.set(course, courses);
      }
    } else {
      map.set(course, [require]);
    }
  }
  for (var key of map.keys()) {
    var m = new Map(map);
    for (var k of m.keys()) {
      m.set(k, [...m.get(k)]);
    }
    var r = traverse(key, new Set().add(key), m, key);
    //console.log({r});
    if (!r) {
      return false;
    }
  }
  return true;
};
