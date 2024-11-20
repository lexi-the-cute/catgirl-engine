use build_info::{BuildInfo, CrateInfo};
use std::collections::BTreeMap;

/// Get the list of dependencies used in the engine
fn get_dependencies(info: &BuildInfo) -> BTreeMap<String, CrateInfo> {
    let mut dependencies: BTreeMap<String, CrateInfo> = BTreeMap::new();
    let mut stack: Vec<&CrateInfo> = info.crate_info.dependencies.iter().collect();

    // Add each dependency only once
    while let Some(dep) = stack.pop() {
        if dep.name.starts_with("catgirl-engine") {
            // If one of my own crates, remove from results
            continue;
        }

        if dependencies
            .insert(dep.name.as_str().to_string(), dep.to_owned())
            .is_none()
        {
            stack.extend(dep.dependencies.iter());
        }
    }

    dependencies
}

/// Get all dependencies from the workspace used to build the engine
#[must_use]
pub(crate) fn get_all_dependencies() -> BTreeMap<String, CrateInfo> {
    let mut dependencies: BTreeMap<String, CrateInfo> =
        get_dependencies(crate::build::build_info());

    let mut util_dependencies: BTreeMap<String, CrateInfo> =
        get_dependencies(utils::build::build_info());

    dependencies.append(&mut util_dependencies);

    #[cfg(feature = "client")]
    {
        let mut client_dependencies: BTreeMap<String, CrateInfo> =
            get_dependencies(client::build::build_info());

        dependencies.append(&mut client_dependencies);
    }

    #[cfg(feature = "server")]
    {
        let mut server_dependencies: BTreeMap<String, CrateInfo> =
            get_dependencies(server::build::build_info());

        dependencies.append(&mut server_dependencies);
    }

    dependencies
}

#[cfg(test)]
mod tests {
    // I have no idea how to unit test these, but I do want to make sure tests exist at all
    use build_info::CrateInfo;
    use std::collections::BTreeMap;

    #[test]
    fn test_get_dependencies() {
        use crate::build::build_info;
        use build_info::BuildInfo;

        let build_info: &BuildInfo = build_info();
        let dependencies: BTreeMap<String, CrateInfo> = super::get_dependencies(build_info);

        for (_, _) in dependencies {}
    }

    #[test]
    fn test_get_all_dependencies() {
        let dependencies: BTreeMap<String, CrateInfo> = super::get_all_dependencies();

        for (_, _) in dependencies {}
    }
}
