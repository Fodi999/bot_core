use std::ffi::{CStr, CString};
use crate::core::dialog::handle_input;

/// C-интерфейс, вызываемый из Swift (в iOS)
#[no_mangle]
pub extern "C" fn bot_handle_input(input_ptr: *const libc::c_char) -> *mut libc::c_char {
    // Безопасное чтение строки
    let c_str = unsafe { CStr::from_ptr(input_ptr) };
    let input = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };

    // Обработка вопроса через ядро
    let response = match handle_input(input.to_string()) {
        Ok(reply) => reply,
        Err(_) => "Ошибка обработки".to_string(),
    };

    // Возврат строки обратно в C
    CString::new(response).unwrap().into_raw()
}

/// Освобождение памяти (вызывать из Swift)
#[no_mangle]
pub extern "C" fn bot_free_response(ptr: *mut libc::c_char) {
    unsafe {
        if ptr.is_null() {
            return;
        }
        CString::from_raw(ptr);
    }
}
