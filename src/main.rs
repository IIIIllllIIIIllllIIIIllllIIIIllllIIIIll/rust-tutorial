fn sum(slice: &[i32]) -> i32 {
    let mut sum = 0;
    for e in slice { sum += e };
    sum
}

#[test]
fn test_sum_small() {
    let array = [1,2,3,4,5];
    assert_eq!(sum(&array), 15);
}

fn dedup(vs: &Vec<i32>) -> Vec<i32> {
    let mut ret: Vec<i32> = Vec::new();
    for e in vs {
        if ret.contains(e) {
            continue;
        }
        ret.push(*e);
    }
    ret
}

#[test]
fn test_dedup_small() {
    let vs = vec![1,2,2,3,4,1];
    assert_eq!(dedup(&vs), vec![1,2,3,4]);
}

fn filter(vs: &Vec<i32>, pred: &dyn Fn(i32) -> bool) -> Vec<i32> {
    let mut ret: Vec<i32> = Vec::new();
    for e in vs {
        if pred(*e) {
            ret.push(*e);
        }
    }
    ret
}

fn even_predicate(x: i32) -> bool {
    (x % 2) == 0
}

#[test]
fn test_filter_small() {
    let vs = vec![1,2,3,4,5];
    assert_eq!(filter(&vs, &even_predicate), vec![2,4]);
}

type Matrix = Vec<Vec<f32>>;

fn dim(mat: &Matrix) -> (usize, usize) {
    (mat.len(), mat[0].len())
}

fn mat_mult(mat1: &Matrix, mat2: &Matrix) -> Matrix {
    let mut ret = Matrix::new();
    let (m1, n1) = dim(mat1);
    let (m2, n2) = dim(mat2);
    assert_eq!(n1, m2);
    for _ in 0..m1 {
        ret.push(vec![0.0; n2]);
    }
    for i in 0..m1 {
        for j in 0..n2 {
            for k in 0..n1 {
                ret[i][j] += mat1[i][k]*mat2[k][j];
            }
        }
    }
    ret
}

#[test]
fn test_mat_mult_identity() {
    let mut mat1 = vec![vec![0.;3]; 3];
    for i in 0..mat1.len() {
        mat1[i][i] = 1.;
    }
    let mat2 = vec![vec![5.;3]; 3];
    let result = mat_mult(&mat1, &mat2);
    for i in 0..result.len() {
        for j in 0..result[i].len() {
            assert_eq!(result[i][j], mat2[i][j]);
        }
    }
}

#[test]
fn test_splay() {
    let mat1 = vec![vec![1.,1.,1.]];
    let mat2 = vec![vec![1.], vec![1.], vec![1.]];
    let result = mat_mult(&mat1, &mat2);
    assert_eq!(dim(&result), (1, 1));
    assert_eq!(result[0][0], 3.);
    let result = mat_mult(&mat2, &mat1);
    assert_eq!(dim(&result), (3, 3));
    for i in 0..3 {
        for j in 0..3 {
            assert_eq!(result[i][j], 1.);
        }
    }
}

fn mark_multiples(arr: &mut Vec<u32>, p: u32) {
    let mut i = p as usize;
    loop {
        if i >= arr.len() { return; };
        arr[i] = 1;
        i += p as usize;
    }
}

fn sieve(n: u32) -> Vec<u32> {
    let mut deleted = vec![0 as u32;n as usize];
    let mut ret = vec![];
    for p in 2..n {
        if deleted[p as usize] == 0 {
            ret.push(p);
            mark_multiples(&mut deleted, p);
        } else {
        }
    }
    ret
}

#[test]
fn test_sieve_basic() {
    println!("res={:?}", sieve(12));
    assert_eq!(vec![2,3,5,7,11], sieve(12));
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Peg {
    A,
    B,
    C,
}

type Move = (Peg, Peg);

fn hanoi(num_discs: u32, src: Peg, aux: Peg, dst: Peg) -> Vec<Move> {
    if num_discs == 0 {
        return vec![];
    }
    let mut ret = hanoi(num_discs - 1, src, dst, aux);
    ret.push((src, dst));
    ret.extend(hanoi(num_discs - 1, aux, src, dst));
    ret
}

#[test]
fn test_hanoi_1_disks() {
    let result = hanoi(1, Peg::A, Peg::B, Peg::C);
    assert_eq!(vec![(Peg::A, Peg::C)], result);
    assert_eq!(1, result.len());
}
