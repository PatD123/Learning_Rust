struct Sorter {}

impl Sorter {
    fn basic_sort<T: std::cmp::PartialOrd> (nums: &mut [T]) {
        // O(N^2)

        let n = nums.len();
        for i in (0..n) {
            for j in (0..n - 1) {
                if nums[j] > nums[j + 1] {
                    nums.swap(j, j + 1);
                }
            }
        }
    }

    fn merge_sort<T: std::cmp::PartialOrd + std::marker::Copy> (nums: &mut [T], left: usize, right: usize) {
        if left >= right {
            return;
        }

        let mid = left + (right - left) / 2;
        Self::merge_sort(nums, left, mid);
        Self::merge_sort(nums, mid + 1, right);
        Self::merge(nums, left, mid, right);
    }

    fn merge<T: std::cmp::PartialOrd + std::marker::Copy> (nums: &mut [T], left: usize, mut mid: usize, right: usize) {
        let mut start1 = left;
        let mut start2 = mid + 1;

        while start1 <= mid && start2 <= right {
            if nums[start1] <= nums[start2] {
                start1 += 1;
            }
            else {
                let temp = nums[start2];
                for i in (start1..start2 - 1).rev() {
                    nums.swap(i, i + 1);
                }
                nums[start1] = temp;

                start1 += 1;
                mid += 1;
                start2 += 1;
            }
        }
    }

    fn heapify<T>(nums: &mut [T], n: usize, i: usize)
    where
        T: std::cmp::PartialOrd + std::marker::Copy,
    {
        let mut largest = i;
        let l = 2 * i + 1;
        let r = 2 * i + 2;

        if l < n && nums[l] > nums[largest]{
            largest = l;
        }

        if r < n && nums[r] > nums[largest]{
            largest = r;
        }

        if largest != i {
            nums.swap(i, largest);
            Self::heapify(nums, n, largest);
        }
    }

    fn heap_sort<T>(nums: &mut [T])
    where
        T: std::cmp::PartialOrd + std::marker::Copy + std::fmt::Debug
    {
        let n = nums.len();

        // Heapify
        for i in (0..n / 2 - 1).rev() {
            Self::heapify(nums, n, i);
        }

        for i in (1..n).rev() {
            nums.swap(0, i);
            Self::heapify(nums, i, 0);
        }
    }
}

fn main() {
    let mut vec_of_ints = vec![9, 8, 7, 6, 5, 4, 3, 2, 1];
    // Sorter::basic_sort(&mut vec_of_ints);

    let mut vec_of_chars = vec!['p', 'a', 't', 'r', 'i', 'c', 'k',];
    // Sorter::basic_sort(&mut vec_of_chars);

    // Sorter::merge_sort(&mut vec_of_ints, 0, 8);

    // Sorter::merge_sort(&mut vec_of_chars, 0, 6);

    Sorter::heap_sort(&mut vec_of_chars);

    println!("{:?}", vec_of_chars);
}
