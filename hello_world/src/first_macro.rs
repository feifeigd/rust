// 定义一个宏
#[macro_export]
macro_rules! scanline {
    // 模式匹配器
    ($x:expr) => {{
        std::io::stdin().read_line(&mut $x).unwrap();
        $x.trim()
    }};
}
