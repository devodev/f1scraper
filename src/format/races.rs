#[derive(Default, Debug)]
pub struct Race {
    pub name: String,
    pub circuit: String,
    pub date: String,

    pub race_results: Vec<RaceResult>,
    pub fastest_laps: Vec<FastestLap>,
    pub qualifyings: Vec<Qualifying>,
}

#[derive(Default, Debug)]
pub struct RaceResult {
    pub pos: String,
    pub no: String,
    pub driver: String,
    pub car: String,
    pub laps: String,
    pub time_retired: String,
    pub pts: String,
}
#[derive(Default, Debug)]
pub struct RaceResultSummary {
    pub year: String,
    pub headers: RaceResultSummaryHeaders,
    pub summaries: Vec<RaceResultSummaryData>,
}

#[derive(Default, Debug)]
pub struct RaceResultSummaryHeaders {
    pub grand_prix: String,
    pub date: String,
    pub winner: String,
    pub car: String,
    pub laps: String,
    pub time: String,
}

#[derive(Default, Debug)]
pub struct RaceResultSummaryData {
    pub grand_prix: String,
    pub url: String,
    pub date: String,
    pub winner: String,
    pub car: String,
    pub laps: String,
    pub time: String,
}

#[derive(Default, Debug)]
pub struct FastestLap {
    pub pos: String,
    pub no: String,
    pub driver: String,
    pub car: String,
    pub lap: String,
    pub time: String,
}
#[derive(Default, Debug)]
pub struct Qualifying {
    pub pos: String,
    pub no: String,
    pub driver: String,
    pub car: String,
    pub time: String,
}
