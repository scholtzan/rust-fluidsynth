extern crate libc;
use libc::{c_int, c_char, c_double, c_void};
use ffi::*;
use std::ffi::{CString, CStr};
use std::mem::*;
use std::str;
use std::ptr;


#[repr(C)]
#[derive(PartialEq, Debug)]
pub enum SettingsType {
    NoType = -1,
    NumType, 
    IntType, 
    StrType,   
    SetType, 
}


pub struct Settings {
    c_fluid_settings: *mut fluid_settings_t
}

impl Settings {
    pub fn new() -> Settings {
        unsafe { 
            Settings {
                c_fluid_settings: new_fluid_settings() 
            }
        }
    }

    pub fn get_type(&self, name: &str) -> SettingsType {
        let name_str = CString::new(name).unwrap();
        unsafe {
            transmute(fluid_settings_get_type(self.to_raw(), name_str.as_ptr()))
        }
    }

    pub fn get_hints(&self, name: &str) -> i32 {
        let name_str = CString::new(name).unwrap();
        unsafe {
            fluid_settings_get_hints(self.to_raw(), name_str.as_ptr())
        }
    }

    pub fn is_realtime(&self, name: &str) -> bool {
        let name_str = CString::new(name).unwrap();
        unsafe {
            fluid_settings_is_realtime(self.to_raw(), name_str.as_ptr()) != 0
        }
    }

    pub fn setstr(&self, name: &str, string: &str) -> bool {
        let name_str = CString::new(name).unwrap();
        let string_str = CString::new(string).unwrap();
        unsafe {
            fluid_settings_setstr(self.c_fluid_settings, name_str.as_ptr(), string_str.as_ptr()) != 0
        }
    }

    // TODO
    /*pub fn copystr(&self, name: &str, mut string: &mut String, length: i32) -> bool {
        let name_str = CString::new(name).unwrap();
        let string_str = CString::new(string.clone()).unwrap();
        unsafe {
            let res = fluid_settings_copystr(self.c_fluid_settings, name_str.as_ptr(), string_str.as_ptr() as *mut c_char, length as c_int);
            *string = str::from_utf8(CStr::from_ptr(string_str).to_bytes()).unwrap().to_string();

            res != 0
        }
    }*/

    // TODO
    /*pub fn getstr(&self, name: &str) -> Option<String>{
        unsafe {
            let length = 100;
            let mut s = String::with_capacity(length);

            let res = self.copystr(name, &mut s, length as i32);

            match res {
                true => Some(s),
                _ => None
            }
        }
    }*/

    pub fn getstr_default(&self, name: &str) -> Option<String> {
        unsafe {
            let name_str = CString::new(name).unwrap();
            let res = fluid_settings_getstr_default(self.c_fluid_settings, name_str.as_ptr());

            if res.is_null() {
                None
            } else {
                Some(str::from_utf8(CStr::from_ptr(res).to_bytes()).unwrap().to_string())
            }
        }
    }

    pub fn getstr_equal(&self, name: &str, value: &str) -> bool {
        let name_str = CString::new(name).unwrap();
        let value_str = CString::new(value).unwrap();
        unsafe {
            fluid_settings_str_equal(self.to_raw(), name_str.as_ptr(), value_str.as_ptr()) != 0
        }
    }

    pub fn setnum(&self, name: &str, value: f64) -> bool {
        let name_str = CString::new(name).unwrap();
        unsafe {
            fluid_settings_setnum(self.c_fluid_settings, name_str.as_ptr(), value as c_double) != 0
        }
    }

    pub fn getnum(&self, name: &str) -> Option<f64> {
        let mut value: f64 = 0.0;
        let name_str = CString::new(name).unwrap();
        unsafe {
            let res = fluid_settings_getnum(self.to_raw(), name_str.as_ptr(), &mut value);

            match res {
                1 => Some(value),
                _ => None
            }
        }
    }

    pub fn getnum_default(&self, name: &str) -> Option<f64> {
        let name_str = CString::new(name).unwrap();
        unsafe {
            let res: f64 = fluid_settings_getnum_default(self.to_raw(), name_str.as_ptr());

            match res {
                0.0 => None,
                _ => Some(res)
            }
        }
    }

    pub fn getnum_range(&self, name: &str) -> Option<(f64, f64)> {
        unsafe {
            if SettingsType::NoType == self.get_type(name) {
                return None;
            }

            let name_str = CString::new(name).unwrap();
            let mut min: f64 = 0.0;
            let mut max: f64 = 0.0;

            fluid_settings_getnum_range(self.to_raw(), name_str.as_ptr(), &mut min, &mut max);

            Some((min, max))    
        }
    }

    pub fn setint(&self, name: &str, value: i32) -> bool {
        let name_str = CString::new(name).unwrap();
        unsafe {
            fluid_settings_setint(self.c_fluid_settings, name_str.as_ptr(), value as c_int) != 0
        }
    }

    pub fn getint(&self, name: &str) -> Option<i32> {
        let mut value: i32 = 0;
        let name_str = CString::new(name).unwrap();
        unsafe {
            let res = fluid_settings_getint(self.to_raw(), name_str.as_ptr(), &mut value);

            match res {
                1 => Some(value),
                _ => None
            }
        }
    }

    pub fn getint_default(&self, name: &str) -> Option<i32> {
        let name_str = CString::new(name).unwrap();
        unsafe {
            let res: i32 = fluid_settings_getint_default(self.to_raw(), name_str.as_ptr());

            match res {
                0 => None,
                _ => Some(res)
            }
        }
    }

    pub fn getint_range(&self, name: &str) -> Option<(i32, i32)> {
        unsafe {
            if SettingsType::NoType == self.get_type(name) {
                return None;
            }

            let name_str = CString::new(name).unwrap();
            let mut min: i32 = 0;
            let mut max: i32 = 0;

            fluid_settings_getint_range(self.to_raw(), name_str.as_ptr(), &mut min, &mut max);

            Some((min, max))    
        }
    }

    pub fn foreach_option<T: Fn(&str, &str)>(&self, name: &str, callback: T) {
        let user_data = &callback as *const _ as *mut c_void;
        let name_str = CString::new(name).unwrap();
        unsafe {
            fluid_settings_foreach_option(self.to_raw(), name_str.as_ptr(), user_data, foreach_option_callback_wrapper::<T>);  
        }

        extern fn foreach_option_callback_wrapper<T>(closure: *mut c_void, name: *const c_char, option: *const c_char)
            where T: Fn(&str, &str) {
            let closure = closure as *mut T;

            unsafe {
                let name_str = str::from_utf8(CStr::from_ptr(name).to_bytes()).unwrap();
                let option_str = str::from_utf8(CStr::from_ptr(option).to_bytes()).unwrap();

                (*closure)(name_str, option_str);
            }
        }
    }

    pub fn option_count(&self, name: &str) -> Option<(i32)> {
        let name_str = CString::new(name).unwrap();
        unsafe {
            let res = fluid_settings_option_count(self.to_raw(), name_str.as_ptr());

            match res {
                -1 => None,
                _ => Some(res)
            }   
        }
    }

    pub fn option_concat(&self, name: &str, separator: &str) -> Option<(&str)> {
        let name_str = CString::new(name).unwrap();
        let separator_str = CString::new(separator).unwrap();

        unsafe {
            let res = fluid_settings_option_concat(self.to_raw(), name_str.as_ptr(), separator_str.as_ptr());

            if res.is_null() {
                None
            } else {
                Some(str::from_utf8(CStr::from_ptr(res).to_bytes()).unwrap())
            }  
        }
    }

    pub fn foreach<T: Fn(&str, SettingsType)>(&self, callback: T) {
        unsafe {
            let user_data = &callback as *const _ as *mut c_void;

            fluid_settings_foreach(self.to_raw(), user_data, foreach_callback_wrapper::<T>);  
        }

        extern fn foreach_callback_wrapper<T>(closure: *mut c_void, name: *const c_char, setting_type: c_int)
            where T: Fn(&str, SettingsType) {
            let closure = closure as *mut T;

            unsafe {
                let name_str = str::from_utf8(CStr::from_ptr(name).to_bytes()).unwrap();
                let converted_type: SettingsType = transmute(setting_type);
                (*closure)(name_str, converted_type);
            }
        }
    }

    pub fn from_raw(settings: *mut fluid_settings_t) -> Settings {
        Settings {
            c_fluid_settings: settings
        }
    }

    pub fn to_raw(&self) -> *mut fluid_settings_t {
        self.c_fluid_settings
    }
}

impl Drop for Settings {
    fn drop(&mut self) -> () {
        unsafe {
            delete_fluid_settings(self.c_fluid_settings)
        }
    }
}



