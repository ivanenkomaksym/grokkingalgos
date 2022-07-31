# O(nlogn) best case, O(n^2) worst case when array is sorted
def quicksort(arr):
    if (len(arr) < 2):
        return arr
    else:
        pivot = arr[0]
        less = [i for i in arr[1:] if i <= pivot]
        greater = [i for i in arr[1:] if i > pivot]
        print(less, pivot, greater)
        return quicksort(less) + [pivot] + quicksort(greater)
    
print(quicksort([1, 2, 3, 4, 5, 6, 7, 8]))