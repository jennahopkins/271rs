type Heap<T> = Vec<T>;

fn heapify<T>(mut h: Heap<T>, i: usize, gt: fn(&T, &T) -> bool) -> Heap<T> {
    // fix from parent downwards
    let n = h.len(); // number of nodes
    let mut k = i; // index of the root node of the subtree

    let mut keep_going = true;
    while keep_going {
        keep_going = false;

        // children: left = 2*i, right = 2*i+1
        // zero-index children: l = 2*k+1, r = 2*k+2
        let l = 2 * k + 1;
        let r = 2 * k + 2;

        let mut largest = k;

        if l < n && gt(&h[l], &h[largest]) {
            largest = l;
        }
        if r < n && gt(&h[r], &h[largest]) {
            largest = r;
        }

        if largest != k {
            h.swap(k, largest);
            k = largest;
            keep_going = true;
        }
    }

    h
}

fn reheapify<T>(mut h: Heap<T>, i: usize, gt: fn(&T, &T) -> bool) -> Heap<T> {
    // insert and swap with parent
    let mut k = i; // index of the newly added node

    // parent: p = i/2
    // zero-index parent: p = (k - 1)/2
    while k > 0 {
        let p = (k - 1) / 2;
        if gt(&h[k], &h[p]) {
            h.swap(k, p);
            k = p;
        } else {
            break;
        }
    }

    h
}

fn vec_to_heap<T>(xs: Vec<T>, gt: fn(&T, &T) -> bool) -> Heap<T> {
    // build heap from vector bottom-up
    let mut h = xs;
    let n = h.len(); // number of nodes the heap will have

    if n > 1 {
        // last parent = n/2
        // zero-index last parent = (n/2)-1
        for i in (0..=(n / 2)).rev() {
            h = heapify(h, i, gt);
        }
    }

    h
}

fn heap_to_vec<T>(mut h: Heap<T>, gt: fn(&T, &T) -> bool) -> Vec<T> {
    // remove max element from heap one by one into vector to make sorted vector
    let mut out = Vec::with_capacity(h.len());

    while !h.is_empty() {
        let last = h.len() - 1;
        h.swap(0, last);
        out.push(h.pop().unwrap());

        if !h.is_empty() {
            h = heapify(h, 0, gt);
        }
    }

    out
}

fn hsort<T>(xs: Vec<T>, gt: fn(&T, &T) -> bool) -> Vec<T> {
    heap_to_vec(vec_to_heap(xs, gt), gt)
}

fn main() {
    let xs: Vec<u64> = vec![2, 4, 6, 8, 5, 3, 7];

    fn f(x: &u64, y: &u64) -> bool {
        x > y
    }

    dbg!(&xs);
    let sorted = hsort(xs, f);
    dbg!(&sorted);
}
