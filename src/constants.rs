use serde::Deserialize;

pub const LOGO: &str = "
██████╗ ███████╗██████╗  ██████╗     ██████╗ ██╗   ██╗███╗   ███╗██████╗ 
██╔══██╗██╔════╝██╔══██╗██╔═══██╗    ██╔══██╗██║   ██║████╗ ████║██╔══██╗
██████╔╝█████╗  ██████╔╝██║   ██║    ██████╔╝██║   ██║██╔████╔██║██████╔╝
██╔══██╗██╔══╝  ██╔═══╝ ██║   ██║    ██╔═══╝ ██║   ██║██║╚██╔╝██║██╔═══╝ 
██║  ██║███████╗██║     ╚██████╔╝    ██║     ╚██████╔╝██║ ╚═╝ ██║██║     
╚═╝  ╚═╝╚══════╝╚═╝      ╚═════╝     ╚═╝      ╚═════╝ ╚═╝     ╚═╝╚═╝     
";

#[derive(Deserialize, Clone)]
pub struct Text {
    pub input_repo_path: &'static str,
    pub error: &'static str,
    pub done: &'static str,
    pub directory_not_found: &'static str,
    pub analyzing: &'static str,
    pub scanning: &'static str,
    pub tech_detected: &'static str,
    pub no_tech: &'static str,
    pub included_ext: &'static str,
    pub excluded_dirs: &'static str,
    pub generating_tree: &'static str,
}

pub const TEXT_EN: Text = Text {
    input_repo_path: "📂 Enter the folder path: ",
    done: "✨ Done! The source_dump.txt file is ready.",
    error: "❌ An error occurred during processing.",
    directory_not_found: "❌ Directory '{path}' not found",
    analyzing: "🔎 Analyzing project at: ",
    scanning: "🔍 Scanning directory...",
    tech_detected: "⚡ Detected technology: ",
    no_tech: "⚠️ No specific technology detected, including all code files",
    included_ext: "📋 Extensions included: ",
    excluded_dirs: "📂 Excluded directories: ",
    generating_tree: "🌳 Generating directory tree...",
};

pub const TEXT_VI: Text = Text {
    input_repo_path: "📂 Nhập đường dẫn thư mục: ",
    done: "✨ Hoàn thành! File source_dump.txt đã sẵn sàng.",
    error: "❌ Có lỗi xảy ra trong quá trình xử lý.",
    directory_not_found: "❌ Thư mục '{path}' không tồn tại",
    analyzing: "🔎 Đang phân tích dự án tại: ",
    scanning: "🔍 Đang quét thư mục...",
    tech_detected: "⚡ Phát hiện công nghệ: ",
    no_tech: "⚠️ Không phát hiện được công nghệ cụ thể, bao gồm tất cả các tệp mã",
    included_ext: "📋 Các phần mở rộng được bao gồm: ",
    excluded_dirs: "📂 Các thư mục bị loại trừ: ",
    generating_tree: "🌳 Tạo cây thư mục...",
};