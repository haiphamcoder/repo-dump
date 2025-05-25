use serde::Deserialize;

pub const MAX_FILE_SIZE: u64 = 1024 * 1024 * 10; // 10MB

pub const LOGO: &str = "
██████╗ ███████╗██████╗  ██████╗     ██████╗ ██╗   ██╗███╗   ███╗██████╗ 
██╔══██╗██╔════╝██╔══██╗██╔═══██╗    ██╔══██╗██║   ██║████╗ ████║██╔══██╗
██████╔╝█████╗  ██████╔╝██║   ██║    ██║  ██║██║   ██║██╔████╔██║██████╔╝
██╔══██╗██╔══╝  ██╔═══╝ ██║   ██║    ██║  ██║██║   ██║██║╚██╔╝██║██╔═══╝ 
██║  ██║███████╗██║     ╚██████╔╝    ██████╔╝╚██████╔╝██║ ╚═╝ ██║██║     
╚═╝  ╚═╝╚══════╝╚═╝      ╚═════╝     ╚═════╝  ╚═════╝ ╚═╝     ╚═╝╚═╝     
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
    pub processing_files: &'static str,
    pub skip_large: &'static str,
    pub processing: &'static str,
    pub summary: &'static str,
    pub file_count: &'static str,
    pub size: &'static str,
    pub line_count: &'static str,
    pub success: &'static str,
    pub write_error: &'static str,
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
    processing_files: "📄 Processing files...",
    skip_large: "⏭️ Skipping large file: {file} ({size} bytes > {limit} bytes)",
    processing: "   📄 Processing file: {file}",
    summary: "📊 Summary",
    file_count: "   📑 File count: {count}",
    size: "   💾 Size: {size} bytes (~{kb} KB)",
    line_count: "   📊 Line count: {lines}",
    success: "✨ Success! The result is saved in: ",
    write_error: "❌ Error writing to file: {error}",

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
    processing_files: "🔍 Xử lý tệp...",
    skip_large: "⚠️ Bỏ qua tệp lớn: {file} ({size} bytes > {limit} bytes)",
    processing: "🔍 Xử lý tệp: {file}",
    summary: "📊 Tóm tắt",
    file_count: "   📑 Số lượng tệp: {count}",
    size: "   💾 Kích thước: {size} bytes (~{kb} KB)",
    line_count: "   📊 Số dòng: {lines}",
    success: "✨ Hoàn thành! Kết quả đã được lưu tại: ",
    write_error: "❌ Lỗi ghi vào tệp: {error}",
};