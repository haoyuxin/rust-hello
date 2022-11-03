// 未使用到的方法禁止报错
#![allow(dead_code)]

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");
    if 1 == 2 {
        println!("Hello, world!");
    }
    // mut修饰可变的量
    let mut mut_attr = 10;
    mut_attr += 10;
    println!("mutAttr is {mut_attr}");
    let num = 10;
    let nums = [1, 2, 3, 4, 5, 6, 7, 8];
    let sum = sum_of_val(&nums, num);
    println!("sum is {sum}");

    // 文件读取
    let path = "lines.txt";
    // 创建文件
    let mut output = File::create(path)?;
    // 写入三行内容
    write!(output, "Rust\n💖\nFun")?;
    let input = File::open(path)?;
    let buffered = BufReader::new(input);
    // 迭代文件中的每一行内容，line 是字符串
    for line in buffered.lines() {
        println!("{}", line?);
    }

    //http请求
    let resp = reqwest::blocking::get("https://httpbin.org/ip")?.json::<HashMap<String, String>>()?;
    println!("{:#?}", resp);
    let x: f32 = 2.0;
    return_sin(x);
    Ok(())
}


fn sum_of_val(nums: &[i32], num: i32) -> i32 {
    let mut sum: i32 = 0;
    for n in nums {
        sum += n;
    }
    sum + num
}

// 优化版
fn sum_of_val2(nums: &[i32], num: i32) -> i32 {
    nums.iter().sum::<i32>() + num
}

fn return_sin(x: f32) {
    let y = x.sin();
    println!("sin(2) is {y}")
}
