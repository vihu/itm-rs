pub mod enums;
mod error;

pub use crate::error::ItmErrCode;
use enums::{Climate, ModeVariability, Polarization};

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
///
/// See [C++] docs for info.
///
/// [C++]: https://github.com/dirkcgrunwald/itm/blob/31d068635380f61211e4ba43d50b03f0711b758e/src/itm_p2p.cpp#L5-L39
///
/// # Parameters
///
/// - `h_tx_meter`: transmiter height above ground (meters)
/// - `h_rx_meter`: receiver height above ground (meters)
/// - `terrain_delta_m`: distance between each elevation sample (meters)
/// - `terrain`: elevation samples spaced `terrain_delta_m` apart from eachother (meters)
/// - `climate`: see [`Climate`]
/// - `n0`: refractivity (N-Units, where 301 = 4/3 earth radius)
/// - `freq_hz`: frequency of signal (Hertz)
/// - `pol`: see [`Polarization`]
/// - `epsilon`: relative permittivity (Farads/meter)
/// - `sigma`: ground conductivity (Siemens/meter)
/// - `mdvar`: see [`ModeVariability`]
/// - `time`: time percentage (0.0 < time < 100.0)
/// - `location`: location percentage (0.0 < time < 100.0)
/// - `situation`: situation percentage (0.0 < time < 100.0)
///
/// # Suggested Surface Paramters
///
/// | Ground attribute | Ground Conductivity | Relative ground Permittivity |
/// |------------------|--------------------:|-----------------------------:|
/// | Poor ground      |               0.001 |                            4 |
/// | Average ground   |               0.005 |                           15 |
/// | Good ground      |                0.02 |                           25 |
/// | Fresh water      |                0.01 |                           25 |
/// | Sea water        |                 5.0 |                           25 |
///
/// See [Radio Mobile] for source of this table.
///
/// [Radio Mobile]: http://radiomobile.pe1mew.nl/?Calculations___ITM_model_propagation_settings
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
    mode_variability: ModeVariability,
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
        mode_variability as i32,
        time.into(),
        location.into(),
        situation.into(),
    );
    ItmErrCode::from_retcode(ret_code, attenuation_db)
}

#[cfg(test)]
mod tests {
    use super::{p2p, Climate, ModeVariability, Polarization};

    #[test]
    fn test_p2p() {
        // The parameters are generated with the following [`geopath`](https://github.com/JayKickliter/geoprop/tree/jsk/add-hexit-bin/geopath) call:
        //
        // ```
        // ./target/release/geopath --f32 --tile-dir=/path/to/3-arcsecond/srtm --start=38.868014,-104.920129,4 --dest=38.851764,-104.904427,30 csv
        // ```
        let terrain: &[f32] = &[
            2090.0, 2068.0, 2067.0, 2026.0, 1995.0, 1985.0, 1990.0, 1990.0, 2005.0, 1985.0, 1989.0,
            1954.0, 1962.0, 1938.0, 1928.0, 1920.0, 1920.0, 1916.0, 1925.0, 1931.0, 1931.0, 1960.0,
            1942.0, 1961.0, 1961.0, 1990.0, 1969.0,
        ];
        let delta_d = 86.97297;
        let tx_alt = 4.0;
        let rx_alt = 30.0;
        let attenuation_db = p2p(
            tx_alt,
            rx_alt,
            delta_d,
            terrain,
            Climate::Desert,
            301.0,
            900e6,
            Polarization::Vertical,
            15.0,
            0.001,
            ModeVariability::SingleMessage,
            99.0,
            99.0,
            99.0,
        )
        .unwrap();

        println!("attenuation: {attenuation_db} dB");

        // The code is currently returning this ridiculous value.
        assert_ne!(attenuation_db, 273.6794585392174);

        // FSPL over 2261.297 meters
        let expected_free_space_path_loss = 98.61;
        assert!(attenuation_db < expected_free_space_path_loss + 20.0);
    }
}
