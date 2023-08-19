//! Build Finschia proto files. This build script clones the FinschiaSDK and Finschia version
//! specified in the FINSCHIA_SDK_REV and FINSCHIA_REV constant respectively and then
//! uses that to build the required proto files for further compilation.
//! This is based on the proto-compiler code in github.com/informalsystems/ibc-rs

use std::{env, path::PathBuf};

use proto_build::{
    code_generator::{CodeGenerator, FinschiaProject},
    git,
};

/// The Finschia SDK commit or tag to be cloned and used to build the proto files
const FINSCHIA_SDK_REV: &str = "main";

/// The wasmd commit or tag to be cloned and used to build the proto files
const WASMD_REV: &str = "main";

// All paths must end with a / and either be absolute or include a ./ to reference the current
// working directory.

/// The directory generated finschia-sdk proto files go into in this repo
const OUT_DIR: &str = "../finschia-std/src/types/";
/// Directory where the finschia-sdk submodule is located
const FINSCHIA_SDK_DIR: &str = "../../dependencies/finschia-sdk/";
/// Directory where the wasmd submodule is located
const WASMD_DIR: &str = "../../dependencies/wasmd/";
/// The tendermint commit or tag to be cloned and used to build the proto files
const TENDERMINT_DIR: &str = "../../dependencies/finschia-sdk/third_party/";
/// The ostracon commit or tag to be cloned and used to build the proto files
const OSTRACON_DIR: &str = "../../dependencies/finschia-sdk/third_party/";

/// A temporary directory for proto building
const TMP_BUILD_DIR: &str = "/tmp/tmp-protobuf/";

pub fn generate() {
    let args: Vec<String> = env::args().collect();
    if args.iter().any(|arg| arg == "--update-deps") {
        git::update_submodule(FINSCHIA_SDK_DIR, FINSCHIA_SDK_REV);
        git::update_submodule(WASMD_DIR, WASMD_REV);
    }

    let tmp_build_dir: PathBuf = TMP_BUILD_DIR.parse().unwrap();
    let out_dir: PathBuf = OUT_DIR.parse().unwrap();

    let wasmd_project = FinschiaProject {
        name: "lbm".to_string(),
        version: WASMD_REV.to_string(),
        project_dir: WASMD_DIR.to_string(),
        include_mods: vec!["wasm".to_string()],
    };
    let cosmwasm_project = FinschiaProject {
        name: "cosmwasm".to_string(),
        version: WASMD_REV.to_string(),
        project_dir: WASMD_DIR.to_string(),
        include_mods: vec!["wasm".to_string()],
    };
    let tendermint_project = FinschiaProject {
        name: "tendermint".to_string(),
        version: FINSCHIA_SDK_REV.to_string(),
        project_dir: TENDERMINT_DIR.to_string(),
        include_mods: vec![
            "abci".to_string(),
            "crypto".to_string(),
            "p2p".to_string(),
            "types".to_string(),
            "version".to_string(),
        ],
    };
    let ostracon_project = FinschiaProject {
        name: "ostracon".to_string(),
        version: FINSCHIA_SDK_REV.to_string(),
        project_dir: OSTRACON_DIR.to_string(),
        include_mods: vec!["types".to_string()],
    };
    let finschia_sdk_project = FinschiaProject {
        name: "lbm".to_string(),
        version: FINSCHIA_SDK_REV.to_string(),
        project_dir: FINSCHIA_SDK_DIR.to_string(),
        include_mods: vec![
            "bankplus".to_string(),
            "base".to_string(),
            "collection".to_string(),
            "foundation".to_string(),
            "stakingplus".to_string(),
            "token".to_string(),
            "tx".to_string(),
        ],
    };
    let cosmos_sdk_project = FinschiaProject {
        name: "cosmos".to_string(),
        version: FINSCHIA_SDK_REV.to_string(),
        project_dir: FINSCHIA_SDK_DIR.to_string(),
        include_mods: vec![
            "auth".to_string(),
            "authz".to_string(),
            "bank".to_string(),
            "base".to_string(),
            "capability".to_string(),
            "crisis".to_string(),
            "crypto".to_string(),
            "distribution".to_string(),
            "evidence".to_string(),
            "feegrant".to_string(),
            "genutil".to_string(),
            "gov".to_string(),
            "mint".to_string(),
            "params".to_string(),
            "slashing".to_string(),
            "staking".to_string(),
            "tx".to_string(),
            "upgrade".to_string(),
            "vesting".to_string(),
        ],
    };

    let finschia_code_generator = CodeGenerator::new(
        out_dir,
        tmp_build_dir,
        finschia_sdk_project,
        vec![
            cosmos_sdk_project,
            wasmd_project,
            cosmwasm_project,
            tendermint_project,
            ostracon_project,
        ],
    );

    finschia_code_generator.generate();
}

fn main() {
    pretty_env_logger::init();
    generate();
}
