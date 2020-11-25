
/**
 * Definition for a binary tree node.
 * function TreeNode(val, left, right) {
 *     this.val = (val===undefined ? 0 : val)
 *     this.left = (left===undefined ? null : left)
 *     this.right = (right===undefined ? null : right)
 * }
 */
/**
 * @param {TreeNode} root
 * @param {number} target
 * @param {number} k
 * @return {number[]}
 */
var closestKValues = function(root, target, k) {
  var nums = [];
  function inorder(node, nums) {
    if (node == null) {
      return;
    }
    inorder(node.left, nums);
    nums.push(node.val);
    inorder(node.right, nums);
  }

  function partition(left, right, pivotIndex) {
    var pivotDist = dist(pivotIndex);
    swap(pivotIndex, right);
    var storeIndex = left;

    for (var i=left;i<=right;i++) {
      if (dist(i) < pivotDist) {
        swap(storeIndex, i);
        storeIndex++;
      }
    }

    swap(storeIndex, right);

    return storeIndex;
  }

  function quickselect(left, right, kSmallest) {
    if (left >= right) {
      return;
    }

    var r = Math.random();
    var pivotIndex = Math.floor(left + (r % (right + 1 - left)));
    pivotIndex = partition(left, right, pivotIndex);

    if (kSmallest == pivotIndex) {
      return;
    } else if (kSmallest < pivotIndex) {
      quickselect(left, pivotIndex - 1, kSmallest);
    } else {
      quickselect(pivotIndex+1, right, kSmallest);
    }
  }

  function dist(idx) {
    return Math.abs(nums[idx] - target);
  }
  
  function swap(a, b) {
    var temp = nums[a];
    nums[a] = nums[b];
    nums[b] = temp;
  }

  inorder(root, nums);
  quickselect(0, nums.length-1, k);
  return nums.splice(0, k);
};


