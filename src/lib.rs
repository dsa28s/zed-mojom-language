// Copyright 2026 Dora Lee
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::{
    fs,
    path::{Path, PathBuf},
};

use zed_extension_api::{
    self as zed, Extension, LanguageServerId, Result, Worktree, register_extension,
    settings::LspSettings,
};

const SERVER_NAME: &str = "mojom-lsp";
const SERVER_RELEASE_REPOSITORY: &str = "dsa28s/zed-mojom-language";

struct MojomExtension {
    binary_cache: Option<PathBuf>,
}

#[derive(Clone)]
struct MojomLspBinary {
    path: PathBuf,
    args: Vec<String>,
    env: zed::EnvVars,
}

impl MojomExtension {
    fn executable_name() -> &'static str {
        if zed::current_platform().0 == zed::Os::Windows {
            "mojom-lsp.exe"
        } else {
            SERVER_NAME
        }
    }

    fn configured_binary(
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<Option<MojomLspBinary>> {
        let settings = LspSettings::for_worktree(language_server_id.as_ref(), worktree)?;

        let Some(binary) = settings.binary else {
            return Ok(None);
        };

        let args = binary.arguments.unwrap_or_default();
        let Some(path) = binary.path else {
            return Ok(None);
        };

        Ok(Some(MojomLspBinary {
            path: PathBuf::from(path),
            args,
            env: worktree.shell_env(),
        }))
    }

    fn worktree_binary(worktree: &Worktree, args: Vec<String>) -> Option<MojomLspBinary> {
        worktree
            .which(Self::executable_name())
            .or_else(|| worktree.which(SERVER_NAME))
            .map(|path| MojomLspBinary {
                path: PathBuf::from(path),
                args,
                env: worktree.shell_env(),
            })
    }

    fn release_asset_name() -> Result<(String, zed::DownloadedFileType)> {
        let (platform, architecture) = zed::current_platform();

        let architecture = match architecture {
            zed::Architecture::Aarch64 => "aarch64",
            zed::Architecture::X8664 => "x86_64",
            other => return Err(format!("unsupported Mojom LSP architecture: {other:?}")),
        };

        let (platform, archive_type, file_type) = match platform {
            zed::Os::Mac => (
                "apple-darwin",
                "tar.gz",
                zed::DownloadedFileType::GzipTar,
            ),
            zed::Os::Windows => (
                "pc-windows-msvc",
                "zip",
                zed::DownloadedFileType::Zip,
            ),
            other => return Err(format!("unsupported Mojom LSP platform: {other:?}")),
        };

        Ok((
            format!("{SERVER_NAME}-{architecture}-{platform}.{archive_type}"),
            file_type,
        ))
    }

    fn cleanup_old_versions(current_version_dir: &str) {
        let Ok(entries) = fs::read_dir(".") else {
            return;
        };

        for entry in entries.flatten() {
            let Ok(file_name) = entry.file_name().into_string() else {
                continue;
            };

            if file_name.starts_with("mojom-lsp-") && file_name != current_version_dir {
                fs::remove_dir_all(entry.path()).ok();
            }
        }
    }

    fn install_binary(
        &mut self,
        language_server_id: &LanguageServerId,
        args: Vec<String>,
    ) -> Result<MojomLspBinary> {
        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );

        let release = zed::latest_github_release(
            SERVER_RELEASE_REPOSITORY,
            zed::GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )
        .map_err(|error| format!("failed to fetch latest Mojom LSP release: {error}"))?;

        let (asset_name, file_type) = Self::release_asset_name()?;
        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == asset_name)
            .ok_or_else(|| {
                format!(
                    "no compatible Mojom LSP binary asset found for this platform; expected {asset_name}"
                )
            })?;

        let version_dir = format!("{SERVER_NAME}-{}", release.version);
        let binary_path = Path::new(&version_dir).join(Self::executable_name());

        if !binary_path.exists() {
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );

            let download_result = (|| -> Result<()> {
                zed::download_file(&asset.download_url, &version_dir, file_type)
                    .map_err(|error| format!("failed to download Mojom LSP: {error}"))?;

                if !binary_path.exists() {
                    return Err(format!(
                        "downloaded Mojom LSP archive did not contain {} at its root",
                        Self::executable_name()
                    ));
                }

                if zed::current_platform().0 != zed::Os::Windows {
                    let binary_path = binary_path
                        .to_str()
                        .ok_or_else(|| "invalid Mojom LSP binary path".to_string())?;
                    zed::make_file_executable(binary_path).map_err(|error| {
                        format!("failed to mark Mojom LSP binary executable: {error}")
                    })?;
                }

                Ok(())
            })();

            if let Err(error) = download_result {
                fs::remove_dir_all(&version_dir).ok();
                return Err(error);
            }

            Self::cleanup_old_versions(&version_dir);
        }

        self.binary_cache = Some(binary_path.clone());
        Ok(MojomLspBinary {
            path: binary_path,
            args,
            env: Default::default(),
        })
    }

    fn language_server_binary(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<MojomLspBinary> {
        if let Some(binary) = Self::configured_binary(language_server_id, worktree)? {
            return Ok(binary);
        }

        let args = Vec::new();

        if let Some(binary) = Self::worktree_binary(worktree, args.clone()) {
            return Ok(binary);
        }

        if let Some(path) = &self.binary_cache {
            if path.exists() {
                return Ok(MojomLspBinary {
                    path: path.clone(),
                    args,
                    env: Default::default(),
                });
            }
        }

        self.install_binary(language_server_id, args)
    }
}

impl Extension for MojomExtension {
    fn new() -> Self {
        Self { binary_cache: None }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<zed::Command> {
        let binary = self.language_server_binary(language_server_id, worktree)?;

        Ok(zed::Command {
            command: binary
                .path
                .to_str()
                .ok_or_else(|| "failed to convert Mojom LSP binary path to string".to_string())?
                .to_string(),
            args: binary.args,
            env: binary.env,
        })
    }
}

register_extension!(MojomExtension);
