use crate::core::{PackageIdSpec, Workspace};
use crate::ops;
use crate::util::PayloadResult;

pub fn pkgid(ws: &Workspace<'_>, spec: Option<&str>) -> PayloadResult<PackageIdSpec> {
    let resolve = match ops::load_pkg_lockfile(ws)? {
        Some(resolve) => resolve,
        None => anyhow::bail!("a Payload.lock must exist for this command"),
    };

    let pkgid = match spec {
        Some(spec) => PackageIdSpec::query_str(spec, resolve.iter())?,
        None => ws.current()?.package_id(),
    };
    Ok(PackageIdSpec::from_package_id(pkgid))
}
