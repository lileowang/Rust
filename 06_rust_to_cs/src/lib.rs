#[no_mangle]
pub extern "C" fn add_numbers(a: u32, b: u32) -> u32 {
    println!("hello from rust");
    a + b
}

#[repr(C)]
pub struct SampleStruct {
    pub field_one: i16,
    pub field_two: i32,
}

#[no_mangle]
pub extern "C" fn get_sample_struct() -> SampleStruct {
    SampleStruct {
        field_one: 1,
        field_two: 2,
    }
}
