
# algo from wikipedia
# algorithm quicksort(A, lo, hi) is
#     if lo < hi then
#         p := partition(A, lo, hi)
#         quicksort(A, lo, p - 1 )
#         quicksort(A, p + 1, hi)


def quicksort(els, lo, hi):
  if lo < hi:
    pivot = partition(els, lo, hi)
    quicksort(els, lo, pivot - 1)
    quicksort(els, pivot + 1, hi)


# algorithm partition(A, lo, hi) is
#     pivot := A[hi]
#     i := lo - 1    
#     for j := lo to hi - 1 do
#         if A[j] < pivot then
#             i := i + 1
#             swap A[i] with A[j]
#     if A[hi] < A[i + 1] then
#         swap A[i + 1] with A[hi]
#     return i + 1


def partition(els, lo, hi):
  pivot = els[hi]
  i = lo - 1
  for j in range(lo, hi):
    if els[j] < pivot:
      i += 1
      els[i], els[j] = els[j], els[i]
  if els[hi] < els[i + 1]:
    els[hi], els[i + 1] = els[i + 1], els[hi]
  return i + 1


tosort = [9, 8, 7, 6, 5, 4, 3, 2, 1]


print("Hello quicksort!", tosort)
quicksort(tosort, 0, len(tosort) - 1)
print("Sorted", tosort)


