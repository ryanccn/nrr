use crate::package_json::PackageJson;

#[must_use]
pub fn suggest(input: &str, package: &PackageJson) -> Option<String> {
    let mut distances = package
        .scripts
        .keys()
        .map(|v| (v, strsim::jaro(input, v)))
        .filter(|(_, confidence)| *confidence > 0.7)
        .collect::<Vec<_>>();

    distances.sort_unstable_by(|a, b| b.1.total_cmp(&a.1));
    distances.first().map(|a| a.0.to_owned())
}
