use super::signature::WFSignature;

/// Require ability to evaluate a wavefunction at points in domain
pub trait Wavefunction<S: WFSignature> {
    /// Evaluate the wavefunction at a point in space and time
    fn f(&self, x: S::Space, t: S::Time) -> S::Out;
    /// Evaluate the probability density at a point in space and time
    fn p(&self, x: S::Space, t: S::Time) -> S::Out;
    /// Return the wavefunction with a translation applied in space.
    fn translate_space(self, offset: S::Space) -> Self;
    /// Return the wavefunction with a translation applied in space.
    fn translate_time(self, offset: S::Time) -> Self;
}