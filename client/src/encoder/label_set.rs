use std::{
    borrow::Cow,
    collections::{BTreeMap, BTreeSet, LinkedList, VecDeque},
    fmt,
    rc::Rc,
    sync::Arc,
};

#[cfg(feature = "derive")]
pub use openmetrics_client_derive::{EncodeLabelSet, EncodeLabelValue};

/// Trait for encoding a set of labels.
pub trait LabelSetEncoder {
    /// Encodes a single label.
    fn encode(&mut self, label: &dyn EncodeLabel) -> fmt::Result;
}

/// Trait for types that can be encoded as a set of labels.
///
/// This trait is implemented by types that represent collections of labels, such as
/// vectors or maps of label pairs. It provides the ability to encode multiple labels
/// as a complete set.
///
/// # Example
///
/// ```rust
/// # use openmetrics_client::encoder::EncodeLabelSet;
/// // labels implements `EncodeLabelSet` and can be encoded as a label set
/// let labels = vec![("method", "GET"), ("status", "200")];
/// ```
pub trait EncodeLabelSet {
    /// Encodes this set of labels using the provided [`LabelSetEncoder`].
    ///
    /// This method should encode all labels in the set using the provided encoder,
    /// typically by obtaining individual label encoder for each label in the set.
    fn encode(&self, encoder: &mut dyn LabelSetEncoder) -> fmt::Result;

    /// Returns whether the label set is empty.
    ///
    /// Returns `true` if the label set contains no labels, and `false` otherwise.
    fn is_empty(&self) -> bool {
        false
    }
}

impl EncodeLabelSet for () {
    fn encode(&self, _encoder: &mut dyn LabelSetEncoder) -> fmt::Result {
        Ok(())
    }

    fn is_empty(&self) -> bool {
        true
    }
}

macro_rules! impl_encode_label_set_for_container {
    (<$($desc:tt)+) => (
        impl <$($desc)+ {
            #[inline]
            fn encode(&self, encoder: &mut dyn LabelSetEncoder) -> fmt::Result {
                for label in self.iter() {
                    encoder.encode(label)?;
                }
                Ok(())
            }

            #[inline]
            fn is_empty(&self) -> bool {
                self.len() == 0
            }
        }
    )
}

impl_encode_label_set_for_container! { <T: EncodeLabel> EncodeLabelSet for [T] }
impl_encode_label_set_for_container! { <T: EncodeLabel, const N: usize> EncodeLabelSet for [T; N] }
impl_encode_label_set_for_container! { <T: EncodeLabel> EncodeLabelSet for Vec<T> }
impl_encode_label_set_for_container! { <T: EncodeLabel> EncodeLabelSet for VecDeque<T> }
impl_encode_label_set_for_container! { <T: EncodeLabel> EncodeLabelSet for LinkedList<T> }
impl_encode_label_set_for_container! { <T: EncodeLabel> EncodeLabelSet for BTreeSet<T> }

impl<K: EncodeLabelName, V: EncodeLabelValue> EncodeLabelSet for BTreeMap<K, V> {
    fn encode(&self, encoder: &mut dyn LabelSetEncoder) -> fmt::Result {
        for label in self.iter() {
            encoder.encode(&label)?;
        }
        Ok(())
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}

macro_rules! impl_enable_label_set_for_deref {
    (<$($desc:tt)+) => (
        impl <$($desc)+ {
            #[inline]
            fn encode(&self, encoder: &mut dyn LabelSetEncoder) -> fmt::Result {
                (**self).encode(encoder)
            }

            #[inline]
            fn is_empty(&self) -> bool {
                (**self).is_empty()
            }
        }
    )
}

impl_enable_label_set_for_deref! { <'a, T> EncodeLabelSet for &'a T where T: ?Sized + EncodeLabelSet }
impl_enable_label_set_for_deref! { <'a, T> EncodeLabelSet for &'a mut T where T: ?Sized + EncodeLabelSet }
impl_enable_label_set_for_deref! { <'a, T> EncodeLabelSet for Cow<'a, T> where T: ?Sized + EncodeLabelSet + ToOwned }
impl_enable_label_set_for_deref! { <T> EncodeLabelSet for Box<T> where T: ?Sized + EncodeLabelSet }
impl_enable_label_set_for_deref! { <T> EncodeLabelSet for Rc<T> where T: ?Sized + EncodeLabelSet }
impl_enable_label_set_for_deref! { <T> EncodeLabelSet for Arc<T> where T: ?Sized + EncodeLabelSet }

////////////////////////////////////////////////////////////////////////////////

/// Trait for encoding an individual label.
pub trait LabelEncoder {
    /// Encodes a label name.
    fn encode_label_name(&mut self, name: &str) -> fmt::Result;

    /// Encodes a string as a label value.
    fn encode_str_value(&mut self, value: &str) -> fmt::Result;
    /// Encodes a boolean as a label value.
    fn encode_bool_value(&mut self, value: bool) -> fmt::Result;
    /// Encodes an 8-bit signed integer as a label value.
    fn encode_i8_value(&mut self, value: i8) -> fmt::Result;
    /// Encodes a 16-bit signed integer as a label value.
    fn encode_i16_value(&mut self, value: i16) -> fmt::Result;
    /// Encodes a 32-bit signed integer as a label value.
    fn encode_i32_value(&mut self, value: i32) -> fmt::Result;
    /// Encodes a 64-bit signed integer as a label value.
    fn encode_i64_value(&mut self, value: i64) -> fmt::Result;
    /// Encodes a 128-bit signed integer as a label value.
    fn encode_i128_value(&mut self, value: i128) -> fmt::Result;
    /// Encodes a platform-specific signed integer as a label value.
    fn encode_isize_value(&mut self, value: isize) -> fmt::Result;
    /// Encodes an 8-bit unsigned integer as a label value.
    fn encode_u8_value(&mut self, value: u8) -> fmt::Result;
    /// Encodes a 16-bit unsigned integer as a label value.
    fn encode_u16_value(&mut self, value: u16) -> fmt::Result;
    /// Encodes a 32-bit unsigned integer as a label value.
    fn encode_u32_value(&mut self, value: u32) -> fmt::Result;
    /// Encodes a 64-bit unsigned integer as a label value.
    fn encode_u64_value(&mut self, value: u64) -> fmt::Result;
    /// Encodes a 128-bit unsigned integer as a label value.
    fn encode_u128_value(&mut self, value: u128) -> fmt::Result;
    /// Encodes a platform-specific unsigned integer as a label value.
    fn encode_usize_value(&mut self, value: usize) -> fmt::Result;
    /// Encodes a 32-bit floating point as a label value.
    fn encode_f32_value(&mut self, value: f32) -> fmt::Result;
    /// Encodes a 64-bit floating point as a label value.
    fn encode_f64_value(&mut self, value: f64) -> fmt::Result;

    /// Encodes a `Some` type that implements [`EncodeLabelValue`] as a label value.
    fn encode_some_value(&mut self, value: &dyn EncodeLabelValue) -> fmt::Result;
    /// Encodes a `None` type as a label value.
    fn encode_none_value(&mut self) -> fmt::Result;
}

/// Trait for types that represent complete labels (name-value pairs).
///
/// This trait is implemented by types that can be encoded as complete labels,
/// typically containing both a name and value component.
///
/// # Example
///
/// ```rust
/// # use openmetrics_client::encoder::EncodeLabel;
/// let label = ("method", "GET"); // implements `EncodeLabel`
/// ```
pub trait EncodeLabel {
    /// Encodes this label using the provided [`LabelEncoder`].
    ///
    /// This should encode both the name and value components of the label.
    fn encode(&self, encoder: &mut dyn LabelEncoder) -> fmt::Result;
}

impl<N, V> EncodeLabel for (N, V)
where
    N: EncodeLabelName,
    V: EncodeLabelValue,
{
    fn encode(&self, encoder: &mut dyn LabelEncoder) -> fmt::Result {
        let (name, value) = self;
        name.encode(encoder)?;
        value.encode(encoder)?;
        Ok(())
    }
}

////////////////////////////////////////////////////////////////////////////////

/// Trait for types that can be encoded as label names.
///
/// # Example
///
/// ```rust
/// # use openmetrics_client::encoder::EncodeLabelName;
/// let name: &str = "name";                   // str implements `EncodeLabelName`
/// let name: String = String::from("name");   // String implement `EncodeLabelName`
/// ```
pub trait EncodeLabelName {
    /// Encodes this type as a label name using the provided [`LabelEncoder`].
    fn encode(&self, encoder: &mut dyn LabelEncoder) -> fmt::Result;
}

impl EncodeLabelName for str {
    fn encode(&self, encoder: &mut dyn LabelEncoder) -> fmt::Result {
        encoder.encode_label_name(self)
    }
}

impl EncodeLabelName for String {
    fn encode(&self, encoder: &mut dyn LabelEncoder) -> fmt::Result {
        encoder.encode_label_name(self)
    }
}

macro_rules! impl_encode_label_name_for_deref {
    (<$($desc:tt)+) => (
        impl <$($desc)+ {
            #[inline]
            fn encode(&self, encoder: &mut dyn LabelEncoder) -> fmt::Result {
                (**self).encode(encoder)
            }
        }
    )
}

impl_encode_label_name_for_deref! { <'a, T> EncodeLabelName for &'a T where T: ?Sized + EncodeLabelName }
impl_encode_label_name_for_deref! { <'a, T> EncodeLabelName for &'a mut T where T: ?Sized + EncodeLabelName }
impl_encode_label_name_for_deref! { <'a, T> EncodeLabelName for Cow<'a, T> where T: ?Sized + EncodeLabelName + ToOwned }
impl_encode_label_name_for_deref! { <T> EncodeLabelName for Box<T> where T: ?Sized + EncodeLabelName }
impl_encode_label_name_for_deref! { <T> EncodeLabelName for Rc<T> where T: ?Sized + EncodeLabelName }
impl_encode_label_name_for_deref! { <T> EncodeLabelName for Arc<T> where T: ?Sized + EncodeLabelName }

////////////////////////////////////////////////////////////////////////////////

/// Trait for types that can be encoded as label values.
///
/// # Example
///
/// ```rust
/// # use openmetrics_client::encoder::EncodeLabelValue;
/// let value: &str = "200"; // str implements `EncodeLabelValue`
/// let value: i32 = 200;    // integers implement `EncodeLabelValue`
/// let value: bool = true;  // bool implements `EncodeLabelValue`
/// ```
pub trait EncodeLabelValue {
    /// Encodes this type as a label value using the provided [`LabelEncoder`].
    fn encode(&self, encoder: &mut dyn LabelEncoder) -> fmt::Result;
}

impl EncodeLabelValue for str {
    #[inline]
    fn encode(&self, encoder: &mut dyn LabelEncoder) -> fmt::Result {
        encoder.encode_str_value(self)
    }
}

impl EncodeLabelValue for String {
    #[inline]
    fn encode(&self, encoder: &mut dyn LabelEncoder) -> fmt::Result {
        encoder.encode_str_value(self)
    }
}

macro_rules! impl_encode_label_value_for {
    ($($ty:ty),*) => (
        paste::paste! { $(
            impl EncodeLabelValue for $ty {
                #[inline]
                fn encode(&self, encoder: &mut dyn LabelEncoder) -> fmt::Result {
                    encoder.[<encode_ $ty _value>](*self)
                }
            }
        )* }
    )
}

impl_encode_label_value_for! {
    bool,
    i8, i16, i32, i64, i128, isize,
    u8, u16, u32, u64, u128, usize,
    f32, f64
}

impl<T> EncodeLabelValue for Option<T>
where
    T: EncodeLabelValue,
{
    #[inline]
    fn encode(&self, encoder: &mut dyn LabelEncoder) -> fmt::Result {
        match self {
            Some(value) => encoder.encode_some_value(value),
            None => encoder.encode_none_value(),
        }
    }
}

macro_rules! impl_encode_label_value_for_deref {
    (<$($desc:tt)+) => (
        impl <$($desc)+ {
            #[inline]
            fn encode(&self, encoder: &mut dyn LabelEncoder) -> fmt::Result {
                (**self).encode(encoder)
            }
        }
    )
}

impl_encode_label_value_for_deref! { <'a, T> EncodeLabelValue for &'a T where T: ?Sized + EncodeLabelValue }
impl_encode_label_value_for_deref! { <'a, T> EncodeLabelValue for &'a mut T where T: ?Sized + EncodeLabelValue }
impl_encode_label_value_for_deref! { <'a, T> EncodeLabelValue for Cow<'a, T> where T: ?Sized + EncodeLabelValue + ToOwned }
impl_encode_label_value_for_deref! { <T> EncodeLabelValue for Box<T> where T: ?Sized + EncodeLabelValue }
impl_encode_label_value_for_deref! { <T> EncodeLabelValue for Rc<T> where T: ?Sized + EncodeLabelValue }
impl_encode_label_value_for_deref! { <T> EncodeLabelValue for Arc<T> where T: ?Sized + EncodeLabelValue }
