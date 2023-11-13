pub mod enums;
mod error;

pub use crate::error::ItmErrCode;
use enums::{Climate, Polarization};

#[cxx::bridge]
mod ffi {

    #[derive(Default, Debug)]
    struct P2PRes {
        ret_code: i32,
        attenuation_db: f64,
    }

    unsafe extern "C++" {
        include!("itm-sys/wrapper/itm-wrapper.h");

        fn p2p(
            h_tx_meter: f64,
            h_rx_meter: f64,
            pfl: &[f64],
            climate: i32,
            N_0: f64,
            f_mhz: f64,
            pol: i32,
            epsilon: f64,
            sigma: f64,
            mdvar: i32,
            time: f64,
            location: f64,
            situation: f64,
        ) -> P2PRes;
    }
}

/// Returns the attenuation between two points.
pub fn p2p<T>(
    h_tx_meter: T,
    h_rx_meter: T,
    terrain_delta_m: T,
    terrain: &[T],
    climate: Climate,
    n0: T,
    f_hz: T,
    pol: Polarization,
    epsilon: T,
    sigma: T,
    mdvar: i32,
    time: T,
    location: T,
    situation: T,
) -> Result<f64, ItmErrCode>
where
    T: Copy,
    f64: From<T>,
{
    let pfl = {
        let mut pfl: Vec<f64> = Vec::with_capacity(terrain.len() + 2);
        pfl.extend(terrain.iter().map(|elev| f64::from(*elev)));
        pfl.push(terrain.len() as f64);
        pfl.push(f64::from(terrain_delta_m));
        pfl.rotate_right(2);
        pfl
    };

    let ffi::P2PRes {
        ret_code,
        attenuation_db,
    } = ffi::p2p(
        h_tx_meter.into(),
        h_rx_meter.into(),
        &pfl,
        climate as i32,
        n0.into(),
        f64::from(f_hz) / 1e6,
        pol as i32,
        epsilon.into(),
        sigma.into(),
        mdvar,
        time.into(),
        location.into(),
        situation.into(),
    );
    ItmErrCode::from_retcode(ret_code, attenuation_db)
}

#[cfg(test)]
mod tests {
    use super::p2p;
    #[test]
    fn test_poc() {}
}
