// ============================================================================
// Rust 文件操作 - 基础文件 I/O
// ============================================================================
//
// Rust 的文件操作主要通过 std::fs 和 std::io 模块提供。
//
// 主要特点：
// 1. std::fs - 文件系统操作（创建、删除、移动等）
// 2. std::io - 文件读写操作
// 3. std::path - 路径操作
// 4. 操作返回 Result，处理错误

use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Write, BufRead, BufReader, BufWriter};
use std::path::Path;

// ============================================================================
// 示例 1: 读取文件内容（最简单的方式）
// ============================================================================
fn example1_read_to_string() -> io::Result<()> {
    // 写入一个测试文件
    let filename = "test_read.txt";
    fs::write(filename, "Hello, Rust!\nThis is a test file.")?;

    // 读取整个文件到字符串
    let content = fs::read_to_string(filename)?;
    println!("读取到内容:\n{}", content);

    // 清理
    fs::remove_file(filename)?;
    Ok(())
}

// ============================================================================
// 示例 2: 读取文件到字节数组
// ============================================================================
fn example2_read_to_bytes() -> io::Result<()> {
    let filename = "test_bytes.txt";
    fs::write(filename, "Binary data: \x00\x01\x02")?;

    // 读取为 Vec<u8>
    let bytes = fs::read(filename)?;
    println!("字节数组: {:?}", bytes);
    println!("字节数组长度: {}", bytes.len());

    fs::remove_file(filename)?;
    Ok(())
}

// ============================================================================
// 示例 3: 写入文件（创建或覆盖）
// ============================================================================
fn example3_write_file() -> io::Result<()> {
    let filename = "test_write.txt";

    // 写入内容（如果文件存在则覆盖）
    fs::write(filename, "这是写入的内容\n")?;

    // 读取验证
    let content = fs::read_to_string(filename)?;
    println!("写入内容: {}", content.trim());

    // 追加写入
    fs::write(filename, "这是追加的内容\n")?;

    let content = fs::read_to_string(filename)?;
    println!("追加强制覆盖后: {}", content.trim());

    fs::remove_file(filename)?;
    Ok(())
}

// ============================================================================
// 示例 4: 使用 File 和 OpenOptions 精确控制
// ============================================================================
fn example4_file_open_options() -> io::Result<()> {
    let filename = "test_options.txt";

    // 创建新文件（如果存在则失败）
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(filename)?;
    writeln!(file, "创建新文件")?;

    // 追加内容
    let mut file = OpenOptions::new()
        .append(true)
        .open(filename)?;
    writeln!(file, "追加的内容")?;

    // 读取内容
    let content = fs::read_to_string(filename)?;
    println!("文件内容:\n{}", content);

    fs::remove_file(filename)?;
    Ok(())
}

// ============================================================================
// 示例 5: 使用 BufReader 逐行读取
// ============================================================================
fn example5_read_lines() -> io::Result<()> {
    let filename = "test_lines.txt";
    let lines = ["第一行", "第二行", "第三行"];
    fs::write(filename, lines.join("\n"))?;

    // 打开文件
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    // 逐行读取
    println!("逐行读取:");
    for (index, line) in reader.lines().enumerate() {
        let line = line?;
        println!("  行 {}: {}", index + 1, line);
    }

    fs::remove_file(filename)?;
    Ok(())
}

// ============================================================================
// 示例 6: 使用 BufWriter 提高写入性能
// ============================================================================
fn example6_bufwriter() -> io::Result<()> {
    let filename = "test_bufwriter.txt";

    let file = File::create(filename)?;
    let mut writer = BufWriter::new(file);

    // 使用 BufWriter 可以减少磁盘 I/O 次数
    for i in 0..1000 {
        writeln!(writer, "行 {}", i)?;
    }

    // 必须调用 flush 确保所有数据都写入
    writer.flush()?;

    println!("使用 BufWriter 写入 {} 行", 1000);

    fs::remove_file(filename)?;
    Ok(())
}

// ============================================================================
// 示例 7: 使用 Read trait 读取指定字节数
// ============================================================================
fn example7_read_bytes() -> io::Result<()> {
    let filename = "test_read_bytes.txt";
    fs::write(filename, "这是一个测试文件，用于读取指定字节数")?;

    let mut file = File::open(filename)?;
    let mut buffer = [0u8; 10]; // 读取 10 字节

    // 读取到 buffer
    let bytes_read = file.read(&mut buffer)?;
    let text = std::str::from_utf8(&buffer[..bytes_read])
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    println!("收到消息: {}", text);

    fs::remove_file(filename)?;
    Ok(())
}

// ============================================================================
// 示例 8: 使用 Write trait 写入
// ============================================================================
fn example8_write_bytes() -> io::Result<()> {
    let filename = "test_write_bytes.txt";

    let mut file = File::create(filename)?;

    let data = b"Hello";
    file.write_all(data)?;

    let more_data = " World!".as_bytes();
    file.write_all(more_data)?;

    // 读取验证
    let content = fs::read_to_string(filename)?;
    println!("写入的内容: {}", content);

    fs::remove_file(filename)?;
    Ok(())
}

// ============================================================================
// 示例 9: 文件元信息（Metadata）
// ============================================================================
fn example9_metadata() -> io::Result<()> {
    let filename = "test_metadata.txt";
    fs::write(filename, "测试文件")?;

    // 获取文件元信息
    let metadata = fs::metadata(filename)?;

    println!("文件元信息:");
    println!("  是否为目录: {}", metadata.is_dir());
    println!("  是否为文件: {}", metadata.is_file());
    println!("  文件大小: {} 字节", metadata.len());

    if let Ok(modified) = metadata.modified() {
        println!("  修改时间: {:?}", modified);
    }

    if let Ok(created) = metadata.created() {
        println!("  创建时间: {:?}", created);
    }

    if let Ok(accessed) = metadata.accessed() {
        println!("  访问时间: {:?}", accessed);
    }

    fs::remove_file(filename)?;
    Ok(())
}

// ============================================================================
// 示例 10: 文件存在性检查
// ============================================================================
fn example10_file_exists() {
    let filename = "test_exists.txt";

    println!("文件存在? {}", Path::new(filename).exists());

    fs::write(filename, "test").unwrap();
    println!("创建后，文件存在? {}", Path::new(filename).exists());

    fs::remove_file(filename).unwrap();
    println!("删除后，文件存在? {}", Path::new(filename).exists());
}

// ============================================================================
// 示例 11: 复制文件
// ============================================================================
fn example11_copy_file() -> io::Result<()> {
    let src = "test_copy_src.txt";
    let dst = "test_copy_dst.txt";

    fs::write(src, "这是源文件内容")?;

    // 复制文件
    fs::copy(src, dst)?;

    let content = fs::read_to_string(dst)?;
    println!("复制后的内容: {}", content);

    fs::remove_file(src)?;
    fs::remove_file(dst)?;
    Ok(())
}

// ============================================================================
// 示例 12: 重命名文件
// ============================================================================
fn example12_rename_file() -> io::Result<()> {
    let old_name = "test_old.txt";
    let new_name = "test_new.txt";

    fs::write(old_name, "重命名测试")?;

    // 重命名
    fs::rename(old_name, new_name)?;

    println!("重命名后的文件存在? {}", Path::new(new_name).exists());

    fs::remove_file(new_name)?;
    Ok(())
}

// ============================================================================
// 示例 13: 使用 Path 处理路径
// ============================================================================
fn example13_path_operations() {
    let path = Path::new("/home/user/documents/file.txt");

    println!("路径操作:");
    println!("  文件名: {:?}", path.file_name());
    println!("  文件扩展名: {:?}", path.extension());
    println!("  父目录: {:?}", path.parent());
    println!("  路径是否存在: {}", path.exists());

    // 路径拼接
    let base = Path::new("/home/user");
    let full = base.join("documents/file.txt");
    println!("  拼接路径: {:?}", full);

    // 路径规范化
    let path = Path::new("/home/user/../user/./documents");
    println!("  规范化路径: {:?}", path.canonicalize());
}

// ============================================================================
// 示例 14: 错误处理
// ============================================================================
fn example14_error_handling() {
    // 尝试打开不存在的文件
    match File::open("nonexistent.txt") {
        Ok(file) => {
            println!("成功打开文件: {:?}", file);
        }
        Err(error) => {
            match error.kind() {
                io::ErrorKind::NotFound => {
                    println!("错误: 文件不存在");
                }
                io::ErrorKind::PermissionDenied => {
                    println!("错误: 权限不足");
                }
                _ => {
                    println!("错误: {:?}", error);
                }
            }
        }
    }
}

// ============================================================================
// 示例 15: 读取配置文件（实际应用）
// ============================================================================
fn example15_read_config() -> io::Result<()> {
    let config_file = "test_config.ini";

    // 创建配置文件
    let config_content = r#"
[server]
host = localhost
port = 8080

[database]
name = mydb
user = admin
password = secret
"#;
    fs::write(config_file, config_content)?;

    // 读取配置文件
    let file = File::open(config_file)?;
    let reader = BufReader::new(file);

    println!("解析配置文件:");
    for line in reader.lines() {
        let line = line?;
        if !line.trim().is_empty() && !line.starts_with('[') {
            println!("  {}", line.trim());
        }
    }

    fs::remove_file(config_file)?;
    Ok(())
}

// ============================================================================
// 示例 16: 文件锁（简单实现）
// ============================================================================
fn example16_file_lock() -> io::Result<()> {
    // 注意：这是一个简化的示例
    // 实际生产环境建议使用 fs2 或其他专门的 crate

    let filename = "test_lock.txt";
    fs::write(filename, "测试文件锁")?;

    // 打开文件
    let file = OpenOptions::new()
        .write(true)
        .open(filename)?;

    println!("文件已打开，可以进行操作");

    // 读写操作...

    drop(file);
    fs::remove_file(filename)?;
    Ok(())
}

// ============================================================================
// 示例 17: 大文件处理（流式处理）
// ============================================================================
fn example17_large_file() -> io::Result<()> {
    let filename = "test_large.txt";

    // 创建一个较大的文件
    let file = File::create(filename)?;
    let mut writer = BufWriter::new(file);

    for i in 0..10000 {
        writeln!(writer, "行 {}", i)?;
    }
    writer.flush()?;

    // 流式读取大文件
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut count = 0;
    for line in reader.lines() {
        count += 1;
        if count > 5 {
            break; // 只读取前几行演示
        }
        println!("读取行 {}: {}", count, line?);
    }

    println!("总共 {} 行", 10000);

    fs::remove_file(filename)?;
    Ok(())
}

// ============================================================================
// 示例 18: 临时文件
// ============================================================================
fn example18_temp_file() -> io::Result<()> {
    // 使用 NamedTempFile（需要 tempfile crate）
    // 这里演示简单的临时文件创建

    let temp_dir = std::env::temp_dir();
    let temp_file = temp_dir.join("temp_test.txt");

    println!("临时文件路径: {:?}", temp_file);

    fs::write(&temp_file, "临时文件内容")?;

    let content = fs::read_to_string(&temp_file)?;
    println!("临时文件内容: {}", content);

    fs::remove_file(temp_file)?;
    Ok(())
}

// ============================================================================
// 示例 19: 读取二进制文件
// ============================================================================
fn example19_read_binary() -> io::Result<()> {
    let filename = "test_binary.bin";

    // 写入一些二进制数据
    let data: Vec<u8> = vec
![0xDE, 0xAD, 0xBE, 0xEF, 0x00, 0x01, 0x02, 0x03];
    fs::write(filename, &data)?;

    // 读取二进制文件
    let bytes = fs::read(filename)?;

    println!("读取二进制数据:");
    println!("  原始: {:?}", bytes);
    println!("  十六进制: {:02X}", bytes[0]);
    println!("  整数 (前4字节): {:08X}",
             u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]));

    fs::remove_file(filename)?;
    Ok(())
}

// ============================================================================
// 示例 20: 批量文件操作
// ============================================================================
fn example20_batch_operations() -> io::Result<()> {
    // 创建多个文件
    for i in 1..=5 {
        let filename = format!("test_batch_{}.txt", i);
        fs::write(filename, format!("文件 {} 内容", i))?;
    }

    // 批量读取
    println!("批量读取文件:");
    for i in 1..=5 {
        let filename = format!("test_batch_{}.txt", i);
        let content = fs::read_to_string(&filename)?;
        println!("  {}: {}", filename, content.trim());
    }

    // 批量删除
    println!("\n批量删除文件:");
    for i in 1..=5 {
        let filename = format!("test_batch_{}.txt", i);
        fs::remove_file(&filename)?;
        println!("  删除 {}", filename);
    }

    Ok(())
}

// ============================================================================
// 主函数
// ============================================================================
fn main() {
    println!("=== Rust 文件操作示例 ===\n");

    println!("示例 1: 读取文件到字符串");
    example1_read_to_string().unwrap();
    println!();

    println!("示例 2: 读取文件到字节数组");
    example2_read_to_bytes().unwrap();
    println!();

    println!("示例 3: 写入文件");
    example3_write_file().unwrap();
    println!();

    println!("示例 4: 使用 File 和 OpenOptions");
    example4_file_open_options().unwrap();
    println!();

    println!("示例 5: 逐行读取");
    example5_read_lines().unwrap();
    println!();

    println!("示例 6: 使用 BufWriter");
    example6_bufwriter().unwrap();
    println!();

    println!("示例 7: 读取指定字节");
    example7_read_bytes().unwrap();
    println!();

    println!("示例 8: 写入字节");
    example8_write_bytes().unwrap();
    println!();

    println!("示例 9: 文件元信息");
    example9_metadata().unwrap();
    println!();

    println!("示例 10: 文件存在性检查");
    example10_file_exists();
    println!();

    println!("示例 11: 复制文件");
    example11_copy_file().unwrap();
    println!();

    println!("示例 12: 重命名文件");
    example12_rename_file().unwrap();
    println!();

    println!("示例 13: 路径操作");
    example13_path_operations();
    println!();

    println!("示例 14: 错误处理");
    example14_error_handling();
    println!();

    println!("示例 15: 读取配置文件");
    example15_read_config().unwrap();
    println!();

    println!("示例 16: 文件锁");
    example16_file_lock().unwrap();
    println!();

    println!("示例 17: 大文件处理");
    example17_large_file().unwrap();
    println!();

    println!("示例 18: 临时文件");
    example18_temp_file().unwrap();
    println!();

    println!("示例 19: 读取二进制文件");
    example19_read_binary().unwrap();
    println!();

    println!("示例 20: 批量文件操作");
    example20_batch_operations().unwrap();

    println!("\n=== 总结 ===");
    println!("Rust 文件操作特点:");
    println!("  - std::fs: 文件系统操作");
    println!("  - std::io: 文件读写");
    println!("  - std::path: 路径处理");
    println!("  - 所有操作返回 Result，需要错误处理");
    println!("  - BufReader/BufWriter 提高性能");
    println!("  - Read/Write trait 提供通用接口");
    println!("  - 类型安全，编译时检查");
}
