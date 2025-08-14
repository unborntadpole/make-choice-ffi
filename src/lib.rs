use std::ffi::{CStr, CString};
use rand::Rng;
use std::os::raw::{c_char, c_ulonglong};


#[repr(C)]
pub struct ResultData {
    expectation: c_ulonglong,
    n_turns: c_ulonglong,
    choice: *mut c_char,
}

//  array pointer must not be empty and there must be exactly len_choices number of elements in choices array otherwise the call will fail
#[unsafe(no_mangle)]
pub unsafe extern "C" fn make_choice(
    limit: c_ulonglong,
    choices_arr: *const *const c_char,
    len_choices: c_ulonglong
) -> *mut ResultData {
    if choices_arr.is_null() || len_choices <=0 {
        return Box::into_raw(
            Box::new(ResultData{
                expectation: 0,
                n_turns: 0,
                choice: CString::new("Please enter valid data").unwrap().into_raw()
            })
        )
    }
    let len_choices = len_choices as usize;
    let slice = unsafe { std::slice::from_raw_parts(choices_arr, len_choices) };
    let choices: Vec<String> = slice
        .iter()
        .filter_map(|&p| {
            if p.is_null(){
                None
            } else {
                Some( unsafe { CStr::from_ptr(p).to_string_lossy().into_owned() })
            }
        })
        .collect();
    
    let (expectation, counter, choice) = run(choices, limit as u64);
    let choice = CString::new(choice).unwrap().into_raw();
    let result = Box::new(ResultData{
        expectation: expectation as c_ulonglong,
        n_turns: counter as c_ulonglong,
        choice: choice
    });
    Box::into_raw(result)
}

#[unsafe(no_mangle)]
pub extern "C" fn free_result_data(data: *mut ResultData) {
    if data.is_null(){
        return;
    }
    let boxed = unsafe { Box::from_raw(data) };
    if !boxed.choice.is_null() {
        let _ = unsafe { CString::from_raw(boxed.choice) };
    }
}


fn run(choices: Vec<String>, limit: u64) -> (u64, u64, String) {
    let mut rng = rand::rng();
    let mut last_choice: Option<&str> = None;
    let mut same_choice_counter = 0;
    let mut counter = 0;
    let num = choices.len();
    loop {
        let choice: usize = rng.random_range(0..num);
        let choice: &str = choices[choice].as_str();
        if let Some(s) = last_choice {
            if s == choice {
                same_choice_counter += 1;
            } else {
                same_choice_counter = 1;
                last_choice = Some(choice);
            }
        } else {
            same_choice_counter = 1;
            last_choice = Some(choice);
        }
        counter += 1;
        if same_choice_counter == limit {
            let num = num as u64;
            let expectation = (num.pow(limit as u32) - 1) / (num - 1);
            return (expectation, counter, choice.to_string());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_choices() {
        let choices = vec![String::from("Red"), String::from("Blue"), String::from("Green"), String::from("Violet"), String::from("Indigo"), String::from("Yellow"), String::from("Orange")];
        let limit = 4;

        println!("{:?}",run(choices, limit));
    }
}
