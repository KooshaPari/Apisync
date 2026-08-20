//! Executable guard for the release-scope traceability matrix.

const MATRIX: &str = include_str!("../TEST_COVERAGE_MATRIX.md");
const EVIDENCE: &str = concat!(
    include_str!("rest_integration_tests.rs"),
    include_str!("property_tests.rs"),
    include_str!("../src/domain/mod.rs"),
    include_str!("../src/adapters/graphql/schema.rs"),
    include_str!("../src/adapters/graphql/server.rs"),
    include_str!("../src/adapters/websocket/server.rs"),
    include_str!("../Cargo.toml"),
    include_str!("../.github/workflows/ci.yml"),
);

#[test]
fn release_requirements_have_at_least_85_percent_traceability() {
    let rows: Vec<_> = MATRIX.lines().filter(|line| line.starts_with("| RQ-")).collect();

    assert_eq!(rows.len(), 16, "release requirement denominator changed");

    let covered: Vec<_> = rows.iter().filter(|line| line.ends_with("| Covered |")).collect();
    assert_eq!(covered.len(), 16, "every release requirement needs evidence");
    assert!(
        covered.len() * 100 >= rows.len() * 85,
        "traceability coverage must remain at or above 85%"
    );
}

#[test]
fn all_defined_e2e_journeys_are_covered() {
    let e2e_rows: Vec<_> = MATRIX.lines().filter(|line| line.starts_with("| RQ-E2E-")).collect();

    assert_eq!(e2e_rows.len(), 10, "E2E journey denominator changed");
    assert!(
        e2e_rows.iter().all(|line| line.ends_with("| Covered |")),
        "every defined E2E journey must be covered"
    );
}

#[test]
fn matrix_evidence_names_exist_in_test_or_gate_sources() {
    for row in MATRIX.lines().filter(|line| line.starts_with("| RQ-")) {
        let columns: Vec<_> = row.split('|').map(str::trim).collect();
        let requirement_id = columns[1];
        let evidence = columns[4].trim_matches('`');

        for symbol in evidence.split("<br>") {
            let symbol = symbol.trim().trim_matches('`');
            assert!(
                EVIDENCE.contains(symbol),
                "{requirement_id} cites missing evidence symbol {symbol}"
            );
        }
    }
}
