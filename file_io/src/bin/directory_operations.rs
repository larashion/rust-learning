// ============================================================================
// Rust 目录操作
// ============================================================================
//
// Rust 的目录操作主要通过 std::fs 模块提供。
//
// 主要特点：
// 1. std::fs - 目录和文件系统操作
// 2. 支持创建、删除、遍历目录
// 3. 使用迭代器模式遍历目录
// 4. 返回 Result，安全处理错误

use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

// ============================================================================
// 示例 1: 创建目录
// ============================================================================
fn example1_create_directory() -> io::Result<()> {
    // 创建单级目录
    fs::create_dir("test_dir")?;

    println!("单级目录创建成功");

    // 清理
    fs::remove_dir("test_dir")?;
    Ok(())
}

// ============================================================================
// 示例 2: 创建多级目录
// ============================================================================
fn example2_create_directory_all() -> io::Result<()> {
    // 创建多级目录（如果父目录不存在会自动创建）
    fs::create_dir_all("test_dir/level1/level2/level3")?;

    println!("多级目录创建成功");

    // 清理
    fs::remove_dir_all("test_dir")?;
    Ok(())
}

// ============================================================================
// 示例 3: 读取目录内容
// ============================================================================
fn example3_read_directory() -> io::Result<()> {
    // 创建测试目录和文件
    fs::create_dir_all("test_read_dir/subdir")?;
    File::create("test_read_dir/file1.txt")?;
    File::create("test_read_dir/file2.txt")?;
    File::create("test_read_dir/subdir/file3.txt")?;

    // 读取目录内容
    println!("读取目录内容:");
    let entries = fs::read_dir("test_read_dir")?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            println!("  [目录] {:?}", path.file_name().unwrap());
        } else {
            println!("  [文件] {:?}", path.file_name().unwrap());
        }
    }

    // 清理
    fs::remove_dir_all("test_read_dir")?;
    Ok(())
}

// ============================================================================
// 示例 4: 递归遍历目录
// ============================================================================
fn example4_recursive_traverse() -> io::Result<()> {
    // 创建测试目录结构
    fs::create_dir_all("test_recursive/dir1/subdir1")?;
    fs::create_dir_all("test_recursive/dir2/subdir2")?;
    File::create("test_recursive/file1.txt")?;
    File::create("test_recursive/file2.txt")?;
    File::create("test_recursive/dir1/subdir1/file3.txt")?;

    println!("递归遍历目录结构:");
    visit_dir(&PathBuf::from("test_recursive"), 0)?;

    // 清理
    fs::remove_dir_all("test_recursive")?;
    Ok(())
}

fn visit_dir(dir: &Path, depth: usize) -> io::Result<()> {
    if dir.is_dir() {
        let indent = "  ".repeat(depth);
        println!("{}[目录] {:?}", indent, dir.file_name().unwrap());

        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                visit_dir(&path, depth + 1)?;
            } else {
                println!("{}  [文件] {:?}", indent, path.file_name().unwrap());
            }
        }
    }
    Ok(())
}

// ============================================================================
// 示例 5: 删除目录
// ============================================================================
fn example5_remove_directory() -> io::Result<()> {
    // 创建测试目录
    fs::create_dir("test_remove_dir")?;

    println!("目录创建成功");

    // 删除空目录
    fs::remove_dir("test_remove_dir")?;

    println!("空目录删除成功");

    // 创建非空目录
    fs::create_dir_all("test_remove_dir/subdir")?;
    File::create("test_remove_dir/file.txt")?;

    // 删除非空目录
    fs::remove_dir_all("test_remove_dir")?;

    println!("非空目录删除成功");
    Ok(())
}

// ============================================================================
// 示例 6: 获取当前工作目录
// ============================================================================
fn example6_current_dir() {
    let current_dir = std::env::current_dir().unwrap();
    println!("当前工作目录: {:?}", current_dir);
}

// ============================================================================
// 示例 7: 更改工作目录
// ============================================================================
fn example7_change_dir() -> io::Result<()> {
    // 创建测试目录
    fs::create_dir("test_change_dir")?;

    let original = std::env::current_dir()?;
    println!("原始目录: {:?}", original);

    // 更改工作目录
    std::env::set_current_dir("test_change_dir")?;

    println!("更改后目录: {:?}", std::env::current_dir()?);

    // 恢复
    std::env::set_current_dir(&original)?;
    println!("恢复目录: {:?}", std::env::current_dir()?);

    // 清理
    std::env::set_current_dir(&original)?;
    fs::remove_dir("test_change_dir")?;
    Ok(())
}

// ============================================================================
// 示例 8: 检查路径类型
// ============================================================================
fn example8_path_types() -> io::Result<()> {
    // 创建测试目录和文件
    fs::create_dir("test_path_types")?;
    File::create("test_path_types/file.txt")?;

    let dir_path = Path::new("test_path_types");
    let file_path = Path::new("test_path_types/file.txt");
    let nonexistent = Path::new("nonexistent");

    println!("路径类型检查:");
    println!("  {:?} 是目录? {}", dir_path, dir_path.is_dir());
    println!("  {:?} 是文件? {}", dir_path, dir_path.is_file());
    println!("  {:?} 是目录? {}", file_path, file_path.is_dir());
    println!("  {:?} 是文件? {}", file_path, file_path.is_file());
    println!("  {:?} 存在? {}", nonexistent, nonexistent.exists());

    // 清理
    fs::remove_dir_all("test_path_types")?;
    Ok(())
}

// ============================================================================
// 示例 9: 获取目录大小
// ============================================================================
fn example9_directory_size() -> io::Result<()> {
    // 创建测试目录和文件
    fs::create_dir_all("test_size/dir1/dir2")?;
    File::create("test_size/file1.txt")?.write_all(b"Hello")?;
    File::create("test_size/file2.txt")?.write_all(b"World")?;
    File::create("test_size/dir1/file3.txt")?.write_all(b"Test")?;

    let size = get_directory_size("test_size")?;
    println!("目录总大小: {} 字节", size);

    // 清理
    fs::remove_dir_all("test_size")?;
    Ok(())
}

fn get_directory_size(path: &str) -> io::Result<u64> {
    let mut total_size = 0;
    let path = Path::new(path);

    if path.is_file() {
        total_size += fs::metadata(path)?.len();
    } else if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let entry_path = entry.path();
            total_size += get_directory_size(entry_path.to_str().unwrap())?;
        }
    }

    Ok(total_size)
}

// ============================================================================
// 示例 10: 复制目录
// ============================================================================
fn example10_copy_directory() -> io::Result<()> {
    // 创建源目录
    fs::create_dir_all("test_copy_src/subdir")?;
    File::create("test_copy_src/file1.txt")?.write_all(b"Content1")?;
    File::create("test_copy_src/subdir/file2.txt")?.write_all(b"Content2")?;

    println!("复制目录...");
    copy_directory("test_copy_src", "test_copy_dst")?;

    // 验证
    println!("目标目录内容:");
    for entry in fs::read_dir("test_copy_dst")? {
        let entry = entry?;
        println!("  {:?}", entry.file_name());
    }

    // 清理
    fs::remove_dir_all("test_copy_src")?;
    fs::remove_dir_all("test_copy_dst")?;
    Ok(())
}

fn copy_directory(src: &str, dst: &str) -> io::Result<()> {
    let src_path = Path::new(src);
    let dst_path = Path::new(dst);

    if src_path.is_file() {
        fs::copy(src_path, dst_path)?;
    } else if src_path.is_dir() {
        if !dst_path.exists() {
            fs::create_dir_all(dst_path)?;
        }

        for entry in fs::read_dir(src_path)? {
            let entry = entry?;
            let entry_name = entry.file_name();
            let src_child = src_path.join(&entry_name);
            let dst_child = dst_path.join(&entry_name);
            copy_directory(src_child.to_str().unwrap(), dst_child.to_str().unwrap())?;
        }
    }

    Ok(())
}

// ============================================================================
// 示例 11: 查找文件
// ============================================================================
fn example11_find_files() -> io::Result<()> {
    // 创建测试目录结构
    fs::create_dir_all("test_find/dir1/dir2")?;
    File::create("test_find/file1.txt")?;
    File::create("test_find/file2.rs")?;
    File::create("test_find/dir1/file3.txt")?;
    File::create("test_find/dir1/dir2/file4.txt")?;

    println!("查找所有 .txt 文件:");
    let txt_files = find_files("test_find", "txt");
    for file in txt_files {
        println!("  {:?}", file);
    }

    // 清理
    fs::remove_dir_all("test_find")?;
    Ok(())
}

fn find_files(dir: &str, extension: &str) -> Vec<PathBuf> {
    let mut results = Vec::new();
    let path = Path::new(dir);

    if path.is_dir() {
        for entry in fs::read_dir(path).unwrap() {
            let entry = entry.unwrap();
            let entry_path = entry.path();

            if entry_path.is_dir() {
                results.extend(find_files(entry_path.to_str().unwrap(), extension));
            } else if entry_path.extension().is_some_and(|ext| ext == extension) {
                results.push(entry_path);
            }
        }
    }

    results
}

// ============================================================================
// 示例 12: 检查目录权限
// ============================================================================
fn example12_permissions() -> io::Result<()> {
    let filename = "test_permissions.txt";
    File::create(filename)?;

    let metadata = fs::metadata(filename)?;
    let permissions = metadata.permissions();
    let readonly = permissions.readonly();

    println!("文件权限:");
    println!("  只读: {}", readonly);

    // 设置为只读
    let mut perms = metadata.permissions();
    perms.set_readonly(true);
    fs::set_permissions(filename, perms)?;

    let new_metadata = fs::metadata(filename)?;
    println!("  设置后只读: {}", new_metadata.permissions().readonly());

    // 清理
    fs::remove_file(filename)?;
    Ok(())
}

// ============================================================================
// 示例 13: 目录存在性检查
// ============================================================================
fn example13_directory_exists() {
    let dir = "test_exists";
    let path = Path::new(dir);

    println!("目录存在? {}", path.exists());

    fs::create_dir(dir).unwrap();
    println!("创建后目录存在? {}", path.exists());

    fs::remove_dir(dir).unwrap();
    println!("删除后目录存在? {}", path.exists());
}

// ============================================================================
// 示例 14: 重命名目录
// ============================================================================
fn example14_rename_directory() -> io::Result<()> {
    let old_name = "test_old_dir";
    let new_name = "test_new_dir";

    fs::create_dir(old_name)?;

    println!("重命名目录...");
    fs::rename(old_name, new_name)?;

    println!("新目录存在? {}", Path::new(new_name).exists());

    fs::remove_dir(new_name)?;
    Ok(())
}

// ============================================================================
// 示例 15: 获取目录中的特定类型文件
// ============================================================================
fn example15_filter_files() -> io::Result<()> {
    // 创建测试文件
    fs::create_dir_all("test_filter")?;
    File::create("test_filter/file1.txt")?;
    File::create("test_filter/file2.txt")?;
    File::create("test_filter/file3.rs")?;
    File::create("test_filter/file4.md")?;

    println!("过滤文件:");
    let files = fs::read_dir("test_filter")?;
    for file in files.flatten() {
        let path = file.path();
        if let Some(ext) = path.extension() {
            if ext == "txt" {
                println!("  [文本文件] {:?}", path.file_name().unwrap());
            } else if ext == "rs" {
                println!("  [Rust 文件] {:?}", path.file_name().unwrap());
            } else {
                println!("  [其他文件] {:?}", path.file_name().unwrap());
            }
        }
    }

    // 清理
    fs::remove_dir_all("test_filter")?;
    Ok(())
}

// ============================================================================
// 示例 16: 创建目录并设置权限
// ============================================================================
#[cfg(unix)]
fn example16_create_with_permissions() -> io::Result<()> {
    use std::os::unix::fs::PermissionsExt;

    let dir = "test_perms";
    fs::create_dir(dir)?;

    let metadata = fs::metadata(dir)?;
    let mut perms = metadata.permissions();
    perms.set_mode(0o755); // rwxr-xr-x
    fs::set_permissions(dir, perms)?;

    println!("目录创建并设置权限完成");

    // 清理
    fs::remove_dir(dir)?;
    Ok(())
}

// ============================================================================
// 示例 17: 目录统计
// ============================================================================
struct DirectoryStats {
    total_files: usize,
    total_dirs: usize,
    total_size: u64,
}

fn example17_directory_stats() -> io::Result<()> {
    // 创建测试目录
    fs::create_dir_all("test_stats/dir1/dir2")?;
    File::create("test_stats/file1.txt")?.write_all(b"Hello")?;
    File::create("test_stats/file2.txt")?.write_all(b"World")?;
    File::create("test_stats/dir1/file3.txt")?.write_all(b"Test")?;

    let stats = get_directory_stats("test_stats")?;

    println!("目录统计:");
    println!("  总文件数: {}", stats.total_files);
    println!("  总目录数: {}", stats.total_dirs);
    println!("  总大小: {} 字节", stats.total_size);

    // 清理
    fs::remove_dir_all("test_stats")?;
    Ok(())
}

fn get_directory_stats(path: &str) -> io::Result<DirectoryStats> {
    let mut stats = DirectoryStats {
        total_files: 0,
        total_dirs: 0,
        total_size: 0,
    };

    let path = Path::new(path);

    if path.is_file() {
        stats.total_files += 1;
        stats.total_size += fs::metadata(path)?.len();
    } else if path.is_dir() {
        stats.total_dirs += 1;
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let entry_path = entry.path();
            let child_stats = get_directory_stats(entry_path.to_str().unwrap())?;
            stats.total_files += child_stats.total_files;
            stats.total_dirs += child_stats.total_dirs;
            stats.total_size += child_stats.total_size;
        }
    }

    Ok(stats)
}

// ============================================================================
// 示例 18: 按名称排序目录内容
// ============================================================================
fn example18_sort_directory() -> io::Result<()> {
    // 创建测试文件
    fs::create_dir_all("test_sort")?;
    for name in ["c.txt", "a.txt", "b.txt", "d.rs"] {
        File::create(format!("test_sort/{}", name))?;
    }

    println!("排序后的目录内容:");
    let mut entries: Vec<_> = fs::read_dir("test_sort")?
        .collect::<Result<Vec<_>, _>>()?;
    entries.sort_by_key(|entry| entry.file_name());

    for entry in entries {
        println!("  {:?}", entry.file_name());
    }

    // 清理
    fs::remove_dir_all("test_sort")?;
    Ok(())
}

// ============================================================================
// 示例 19: 检查目录是否为空
// ============================================================================
fn example19_is_empty() -> io::Result<()> {
    // 创建空目录
    fs::create_dir("test_empty")?;

    println!("空目录是空的? {}", is_directory_empty("test_empty")?);

    // 添加文件
    File::create("test_empty/file.txt")?;

    println!("添加文件后目录是空的? {}", is_directory_empty("test_empty")?);

    // 清理
    fs::remove_dir_all("test_empty")?;
    Ok(())
}

fn is_directory_empty(dir: &str) -> io::Result<bool> {
    let path = Path::new(dir);
    if !path.is_dir() {
        return Ok(false);
    }

    let mut entries = fs::read_dir(path)?;
    Ok(entries.next().is_none())
}

// ============================================================================
// 示例 20: 同步目录（简单实现）
// ============================================================================
fn example20_sync_directories() -> io::Result<()> {
    // 创建源目录
    fs::create_dir_all("test_sync_src/subdir")?;
    File::create("test_sync_src/file1.txt")?.write_all(b"Content1")?;
    File::create("test_sync_src/file2.txt")?.write_all(b"Content2")?;
    File::create("test_sync_src/subdir/file3.txt")?.write_all(b"Content3")?;

    println!("同步目录...");
    sync_directories("test_sync_src", "test_sync_dst")?;

    // 验证
    println!("目标目录文件:");
    for entry in fs::read_dir("test_sync_dst")? {
        let entry = entry?;
        println!("  {:?}", entry.file_name());
    }

    // 清理
    fs::remove_dir_all("test_sync_src")?;
    fs::remove_dir_all("test_sync_dst")?;
    Ok(())
}

fn sync_directories(src: &str, dst: &str) -> io::Result<()> {
    copy_directory(src, dst)?;
    Ok(())
}

// ============================================================================
// 主函数
// ============================================================================
fn main() {
    println!("=== Rust 目录操作示例 ===\n");

    println!("示例 1: 创建目录");
    example1_create_directory().unwrap();
    println!();

    println!("示例 2: 创建多级目录");
    example2_create_directory_all().unwrap();
    println!();

    println!("示例 3: 读取目录内容");
    example3_read_directory().unwrap();
    println!();

    println!("示例 4: 递归遍历目录");
    example4_recursive_traverse().unwrap();
    println!();

    println!("示例 5: 删除目录");
    example5_remove_directory().unwrap();
    println!();

    println!("示例 6: 获取当前工作目录");
    example6_current_dir();
    println!();

    println!("示例 7: 更改工作目录");
    example7_change_dir().unwrap();
    println!();

    println!("示例 8: 检查路径类型");
    example8_path_types().unwrap();
    println!();

    println!("示例 9: 获取目录大小");
    example9_directory_size().unwrap();
    println!();

    println!("示例 10: 复制目录");
    example10_copy_directory().unwrap();
    println!();

    println!("示例 11: 查找文件");
    example11_find_files().unwrap();
    println!();

    println!("示例 12: 检查目录权限");
    example12_permissions().unwrap();
    println!();

    println!("示例 13: 目录存在性检查");
    example13_directory_exists();
    println!();

    println!("示例 14: 重命名目录");
    example14_rename_directory().unwrap();
    println!();

    println!("示例 15: 过滤文件");
    example15_filter_files().unwrap();
    println!();

    println!("示例 16: 创建目录并设置权限");
    #[cfg(unix)]
    example16_create_with_permissions().unwrap();
    #[cfg(windows)]
    println!("（跳过：Unix 特定示例）");
    println!();

    println!("示例 17: 目录统计");
    example17_directory_stats().unwrap();
    println!();

    println!("示例 18: 按名称排序目录内容");
    example18_sort_directory().unwrap();
    println!();

    println!("示例 19: 检查目录是否为空");
    example19_is_empty().unwrap();
    println!();

    println!("示例 20: 同步目录");
    example20_sync_directories().unwrap();

    println!("\n=== 总结 ===");
    println!("Rust 目录操作特点:");
    println!("  - std::fs 提供完整的目录操作");
    println!("  - create_dir: 创建单级目录");
    println!("  - create_dir_all: 创建多级目录");
    println!("  - read_dir: 读取目录内容");
    println!("  - remove_dir: 删除空目录");
    println!("  - remove_dir_all: 删除非空目录");
    println!("  - 支持递归遍历和查找");
    println!("  - 所有操作返回 Result");
}
