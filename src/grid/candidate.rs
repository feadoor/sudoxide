use std::fmt;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};

use bitvec::prelude::*;
use itertools::Itertools;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Candidate<const N: usize>(pub usize);

#[derive(Clone, PartialEq, Eq)]
pub struct CandidateSet<const N: usize> {
    candidates: BitVec,
}

impl<const N: usize> CandidateSet<N> {
    
    pub fn full() -> Self {
        Self { candidates: bitvec![1; N] }
    }

    pub fn empty() -> Self {
        Self { candidates: bitvec![0; N] }
    }

    pub fn union<'a, I: IntoIterator<Item = &'a Self>>(candidate_sets: I) -> Self {
        let mut result = Self::empty();
        for candidate_set in candidate_sets { result |= candidate_set; }
        result
    }

    pub fn from_candidates<I: IntoIterator<Item = Candidate<N>>>(from_candidates: I) -> Self {
        let mut candidates = bitvec![0; N];
        for Candidate(value) in from_candidates { candidates.set(value - 1, true); }
        Self { candidates }
    }

    pub fn remove_value(&mut self, value: Candidate<N>) {
        self.candidates.set(value.0 - 1, false);
    }

    pub fn len(&self) -> usize {
        self.candidates.count_ones()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn intersects(&self, other: &CandidateSet<N>) -> bool {
        !(self.clone() & other).is_empty()
    }

    pub fn first(&self) -> Option<Candidate<N>> {
        self.candidates.first_one().map(|value| Candidate(value + 1))
    }

    pub fn into_iter(&self) -> impl Iterator<Item = Candidate<N>> {
        self.candidates.iter_ones().collect::<Vec<_>>().into_iter().map(|value| Candidate(value + 1))
    }

    pub fn iter(&self) -> impl Iterator<Item = Candidate<N>> + Clone + '_ {
        self.candidates.iter_ones().map(|value| Candidate(value + 1))
    }

    pub fn contains(&self, value: Candidate<N>) -> bool {
        self.candidates[value.0 - 1]
    }
}

impl<const N: usize> fmt::Display for CandidateSet<N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({})", self.iter().map(|x| x.0.to_string()).join(", "))
    }
}

impl<const N: usize> Not for CandidateSet<N> {
    type Output = CandidateSet<N>;

    fn not(self) -> Self::Output {
        CandidateSet { candidates: !self.candidates }
    }
}

impl<const N: usize> Not for &CandidateSet<N> {
    type Output = CandidateSet<N>;

    fn not(self) -> Self::Output {
        !self.clone()
    }
}

impl<const N: usize> BitOrAssign for CandidateSet<N> {

    fn bitor_assign(&mut self, rhs: Self) {
        self.candidates |= rhs.candidates;
    }
}

impl<const N: usize> BitOrAssign<&Self> for CandidateSet<N> {

    fn bitor_assign(&mut self, rhs: &Self) {
        self.candidates |= &rhs.candidates;
    }
}

impl<T: Copy, const N: usize> BitOrAssign<&T> for CandidateSet<N> where CandidateSet<N>: BitOrAssign<T> {
    
    fn bitor_assign(&mut self, rhs: &T) {
        self.bitor_assign(*rhs) 
    }
}

impl<const N: usize> BitAndAssign for CandidateSet<N> {

    fn bitand_assign(&mut self, rhs: Self) {
        self.candidates &= rhs.candidates;
    }
}

impl<const N: usize> BitAndAssign<&Self> for CandidateSet<N> {

    fn bitand_assign(&mut self, rhs: &Self) {
        self.candidates &= &rhs.candidates;
    }
}

impl<T: Copy, const N: usize> BitAndAssign<&T> for CandidateSet<N> where CandidateSet<N>: BitAndAssign<T> {
    
    fn bitand_assign(&mut self, rhs: &T) {
        self.bitand_assign(*rhs) 
    }
}

impl<const N: usize> BitXorAssign for CandidateSet<N> {

    fn bitxor_assign(&mut self, rhs: Self) {
        self.candidates ^= rhs.candidates;
    }
}

impl<const N: usize> BitXorAssign<&Self> for CandidateSet<N> {

    fn bitxor_assign(&mut self, rhs: &Self) {
        self.candidates ^= &rhs.candidates;
    }
}

impl<T: Copy, const N: usize> BitXorAssign<&T> for CandidateSet<N> where CandidateSet<N>: BitXorAssign<T> {
    
    fn bitxor_assign(&mut self, rhs: &T) {
        self.bitxor_assign(*rhs) 
    }
}

impl<const N: usize> BitAnd for CandidateSet<N> {
    type Output = Self;

    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs; self
    }
}

impl<const N: usize> BitAnd<&Self> for CandidateSet<N> {
    type Output = Self;

    fn bitand(mut self, rhs: &Self) -> Self::Output {
        self &= rhs; self
    }
}

impl<const N: usize> BitAnd<CandidateSet<N>> for &CandidateSet<N> {
    type Output = CandidateSet<N>;

    fn bitand(self, mut rhs: CandidateSet<N>) -> Self::Output {
        rhs &= self; rhs
    }
}

impl<const N: usize> BitAnd for &CandidateSet<N> {
    type Output = CandidateSet<N>;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.clone() & rhs
    }
}

impl<const N: usize> BitXor for CandidateSet<N> {
    type Output = Self;

    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs; self
    }
}

impl<const N: usize> BitXor<&Self> for CandidateSet<N> {
    type Output = Self;

    fn bitxor(mut self, rhs: &Self) -> Self::Output {
        self ^= rhs; self
    }
}

impl<const N: usize> BitXor<CandidateSet<N>> for &CandidateSet<N> {
    type Output = CandidateSet<N>;

    fn bitxor(self, mut rhs: CandidateSet<N>) -> Self::Output {
        rhs ^= self; rhs
    }
}

impl<const N: usize> BitXor for &CandidateSet<N> {
    type Output = CandidateSet<N>;

    fn bitxor(self, rhs: Self) -> Self::Output {
        self.clone() ^ rhs
    }
}

impl<const N: usize> BitOr for CandidateSet<N> {
    type Output = Self;

    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs; self
    }
}

impl<const N: usize> BitOr<&Self> for CandidateSet<N> {
    type Output = Self;

    fn bitor(mut self, rhs: &Self) -> Self::Output {
        self |= rhs; self
    }
}

impl<const N: usize> BitOr<CandidateSet<N>> for &CandidateSet<N> {
    type Output = CandidateSet<N>;

    fn bitor(self, mut rhs: CandidateSet<N>) -> Self::Output {
        rhs |= self; rhs
    }
}

impl<const N: usize> BitOr for &CandidateSet<N> {
    type Output = CandidateSet<N>;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.clone() | rhs
    }
}
