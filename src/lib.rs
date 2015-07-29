/*!
This crate provides a number of conversion traits with more specific semantics than those provided by `as` or `From`/`Into`.
*/

#![deny(missing_docs)]

pub use errors::{
    NoError, Underflow, Overflow,
    FloatError, RangeError,
    UnwrapOrInf, UnwrapOrInvalid,
};

pub mod errors;
pub mod misc;

mod impls;

/**
This trait is used to perform a conversion that is permitted to approximate the result.

All implementations of this trait must provide a conversion that can be separated into two logical steps: an approximation transform, and a representation transform.

The "approximation transform" step involves transforming the input value into an approximately equivalent value which is supported by the target type *without* taking the target type's representable range into account.  For example, this might involve rounding or truncating a floating point value to an integer, or reducing the accuracy of a floating point value.

The "representation transform" step *exactly* rewrites the value from the source type's binary representation into the destination type's binary representation.  This step *may not* transform the value in any way.  If the result of the approximation is not representable, the conversion *must* fail.

The major reason for this formulation is to exactly define what happens when converting between floating point and integer types.  Often, it is unclear what happens to floating point values beyond the range of the target integer type.  Do they saturate, wrap, or cause a failure?

With this formulation, it is well-defined: if a floating point value is outside the representable range, the conversion fails.  This allows users to distinguish between approximation and range violation, and act accordingly.
*/
pub trait ApproxFrom<Src, Scheme=DefaultApprox> where Scheme: ApproxScheme {
    /// The error type produced by a failed conversion.
    type Err;

    /// Convert the given value into an approximately equivalent representation.
    fn approx_from(src: Src) -> Result<Self, Self::Err>;
}

impl<Src, Scheme> ApproxFrom<Src, Scheme> for Src where Scheme: ApproxScheme {
    type Err = NoError;
    fn approx_from(src: Src) -> Result<Self, Self::Err> {
        Ok(src)
    }
}

/**
This is the dual of `ApproxFrom`; see that trait for information.
*/
pub trait ApproxInto<Dst, Scheme=DefaultApprox> where Scheme: ApproxScheme {
    /// The error type produced by a failed conversion.
    type Err;

    /// Convert the subject into an approximately equivalent representation.
    fn approx_into(self) -> Result<Dst, Self::Err>;
}

impl<Dst, Src, Scheme> ApproxInto<Dst, Scheme> for Src
where
    Dst: ApproxFrom<Src, Scheme>,
    Scheme: ApproxScheme,
{
    type Err = Dst::Err;
    fn approx_into(self) -> Result<Dst, Self::Err> {
        ApproxFrom::approx_from(self)
    }
}

/**
This trait is used to mark approximation scheme types.
*/
pub trait ApproxScheme {}

/**
The "default" approximation scheme.  This scheme does whatever would generally be expected of a lossy conversion, assuming no additional context or instruction is given.

This is a double-edged sword: it has the loosest semantics, but is far more likely to exist than more complicated approximation schemes.
*/
pub enum DefaultApprox {}
impl ApproxScheme for DefaultApprox {}

// TODO: RoundToNearest, RoundToPosInf, RoundToNegInf, RoundToZero

/**
This trait is used to perform an exact, value-preserving conversion.

Implementations of this trait should be reflexive, associative and commutative (in the absence of conversion errors).  That is, all possible cycles of `ValueFrom` conversions (for which each "step" has a defined implementation) should produce the same result, with a given value either being "round-tripped" exactly, or an error being produced.
*/
pub trait ValueFrom<Src> {
    /// The error type produced by a failed conversion.
    type Err;

    /// Convert the given value into an exactly equivalent representation.
    fn value_from(src: Src) -> Result<Self, Self::Err>;
}

impl<Src> ValueFrom<Src> for Src {
    type Err = NoError;
    fn value_from(src: Src) -> Result<Self, Self::Err> {
        Ok(src)
    }
}

/**
This is the dual of `ValueFrom`; see that trait for information.
*/
pub trait ValueInto<Dst> {
    /// The error type produced by a failed conversion.
    type Err;
    
    /// Convert the subject into an exactly equivalent representation.
    fn value_into(self) -> Result<Dst, Self::Err>;
}

impl<Src, Dst> ValueInto<Dst> for Src where Dst: ValueFrom<Src> {
    type Err = Dst::Err;
    fn value_into(self) -> Result<Dst, Self::Err> {
        ValueFrom::value_from(self)
    }
}