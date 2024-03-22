use all_values_as_string::{AllFieldNamesAsString, AllValuesAsString};
use chrono::{NaiveDateTime, Utc};

#[derive(AllFieldNamesAsString, AllValuesAsString)]
pub struct Foo {
    #[NumericField]
    a: f64,
    b: String,
}

#[derive(AllFieldNamesAsString, AllValuesAsString)]
pub struct PageTableEntry {
    #[StructFieldAsString]
    f: Foo,
    #[BooleanField]
    valid: bool,
    #[BooleanField]
    read: bool,
    #[BooleanField]
    write: bool,
    #[BooleanField]
    executable: bool,
    #[BooleanField]
    user: bool,
    #[BooleanField]
    global: bool,
    #[BooleanField]
    accessed: bool,
    #[BooleanField]
    dirty: bool,
    #[NumericField]
    rsvd1: usize,
    #[NumericField]
    ppn: usize,
    #[NumericField]
    rsvd2: usize,
    time: NaiveDateTime
}

fn main() {
    let a= PageTableEntry{
        f: Foo {
            a: 1000.,
            b: "test".into(),
        },
        valid: false,
        read: false,
        write: false,
        executable: false,
        user: false,
        global: false,
        accessed: false,
        dirty: false,
        rsvd1: 0,
        ppn: 0,
        rsvd2: 0,
        time: Utc::now().naive_local(),
    };
    println!("{}", a.get_all_field_names_as_string());
    println!("{}", a.get_all_values_as_string());
}
