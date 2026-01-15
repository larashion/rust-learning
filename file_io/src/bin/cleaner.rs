use std::fs;
use std::path::Path;
use walkdir::WalkDir;

fn main() {
    // 实际运行时，你可以传入命令行参数，这里为了演示直接写当前目录
    let path = Path::new(".");
    if let Err(e) = clean_empty_directories(path) {
        eprintln!("Error: {}", e);
    }
}

/// 核心业务逻辑：递归清理空目录
/// 参数：target_root - 要清理的目标根目录
pub fn clean_empty_directories(target_root: &Path) -> std::io::Result<()> {
    // contents_first(true) 是关键：后序遍历（先子后父）
    let walker = WalkDir::new(target_root).contents_first(true);

    for entry in walker.into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();

        // 1. 必须是目录
        // 2. 不能是根目录自己（通常我们不想删掉用户指定的那个顶层文件夹，防止误删挂载点）
        if path.is_dir() && path != target_root {
            match fs::remove_dir(path) {
                Ok(_) => println!("[Deleted] {:?}", path),
                Err(e) => {
                    // 忽略“非空”错误，这是预期行为
                    if !is_dir_not_empty_error(&e) {
                        eprintln!("[Error] Cannot delete {:?}: {}", path, e);
                    }
                }
            }
        }
    }
    Ok(())
}

/// 辅助函数：判断错误是否为“文件夹非空”
fn is_dir_not_empty_error(e: &std::io::Error) -> bool {
    // Windows: 145, Unix-like: DirectoryNotEmpty
    e.kind() == std::io::ErrorKind::DirectoryNotEmpty
        || (cfg!(windows) && e.raw_os_error() == Some(145))
}

// ==========================================
//            以下是测试代码
// ==========================================
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;

    // 一个辅助函数，用来创建测试环境
    fn setup_test_environment(root: &str) {
        // 结构设计：
        // root/
        // ├── empty_chain_1/     (空)
        // │   └── empty_chain_2/ (空) -> 应该被连坐删除
        // ├── keep_me/
        // │   └── data.txt       (文件) -> 应该保留 keep_me
        // └── mixed/
        //     ├── trash/         (空)   -> 应该被删除
        //     └── treasure.txt   (文件) -> 应该导致 mixed 被保留

        let root_path = Path::new(root);
        if root_path.exists() {
            fs::remove_dir_all(root_path).unwrap(); // 清理旧环境
        }
        fs::create_dir_all(root_path.join("empty_chain_1/empty_chain_2")).unwrap();
        fs::create_dir_all(root_path.join("keep_me")).unwrap();
        fs::create_dir_all(root_path.join("mixed/trash")).unwrap();

        // 创建文件
        let mut f1 = File::create(root_path.join("keep_me/data.txt")).unwrap();
        f1.write_all(b"content").unwrap();

        let mut f2 = File::create(root_path.join("mixed/treasure.txt")).unwrap();
        f2.write_all(b"gold").unwrap();
    }

    #[test]
    fn test_clean_empty_dirs_logic() {
        let test_root = "test_env_temp";
        setup_test_environment(test_root);
        let root_path = Path::new(test_root);

        // --- 执行前断言 ---
        assert!(
            root_path.join("empty_chain_1/empty_chain_2").exists(),
            "Setup 失败"
        );
        assert!(root_path.join("mixed/trash").exists(), "Setup 失败");

        // --- 执行逻辑 ---
        clean_empty_directories(root_path).expect("执行失败");

        // --- 执行后验证 (Assert) ---

        // 1. 验证连锁删除：最底层的空文件夹没了
        assert!(
            !root_path.join("empty_chain_1/empty_chain_2").exists(),
            "底层空文件夹未删除"
        );
        // 2. 验证连锁删除：父级变空后也该没了 (后序遍历的威力)
        assert!(
            !root_path.join("empty_chain_1").exists(),
            "父级空文件夹未被连锁删除"
        );

        // 3. 验证非空保护：有文件的文件夹还在
        assert!(root_path.join("keep_me").exists(), "包含文件的文件夹误删");
        assert!(root_path.join("keep_me/data.txt").exists(), "文件误删");

        // 4. 验证混合情况：删了空的子目录，但保留了有内容的父目录
        assert!(
            !root_path.join("mixed/trash").exists(),
            "混合目录下的空文件夹未删除"
        );
        assert!(root_path.join("mixed").exists(), "混合目录被误删");
        assert!(
            root_path.join("mixed/treasure.txt").exists(),
            "混合目录下的文件被误删"
        );

        // 清理测试现场
        fs::remove_dir_all(root_path).unwrap();
    }
}
