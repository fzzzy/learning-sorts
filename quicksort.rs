/*
 algo from wikipedia

algorithm quicksort(A, lo, hi) is
    if lo < hi then
        p := partition(A, lo, hi)
        quicksort(A, lo, p - 1 )
        quicksort(A, p + 1, hi)
*/

fn quicksort<T: Ord + Copy>(els: &mut [T], lo: isize, hi: isize) {
  if lo < hi {
    let p = partition(els, lo, hi);
    quicksort(els, lo, p - 1);
    quicksort(els, p + 1, hi);
  }
}

/*
algorithm partition(A, lo, hi) is
    pivot := A[hi]
    i := lo - 1    
    for j := lo to hi - 1 do
        if A[j] < pivot then
            i := i + 1
            swap A[i] with A[j]
    if A[hi] < A[i + 1] then
        swap A[i + 1] with A[hi]
    return i + 1
*/

fn partition<T: Ord + Copy>(els: &mut [T], lo: isize, hi: isize) -> isize {
  let pivot = els[hi as usize];
  let mut i = lo - 1;
  for j in lo..hi {
    if els[j as usize] < pivot {
      i += 1;
      els.swap(i as usize, j as usize);
    }
  }
  if els[hi as usize] < els[(i + 1) as usize] {
    els.swap((i + 1) as usize, hi as usize);
  }
  return i + 1;
}

fn main() {
  let mut tosort = [9, 8, 7, 6, 5, 4, 3, 2, 1];
  let sortlen = tosort.len() as isize;
  println!("Hello Quicksort! {:?}", tosort);
  quicksort(&mut tosort, 0, sortlen - 1);
  println!("Sorted! {:?}", tosort);
}