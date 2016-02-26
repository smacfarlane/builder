// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

//! Uploads a package to a [repository](../repo).
//!
//! # Examples
//!
//! ```bash
//! $ bldr upload chef/redis -u http://localhost:9632
//! ```
//!
//! Will upload a package to the repository.
//!
//! # Notes
//!
//! This should be extended to cover uploading specific packages, and finding them by ways more
//! complex than just latest version.
//!

use hyper::status::StatusCode;

use error::{BldrResult, BldrError, ErrorKind};
use config::Config;
use package::Package;
use repo;

static LOGKEY: &'static str = "CU";

/// Upload a package from the cache to a repository. The latest version/release of the package
/// will be uploaded if not specified.
///
/// # Failures
///
/// * Fails if it cannot find a package
/// * Fails if the package doesn't have a `.bldr` file in the cache
/// * Fails if it cannot upload the file
pub fn package(config: &Config) -> BldrResult<()> {
    let url = config.url().as_ref().unwrap();
    let package = try!(Package::load(config.package(), None));
    outputln!("Uploading from {}", package.cache_file().to_string_lossy());
    match repo::client::put_package(url, &package) {
        Ok(()) => (),
        Err(BldrError{err: ErrorKind::HTTP(StatusCode::Conflict), ..}) => {
            outputln!("Package already exists on remote; skipping.");
        }
        Err(BldrError{err: ErrorKind::HTTP(StatusCode::UnprocessableEntity), ..}) => {
            return Err(bldr_error!(ErrorKind::PackageArchiveMalformed(format!("{}",
                                                                          package.cache_file().to_string_lossy()))));
        }
        Err(e) => {
            outputln!("Unexpected response from remote");
            return Err(e);
        }
    }
    outputln!("Complete");
    Ok(())
}
