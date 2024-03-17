use build_info_build::DependencyDepth;
use std::env;

fn main() {
    // Generate build info
    generate_build_info();
}

fn matches_environment_var(key: &str, value: &str) -> bool {
    let environment_var: Result<String, env::VarError> = env::var(key);
    environment_var.is_ok() && environment_var.unwrap() == value
}

fn generate_build_info() {
    let mut depth: DependencyDepth = DependencyDepth::Depth(0);

    // Track environment for rebuilds
    println!("cargo:rerun-if-env-changed=RUST_ANALYZER");
    println!("cargo:rerun-if-env-changed=DOCS_RS");

    // Custom environment variable to speed up writing code
    let rust_analyzer: bool = matches_environment_var("RUST_ANALYZER", "true");
    let docs_rs: bool = env::var("DOCS_RS").is_ok();
    if rust_analyzer || docs_rs {
        depth = DependencyDepth::None;
    }

    if !docs_rs {
        build_info_build::build_script().collect_runtime_dependencies(depth);
    } else {
        // Waiting for https://github.com/danielschemmel/build-info/pull/22
        let fake_data: &str = "{\"version\":\"0.0.36\",\"string\":\"KLUv/QCIjRMAxiBoJ9Cy6gFCxM4ixSuU/HBxd1D/SOgQX0ocwXafudTHUGTjqJYqkxTYEV8AXABZAJ6pEV23LyseLApKFZy9ffRaDrtI9uVk5tSC2csxgSboyws7nHVkRyLStvNLwQBlJhgnKFn0tZt7/ra+v5tt3XImVA5zSuGorMwUOXFDwSlGWFOEJW1opkdJr4pSjDQElc9LqG85PW8XXLNE8Co1px/brb1WToBuLdmue3bn+/F8GefsA/6xuZLxxZMCEGEGVpu8cxKRmobnoxlnHcZIXLP7uSzB+Nwz5UYRZWBEGxYXB9DE9oe6QyU0zQMOOE+pvT0xj3XHdfEQe6XNrl+B0+wT3ytfrQD17Wffb98NLvrlpik6+4avf7wiityzsRA5ML5X+hZ7mpaYyLXJ/BGQ+QhovyBbF0xMn19Mbo1xiqd9S32bdxEoI10gKX1WlMbJoahmVtSUcTITRlWURoNV2cSmakMjtTAICORuT7X/JLjRwIkgTlzrrLtM0ql0E+7r67i39/0Hfo/nXmVd1qZToHhe1LfM/l6v/TZvF8h9fF1d1wVdqFFURiAkRDIkJYVCY5AkmCOxeSIwIAQXlQopRpBFkogK7TMHvkM/vVYVzflXseIRUE0+NHO0m98GdBw4XQD+W66ntC8ilROTbl2C7FwBAOKyxkLoO2ezpb9o8WPVEdGIIAdxNgGEV5psGGq8o53wRBWWg4pYKcxo/krDUU1ea4Emm4gghYTjcwlEEJbFlqJYUB5PyUejJbW8jCElD9ply7onIqiwOYJS9PmJNuJRBlOgBspEXKbcS/c1HbfBudIyZ6H9OwQwPuckTG88CQ==\"}";
        println!("cargo:rustc-env=BUILD_INFO={fake_data}");
    }
}
