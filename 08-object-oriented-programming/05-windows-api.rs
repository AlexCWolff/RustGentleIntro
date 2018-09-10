/* One of the areas where traditional OOP is used extensively is GUI toolkits. An 'EditControl' or a 'ListWindow' is-a 'Window', etc. This makes writing Rust bindings to GUI toolkits more difficult than it should be. Win32 programming can be done directly in Rust, and it's a little less awkward than the original C. A typical Win32 API function is 'ShowWindow' which is used to control the visibility of a window. An 'EditControl' has some specialized functionality, but it's all done with a Win32 'HWND' (handle to window) opaque value. We would like 'EditControl' to also have a 'show' method, which traditionally would be done by implementation inheritance. We don't want to have to type out all these inherited methods for each type, but Rust traits provide a solution. There would be a Window trait */

// A 'Window' trait
trait Window {
    // you need to define this!
    fn get_hwnd(&self) -> HWND;

    // and all these will be provided
    fn show(&self, visible: bool) {
        unsafe {
         user32_sys::ShowWindow(self.get_hwnd(), if visible {1} else {0})
        }
    }

    // methods operating on 'Windows'

}

/* The implementation struct for 'EditControl' can just contain a 'HWND' and implement 'Window' by defining one method; 'EditControl' is a trait that inherits from 'Window' and defines the extended interface. Something like 'ComboxBox', which behaves like an 'EditControl' and a 'ListWindow' can be easily implemented with trait inheritance. The Win32 API ('32' no longer means '32-bit' anymore) is object-oriented, but an older style influenced by Alan Kay's definition: objects contain hidden data and are operated on by messages. At the heart of any Windows application there's a message loop, and the various kinds of windows (called "window classes") implement these methods with their own switch statements. There is a message called 'WM_SETTEXT' but the implementation can be different; a label's text changes, but a top-level window's caption changes. The next edition of The Rust Programming Language book has a very good discussion on what 'object-oriented' means in Rust. */