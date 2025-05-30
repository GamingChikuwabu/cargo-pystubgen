use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None, subcommand_negates_reqs = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Pythonのスタブファイルを生成する
    #[command(name = "pystubgen")]
    Generate {
        /// プロジェクトのルートディレクトリ
        #[arg(short, long)]
        project_dir: Option<PathBuf>,

        /// 出力ディレクトリ
        #[arg(short, long)]
        output_dir: Option<PathBuf>,

        /// デバッグモード
        #[arg(short, long)]
        debug: bool,
    },
}