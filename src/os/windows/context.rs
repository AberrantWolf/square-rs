use super::Window;

use super::util::to_wstring;

use winapi::shared::minwindef::{HINSTANCE};
use winapi::um::winnt::{LPCWSTR};
use winapi::um::winuser::{WNDCLASSEXW, GetClassInfoExW};
use winapi::um::libloaderapi::GetModuleHandleW;

use std::cell::Cell;

pub(crate) struct Context {
    instance: HINSTANCE,
    windclass: WNDCLASSEXW,
}

impl Context {
    pub(crate) fn new() -> Self {
        const WND_CLASS_NAME: Vec<u16> = to_wstring("baseWindowClass");

        let hinst = GetModuleHandleW(0 as LPCWSTR);
        let wndclass: WNDCLASSEXW;

        unsafe {
            if !GetClassInfoExW(hinst, WND_CLASS_NAME.as_ptr(), wndclass.as_pointer()) {
                // make the window class
            }
        }

        // Windows apps don't have menu items natively, so that section is not carried over from
        // the macos implementation.

        Self {
            hinst,
            wndclass,
        }
    }

    pub(crate) fn add_window(&self, window: Window) {
        unsafe {
//            msg_send![window.0, makeKeyAndOrderFront: nil];
        }
    }

    pub(crate) fn run(&self) {
//        let app = unsafe { NSApp() };
//
//        unsafe {
//            // If we are running outside of a bundle (e.g.. `cargo run`) then
//            // force activate our application
//            let policy: NSApplicationActivationPolicy = msg_send![app, activationPolicy];
//            if policy == NSApplicationActivationPolicyProhibited {
//                app.setActivationPolicy_(NSApplicationActivationPolicyRegular);
//
//                NSRunningApplication::currentApplication(nil)
//                    .activateWithOptions_(NSApplicationActivateIgnoringOtherApps);
//            }
//
//            // Activate the application (unhighlights the dock icon).
//            msg_send![app, finishLaunching];
//        }
//
//        if let Some(pool) = self.pool.take() {
//            unsafe {
//                // Release the setup NSAutoReleasePool.
//                msg_send![pool, release];
//            }
//        }
//
//        unsafe {
//            msg_send![app, run];
//        }
    }

    pub(crate) fn execute_on_main_thread(&self, callback: impl FnOnce() -> () + Send) {
//        let is_main_thread: bool = unsafe { msg_send![class("NSThread"), isMainThread] };
//
//        if is_main_thread {
//            callback();
//        } else {
//            dispatch::Queue::main().sync(callback);
//        }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        // TODO: uninitialize anything from Windows API that needs to be cleaned up...

//        if let Some(pool) = self.pool.take() {
//            unsafe {
//                msg_send![pool, release];
//            }
//        }
    }
}

//lazy_static! {
//    static ref DELEGATE_CLASS: &'static Class = declare_delegate();
//}
//
//fn declare_delegate() -> &'static Class {
//    let delegate_super_cls = Class::get("NSObject").unwrap();
//    let mut delegate_cls_decl = ClassDecl::new("SQAppDelegate", delegate_super_cls).unwrap();
//
//    unsafe {
//        delegate_cls_decl.add_method(
//            sel!(applicationShouldTerminateAfterLastWindowClosed:),
//            should_terminate_after_last_window_closed
//                as extern "C" fn(_: &Object, _: Sel, _: id) -> BOOL,
//        );
//
//        delegate_cls_decl.add_method(
//            sel!(applicationShouldTerminate:),
//            should_terminate as extern "C" fn(_: &Object, _: Sel, _: id) -> i32,
//        );
//    }
//
//    delegate_cls_decl.register()
//}
//
//extern "C" fn should_terminate_after_last_window_closed(_: &Object, _: Sel, _: id) -> BOOL {
//    YES
//}
//
//// Translate :terminate with :stop
//// This allows us to cleanly exit the runloop on the rust side
//extern "C" fn should_terminate(_: &Object, _: Sel, _: id) -> i32 {
//    unsafe {
//        let app: id = msg_send![class("NSApplication"), sharedApplication];
//        msg_send![app, stop: nil];
//    }
//
//    // NSTerminateCancel
//    0
//}
