// Take a look at the license at the top of the repository in the LICENSE file.

use crate::ffi;
use glib::{bitflags::bitflags, prelude::*, translate::*};

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    #[doc(alias = "GdkPaintableFlags")]
    pub struct PaintableFlags: u32 {
        #[doc(alias = "GDK_PAINTABLE_STATIC_SIZE")]
        #[deprecated(since = "0.10.3", note = "Please use STATIC_SIZE instead")]
        const SIZE = ffi::GDK_PAINTABLE_STATIC_SIZE as _;
        #[doc(alias = "GDK_PAINTABLE_STATIC_CONTENTS")]
        #[deprecated(since = "0.10.3", note = "Please use STATIC_CONTENTS instead")]
        const CONTENTS = ffi::GDK_PAINTABLE_STATIC_CONTENTS as _;
        #[doc(alias = "GDK_PAINTABLE_STATIC_SIZE")]
        const STATIC_SIZE = ffi::GDK_PAINTABLE_STATIC_SIZE as _;
        #[doc(alias = "GDK_PAINTABLE_STATIC_CONTENTS")]
        const STATIC_CONTENTS = ffi::GDK_PAINTABLE_STATIC_CONTENTS as _;
    }
}

#[doc(hidden)]
impl IntoGlib for PaintableFlags {
    type GlibType = ffi::GdkPaintableFlags;

    #[inline]
    fn into_glib(self) -> ffi::GdkPaintableFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GdkPaintableFlags> for PaintableFlags {
    #[inline]
    unsafe fn from_glib(value: ffi::GdkPaintableFlags) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

impl StaticType for PaintableFlags {
    #[inline]
    #[doc(alias = "gdk_paintable_flags_get_type")]
    fn static_type() -> glib::Type {
        unsafe { from_glib(ffi::gdk_paintable_flags_get_type()) }
    }
}

impl glib::HasParamSpec for PaintableFlags {
    type ParamSpec = glib::ParamSpecFlags;
    type SetValue = Self;
    type BuilderFn = fn(&str) -> glib::ParamSpecFlagsBuilder<Self>;

    fn param_spec_builder() -> Self::BuilderFn {
        Self::ParamSpec::builder
    }
}

impl glib::value::ValueType for PaintableFlags {
    type Type = Self;
}

unsafe impl<'a> glib::value::FromValue<'a> for PaintableFlags {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for PaintableFlags {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl From<PaintableFlags> for glib::Value {
    #[inline]
    fn from(v: PaintableFlags) -> Self {
        skip_assert_initialized!();
        ToValue::to_value(&v)
    }
}
