pub fn version() { 
    println!("yabba {} ({}) [{}]",
        env!("VERGEN_GIT_SEMVER"),
        env!("VERGEN_GIT_SHA"),
        env!("VERGEN_BUILD_TIMESTAMP"),
    );
}