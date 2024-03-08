use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Antenna {
    pub txg: f64,
    pub txl: f64,
    pub ant: u64,
    pub azi: f64,
    pub tlt: f64,
    pub hbw: f64,
    pub vbw: f64,
    pub fbr: f64,
    pub pol: Polarisation,
}

impl Default for Antenna {
    fn default() -> Self {
        Self {
            txg: 21.0,
            txl: 0.0,
            ant: 0,
            azi: 0.0,
            tlt: 0.0,
            hbw: 120.0,
            vbw: 60.0,
            fbr: 30.0,
            pol: Polarisation::Vertical,
        }
    }
}

#[derive(Debug, Serialize)]
pub enum Polarisation {
    #[serde(rename(serialize = "v"))]
    Vertical,
    #[serde(rename(serialize = "h"))]
    Horizontal,
}