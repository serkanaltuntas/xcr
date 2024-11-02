use std::path::{Path, PathBuf};
use std::fs;
use std::io::{Read, Write};
use std::process::Command;
use chrono::Local;
use log::{info, warn, error, LevelFilter};
use log4rs::{
    append::file::FileAppender,
    append::console::ConsoleAppender,
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
};
use glob::glob;
use anyhow::{Context, Result, anyhow};
use clap::Parser;

#[derive(Parser)]
#[command(
    name = "xcr",
    author,
    version,
    about,
    long_about = "A tool for renaming Xcode projects and updating all references within the project"
)]
struct Cli {
    #[arg(help = "Path to the Xcode project directory")]
    project_path: String,

    #[arg(help = "New name for the project")]
    new_name: String,

    #[arg(long, help = "New bundle identifier (optional)")]
    bundle_id: Option<String>,
}

#[derive(Debug)]
struct XcrConfig {
    project_path: PathBuf,
    old_name: String,
    new_name: String,
    old_bundle_id: Option<String>,
    new_bundle_id: Option<String>,
    new_project_path: PathBuf,
}

impl XcrConfig {
    fn new(project_path: String, new_name: String, new_bundle_id: Option<String>) -> Result<Self> {
        let project_path = PathBuf::from(project_path).canonicalize()
            .context("Failed to resolve project path")?;
        let old_name = project_path.file_name()
            .ok_or_else(|| anyhow!("Invalid project path"))?
            .to_string_lossy()
            .into_owned();
        let new_project_path = project_path.parent()
            .ok_or_else(|| anyhow!("Project path has no parent directory"))?
            .join(&new_name);

        Ok(Self {
            project_path,
            old_name,
            new_name,
            old_bundle_id: None,
            new_bundle_id,
            new_project_path,
        })
    }
}

struct Xcr {
    config: XcrConfig,
}

impl Xcr {
    fn new(config: XcrConfig) -> Self {
        Self { config }
    }

    fn setup_logging() -> Result<()> {
        let log_file = format!("xcr_{}.log",
                               Local::now().format("%Y%m%d_%H%M%S"));

        let file_appender = FileAppender::builder()
            .encoder(Box::new(PatternEncoder::new("{d} - {l} - {m}{n}")))
            .build(&log_file)
            .context("Failed to create file appender")?;

        let console_appender = ConsoleAppender::builder()
            .encoder(Box::new(PatternEncoder::new("{d} - {l} - {m}{n}")))
            .build();

        let config = Config::builder()
            .appender(Appender::builder().build("file", Box::new(file_appender)))
            .appender(Appender::builder().build("console", Box::new(console_appender)))
            .build(Root::builder()
                .appender("file")
                .appender("console")
                .build(LevelFilter::Info))
            .context("Failed to build logging config")?;

        log4rs::init_config(config).context("Failed to initialize logging")?;
        Ok(())
    }

    fn validate_project(&self) -> Result<()> {
        let xcodeproj = glob(&format!("{}/*.xcodeproj", self.config.project_path.display()))
            .context("Failed to search for xcodeproj")?
            .next()
            .ok_or_else(|| anyhow!("No .xcodeproj found in {}", self.config.project_path.display()))?
            .context("Failed to read xcodeproj path")?;

        info!("Found Xcode project: {}", xcodeproj.display());
        Ok(())
    }

    fn find_bundle_identifier(&mut self) -> Result<()> {
        let pbxproj_pattern = format!("{}/*.xcodeproj/project.pbxproj", self.config.project_path.display());
        let pbxproj_path = glob(&pbxproj_pattern)
            .context("Failed to search for project.pbxproj")?
            .next()
            .ok_or_else(|| anyhow!("Could not find project.pbxproj"))?
            .context("Failed to read project.pbxproj path")?;

        info!("Searching for bundle identifier in {}", pbxproj_path.display());

        let content = fs::read_to_string(&pbxproj_path)
            .context("Failed to read project.pbxproj")?;

        self.config.old_bundle_id = content.lines()
            .find(|line| line.contains("PRODUCT_BUNDLE_IDENTIFIER"))
            .and_then(|line| {
                line.split('=')
                    .last()
                    .map(|s| s.trim().trim_matches(';').trim_matches('"').to_string())
            });

        if let Some(ref bundle_id) = self.config.old_bundle_id {
            info!("Found bundle identifier: {}", bundle_id);
        } else {
            warn!("Could not find existing bundle identifier");
        }

        Ok(())
    }

    fn create_new_project_copy(&self) -> Result<()> {
        if self.config.new_project_path.exists() {
            return Err(anyhow!("Target directory already exists: {}",
                self.config.new_project_path.display()));
        }

        info!("Creating new project directory at: {}", self.config.new_project_path.display());
        fs::create_dir_all(&self.config.new_project_path)
            .context("Failed to create new project directory")?;

        self.copy_directory(&self.config.project_path, &self.config.new_project_path)
    }

    fn copy_directory(&self, src: &Path, dst: &Path) -> Result<()> {
        for entry in fs::read_dir(src)
            .with_context(|| format!("Failed to read directory: {}", src.display()))? {
            let entry = entry.context("Failed to read directory entry")?;
            let path = entry.path();
            let dest_path = dst.join(path.strip_prefix(src)?);

            if path.is_dir() {
                fs::create_dir_all(&dest_path)
                    .with_context(|| format!("Failed to create directory: {}", dest_path.display()))?;
                self.copy_directory(&path, &dest_path)?;
            } else {
                fs::copy(&path, &dest_path)
                    .with_context(|| format!("Failed to copy file from {} to {}",
                                             path.display(), dest_path.display()))?;
            }
        }
        Ok(())
    }

    fn rename_files_in_new_project(&self) -> Result<()> {
        info!("Starting file renaming process...");

        let mut items_to_rename = Vec::new();
        self.collect_items_to_rename(&self.config.new_project_path, &mut items_to_rename)?;

        // Sort paths by length in descending order to rename deepest paths first
        items_to_rename.sort_by(|a, b| {
            b.to_string_lossy().len().cmp(&a.to_string_lossy().len())
        });

        for item in items_to_rename {
            if let Some(item_name) = item.file_name().and_then(|n| n.to_str()) {
                let new_name = item_name.replace(&self.config.old_name, &self.config.new_name);
                let new_path = item.with_file_name(new_name);

                if let Some(parent) = new_path.parent() {
                    fs::create_dir_all(parent)
                        .with_context(|| format!("Failed to create parent directory: {}",
                                                 parent.display()))?;
                }

                fs::rename(&item, &new_path)
                    .with_context(|| format!("Failed to rename {} to {}",
                                             item.display(), new_path.display()))?;
                info!("Renamed: {} -> {}", item.display(), new_path.display());
            }
        }

        Ok(())
    }

    fn collect_items_to_rename(&self, dir: &Path, items: &mut Vec<PathBuf>) -> Result<()> {
        for entry in fs::read_dir(dir)
            .with_context(|| format!("Failed to read directory: {}", dir.display()))? {
            let entry = entry.context("Failed to read directory entry")?;
            let path = entry.path();

            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if name.contains(&self.config.old_name) && !name.starts_with('.') {
                    items.push(path.clone());
                }
            }

            if path.is_dir() {
                self.collect_items_to_rename(&path, items)?;
            }
        }
        Ok(())
    }

    fn update_file_contents(&self) -> Result<()> {
        info!("Starting content update process...");

        let patterns = vec![
            "*.xcodeproj/project.pbxproj",
            "*.xcworkspace/contents.xcworkspacedata",
            "*.plist",
            "*.swift",
            "*.h",
            "*.m",
            "*.storyboard",
            "*.xib",
            "*.xcscheme",
        ];

        let mut files_updated = 0;

        for pattern in patterns {
            let glob_pattern = format!("{}/**/{}", self.config.new_project_path.display(), pattern);
            for entry in glob(&glob_pattern)
                .with_context(|| format!("Failed to glob pattern: {}", glob_pattern))? {
                let path = entry.context("Failed to read glob entry")?;

                if path.is_file() {
                    info!("Processing file: {}", path.display());

                    let mut content = String::new();
                    fs::File::open(&path)
                        .and_then(|mut file| file.read_to_string(&mut content))
                        .with_context(|| format!("Failed to read file: {}", path.display()))?;

                    let mut updated_content = content.replace(&self.config.old_name, &self.config.new_name);

                    if let (Some(ref old_id), Some(ref new_id)) = (&self.config.old_bundle_id, &self.config.new_bundle_id) {
                        updated_content = updated_content.replace(old_id, new_id);
                    }

                    if updated_content != content {
                        fs::File::create(&path)
                            .and_then(|mut file| file.write_all(updated_content.as_bytes()))
                            .with_context(|| format!("Failed to write file: {}", path.display()))?;
                        info!("Updated content in: {}", path.display());
                        files_updated += 1;
                    }
                }
            }
        }

        info!("Updated content in {} files", files_updated);
        Ok(())
    }

    fn update_project_settings(&self) -> Result<()> {
        if let Some(ref new_bundle_id) = self.config.new_bundle_id {
            let xcodeproj = glob(&format!("{}/*.xcodeproj", self.config.new_project_path.display()))
                .context("Failed to search for xcodeproj")?
                .next()
                .ok_or_else(|| anyhow!("No .xcodeproj found"))?
                .context("Failed to read xcodeproj path")?;

            info!("Updating project settings in {}", xcodeproj.display());

            // Set Xcode developer path
            Command::new("sudo")
                .args(["xcode-select", "-s", "/Applications/Xcode.app/Contents/Developer"])
                .status()
                .context("Failed to set Xcode developer path")?;

            info!("Set Xcode developer path successfully");

            // Update bundle identifier
            Command::new("xcodebuild")
                .args([
                    "-project",
                    xcodeproj.to_str().unwrap(),
                    &format!("PRODUCT_BUNDLE_IDENTIFIER={}", new_bundle_id),
                ])
                .status()
                .context("Failed to update bundle identifier")?;

            info!("Updated bundle identifier to: {}", new_bundle_id);
        }
        Ok(())
    }

    fn execute(&mut self) -> Result<()> {
        info!("Starting project rename process: {} -> {}",
            self.config.old_name, self.config.new_name);

        let result = (|| -> Result<()> {
            self.validate_project()?;
            self.find_bundle_identifier()?;
            self.create_new_project_copy()?;
            self.update_file_contents()?;
            self.rename_files_in_new_project()?;
            self.update_project_settings()?;
            Ok(())
        })();

        match result {
            Ok(()) => {
                info!("\nProject rename completed successfully!");
                info!("New project location: {}", self.config.new_project_path.display());
                if let Some(ref bundle_id) = self.config.new_bundle_id {
                    info!("New bundle identifier: {}", bundle_id);
                }
                Ok(())
            }
            Err(e) => {
                error!("Error during project rename: {}", e);
                if self.config.new_project_path.exists() {
                    info!("Cleaning up new project directory...");
                    fs::remove_dir_all(&self.config.new_project_path)
                        .context("Failed to clean up new project directory")?;
                }
                Err(e)
            }
        }
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    Xcr::setup_logging()?;

    let config = XcrConfig::new(
        cli.project_path,
        cli.new_name,
        cli.bundle_id,
    )?;

    let mut xcr = Xcr::new(config);
    xcr.execute()
}