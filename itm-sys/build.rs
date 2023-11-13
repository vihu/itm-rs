fn main() {
    let cxx_sources = [
        "../vendor/itm/src/ComputeDeltaH.cpp",
        "../vendor/itm/src/DiffractionLoss.cpp",
        "../vendor/itm/src/FindHorizons.cpp",
        "../vendor/itm/src/FreeSpaceLoss.cpp",
        "../vendor/itm/src/FresnelIntegral.cpp",
        "../vendor/itm/src/H0Function.cpp",
        "../vendor/itm/src/InitializeArea.cpp",
        "../vendor/itm/src/InitializePointToPoint.cpp",
        "../vendor/itm/src/InverseComplementaryCumulativeDistributionFunction.cpp",
        "../vendor/itm/src/KnifeEdgeDiffraction.cpp",
        "../vendor/itm/src/LineOfSightLoss.cpp",
        "../vendor/itm/src/LinearLeastSquaresFit.cpp",
        "../vendor/itm/src/LongleyRice.cpp",
        "../vendor/itm/src/QuickPfl.cpp",
        "../vendor/itm/src/SigmaHFunction.cpp",
        "../vendor/itm/src/SmoothEarthDiffraction.cpp",
        "../vendor/itm/src/TerrainRoughness.cpp",
        "../vendor/itm/src/TroposcatterLoss.cpp",
        "../vendor/itm/src/ValidateInputs.cpp",
        "../vendor/itm/src/Variability.cpp",
        "../vendor/itm/src/itm_area.cpp",
        "../vendor/itm/src/itm_p2p.cpp",
        "wrapper/itm-wrapper.cpp",
    ];

    let mut bridge = cxx_build::bridge("src/lib.rs");
    bridge.flag("-std=c++11");
    bridge.include("../vendor/itm/include");
    #[cfg(feature = "address_sanitizer")]
    {
        bridge.flag("-fno-omit-frame-pointer");
        bridge.flag("-fsanitize=address");
        bridge.flag("-ggdb");
    }
    for path in &cxx_sources {
        bridge.file(path);
    }

    bridge.compile("itm_wrapper");

    for path in cxx_sources.iter() {
        println!("cargo:rerun-if-changed={path}");
    }

    println!("cargo:rerun-if-changed=wrapper/itm-wrapper.h");
}
