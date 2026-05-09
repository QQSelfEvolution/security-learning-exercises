// 小暗 Day1练习 #3: 内存安全对比
// Rust vs C vs Zig - 内存安全特性对比
use std::fmt;

// ==================== Rust: 安全的内存管理 ====================

fn rust_memory_safety() {
    println!("\n=== Rust: 安全的内存管理 ===");
    
    // 1. 所有权系统 - 自动内存回收
    let s1 = String::from("hello"); // 堆分配
    let s2 = s1; // 所有权转移
    // println!("{}", s1); // 编译错误！s1已失效
    println!("s2 (所有权已转移): {}", s2);
    
    // 2. 借用检查 - 防止数据竞争
    let mut data = vec![1, 2, 3];
    let borrow1 = &data;     // 不可变借用
    let borrow2 = &data;     // 可以有多个不可变借用
    println!("借用: {:?}, {:?}", borrow1, borrow2);
    // borrow1.push(4); // 编译错误！有不可变借用时不能修改
    
    drop(borrow1);
    drop(borrow2);
    
    let borrow_mut = &mut data; // 可变借用
    borrow_mut.push(4); // OK!
    println!("修改后: {:?}", borrow_mut);
    
    // 3. 生命周期 - 防止悬垂引用
    let novel = String::from("白日依山尽...");
    let first_line = get_first_line(&novel);
    println!("第一行: {}", first_line);
    
    // 4. 线程安全 - Send和Sync
    println!("String是Send: {}", std::marker::Send);
    println!("String是Sync: {}", std::marker::Sync);
}

fn get_first_line(s: &str) -> &str {
    s.lines().next().unwrap_or("")
}

// ==================== C: 不安全的内存操作 ====================

fn c_memory_unsafe() {
    println!("\n=== C: 手动内存管理 ===");
    
    // C风格的内存问题（模拟展示）
    println!("C语言内存特点:");
    println!("  - 需要手动malloc/free");
    println!("  - 缓冲区溢出风险");
    println!("  - 悬垂指针问题");
    println!("  - 数据竞争无检查");
    println!("  - use-after-free漏洞");
    
    // 常见C内存错误示例（注释说明）
    println!("\n示例代码（示意）:");
    println!("  char* p = malloc(10);");
    println!("  strcpy(p, \"hello world\");  // 缓冲区溢出!");
    println!("  free(p);");
    println!("  printf(\"%s\", p);  // use-after-free!");
}

// ==================== Zig: 显式但安全的内存 ====================

fn zig_memory_model() {
    println!("\n=== Zig: 显式内存管理 ===");
    
    println!("Zig内存特点:");
    println!("  - 无垃圾回收");
    println!("  - 显式内存分配");
    println!("  - defer自动释放");
    println!("  - 编译器检查边界");
    println!("  - 无null引用（可选类型）");
    
    // defer示例（概念）
    println!("\ndefer使用:");
    println!("  const file = try openFile(\"data.txt\");");
    println!("  defer closeFile(file);  // 自动关闭");
    println!("  // 文件会在作用域结束时自动关闭");
}

// ==================== 内存安全对比表 ====================

struct SafetyComparison {
    language: &'static str,
    buffer_overflow: &'static str,
    use_after_free: &'static str,
    data_race: &'static str,
    null_deref: &'static str,
}

impl fmt::Display for SafetyComparison {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "| {:<8} | {:<14} | {:<14} | {:<12} | {:<10} |",
            self.language,
            self.buffer_overflow,
            self.use_after_free,
            self.data_race,
            self.null_deref
        )
    }
}

fn print_comparison_table() {
    println!("\n=== 内存安全对比表 ===");
    println!("| 语言     | 缓冲区溢出    | 释放后使用   | 数据竞争     | 空解引用   |");
    println!("|----------|--------------|--------------|--------------|------------|");
    
    let comparisons = vec![
        SafetyComparison {
            language: "Rust",
            buffer_overflow: "❌ 不可能",
            use_after_free: "❌ 不可能",
            data_race: "❌ 编译阻止",
            null_deref: "⚠️ Option",
        },
        SafetyComparison {
            language: "Zig",
            buffer_overflow: "⚠️ 可选检查",
            use_after_free: "⚠️ 手动",
            data_race: "⚠️ 手动",
            null_deref: "⚠️ 可选类型",
        },
        SafetyComparison {
            language: "C",
            buffer_overflow: "✅ 可能",
            use_after_free: "✅ 可能",
            data_race: "✅ 可能",
            null_deref: "✅ 可能",
        },
        SafetyComparison {
            language: "Python",
            buffer_overflow: "❌ 不可能",
            use_after_free: "❌ 不可能",
            data_race: "⚠️ GIL限制",
            null_deref: "✅ 可能",
        },
    ];
    
    for c in comparisons {
        println!("{}", c);
    }
}

// ==================== 主函数 ====================

fn main() {
    println!("=== 小暗 Rust学习 Day1 - 内存安全对比 ===");
    
    rust_memory_safety();
    c_memory_unsafe();
    zig_memory_model();
    print_comparison_table();
    
    println!("\n总结:");
    println!("  Rust: 编译时保证内存安全");
    println!("  Zig: 显式管理 + 编译器辅助");
    println!("  C: 完全手动，需要工具辅助");
    
    println!("\n=== Day1练习3完成 ===");
}
