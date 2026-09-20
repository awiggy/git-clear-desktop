//! Wiring checks for the Clear edition release stream (`clear-v*` tags). The updater only
//! enables self-update when the build was compiled with the Clear update source, and the
//! installers must emit the asset names the updater's Clear stem matches.

#[test]
fn clear_tag_trigger_and_update_source_are_wired_into_the_build_workflow() {
    let workflow = include_str!("../.github/workflows/build.yml");

    // The Clear stream publishes from its own tag prefix.
    assert!(workflow.contains("- \"clear-v*\""));
    // The version check must strip the edition prefix before comparing with Cargo.toml.
    assert!(workflow.contains("-replace '^clear-v',''"));

    // Only Clear release tags are published from this repository.
    assert!(!workflow.contains("- \"v*\""));
    assert!(workflow.contains("startsWith(github.ref_name, 'clear-v')"));
    assert!(workflow.contains("GIT_AGENT_SELF_UPDATE=1"));
    assert!(workflow.contains("GIT_AGENT_UPDATE_TAG_PREFIX=clear-v"));
    assert!(workflow.contains("GIT_AGENT_UPDATE_ASSET_STEM=GitAgent-Clear"));
    // The Clear update feed lives in the edition's own repository, never the upstream one.
    assert!(workflow.contains("GIT_AGENT_UPDATE_RELEASES_API=https://api.github.com/repos/awiggy/git-clear-desktop/releases"));
    assert!(
        workflow.contains("GIT_AGENT_UPDATE_DOWNLOAD_PREFIX=https://github.com/awiggy/git-clear-desktop/releases/download/")
    );

    // The updater looks for these exact Clear asset names.
    assert!(workflow.contains("dist/GitAgent-Clear_*.deb"));
    assert!(workflow.contains("dist/GitAgent-ClearSetup-*.exe"));
    assert!(workflow.contains("/DOutputName=GitAgent-ClearSetup-"));

    // Clear releases get their own title.
    assert!(workflow.contains("Git Agent Clear $TAG_NAME"));

    // Tests must run against the safe default configuration, so the Clear update source
    // is injected only after the test steps, before the release builds.
    let test_step = workflow.find("- name: Test").unwrap();
    let configure_step = workflow
        .find("- name: Configure Clear edition update source")
        .unwrap();
    let build_step = workflow.find("- name: Build release binaries").unwrap();
    assert!(test_step < configure_step && configure_step < build_step);
}

#[test]
fn linux_packaging_supports_a_clear_edition_variant() {
    let script = include_str!("../installer/linux/package-deb.sh");

    assert!(script.contains("[edition]"));
    assert!(script.contains("clear)"));
    assert!(script.contains("package_name=\"git-agent-clear\""));
    assert!(script.contains("asset_stem=\"GitAgent-Clear\""));
    assert!(script.contains("${asset_stem}_${version}_${architecture}.deb"));
    // The edition version strip keeps clear-v1.3.15 -> 1.3.15.
    assert!(script.contains("version=\"${version#clear-v}\""));
    // Manual builds use the same identity as release builds.
    assert!(script.contains("edition=\"${5:-clear}\""));
}

#[test]
fn windows_installer_accepts_clear_edition_overrides() {
    let installer = include_str!("../installer/windows/git-agent.iss");

    // Defaults match the published Clear installer, including its upgrade identity.
    assert!(installer.contains("#define AppName \"Git Agent Clear\""));
    assert!(installer.contains("#define OutputName \"GitAgent-ClearSetup-\""));
    assert!(installer.contains("#define InstallDirName \"GitAgentClear\""));
    assert!(installer.contains("#define AppGuid \"9C4E7A21-5B3D-4E6F-8A1C-2D0E4F6A7B8C\""));
    // Every identity knob can be overridden from the CI command line.
    assert!(installer.contains("#ifndef AppGuid"));
    assert!(installer.contains("AppId={{{#AppGuid}}}"));
    assert!(installer.contains("DefaultDirName={localappdata}\\Programs\\{#InstallDirName}"));
    assert!(installer.contains("OutputBaseFilename={#OutputName}{#AppVersion}"));
}

#[test]
fn macos_packaging_strips_the_clear_tag_prefix_from_versions() {
    let script = include_str!("../installer/macos/package.sh");

    assert!(script.contains("version=\"${version#clear-v}\""));
    assert!(script.contains("app_name=\"Git Agent Clear\""));
    assert!(script.contains("io.github.awiggy.git-clear"));
}
