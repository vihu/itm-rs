use std::ffi::c_int;

#[derive(Debug, thiserror::Error)]
pub enum ItmErrCode {
    #[error("TX terminal height is out of range")]
    TxTerminalHeight = 1000,
    #[error("RX terminal height is out of range")]
    RxTerminalHeight = 1001,
    #[error("Invalid value for radio climate")]
    InvalidRadioClimate = 1002,
    #[error("Time percentage is out of range")]
    InvalidTime = 1003,
    #[error("Location percentage is out of range")]
    InvalidLocation = 1004,
    #[error("Situation percentage is out of range")]
    InvalidSituation = 1005,
    #[error("Confidence percentage is out of range")]
    InvalidConfidence = 1006,
    #[error("Reliability percentage is out of range")]
    InvalidReliability = 1007,
    #[error("Refractivity is out of range")]
    Refractivity = 1008,
    #[error("Frequency is out of range")]
    Frequency = 1009,
    #[error("Invalid value for polarization")]
    Polarization = 1010,
    #[error("Epsilon is out of range")]
    Epsilon = 1011,
    #[error("Sigma is out of range")]
    Sigma = 1012,
    #[error("The imaginary portion of the complex impedance is larger than the real portion")]
    GroundImpedance = 1013,
    #[error("Invalid value for mode of variability")]
    Mdvar = 1014,
    #[error("Internally computed effective earth radius is invalid")]
    EffectiveEarth = 1016,
    #[error("Path distance is out of range")]
    PathDistance = 1017,
    #[error("Delta H (terrain irregularity parameter) is out of range")]
    DeltaH = 1018,
    #[error("Invalid value for TX siting criteria")]
    TxSitingCriteria = 1019,
    #[error("Invalid value for RX siting criteria")]
    RxSitingCriteria = 1020,
    #[error("Internally computed surface refractivity value is too small")]
    SurfaceRefractivitySmall = 1021,
    #[error("Internally computed surface refractivity value is too large")]
    SurfaceRefractivityLarge = 1022,
}

impl ItmErrCode {
    pub fn result(err_code: c_int) -> Result<(), ItmErrCode> {
        let err = match err_code {
            0 | 1 => return Ok(()),
            1000 => ItmErrCode::TxTerminalHeight,
            1001 => ItmErrCode::RxTerminalHeight,
            1002 => ItmErrCode::InvalidRadioClimate,
            1003 => ItmErrCode::InvalidTime,
            1004 => ItmErrCode::InvalidLocation,
            1005 => ItmErrCode::InvalidSituation,
            1006 => ItmErrCode::InvalidConfidence,
            1007 => ItmErrCode::InvalidReliability,
            1008 => ItmErrCode::Refractivity,
            1009 => ItmErrCode::Frequency,
            1010 => ItmErrCode::Polarization,
            1011 => ItmErrCode::Epsilon,
            1012 => ItmErrCode::Sigma,
            1013 => ItmErrCode::GroundImpedance,
            1014 => ItmErrCode::Mdvar,
            1016 => ItmErrCode::EffectiveEarth,
            1017 => ItmErrCode::PathDistance,
            1018 => ItmErrCode::DeltaH,
            1019 => ItmErrCode::TxSitingCriteria,
            1020 => ItmErrCode::RxSitingCriteria,
            1021 => ItmErrCode::SurfaceRefractivitySmall,
            1022 => ItmErrCode::SurfaceRefractivityLarge,
            _ => unreachable!(),
        };
        Err(err)
    }
}
