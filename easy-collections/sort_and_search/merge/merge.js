var merge = function(nums1, m, nums2, n) {
    var length = m + n;
    m--;
    n--;
    while(length--) {
        if (n < 0 || nums1[m] > nums2[n]) {
            nums1[length] = nums1[m];
            m--;
        } else {
            nums1[length] = nums2[n];
            n--
        }
    }
};
