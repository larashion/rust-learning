// #![allow(unused)] // Cleaned up: Removed global suppression
// ============================================================================
// Walkdir - 高效目录遍历
// ============================================================================
//
// walkdir 是一个流行的 Rust crate，用于高效遍历目录树。
//
// 主要特点：
// 1. 高性能：优化的目录遍历
// 2. 灵活的迭代器模式
// 3. 支持按最小/最大深度遍历
// 4. 支持并行遍历
// 5. 可以控制遍历顺序
// 6. 处理符号链接等特殊情况
//
// 依赖：walkdir = "2.3"

use std::fs;
use std::io;
use std::path::Path;

// ============================================================================
// 示例 1: 基本用法 - 遍历目录树
// ============================================================================
// 使用方式：walkdir crate
// Cargo.toml 需要添加: walkdir = "2.3"

#[allow(dead_code)]
fn example1_basic_walkdir() {
    use walkdir::WalkDir;

    println!("遍历当前目录:");

    for entry in WalkDir::new(".")
        .max_depth(2)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        println!("  {:?}", entry.path());
    }
}

// ============================================================================
// 示例 2: 使用 fs 递归遍历（标准库版本）
// ============================================================================
fn example2_std_traversal() -> io::Result<()> {
    // 创建测试目录结构
    fs::create_dir_all("test_walkdir/dir1/subdir1")?;
    fs::create_dir_all("test_walkdir/dir2/subdir2")?;
    fs::write("test_walkdir/file1.txt", "content1")?;
    fs::write("test_walkdir/file2.rs", "content2")?;
    fs::write("test_walkdir/dir1/subdir1/file3.txt", "content3")?;

    println!("使用标准库递归遍历:");
    traverse_directory("test_walkdir", 0)?;

    // 清理
    fs::remove_dir_all("test_walkdir")?;
    Ok(())
}

fn traverse_directory(path: &str, depth: usize) -> io::Result<()> {
    let path = Path::new(path);
    let indent = "  ".repeat(depth);

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let entry_path = entry.path();

        if entry_path.is_dir() {
            println!("{}[DIR] {:?}", indent, entry_path.file_name().unwrap());
            traverse_directory(entry_path.to_str().unwrap(), depth + 1)?;
        } else {
            println!("{}[FILE] {:?}", indent, entry_path.file_name().unwrap());
        }
    }

    Ok(())
}

// ============================================================================
// 示例 3: 自定义 WalkDir 实现（简化版）
// ============================================================================
struct WalkEntry {
    path: std::path::PathBuf,
    depth: usize,
}

impl WalkEntry {
    fn file_name(&self) -> Option<&std::ffi::OsStr> {
        self.path.file_name()
    }
    #[allow(dead_code)]
    fn is_file(&self) -> bool {
        self.path.is_file()
    }
}

fn example3_custom_walkdir() -> io::Result<()> {
    // 创建测试目录
    fs::create_dir_all("test_custom/dir1/subdir")?;
    fs::write("test_custom/file1.txt", "test")?;
    fs::write("test_custom/dir1/file2.txt", "test")?;

    println!("自定义 WalkDir 实现:");
    for entry in custom_walkdir("test_custom", 3)? {
        println!(
            "  {} {:?}",
            "  ".repeat(entry.depth),
            entry.file_name().unwrap()
        );
    }

    // 清理
    fs::remove_dir_all("test_custom")?;
    Ok(())
}

fn custom_walkdir(root: &str, max_depth: usize) -> io::Result<Vec<WalkEntry>> {
    let mut entries = Vec::new();
    walk_recursive(root, 0, max_depth, &mut entries)?;
    Ok(entries)
}

fn walk_recursive(
    path: &str,
    depth: usize,
    max_depth: usize,
    entries: &mut Vec<WalkEntry>,
) -> io::Result<()> {
    let path_buf = std::path::PathBuf::from(path);
    entries.push(WalkEntry {
        path: path_buf.clone(),
        depth,
    });

    if depth < max_depth {
        if let Ok(dir_entries) = fs::read_dir(&path_buf) {
            for entry in dir_entries {
                let entry = entry?;
                let entry_path = entry.path();

                if entry_path.is_dir() {
                    walk_recursive(entry_path.to_str().unwrap(), depth + 1, max_depth, entries)?;
                } else {
                    entries.push(WalkEntry {
                        path: entry_path,
                        depth: depth + 1,
                    });
                }
            }
        }
    }

    Ok(())
}

// ============================================================================
// 示例 4: 按文件类型过滤
// ============================================================================
fn example4_filter_by_type() -> io::Result<()> {
    // 创建测试文件
    fs::create_dir_all("test_filter_types/subdir")?;
    fs::write("test_filter_types/file1.txt", "test")?;
    fs::write("test_filter_types/file2.rs", "test")?;
    fs::write("test_filter_types/file3.md", "test")?;
    fs::write("test_filter_types/subdir/file4.txt", "test")?;

    println!("过滤 .txt 文件:");
    let txt_files = filter_files_by_extension("test_filter_types", "txt")?;
    for file in txt_files {
        println!("  {}", file);
    }

    println!("\n过滤 .rs 文件:");
    let rs_files = filter_files_by_extension("test_filter_types", "rs")?;
    for file in rs_files {
        println!("  {}", file);
    }

    // 清理
    fs::remove_dir_all("test_filter_types")?;
    Ok(())
}

fn filter_files_by_extension(root: &str, ext: &str) -> io::Result<Vec<String>> {
    let mut results = Vec::new();
    find_files_recursive(root, ext, &mut results)?;
    Ok(results)
}

fn find_files_recursive(path: &str, ext: &str, results: &mut Vec<String>) -> io::Result<()> {
    let path = Path::new(path);

    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let entry_path = entry.path();
            find_files_recursive(entry_path.to_str().unwrap(), ext, results)?;
        }
    } else if path.extension().is_some_and(|e| e == ext) {
        results.push(path.to_str().unwrap().to_string());
    }

    Ok(())
}

// ============================================================================
// 示例 5: 按深度限制
// ============================================================================
fn example5_depth_limit() -> io::Result<()> {
    // 创建测试目录
    fs::create_dir_all("test_depth/dir1/dir2/dir3")?;
    fs::write("test_depth/file1.txt", "test")?;
    fs::write("test_depth/dir1/file2.txt", "test")?;
    fs::write("test_depth/dir1/dir2/file3.txt", "test")?;
    fs::write("test_depth/dir1/dir2/dir3/file4.txt", "test")?;

    println!("限制深度为 2:");
    for entry in custom_walkdir("test_depth", 2)? {
        println!(
            "  {} {:?}",
            "  ".repeat(entry.depth),
            entry.file_name().unwrap()
        );
    }

    // 清理
    fs::remove_dir_all("test_depth")?;
    Ok(())
}

// ============================================================================
// 示例 6: 获取目录大小
// ============================================================================
fn example6_directory_size() -> io::Result<()> {
    // 创建测试文件
    fs::create_dir_all("test_size/dir1/dir2")?;
    fs::write("test_size/file1.txt", "Hello")?;
    fs::write("test_size/file2.txt", "World")?;
    fs::write("test_size/dir1/file3.txt", "Test")?;

    let size = calculate_directory_size("test_size")?;
    println!("目录总大小: {} 字节", size);

    // 清理
    fs::remove_dir_all("test_size")?;
    Ok(())
}

fn calculate_directory_size(path: &str) -> io::Result<u64> {
    let mut total_size = 0u64;
    let path = Path::new(path);

    for entry in walkdir_recursive(path)? {
        if entry.is_file() {
            if let Ok(metadata) = fs::metadata(&entry) {
                total_size += metadata.len();
            }
        }
    }

    Ok(total_size)
}

fn walkdir_recursive(path: &Path) -> io::Result<Vec<std::path::PathBuf>> {
    let mut entries = Vec::new();

    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let entry_path = entry.path();
            entries.push(entry_path.clone());
            entries.extend(walkdir_recursive(&entry_path)?);
        }
    }

    Ok(entries)
}

// ============================================================================
// 示例 7: 查找特定文件
// ============================================================================
fn example7_find_specific_file() -> io::Result<()> {
    // 创建测试文件
    fs::create_dir_all("test_find/subdir")?;
    fs::write("test_find/target.txt", "found me!")?;
    fs::write("test_find/other.txt", "not me")?;
    fs::write("test_find/subdir/target.txt", "found me too!")?;

    println!("查找 target.txt 文件:");
    let found = find_file("test_find", "target.txt")?;
    for file in found {
        println!("  {}", file);
    }

    // 清理
    fs::remove_dir_all("test_find")?;
    Ok(())
}

fn find_file(root: &str, filename: &str) -> io::Result<Vec<String>> {
    let mut results = Vec::new();
    find_file_recursive(root, filename, &mut results)?;
    Ok(results)
}

fn find_file_recursive(path: &str, filename: &str, results: &mut Vec<String>) -> io::Result<()> {
    let path = Path::new(path);

    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let entry_path = entry.path();

            if entry_path.is_dir() {
                find_file_recursive(entry_path.to_str().unwrap(), filename, results)?;
            } else if entry_path.file_name().is_some_and(|n| n == filename) {
                results.push(entry_path.to_str().unwrap().to_string());
            }
        }
    }

    Ok(())
}

// ============================================================================
// 示例 8: 统计文件和目录数量
// ============================================================================
struct DirectoryStatistics {
    files: usize,
    directories: usize,
    total_size: u64,
}

fn example8_statistics() -> io::Result<()> {
    // 创建测试目录
    fs::create_dir_all("test_stats/dir1/dir2")?;
    fs::write("test_stats/file1.txt", "a")?;
    fs::write("test_stats/file2.txt", "ab")?;
    fs::write("test_stats/dir1/file3.txt", "abc")?;

    let stats = get_directory_statistics("test_stats")?;
    println!("目录统计:");
    println!("  文件数: {}", stats.files);
    println!("  目录数: {}", stats.directories);
    println!("  总大小: {} 字节", stats.total_size);

    // 清理
    fs::remove_dir_all("test_stats")?;
    Ok(())
}

fn get_directory_statistics(path: &str) -> io::Result<DirectoryStatistics> {
    let mut stats = DirectoryStatistics {
        files: 0,
        directories: 0,
        total_size: 0,
    };

    let path = Path::new(path);

    if path.is_dir() {
        stats.directories += 1;
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let entry_path = entry.path();

            if entry_path.is_dir() {
                let child_stats = get_directory_statistics(entry_path.to_str().unwrap())?;
                stats.files += child_stats.files;
                stats.directories += child_stats.directories;
                stats.total_size += child_stats.total_size;
            } else {
                stats.files += 1;
                if let Ok(metadata) = fs::metadata(&entry_path) {
                    stats.total_size += metadata.len();
                }
            }
        }
    }

    Ok(stats)
}

// ============================================================================
// 示例 9: 并行遍历（概念演示）
// ============================================================================
fn example9_parallel_concept() {
    // 真正的并行遍历需要使用 rayon 等库
    // 这里只演示概念

    println!("并行遍历概念:");
    println!("  walkdir 支持并行遍历，使用:");
    println!("    WalkDir::new(\".\").into_iter().parallel()");
    println!("  需要 walkdir 的并行功能");
    println!("  或者使用 rayon 并行处理目录项");
}

// ============================================================================
// 示例 10: 处理软链接
// ============================================================================
fn example10_symbolic_links() -> io::Result<()> {
    // Windows 需要管理员权限创建符号链接
    #[cfg(unix)]
    {
        // 创建测试目录和文件
        fs::create_dir_all("test_symlinks")?;
        fs::write("test_symlinks/file.txt", "test")?;

        // 创建符号链接
        std::os::unix::fs::symlink("test_symlinks/file.txt", "test_symlinks/link.txt")?;

        println!("处理符号链接:");
        let link = Path::new("test_symlinks/link.txt");
        println!("  是符号链接: {}", link.is_symlink());
        println!("  链接目标: {:?}", fs::read_link(link)?);

        // 清理
        fs::remove_file("test_symlinks/link.txt")?;
        fs::remove_dir_all("test_symlinks")?;
    }

    #[cfg(windows)]
    {
        println!("Windows 系统需要管理员权限创建符号链接");
    }

    Ok(())
}

// ============================================================================
// 示例 11: 按修改时间排序
// ============================================================================
fn example11_sort_by_modified_time() -> io::Result<()> {
    // 创建测试文件
    fs::create_dir_all("test_sort")?;
    for i in 1..=3 {
        fs::write(format!("test_sort/file{}.txt", i), format!("content{}", i))?;
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    println!("按修改时间排序:");
    let mut entries: Vec<_> = fs::read_dir("test_sort")?.collect::<Result<Vec<_>, _>>()?;

    entries.sort_by_key(|entry| {
        entry
            .metadata()
            .and_then(|m| m.modified())
            .unwrap_or(std::time::SystemTime::UNIX_EPOCH)
    });

    for entry in entries {
        println!("  {:?}", entry.file_name());
    }

    // 清理
    fs::remove_dir_all("test_sort")?;
    Ok(())
}

// ============================================================================
// 示例 12: 跳过特定目录（如 .git, node_modules）
// ============================================================================
fn example12_skip_directories() -> io::Result<()> {
    // 创建测试目录
    fs::create_dir_all("test_skip/.git")?;
    fs::create_dir_all("test_skip/node_modules")?;
    fs::create_dir_all("test_skip/src")?;
    fs::write("test_skip/file.txt", "test")?;

    println!("跳过特定目录的遍历:");
    for entry in custom_walkdir_skip("test_skip", 10, &[".git", "node_modules"])? {
        println!(
            "  {} {:?}",
            "  ".repeat(entry.depth),
            entry.file_name().unwrap()
        );
    }

    // 清理
    fs::remove_dir_all("test_skip")?;
    Ok(())
}

fn custom_walkdir_skip(
    root: &str,
    max_depth: usize,
    skip_dirs: &[&str],
) -> io::Result<Vec<WalkEntry>> {
    let mut entries = Vec::new();
    walk_recursive_skip(root, 0, max_depth, skip_dirs, &mut entries)?;
    Ok(entries)
}

fn walk_recursive_skip(
    path: &str,
    depth: usize,
    max_depth: usize,
    skip_dirs: &[&str],
    entries: &mut Vec<WalkEntry>,
) -> io::Result<()> {
    let path_buf = std::path::PathBuf::from(path);

    // 检查是否应该跳过
    if let Some(name) = path_buf.file_name() {
        if skip_dirs.iter().any(|dir| name == *dir) {
            return Ok(());
        }
    }

    entries.push(WalkEntry {
        path: path_buf.clone(),
        depth,
    });

    if depth < max_depth {
        if let Ok(dir_entries) = fs::read_dir(&path_buf) {
            for entry in dir_entries {
                let entry = entry?;
                let entry_path = entry.path();

                if entry_path.is_dir() {
                    walk_recursive_skip(
                        entry_path.to_str().unwrap(),
                        depth + 1,
                        max_depth,
                        skip_dirs,
                        entries,
                    )?;
                } else {
                    entries.push(WalkEntry {
                        path: entry_path,
                        depth: depth + 1,
                    });
                }
            }
        }
    }

    Ok(())
}

// ============================================================================
// 示例 13: 查找大文件
// ============================================================================
fn example13_find_large_files() -> io::Result<()> {
    // 创建测试文件
    fs::create_dir_all("test_large")?;
    fs::write("test_large/small.txt", "small")?;
    fs::write("test_large/medium.txt", "medium content")?;
    fs::write("test_large/large.txt", "large content here!")?;

    println!("查找大于 10 字节的文件:");
    for entry in find_large_files("test_large", 10)? {
        println!("  {:?}", entry);
    }

    // 清理
    fs::remove_dir_all("test_large")?;
    Ok(())
}

fn find_large_files(root: &str, threshold: u64) -> io::Result<Vec<String>> {
    let mut large_files = Vec::new();
    let path = Path::new(root);

    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let entry_path = entry.path();

            if entry_path.is_file() {
                if let Ok(metadata) = fs::metadata(&entry_path) {
                    if metadata.len() > threshold {
                        large_files.push(entry_path.to_str().unwrap().to_string());
                    }
                }
            } else if entry_path.is_dir() {
                large_files.extend(find_large_files(entry_path.to_str().unwrap(), threshold)?);
            }
        }
    }

    Ok(large_files)
}

// ============================================================================
// 示例 14: 复制整个目录树
// ============================================================================
fn example14_copy_tree() -> io::Result<()> {
    // 创建测试目录
    fs::create_dir_all("test_copy_tree/src")?;
    fs::write("test_copy_tree/file1.txt", "content1")?;
    fs::write("test_copy_tree/src/file2.txt", "content2")?;

    println!("复制目录树:");
    copy_tree("test_copy_tree", "test_copy_tree_dst")?;

    println!("目标目录内容:");
    for entry in custom_walkdir("test_copy_tree_dst", 10)? {
        println!(
            "  {} {:?}",
            "  ".repeat(entry.depth),
            entry.file_name().unwrap()
        );
    }

    // 清理
    fs::remove_dir_all("test_copy_tree")?;
    fs::remove_dir_all("test_copy_tree_dst")?;
    Ok(())
}

fn copy_tree(src: &str, dst: &str) -> io::Result<()> {
    let src_path = Path::new(src);
    let dst_path = Path::new(dst);

    if src_path.is_dir() {
        fs::create_dir_all(dst_path)?;

        for entry in fs::read_dir(src_path)? {
            let entry = entry?;
            let entry_name = entry.file_name();
            let src_child = src_path.join(&entry_name);
            let dst_child = dst_path.join(&entry_name);
            copy_tree(src_child.to_str().unwrap(), dst_child.to_str().unwrap())?;
        }
    } else {
        fs::copy(src_path, dst_path)?;
    }

    Ok(())
}

// ============================================================================
// 示例 15: 实际应用 - 代码文件统计
// ============================================================================
fn example15_code_statistics() -> io::Result<()> {
    // 创建测试项目
    fs::create_dir_all("test_project/src")?;
    fs::write("test_project/Cargo.toml", "[package]")?;
    fs::write("test_project/src/main.rs", "fn main() {}")?;
    fs::write("test_project/src/lib.rs", "pub fn hello() {}")?;
    fs::write("test_project/README.md", "# Test Project")?;

    println!("代码文件统计:");
    let stats = get_code_statistics("test_project")?;
    println!("  Rust 文件: {} 个", stats.rs_files);
    println!("  Markdown 文件: {} 个", stats.md_files);
    println!("  其他文件: {} 个", stats.other_files);

    // 清理
    fs::remove_dir_all("test_project")?;
    Ok(())
}

struct CodeStats {
    rs_files: usize,
    md_files: usize,
    other_files: usize,
}

fn get_code_statistics(root: &str) -> io::Result<CodeStats> {
    let mut stats = CodeStats {
        rs_files: 0,
        md_files: 0,
        other_files: 0,
    };

    let path = Path::new(root);

    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let entry_path = entry.path();

            if entry_path.is_dir() {
                let child_stats = get_code_statistics(entry_path.to_str().unwrap())?;
                stats.rs_files += child_stats.rs_files;
                stats.md_files += child_stats.md_files;
                stats.other_files += child_stats.other_files;
            } else if let Some(ext) = entry_path.extension() {
                match ext.to_str().unwrap() {
                    "rs" => stats.rs_files += 1,
                    "md" => stats.md_files += 1,
                    _ => stats.other_files += 1,
                }
            } else {
                stats.other_files += 1;
            }
        }
    }

    Ok(stats)
}

// ============================================================================
// 主函数
// ============================================================================
fn main() {
    println!("=== Rust 目录遍历 (Walkdir 风格) 示例 ===\n");

    println!("示例 1: 基本用法 (需要 walkdir crate)");
    println!("  参考代码注释中的 example1_basic_walkdir");
    println!();

    println!("示例 2: 使用标准库递归遍历");
    example2_std_traversal().unwrap();
    println!();

    println!("示例 3: 自定义 WalkDir 实现");
    example3_custom_walkdir().unwrap();
    println!();

    println!("示例 4: 按文件类型过滤");
    example4_filter_by_type().unwrap();
    println!();

    println!("示例 5: 按深度限制");
    example5_depth_limit().unwrap();
    println!();

    println!("示例 6: 获取目录大小");
    example6_directory_size().unwrap();
    println!();

    println!("示例 7: 查找特定文件");
    example7_find_specific_file().unwrap();
    println!();

    println!("示例 8: 统计文件和目录数量");
    example8_statistics().unwrap();
    println!();

    println!("示例 9: 并行遍历概念");
    example9_parallel_concept();
    println!();

    println!("示例 10: 处理软链接");
    example10_symbolic_links().unwrap();
    println!();

    println!("示例 11: 按修改时间排序");
    example11_sort_by_modified_time().unwrap();
    println!();

    println!("示例 12: 跳过特定目录");
    example12_skip_directories().unwrap();
    println!();

    println!("示例 13: 查找大文件");
    example13_find_large_files().unwrap();
    println!();

    println!("示例 14: 复制整个目录树");
    example14_copy_tree().unwrap();
    println!();

    println!("示例 15: 实际应用 - 代码文件统计");
    example15_code_statistics().unwrap();

    println!("\n=== 总结 ===");
    println!("目录遍历特点:");
    println!("  - walkdir crate 提供高效遍历");
    println!("  - 标准库 fs 也可以实现遍历");
    println!("  - 支持深度限制、文件过滤");
    println!("  - 可以跳过特定目录（.git, node_modules）");
    println!("  - 支持符号链接处理");
    println!("  - 可以并行遍历（使用 rayon）");
    println!("  - 适用于代码分析、文件搜索等场景");
    println!("\n使用 walkdir crate:");
    println!("  1. 在 Cargo.toml 添加: walkdir = \"2.3\"");
    println!("  2. use walkdir::WalkDir;");
    println!("  3. WalkDir::new(\"path\").into_iter()");
    println!("  4. 配置: max_depth, min_depth, follow_links 等");
}
