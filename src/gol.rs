use std::{cmp::{max, min},
          vec::{self, IntoIter}};

use rust_utils::prelude::Tap;

use {rand::random,
     rust_utils::*,
     std::ops::{self, Range}};

const SIDE_LENGTH: usize = 3;
#[derive(Debug, Default)]
pub struct GolWorld([[bool; SIDE_LENGTH]; SIDE_LENGTH]);
impl GolWorld {
  pub fn new() -> Self { Self::default() }
  pub fn random() -> Self {
    iproduct!(0..SIDE_LENGTH, 0..SIDE_LENGTH).fold(Self::new(), |w, (a, b)| {
                                               w.set(a, b, random::<bool>())
                                             })
  }
  pub fn get(&self, i: usize, j: usize) -> bool {
    matches!((i, j), (0..SIDE_LENGTH, 0..SIDE_LENGTH)) && self.0[i][j]
  }
  pub fn set(self, i: usize, j: usize, value: bool) -> Self {
    self.tap_mut(|s| s.0[i][j] = value)
  }
  pub fn nbcount(&self, i: usize, j: usize) -> usize {
    let around = |n: usize| max(n, 1) - 1..=min(n + 1, SIDE_LENGTH);
    iproduct!(around(i), around(j)).filter(|(a, b)| (*a, *b) != (i, j) && self.get(*a, *b))
                                   .count()
  }
  pub fn step(self) -> Self {
    // fold_combinations!(i 0..SIDE_LENGTH,
    //                    j 0..SIDE_LENGTH,
    //                    w self,
    //                    {let val = matches!((&w.get(i, j), n), (true, 2 | 3) | (false, 3));
    //                     w.set(i, j, val)})
    let coords = iproduct!(0..SIDE_LENGTH, 0..SIDE_LENGTH);
    let nbcounts = coords.map(|(i, j)| (i, j, self.nbcount(i, j)));
    fold(|w, (i, j, n)| {
           let val = matches!((w.get(i, j), n), (true, 2 | 3) | (false, 3));
           w.set(i, j, val)
         },
         self,
         nbcounts)
  }
}
// pub fn distinct_pairs<I: std::iter::Iterator, T: Clone, C: IntoIterator<Item = T, IntoIter = I>>(coll: C) {
//   // let coll = coll.into_iter();
//   iproduct!(coll, coll).filter(|(a, b)| a != b)
// }
pub fn test() {
  let w = GolWorld::random();
  println!("{:?}", w);
  println!("{:?}", w.step());
}
fn repeat<T: Clone>(thing: &T, n: usize) -> impl Iterator<Item = T> + '_ {
  (0..n).map(|_| thing.clone())
}
trait Nice: Copy + Clone {
  fn two(self) -> (Self, Self) { (self, self.clone()) }
}
impl<T: Copy + Clone> Nice for T {}
// enum List<const Length:usize,T>{
//   First(T),
//   Rest(List<Length - 1,T>),
// }
// type MyList = Cons<i32, Cons<i32, Cons<i32, Cons<i32, ()>>>>;
enum Cons<A, B> {
  Car(A),
  Cdr(B)
}
fn a() -> (i32, i32) { 4_i32.two() }
const _X: Cons<i32, Cons<i32, Cons<i32, Cons<i32, ()>>>> = Cons::Cdr(Cons::Cdr(Cons::Car(5)));
