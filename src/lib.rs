//! Implements conversions between color spaces which use 3-component vectors.
//!
//! It is intended for use in games or other interactive visual applications,
//! where it can help implement correct color management and wide color gamut
//! rendering.
//!
//! ## Named Color Spaces
//!
//! Named color space definitions can be found in the [`spaces`] module. Some
//! notable color spaces and models:
//!
//! * sRGB/linear sRGB/BT.709
//! * BT.2020
//! * ACEScg
//! * ACES2065-1
//! * Oklab
//! * CIE LAB/Lch/Luv/xyY/uvV
//! * HSL/HSV/HSI
//! * ICtCp
//!
//! You can also construct custom [`ColorSpace`]s from a combination of
//! primaries, whitepoint and transform function.
//!
//! ## Design
//!
//! `kolor` aims to supports all color spaces and color models which use
//! 3-component vectors, such as RGB, LAB, XYZ, HSL and more.
//!
//! In the spirit of keeping things simple, `kolor` uses a single type,
//! [`Color`], to represent a color in any supported color space.
//!
//! `kolor` can programmatically generate an efficient conversion between any
//! two color spaces by using the CIE XYZ color space as a "connecting space",
//! meaning all supported color spaces only need to support a conversion to a
//! reference color space that is a linear transform of the CIE XYZ color space.
//!
//! Transformations between linear color spaces can be implemented with a 3×3
//! matrix, and since 3×3 matrices are composable, these matrices can be
//! pre-composed and applied to a color with a single multiply.
//!
//! `kolor` recognizes that users may want to perform conversions on colors
//! stored in types defined by the user. [`ColorConversion`] represents a
//! conversion between two color spaces and is intended to be compatible with
//! 3-component vectors in many math libraries.
//!
//! `kolor` defines conversions from a source `ColorSpace` to a destination
//! `ColorSpace` as three parts:
//!
//! - if the source color space is non-linear, apply the inverse of its
//!   transform function to convert to the non-linear color space's reference
//!   color space (which is always linear).
//!
//! - a linear 3×3 transformation matrix from the source to the destination
//!   linear color space.
//!
//! - if the destination color space is a non-linear color space, apply its
//!   transform function
//!
//! A "non-linear transform function" means any function that cannot be
//! expressed as a linear transformation of the CIE XYZ color space. Examples
//! include the sRGB logarithmic gamma compensation function,
//! the Oklab transform function, and the HSL/HSV hexagonal/circular transform.
//!
//! For non-linear color spaces, many transform functions are supported
//! to convert between popular spaces, but for GPU contexts, these
//! implementations clearly can't be used directly. To implement data-driven
//! conversions, you can read the required operations for transforming between
//! spaces from a `ColorConversion` value and run these as appropriate.
//! Feel free to port the implementations in the
//! [`transform`](`details::transform`) module to your shaders or other code.
//!
//! ### Gamut-Agnostic Transforms
//!
//! Some color models like CIELAB or HSL are intended to provide an alternate
//! view of some linear color spaces, and need a reference color space to
//! provide information like which white point or RGB primaries to use. To
//! construct these color spaces, refer to associated methods on `ColorSpace`
//! and use an appropriate reference color space.
//!
//! ### Details
//!
//! `kolor` can calculate 3×3 conversion matrices between any linear color space
//! defined by RGB primaries and a white point. `kolor` offers APIs for
//! performing conversions directly, and for extracting the 3×3 matrix to use in
//! a different context, for example on a GPU.
//!
//! ### Generating Conversion Matrices Between RGB Color Spaces
//!
//! [`LinearColorConversion`][details::conversion::LinearColorConversion] can be
//! used to generate conversion matrices "offline",
//! in which case you probably want to use the `f64` feature for better
//! precision. The precision of the derived matrices won't be perfect, but
//! probably good enough for games.
//!
//! Conversions between all combinations of built-in primaries and whitepoints
//! color spaces are bundled with `kolor` as constants with the `color-matrices`
//! feature, which is enabled by default. When a `ColorConversion` without a
//! bundled pre-calculated conversion matrix is created, it is calculated
//! on-demand, meaning the creation will be a bit slower to create than if there
//! is a constant matrix available.
//!
//! ### Chromatic Adaptation Transformation (CAT)
//!
//! `kolor` implements CAT in the [`cat`](details::cat) module and supports the
//! LMS cone spaces defined in [`LmsConeSpace`][details::cat::LmsConeSpace].
//! Chromatic Adaptation Transformation means converting a linear RGB color
//! space from one reference [`WhitePoint`][details::color::WhitePoint] to
//! another.
//!
//! Use [`ColorSpace::with_whitepoint()`] to change the `WhitePoint` for a color
//! space.
//!
//! ### XYZ-RGB Conversions
//!
//! All supported RGB color spaces use the CIE XYZ color space as its reference
//! color space. Functions in the [`xyz`](details::xyz) module can be used to
//! create conversion matrices to/from an RGB color space given a set of
//! primaries and a white point.
//!
//! ## Features
//!
//! ### `no_std` & `glam` Support
//!
//! By default `kolor` uses `std` and `glam`, but both can be disabled
//! separately or together with the folowing features:
//!
//! | |`std`|`no_std`|
//! |-|-|-|
//! |`glam`|`std-glam`|`libm-glam`|
//! |no `glam`|`std`|`libm`|
//!
//! ### List of Features
#![doc = document_features::document_features!()]
#![cfg_attr(not(feature = "std"), no_std)]
#![allow(unexpected_cfgs)]

#[cfg(feature = "f64")]
pub type Float = f64;

#[cfg(not(feature = "f64"))]
pub type Float = f32;

pub use details::math::{Mat3, Vec3};

/// Create a `Mat3` from a `[Float; 9]`. The order of components is
/// column-major.
#[cfg(not(feature = "glam"))]
#[macro_export]
macro_rules! const_mat3 {
    ($ftypex9:expr) => {
        Mat3::from_cols_array_const($ftypex9)
    };
}

#[cfg(not(feature = "f64"))]
pub(crate) use core::f32::consts::{PI, TAU};
#[cfg(feature = "f64")]
pub(crate) use core::f64::consts::{PI, TAU};

pub mod details {
    pub mod cat;
    pub mod color;
    pub mod conversion;
    #[allow(clippy::excessive_precision)]
    #[cfg(feature = "color-matrices")]
    pub mod generated_matrices;
    pub mod math;
    #[allow(clippy::excessive_precision)]
    #[allow(clippy::many_single_char_names)]
    #[allow(non_snake_case)]
    pub mod transform;
    pub mod xyz;
}

pub use details::*;

#[doc(inline)]
pub use details::color::color_spaces as spaces;
#[doc(inline)]
pub use details::color::{Color, ColorSpace};
#[doc(inline)]
pub use details::conversion::ColorConversion;

pub mod prelude {
    pub use crate::details::*;
}
