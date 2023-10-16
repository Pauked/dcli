use std::{env, fs::File, path::Path};

use eyre::Context;
use plist::Value;
use powershell_script::PsScriptBuilder;
use regex::Regex;

use crate::{constants, data, paths};

fn run_powershell_cmd(powershell_cmd: &str) -> Result<Vec<String>, eyre::Report> {
    if env::consts::OS != constants::OS_WINDOWS {
        return Err(eyre::eyre!(format!(
            "PowerShell is only supported on Windows, not on '{}'",
            env::consts::OS
        )));
    }

    let ps = PsScriptBuilder::new()
        .no_profile(true)
        .non_interactive(true)
        .hidden(false)
        .print_commands(false)
        .build();

    let output = ps.run(powershell_cmd).wrap_err(format!(
        "Failed to run powershell command '{}'",
        powershell_cmd
    ))?;

    let stdout_result = &output.stdout();
    match stdout_result {
        None => Err(eyre::eyre!(format!(
            "No stdout from PowerShell, command was '{}'",
            powershell_cmd
        ),)),
        Some(stdout_text) => Ok(stdout_text.split("\r\n").map(|s| s.to_string()).collect()),
    }
}

fn get_property_from_stdout(stdout_strings: Vec<String>, property_name: &str) -> String {
    let binding = "".to_string();
    let property = stdout_strings
        .iter()
        .find(|s| s.starts_with(property_name))
        .unwrap_or(&binding);
    let binding = property.replace(property_name, "");
    let property_value = binding.trim();
    property_value.to_string()
}

fn get_windows_file_version_information(
    full_path: &str,
) -> Result<(String, String, String, String), eyre::Report> {
    let stdout_result = run_powershell_cmd(&format!(
        r#"(Get-Item "{}").VersionInfo.FileVersionRaw | Format-List -Property Major, Minor, Build, Revision"#,
        full_path
    ));

    match stdout_result {
        Ok(stdout_strings) => {
            let major = get_property_from_stdout(stdout_strings.clone(), "Major    : ");
            let minor = get_property_from_stdout(stdout_strings.clone(), "Minor    : ");
            let build = get_property_from_stdout(stdout_strings.clone(), "Build    : ");
            let revision = get_property_from_stdout(stdout_strings, "Revision : ");

            Ok((major, minor, build, revision))
        }
        Err(e) => Err(e),
    }
}

fn get_windows_file_description_information(full_path: &str) -> Result<String, eyre::Report> {
    let stdout_result = run_powershell_cmd(&format!(
        r#"(Get-Item "{}").VersionInfo | Format-List -Property FileDescription, ProductName"#,
        full_path
    ));

    match stdout_result {
        Ok(stdout_strings) => {
            let app_name = get_property_from_stdout(stdout_strings.clone(), "FileDescription :");
            if !app_name.is_empty() {
                return Ok(app_name);
            };
            let app_name = get_property_from_stdout(stdout_strings, "ProductName     :");
            if !app_name.is_empty() {
                return Ok(app_name);
            };
            Ok(paths::extract_file_name(full_path).to_string())
        }
        Err(e) => Err(e),
    }
}

fn get_macos_file_version_information(full_path: &str) -> Result<(String, String), eyre::Report> {
    // Construct path to Info.plist
    let plist_path = Path::new(full_path).join("Contents").join("Info.plist");

    // Open the plist file
    let file = File::open(plist_path)?;

    // Parse the plist file
    let value: Value = plist::from_reader(file)?;

    // Extract the version information from the plist
    let version = value
        .as_dictionary()
        .and_then(|dict| dict.get("CFBundleShortVersionString"))
        .and_then(|title| title.as_string());

    let version_str = version.unwrap_or("");

    let app_name = value
        .as_dictionary()
        .and_then(|dict| dict.get("CFBundleName"))
        .and_then(|title| title.as_string());

    let app_name_str = app_name.unwrap_or("");

    Ok((version_str.to_string(), app_name_str.to_string()))
}

fn parse_file_version_to_u32(version: &str) -> Option<(String, u32, u32, u32, Option<u32>, String)> {
    // Split the version string into parts separated by '.'
    let mut parts: Vec<_> = version
        .split('.')
        .map(|part| part.to_string())
        .collect();

    // Extract the prefix and suffix from the first and last parts
    let mut prefix = String::new();
    let mut suffix = String::new();

    // Does the major number contain a prefix?
    if let Some(index) = parts[0].chars().position(|c| c.is_ascii_digit()) {
        let (pre, post) = parts[0].split_at(index);
        prefix = pre.to_string();
        parts[0] = post.to_string(); // update the parts array with just the number
    }

    // Does the build/revision contain a suffix?
    if let Some(index) = parts.last().unwrap().chars().position(|c| c.is_alphabetic()) {
        let (pre, post) = parts.last_mut().unwrap().split_at(index);
        suffix = post.to_string();
        *parts.last_mut().unwrap() = pre.to_string(); // update the parts array with just the number
    }

    // Ensure we have at least three parts (major, minor, and build)
    if parts.len() < 3 {
        return None;
    }

    // Extract numeric values from parts
    let major = parts[0].parse::<u32>().unwrap_or(0);
    let minor = parts[1].parse::<u32>().unwrap_or(0);
    let build = parts[2].parse::<u32>().unwrap_or(0);

    // Extract the revision number if it exists
    let revision = if parts.len() == 4 {
        Some(parts[3].parse::<u32>().unwrap_or(0))
    } else {
        None
    };

    Some((prefix, major, minor, build, revision, suffix))
}


pub fn get_file_version(full_path: &str) -> Result<data::FileVersion, eyre::Report> {
    if env::consts::OS == constants::OS_WINDOWS {
        let (major, minor, build, revision) = get_windows_file_version_information(full_path)?;
        let app_name = get_windows_file_description_information(full_path)?;

        return Ok(data::FileVersion {
            app_name: app_name.to_string(),
            path: full_path.to_string(),
            prefix: "".to_string(),
            major: major.parse::<u32>().unwrap_or(0),
            minor: minor.parse::<u32>().unwrap_or(0),
            build: build.parse::<u32>().unwrap_or(0),
            revision: Some(revision.parse::<u32>().unwrap_or(0)),
            suffix: "".to_string(),
        });
    }

    if env::consts::OS == constants::OS_MACOS {
        let (version, app_name) = get_macos_file_version_information(full_path)?;

        match parse_file_version_to_u32(&version) {
            Some((prefix, major, minor, build, revision, suffix)) => {
                return Ok(data::FileVersion {
                    app_name,
                    path: full_path.to_string(),
                    prefix,
                    major,
                    minor,
                    build,
                    revision,
                    suffix,
                });
            }
            None => {
                return Err(eyre::eyre!(format!(
                    "Failed to parse the version string - '{}",
                    version,
                )))
            }
        }
    }

    Err(eyre::eyre!(format!(
        "get_file_version is only supported on Windows and macOS, not on '{}'",
        env::consts::OS
    )))
}

pub fn get_prboom_file_version(full_path: &str) -> Result<data::FileVersion, eyre::Report> {
    if env::consts::OS != constants::OS_WINDOWS {
        return Err(eyre::eyre!(format!(
            "prboom_file_version is only supported on Windows, not on '{}'",
            env::consts::OS
        )));
    }

    let stdout_result = run_powershell_cmd(&format!(r#"{} -v"#, full_path));
    match stdout_result {
        Ok(stdout_strings) => {
            let input = stdout_strings[0].clone();

            // Create a regular expression to match the version pattern
            let re = Regex::new(r"v(\d+)\.(\d+)\.(\d+)").unwrap();

            // Use the regex to capture major, minor, and patch versions
            if let Some(captures) = re.captures(&input) {
                let major = captures[1].parse::<u32>().unwrap_or(0);
                let minor = captures[2].parse::<u32>().unwrap_or(0);
                let build = captures[3].parse::<u32>().unwrap_or(0);

                // Extract the app name (assuming it's the part before " v")
                let app_name = input.split(" v").next().unwrap();

                Ok(data::FileVersion {
                    app_name: app_name.to_string(),
                    path: full_path.to_string(),
                    prefix: "".to_string(),
                    major,
                    minor,
                    build,
                    revision: None,
                    suffix: "".to_string(),
                })
            } else {
                Err(eyre::eyre!(format!(
                    "Unable find version in output from '{}",
                    input,
                )))
            }
        }
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod tests {
    use crate::finder::parse_file_version_to_u32;

    #[test]
    fn test_parse_file_version_to_u32_happy_path_simple_number() {
        // Arrange
        let version = "5.0.4.1";
        let expected = ("".to_string(), 5, 0, 4, Some(1), "".to_string());

        // Act
        let actual = parse_file_version_to_u32(version);

        // Assert
        assert!(actual.is_some());
        let actual_unwrapped = actual.unwrap();
        assert_eq!(actual_unwrapped, expected);
    }

    #[test]
    fn test_parse_file_version_to_u32_happy_path_gzdoom_number() {
        // Arrange
        let version = "g4.11.1a";
        let expected = ("g".to_string(), 4, 11, 1, None, "a".to_string());

        // Act
        let actual = parse_file_version_to_u32(version);

        // Assert
        assert!(actual.is_some());
        let actual_unwrapped = actual.unwrap();
        assert_eq!(actual_unwrapped, expected);
    }

    #[test]
    fn test_parse_file_version_to_u32_happy_path_prboom_number() {
        // Arrange
        let version = "0.26.2";
        let expected = ("".to_string(), 0, 26, 2, None, "".to_string());

        // Act
        let actual = parse_file_version_to_u32(version);

        // Assert
        assert!(actual.is_some());
        let actual_unwrapped = actual.unwrap();
        assert_eq!(actual_unwrapped, expected);
    }

    #[test]
    fn test_parse_file_version_to_u32_happy_path_full_number() {
        // Arrange
        let version = "a5.4.3.2z";
        let expected = ("a".to_string(), 5, 4, 3, Some(2), "z".to_string());

        // Act
        let actual = parse_file_version_to_u32(version);

        // Assert
        assert!(actual.is_some());
        let actual_unwrapped = actual.unwrap();
        assert_eq!(actual_unwrapped, expected);
    }
}
