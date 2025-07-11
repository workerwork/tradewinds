use bcrypt::{DEFAULT_COST, hash, verify};

fn main() {
    let password = "admin123";
    match hash(password.as_bytes(), DEFAULT_COST) {
        Ok(hashed) => {
            println!("原始密码: {}", password);
            println!("加密后的密码: {}", hashed);

            // 验证密码
            match verify(password.as_bytes(), &hashed) {
                Ok(valid) => println!("密码验证: {}", if valid { "成功" } else { "失败" }),
                Err(e) => println!("验证失败: {}", e),
            }

            // 打印字符串长度信息
            println!("原始密码长度: {}", password.len());
            println!("加密后密码长度: {}", hashed.len());
            println!("加密后密码字节: {:?}", hashed.as_bytes());
        }
        Err(e) => println!("加密失败: {}", e),
    }
}