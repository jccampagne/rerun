// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/store/re_types/definitions/rerun/blueprint/components/visible_time_range.fbs".

#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::cloned_instead_of_copied)]
#![allow(clippy::map_flatten)]
#![allow(clippy::needless_question_mark)]
#![allow(clippy::new_without_default)]
#![allow(clippy::redundant_closure)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::too_many_lines)]

use ::re_types_core::try_serialize_field;
use ::re_types_core::SerializationResult;
use ::re_types_core::{ComponentBatch as _, SerializedComponentBatch};
use ::re_types_core::{ComponentDescriptor, ComponentType};
use ::re_types_core::{DeserializationError, DeserializationResult};

/// **Component**: The range of values on a given timeline that will be included in a view's query.
///
/// Refer to `VisibleTimeRanges` archetype for more information.
///
/// ⚠️ **This type is _unstable_ and may change significantly in a way that the data won't be backwards compatible.**
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct VisibleTimeRange(pub crate::datatypes::VisibleTimeRange);

impl ::re_types_core::Component for VisibleTimeRange {
    #[inline]
    fn name() -> ComponentType {
        "rerun.blueprint.components.VisibleTimeRange".into()
    }
}

::re_types_core::macros::impl_into_cow!(VisibleTimeRange);

impl ::re_types_core::Loggable for VisibleTimeRange {
    #[inline]
    fn arrow_datatype() -> arrow::datatypes::DataType {
        crate::datatypes::VisibleTimeRange::arrow_datatype()
    }

    fn to_arrow_opt<'a>(
        data: impl IntoIterator<Item = Option<impl Into<::std::borrow::Cow<'a, Self>>>>,
    ) -> SerializationResult<arrow::array::ArrayRef>
    where
        Self: Clone + 'a,
    {
        crate::datatypes::VisibleTimeRange::to_arrow_opt(data.into_iter().map(|datum| {
            datum.map(|datum| match datum.into() {
                ::std::borrow::Cow::Borrowed(datum) => ::std::borrow::Cow::Borrowed(&datum.0),
                ::std::borrow::Cow::Owned(datum) => ::std::borrow::Cow::Owned(datum.0),
            })
        }))
    }

    fn from_arrow_opt(
        arrow_data: &dyn arrow::array::Array,
    ) -> DeserializationResult<Vec<Option<Self>>>
    where
        Self: Sized,
    {
        crate::datatypes::VisibleTimeRange::from_arrow_opt(arrow_data)
            .map(|v| v.into_iter().map(|v| v.map(Self)).collect())
    }
}

impl<T: Into<crate::datatypes::VisibleTimeRange>> From<T> for VisibleTimeRange {
    fn from(v: T) -> Self {
        Self(v.into())
    }
}

impl std::borrow::Borrow<crate::datatypes::VisibleTimeRange> for VisibleTimeRange {
    #[inline]
    fn borrow(&self) -> &crate::datatypes::VisibleTimeRange {
        &self.0
    }
}

impl std::ops::Deref for VisibleTimeRange {
    type Target = crate::datatypes::VisibleTimeRange;

    #[inline]
    fn deref(&self) -> &crate::datatypes::VisibleTimeRange {
        &self.0
    }
}

impl std::ops::DerefMut for VisibleTimeRange {
    #[inline]
    fn deref_mut(&mut self) -> &mut crate::datatypes::VisibleTimeRange {
        &mut self.0
    }
}

impl ::re_byte_size::SizeBytes for VisibleTimeRange {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        self.0.heap_size_bytes()
    }

    #[inline]
    fn is_pod() -> bool {
        <crate::datatypes::VisibleTimeRange>::is_pod()
    }
}
