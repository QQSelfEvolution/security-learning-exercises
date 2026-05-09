// 小暗 Day1练习 #1: Rust Unsafe基础
// Rust语言 - Unsafe代码块
fn main() {
    println!("=== 小暗 Rust学习 Day1 - Unsafe基础 ===");
    
    // 1. 解引用裸指针
    println!("\n--- 1. 解引用裸指针 ---");
    
    let mut num = 42;
    
    // 创建裸指针（不可变）
    let raw_ptr = &num as *const i32;
    println!("裸指针地址: {:?}", raw_ptr);
    
    // 创建裸指针（可变）
    let raw_mut_ptr = &mut num as *mut i32;
    unsafe {
        println!("通过裸指针读取: {}", *raw_ptr);
        println!("修改前的值: {}", *raw_mut_ptr);
        *raw_mut_ptr = 100;
        println!("修改后的值: {}", *raw_mut_ptr);
    }
    println!("原始变量现在也是: {}", num);
    
    // 2. 调用unsafe函数
    println!("\n--- 2. 调用unsafe函数 ---");
    
    unsafe {
        dangerous_function();
    }
    
    // 3. 实现unsafe trait
    println!("\n--- 3. 实现unsafe trait ---");
    
    let vec = unsafe { HolyTable::new(10) };
    println!("HolyTable长度: {}", vec.len());
    println!("HolyTable容量: {}", vec.capacity());
    
    // 4. 访问可变静态变量
    println!("\n--- 4. 访问可变静态变量 ---");
    
    println!("计数器初始值: {}", COUNTER);
    unsafe {
        COUNTER += 10;
    }
    println!("计数器加10后: {}", COUNTER);
    
    // 5. 联合体操作
    println!("\n--- 5. 联合体操作 ---");
    
    let mut u = MyUnion { i: 0 };
    unsafe {
        u.f = 3.14;
        println!("联合体存储的float: {}", u.f);
        println!("联合体存储的int: {}", u.i);
    }
    
    println!("\n=== Day1练习1完成 ===");
}

// Unsafe函数 - 可能未定义行为的函数
unsafe fn dangerous_function() {
    println!("这是一个unsafe函数!");
}

// Unsafe trait - 必须由调用者保证安全性
unsafe trait Holy {
    fn is_holy(&self) -> bool;
}

struct HolyTable {
    data: Vec<i32>,
}

impl HolyTable {
    fn new(size: usize) -> HolyTable {
        HolyTable {
            data: Vec::with_capacity(size),
        }
    }
    
    fn len(&self) -> usize {
        self.data.len()
    }
    
    fn capacity(&self) -> usize {
        self.data.capacity()
    }
}

// 实现unsafe trait
unsafe impl Holy for HolyTable {
    fn is_holy(&self) -> bool {
        self.data.capacity() > 0
    }
}

// 可变静态变量
static mut COUNTER: i32 = 0;

// 联合体 - 可以在同一内存位置存储不同类型
union MyUnion {
    i: i32,
    f: f32,
}
