use crate::constants::{
	HOME_DIR, PEN_CONFIG_FILE, PEN_DIR, PYTHON_PACKAGES_DIR, PYTHON_VERSIONS_DIR, TMP_DIR,
};
use semver::Version;
use std::{
	error::Error,
	fs,
	io::{self, Write},
	path::PathBuf,
	process,
};

// todo docstring
pub fn user_string_to_version(version: Option<&String>) -> Version {
	match version {
		Some(version) => {
			assert_major_minor_patch(version);
			match Version::parse(version) {
				Ok(version) => version,
				Err(e) => abort(&format!("Version {} is invalid.", version), Some(&e)),
			}
		}
		// TODO: Ask the user? Or maybe pick the most recent version?
		None => Version::parse("3.12.3").unwrap(),
	}
}

/// Asserts that a given version string adheres to the "major.minor.patch" format.
///
/// # Arguments
/// - `py_version`: A string slice representing the version number.
///
/// # Output
/// - None.
///
/// # Termination
/// - This function terminates if the `py_version` provided is not well formed.
///
/// # Guarantees
/// - If this function returns, it guarantees that `py_version` adheres to the format.
///
/// # Limitations
/// - The function does not validate if the given version is a valid Python version
pub fn assert_major_minor_patch(py_version: &str) {
	let parts = py_version.split('.').collect::<Vec<&str>>();

	if parts.len() != 3 {
		abort(&format!("Version {} does not match the major.minor.patch format : Version must have exactly three parts", py_version), None);
	}

	for part in parts {
		if part.parse::<u64>().is_err() {
			abort(&format!("Version {} does not match the major.minor.patch format : Each part must be a valid integer", py_version), None);
		}
	}
}

/// Prompts the user to confirm an action.
///
/// # Arguments
/// - `prompt`: A string slice containing the prompt message to display to the user.
///
/// # Output
/// - Returns `true` if the user inputs "y" or "Y"; otherwise, returns `false`.
///
/// # Termination
/// - This function may terminate due to issues with input/output streams.
pub fn confirm_action(prompt: &str) -> bool {
	println!("{}", prompt);

	// Flush stdout to ensure the prompt appears before reading input
	if let Err(e) = io::stdout().flush() {
		abort("Failed to flush standart output", Some(&e));
	}

	// Read user input
	let mut user_input = String::new();
	if let Err(e) = io::stdin().read_line(&mut user_input) {
		abort("Failed to read standart input", Some(&e));
	}

	return user_input.trim().eq_ignore_ascii_case("y");
}

/// Downloads a file from a specified URL to a given file path. If a file already exists at the specified path, it will be deleted before the new file is downloaded
///
/// # Arguments
/// - `file_url`: A string slice representing the URL of the file to download.
/// - `file_path`: A `PathBuf` specifying where to save the downloaded file.
///
/// # Output
/// - None.
///
/// # Termination
/// - The function will terminate the process if it fails to remove an existing file.
/// - It will also terminate if an error occurs during the download.
/// - Additionally, it will terminate if the downloaded file cannot be found afterward.
///
/// # Guarantees
/// - The function guarantees the downloaded file exists.
///
/// # Limitations
/// - The function does not validate the contents of the downloaded file.
pub fn download_file(file_url: &str, file_path: &PathBuf) {
	if let Err(e) = fs::remove_file(file_path) {
		if e.kind() != io::ErrorKind::NotFound {
			abort("Unable to remove file", Some(&e));
		}
	}

	let request = match minreq::get(file_url).send() {
		Ok(res) if (res.status_code == 200) => res,
		Ok(_) => abort("todo", None),
		Err(e) => abort("todo", Some(&e)),
	};

	match fs::write(file_path, request.as_bytes()) {
		Ok(()) => (),
		Err(e) => abort("todo", Some(&e)),
	}
}

/// Takes the iputted version and returns the current patch of python
///
/// # Arguments
/// - `version` : a string inputted to be checked
/// 
/// # Output 
/// - `patch` : the patch found from the provided version
///
/// #Termination
/// - An error should be thrown if the file for checking is not found, or if the version is not found.
///
/// # Guarentees
/// - If the file and version are found, the patch exists
///
/// # Limitations
/// - The function assumes that all data passed to it is in the correct format and clean
pub fn fetch_current(version: &str) {
	// get the file containing the versions
	use serde_json::Value;

	// Value could be any type that implements Deserialize!
	let result = minreq::get("https://endoflife.date/api/python.json").send().unwrap().json::<Value>().unwrap();
	//println!("User name is '{}'", user["name"]);

    //let result = minreq::get("https://endoflife.date/api/python.json").send();

	//let json: serde_json::Value = result.json();

	// match Patch::parse(patch) {
	// 	Ok(patch) => patch,
	// 	Error(e) => abort(&format!("error"), Some(&e))
	// };

	println!("{:?}", result);

	// let response = minreq::get("http://httpbin.org/anything")
    //     .with_body("Hello, world!")
    //     .send()?;

    // // httpbin.org/anything returns the body in the json field "data":
    // let json: serde_json::Value = response.json()?;
    // println!("\"Hello, world!\" == {}", json["data"]);

    // Ok(())
}
// use minreq;
// use serde::Deserialize;
// #[derive(Deserialize, Debug)]
// struct PythonVersion {
//     version: String,
//     end_of_life: String,
// }

// fn fetch_current() -> Result<(), Box<dyn std::error::Error>> {
//     let url = "https://endoflife.date/api/python.json";
    
//     // Send the GET request and deserialize the response directly into the Vec<PythonVersion>
//     let response: Vec<PythonVersion> = minreq::get(url)
//         .send()?
//         .json()?;
    
//     // Print the parsed JSON
//     for version in response {
//         println!("Version: {}, End of Life: {}", version.version, version.end_of_life);
//     }

//     Ok(())
// }

/// Checks if the specified dependencies are installed by running their `--help` command.
///
/// # Arguments
/// - `dependencies`: A vector of string slices representing the names of the dependencies to check.
///
/// # Output
/// - None.
///
/// # Termination
/// -If, for any dependencies, the `--help` command fails, this function terminates.
///
/// # Guarantees
/// - If the function returns, the dependencies are considered installed.
///
/// # Limitations
/// - The function only checks the result of the `--help` command for each dependencies.
pub fn assert_dependencies(dependencies: Vec<&'static str>) {
	for dep in dependencies {
		match process::Command::new("sh")
			.arg("-c")
			.arg(format!("command -v {}", dep))
			.stdin(process::Stdio::null())
			.stdout(process::Stdio::null())
			.stderr(process::Stdio::null())
			.status()
		{
			Ok(status) if status.success() => continue,
			Ok(_) => abort(&format!("{} is not installed", dep), None),
			Err(e) => abort(
				&format!("Failed to check if {} is installed", dep),
				Some(&e),
			),
		}
	}
}

/// Prints an error message and terminates the process.
///
/// # Input
/// - `message`: A string slice containing the error message to display.
/// - `e`: An optional `io::Error` that, if provided, will be included in the error message for additional context.
///
/// # Output
/// - This function does not return. It terminates the process with an exit status of 1.
///
/// # Termination
/// - This function always terminates.
pub fn abort(message: &str, e: Option<&dyn Error>) -> ! {
	if let Some(error) = e {
		eprintln!("Error: {}: {}", message, error);
	} else {
		eprintln!("Error: {}", message);
	}
	process::exit(1);
}

/// Prints a critical error message and terminates the process with a status code of 1. The error message is prefixed with "Catastrophic failure: " and is highlighted in bold red text to emphasize the severity.
///
/// # Arguments
/// - `message`: A string slice containing the critical error message to display.
/// - `e`: An optional `io::Error` that, if provided, will be included in the error message for additional context.
///
/// # Output
/// - This function does not return. It terminates the process with an exit status of 1.
///
/// # Termination
/// - This function always terminates.
pub fn catastrophic_failure(message: &str, e: Option<&dyn Error>) -> ! {
	const RED_BOLD: &str = "\x1b[1;31m"; // Bold red text
	const RESET: &str = "\x1b[0m"; // Reset formatting
	if let Some(error) = e {
		eprintln!(
			"{}Catastrophic failure: {}: {}{}",
			RED_BOLD, message, error, RESET
		);
	} else {
		eprintln!("{}Catastrophic failure: {}{}", RED_BOLD, message, RESET);
	}
	process::exit(1);
} // todo mabye possible to just print RED_BOLD then call abort idk

/// Checks if the paths used in pen exists.
///
/// # Arguments
/// - None
///
/// # Output
/// - None.
///
/// # Termination
/// -If, for any paths, checking the existence fails or returns false, this function exits. Only exeption is if `PYTHON_VERSIONS_DIR` does not exist, then it it created.
///
/// # Guarantees
/// - If the function returns, the paths are considered to be existing.
pub fn assert_global_paths() {
	match HOME_DIR.try_exists() {
		Ok(true) => {
			// not the same as .is_file() because of error coercion
			if !HOME_DIR.is_dir() {
				abort("todo", None);
			}
		}
		Ok(false) => abort(&format!("{} does not exist.", HOME_DIR.display()), None),
		Err(e) => abort(
			&format!("Failed to check if directory {} exists", HOME_DIR.display()),
			Some(&e),
		),
	}

	// No need to check for PEN_BIN since it is only used when uninstalling, where it is checked for existence.
	let dirs_to_check = vec![
		(&*PEN_DIR),
		(&*PYTHON_PACKAGES_DIR),
		(&*TMP_DIR),
		(&*PYTHON_VERSIONS_DIR),
	];

	for path in dirs_to_check {
		match path.try_exists() {
			Ok(true) => {
				if path.is_dir() {
					continue;
				} else {
					abort("todo", None);
				}
			}
			Ok(false) => {
				println!("{}", path.display());
				if let Err(e) = fs::create_dir_all(path) {
					abort(
						&format!("Failed to create directory {}", path.display()),
						Some(&e),
					);
				}
			}
			Err(e) => abort(
				&format!("Failed to check if directory {} exists", path.display()),
				Some(&e),
			),
		}
	}

	match PEN_CONFIG_FILE.try_exists() {
		Ok(true) => (),
		Ok(false) => {
			if let Err(e) = fs::File::create_new(&*PEN_CONFIG_FILE) {
				abort("todo", Some(&e));
			}
		}
		Err(e) => abort(
			&format!("Failed to check if file {} exists", PEN_DIR.display()),
			Some(&e),
		),
	}
}
