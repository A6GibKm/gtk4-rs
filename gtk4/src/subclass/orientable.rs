// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{Orientable, Orientation};
use glib::subclass::prelude::*;
use glib::translate::*;
use glib::Cast;

pub trait OrientableImpl: ObjectImpl {
    fn get_orientation(&self, orientable: &Self::Type) -> Orientation {
        unsafe {
            let type_ = ffi::gtk_orientable_get_type();
            let iface = glib::gobject_ffi::g_type_default_interface_ref(type_)
                as *mut ffi::GtkOrientableIface;
            assert!(!iface.is_null());

            let orientation = ffi::gtk_orientable_get_orientation(
                orientable.unsafe_cast_ref::<Orientable>().to_glib_none().0,
            );

            glib::gobject_ffi::g_type_default_interface_unref(iface as glib::ffi::gpointer);

            from_glib(orientation)
        }
    }

    fn set_orientation(&self, orientable: &Self::Type, orientation: Orientation) {
        unsafe {
            let type_ = ffi::gtk_orientable_get_type();
            let iface = glib::gobject_ffi::g_type_default_interface_ref(type_)
                as *mut ffi::GtkOrientableIface;
            assert!(!iface.is_null());

            ffi::gtk_orientable_set_orientation(
                orientable.unsafe_cast_ref::<Orientable>().to_glib_none().0,
                orientation.to_glib(),
            );

            glib::gobject_ffi::g_type_default_interface_unref(iface as glib::ffi::gpointer);
        }
    }
}
