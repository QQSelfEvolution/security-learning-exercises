// 小暗 Day1练习 #2: Zig Hello World
// Zig语言 - 基础语法
const std = @import("std");

pub fn main() void {
    std.debug.print("=== 小暗 Zig学习 Day1 ===\n", .{});
    std.debug.print("Hello World! 我是小暗!\n", .{});
    std.debug.print("开始学习Zig!\n\n", .{});
    
    // 基础变量
    std.debug.print("--- 基础变量 ---\n", .{});
    const name: []const u8 = "小暗";
    var age: u8 = 24;
    const height: f32 = 175.5;
    const is_student: bool = true;
    
    std.debug.print("姓名: {s}\n", .{name});
    std.debug.print("年龄: {}\n", .{age});
    std.debug.print("身高: {}\n", .{height});
    std.debug.print("是学生: {}\n", .{is_student});
    
    // 数组
    std.debug.print("\n--- 数组 ---\n", .{});
    const numbers: [5]i32 = [_]i32{1, 2, 3, 4, 5};
    std.debug.print("数组: ", .{});
    for (numbers) |n| {
        std.debug.print("{} ", .{n});
    }
    std.debug.print("\n长度: {}\n", .{numbers.len});
    
    // 函数
    std.debug.print("\n--- 函数 ---\n", .{});
    const result = add(10, 20);
    std.debug.print("10 + 20 = {}\n", .{result});
    
    const sum = sumArray(numbers);
    std.debug.print("数组和: {}\n", .{sum});
    
    // 结构体
    std.debug.print("\n--- 结构体 ---\n", .{});
    var person = Person{
        .name = "小暗",
        .age = 24,
        .role = .developer,
    };
    std.debug.print("Person: {s}, age={}, role={}\n", .{
        person.name, person.age, @tagName(person.role)
    });
    person.greet();
    
    // 枚举
    std.debug.print("\n--- 枚举 ---\n", .{});
    const status: Status = .active;
    std.debug.print("Status: {s}\n", .{@tagName(status)});
    
    std.debug.print("\n=== Day1练习2完成 ===\n", .{});
}

// 加法函数
fn add(a: i32, b: i32) i32 {
    return a + b;
}

// 数组求和
fn sumArray(arr: [5]i32) i32 {
    var sum: i32 = 0;
    for (arr) |n| {
        sum += n;
    }
    return sum;
}

// Role枚举
const Role = enum {
    developer,
    designer,
    manager,
};

// Status枚举
const Status = enum {
    pending,
    active,
    completed,
};

// Person结构体
const Person = struct {
    name: []const u8,
    age: u8,
    role: Role,
    
    // 方法
    fn greet(self: *const Person) void {
        std.debug.print("你好, 我是{s}, 是一名{s}!\n", .{
            self.name,
            @tagName(self.role)
        });
    }
};
