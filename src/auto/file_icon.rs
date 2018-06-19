// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use File;
use Icon;
use LoadableIcon;
use ffi;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct FileIcon(Object<ffi::GFileIcon, ffi::GFileIconClass>): Icon, LoadableIcon;

    match fn {
        get_type => || ffi::g_file_icon_get_type(),
    }
}

impl FileIcon {
    pub fn new<P: IsA<File>>(file: &P) -> FileIcon {
        unsafe {
            from_glib_full(ffi::g_file_icon_new(file.to_glib_none().0))
        }
    }
}

pub trait FileIconExt {
    fn get_file(&self) -> Option<File>;

    fn connect_property_file_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<FileIcon> + IsA<glib::object::Object>> FileIconExt for O {
    fn get_file(&self) -> Option<File> {
        unsafe {
            from_glib_none(ffi::g_file_icon_get_file(self.to_glib_none().0))
        }
    }

    fn connect_property_file_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::file",
                transmute(notify_file_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_file_trampoline<P>(this: *mut ffi::GFileIcon, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<FileIcon> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&FileIcon::from_glib_borrow(this).downcast_unchecked())
}
