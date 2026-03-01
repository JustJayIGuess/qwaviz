use std::{
    ops::{Add, Neg, Sub},
    sync::Arc,
};

use super::super::{core::field::Field, wavefunction::signature::WFSignature};

/// A valid wavefunction with signature `S`
pub type WFFunc<S> = dyn Fn(<S as WFSignature>::Space, <S as WFSignature>::Time) -> <S as WFSignature>::Out
    + Send
    + Sync;

/// Operations that can be done on the wavefunctions underlying bras (covectors) and kets (vectors)
#[derive(Clone)]
pub struct WFOperation<S: WFSignature>(WFOperationInner<S>);

#[derive(Clone)]
enum WFOperationInner<S: WFSignature> {
    /// A constant in the function space (i.e., a function from (Space x Time) --> Out)
    Function(Arc<WFFunc<S>>),
    /// Sum n wavefunctions pointwise
    Sum(Arc<Vec<WFOperation<S>>>),
    /// Sum n wavefunctions pointwise with weights
    WeightedSum(Arc<Vec<(S::Out, WFOperation<S>)>>),
    /// Subtract two wavefunctions pointwise
    Sub(Arc<WFOperation<S>>, Arc<WFOperation<S>>),
    /// Scale a wavefunction by a scalar
    Scale(S::Out, Arc<WFOperation<S>>),
    /// Negate a wavefunction pointwise
    Neg(Arc<WFOperation<S>>),
    /// Take the adjoint of a wavefunction (conjugate pointwise)
    Adjoint(Arc<WFOperation<S>>),
    /// Translate the wave function in space
    TranslateSpace(S::Space, Arc<WFOperation<S>>),
    /// Translate the wave function in time
    TranslateTime(S::Time, Arc<WFOperation<S>>),
}

impl<S: WFSignature> WFOperation<S> {
    /// A constant in the function space (i.e., a function from (Space x Time) --> Out)
    pub fn func(f: Arc<WFFunc<S>>) -> Self {
        Self(WFOperationInner::Function(f))
    }

    /// Sum n wavefunctions pointwise
    #[must_use]
    pub fn sum(summands: Vec<Self>) -> Self {
        Self(WFOperationInner::Sum(Arc::new(summands)))
    }

    /// Sum n wavefunctions pointwise with weights
    #[must_use]
    pub fn weighted_sum(summands: Vec<(S::Out, Self)>) -> Self {
        Self(WFOperationInner::WeightedSum(Arc::new(summands)))
    }

    /// Scale a wavefunction by a scalar
    #[must_use]
    pub fn scale(s: S::Out, op: Self) -> Self {
        Self(WFOperationInner::Scale(s, Arc::new(op)))
    }

    /// Take the adjoin of a wavefunction (conjugate pointwise)
    pub fn adjoint(f: Self) -> Self {
        Self(WFOperationInner::Adjoint(Arc::new(f)))
    }

    /// Translate a wavefunction in space
    pub fn translate_space(offset: S::Space, op: Self) -> Self {
        Self(WFOperationInner::TranslateSpace(offset, Arc::new(op)))
    }

    /// Translate a wavefunction in time
    pub fn translate_time(offset: S::Time, op: Self) -> Self {
        Self(WFOperationInner::TranslateTime(offset, Arc::new(op)))
    }
}

impl<S: WFSignature> Add for WFOperation<S> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::sum(vec![self, rhs])
    }
}

impl<S: WFSignature> Sub for WFOperation<S> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(WFOperationInner::Sub(Arc::new(self), Arc::new(rhs)))
    }
}

impl<S: WFSignature> Neg for WFOperation<S> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(WFOperationInner::Neg(Arc::new(self)))
    }
}

impl<S: WFSignature> WFOperation<S> {
    pub(super) fn eval(&self, x: S::Space, t: S::Time) -> S::Out {
        match &self.0 {
            WFOperationInner::Function(f) => f(x, t),
            WFOperationInner::Sum(fs) => fs
                .iter()
                .map(|f| f.eval(x, t))
                .fold(S::Out::zero(), |a, b| a + b),
            WFOperationInner::WeightedSum(summands) => summands
                .iter()
                .map(|(c, f)| *c * f.eval(x, t))
                .fold(S::Out::zero(), |a, b| a + b),
            WFOperationInner::Sub(f, g) => f.eval(x, t) - g.eval(x, t),
            WFOperationInner::Scale(c, f) => *c * f.eval(x, t),
            WFOperationInner::Neg(f) => -f.eval(x, t),
            WFOperationInner::Adjoint(f) => f.eval(x, t).conjugate(),
            WFOperationInner::TranslateSpace(dx, f) => f.eval(x - *dx, t),
            WFOperationInner::TranslateTime(dt, f) => f.eval(x, t - *dt),
        }
    }
}
