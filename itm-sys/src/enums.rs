/// Antenna polarization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Polarization {
    Horizontal = 0,
    Vertical = 1,
}

/// Siting criteria.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SittingCriteria {
    Random = 0,
    Careful = 1,
    VeryCareful = 2,
}

/// Radio climate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Climate {
    Equatorial = 1,
    ContinentalSubtropical = 2,
    MaritimeSubtropical = 3,
    Desert = 4,
    ContinentalTemperate = 5,
    MaritimeTemperateOverLand = 6,
    MaritimeTemperateOverSea = 7,
}

/// Propagation mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Mode {
    NotSet = 0,
    LineOfSight = 1,
    Diffraction = 2,
    Troposcatter = 3,
}

/// Modes of variability.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Variability {
    SingleMessage = 0,
    Accidental = 1,
    Mobile = 2,
    Broadcast = 3,
}