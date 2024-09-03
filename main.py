import time
from typing import List


class Solution:
    def merge(self, nums1: List[int], m: int, nums2: List[int], n: int) -> None:
        """
        Do not return anything, modify nums1 in-place instead.
        """
        nums1 = nums1[:m]
        index = 0

        for element in nums2:
            while element > nums1[index]:
                index += 1

            nums1.insert(index, element)

    def removeElement(self, nums: List[int], val: int) -> int:
        count = 0
        for i in range(0, len(nums)):
            if nums[i] == val:
                nums.pop(i)
                nums.append(0)
            else:
                count += 1

        return count

    def remove_element(self, nums: List[int], val: int) -> int:
        count = 0
        for i in range(0, len(nums)):
            if nums[i] == val:
                nums[i] = 0
                count += 1
            else:
                nums[i-count] = nums[i]

        return len(nums) - count

# nums = [0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55]


def fibonacci(n):
    if n == 1:
        return 0

    a, b = 0, 1
    for i in range(1, n):
        a = a + b
        a, b = b, a

    return a


def recursive_fibonacci(n):
    if n == 1:
        return 0
    if n == 2:
        return 1

    return recursive_fibonacci(n-1) + recursive_fibonacci(n-2)


def factorial(n):
    res = 1
    for i in range(1, n+1):
        res *= i

    return res


def recursive_factorial(n):
    if n == 0:
        return 1

    return n * recursive_fibonacci(n-1)

unsorted_list = [8, 2, 5, 3, 4, 1]


def bubble_sort(nums: list):
    n = len(nums)
    for i in range(n):
        swapped = False
        for j in range(n-i-1):
            if nums[j] > nums[j+1]:
                nums[j], nums[j+1] = nums[j+1], nums[j]
                swapped = True

        if not swapped:
            break

        print(f'After loop {i}: {nums}')

    return nums


def bubble_sort2(nums: list):
    n = len(nums)

    for i in range(n):
        for j in range(n-1):
            if nums[j] > nums[j+1]:
                nums[j], nums[j+1] = nums[j+1], nums[j]

    return nums

# bubble_sort(unsorted_list)
# print(f"Bubble sort = {bubble_sort2(unsorted_list)}")

def merge_sort(nums: list):
    if len(nums) <= 1:
        return nums

    mid = len(nums) // 2
    left_half = nums[:mid]
    right_half = nums[mid:]

    sorted_left = merge_sort(left_half)
    sorted_right = merge_sort(right_half)

    return merge(sorted_left, sorted_right)


def merge(left: list, right: list):
    result = []
    i = j = 0

    while i < len(left) and j < len(right):
        if left[i] < right[j]:
            result.append(left[i])
            i += 1
        else:
            result.append(right[j])
            j += 1

    result.extend(left[i:])
    result.extend(right[j:])

    return result

# print(f"Merge sorted list {merge_sort(unsorted_list)}")

nums = [1, 2, 3, 4]


def get_permutations(nums: list):
    permutations = []

    def backtrack(left: list, right: list):
        if len(right) == 1:
            permutations.append(left + right)
            return

        for i in range(len(right)):
            split = right[:i] + right[i+1:]
            backtrack(left + [right[i]], split)

    backtrack([], nums)
    # print(f"Permutations = {permutations}")


def get_permutations2(lst):
    # Base case: If the list is empty or has one element, return it as the only permutation
    if len(lst) == 0:
        return []
    elif len(lst) == 1:
        return [lst]

    # List to store all permutations
    permutations = []

    # Iterate over the list and recursively generate permutations
    for i in range(len(lst)):
        # Remove the current element and find permutations of the remaining list
        current = lst[i]
        remaining = lst[:i] + lst[i+1:]

        # Recursively get permutations of the remaining list
        for perm in get_permutations2(remaining):
            # Add the current element back to the beginning of each permutation
            permutations.append([current] + perm)

    return permutations

# start_time = time.time()
# get_permutations2(nums)
# end_time = time.time()
# print(f"Started at {start_time}. Ended at {end_time}. Took {end_time-start_time}")


def min_changes_to_palindrome(string):
    n = len(string)
    changes = 0

    for i in range(n // 2):
        if string[i] != string[n - i - 1]:
            changes += 1

    return changes

# print(f"Minimum changes to make it a palindrome = {min_changes_to_palindrome('abcdc')}")


def selection_sort(nums: list):
    n = len(nums)
    for i in range(n-1):
        min_index = i

        for j in range(i+1, n):
            if nums[j] < nums[min_index]:
                min_index = j

        nums[i], nums[min_index] = nums[min_index], nums[i]

    return nums

# print(f"Selection sort = {selection_sort(unsorted_list)}")


def insertion_sort(nums: list):
    n = len(nums)

    for i in range(1,n):
        insert_index = i
        current_value = nums[i]

        for j in range(i-1, -1, -1):
            if nums[j] > current_value:
                nums[j + 1] = nums[j]
                insert_index = j
            else:
                break

        nums[insert_index] = current_value

    return nums

# print(f"Insertion sort = {insertion_sort(unsorted_list)}")


def step_perms(n):
    if n == 0:
        return 1
    elif n == 1:
        return 1
    elif n == 2:
        return 2
    else:
        return step_perms(n-3) + step_perms(n-2) + step_perms(n-1)

print(f"Steps = {step_perms(5)}")

