struct MemTest {
    name: String,
    text: String,
}

impl MemTest {
    pub fn new(name: String, text: String) -> Self {
        MemTest {
            name: name,
            text: text,
        }
    }
    pub fn change_name(&self, name: String) -> Self {
        MemTest {
            name: name,
            text: self.text.to_string(),
        }
    }
}

fn main() {
    #[cfg(feature = "dhat-heap")]
    #[global_allocator]
    static ALLOC: dhat::Alloc = dhat::Alloc;
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    #[cfg(feature = "dhat-ad-hoc")]
    let _profiler = dhat::Profiler::new_ad_hoc();
    #[cfg(feature = "dhat-ad-hoc")]
    dhat::ad_hoc_event(100);
    println!("Hello, world!");
    let test_var = MemTest::new(String::from("test_title"), String::from("Test_text"));
    for _n in 1..50 {
        let _test_var_two = test_var.change_name(String::from("Test"));
    }
}
