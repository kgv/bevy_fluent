#[doc(inline)]
pub use self::resource::Resource;

#[cfg(not(feature = "implicit"))]
#[doc(inline)]
pub use self::bundle::Bundle;

#[cfg(not(feature = "implicit"))]
pub mod bundle;
pub mod resource;
