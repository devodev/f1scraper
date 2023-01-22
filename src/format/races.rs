use std::fmt::Debug;

#[derive(Default, Debug)]
pub struct Race {
    pub name: String,
    pub circuit: String,
    pub date: String,

    pub race_results: Vec<Table<RaceResultHeaders, RaceResultData>>,
    pub fastest_laps: Vec<FastestLap>,
    pub qualifyings: Vec<Qualifying>,
}

#[derive(Default, Debug)]
pub struct Table<Headers: Debug, Data: Debug> {
    pub year: String,
    pub headers: Headers,
    pub data: Vec<Data>,
}

impl<Headers: Debug, Data: Debug> Table<Headers, Data> {
    pub fn new<S: Into<String>, D: Into<Vec<Data>>>(year: S, headers: Headers, data: D) -> Self {
        Self {
            year: year.into(),
            headers: headers,
            data: data.into(),
        }
    }
}

#[derive(Default, Debug)]
pub struct RaceResultData {
    pub pos: String,
    pub no: String,
    pub driver: String,
    pub car: String,
    pub laps: String,
    pub time_retired: String,
    pub pts: String,
}

#[derive(Default, Debug)]
pub struct RaceResultHeaders {
    pub pos: String,
    pub no: String,
    pub driver: String,
    pub car: String,
    pub laps: String,
    pub time_retired: String,
    pub pts: String,
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
