use chrono::NaiveDate;
use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LeagueDto {
    pub id: i32,
    pub name: String,
    pub link: String,
    pub abbreviation: String,
    pub name_short: String,
    pub season_state: String,
    pub has_wild_card: bool,
    pub has_split_season: bool,
    pub num_games: i32,
    pub has_playoff_points: bool,
    pub num_teams: i32,
    pub num_wildcard_teams: i32,
    pub season_date_info: LeagueSeasonDateInfoDto,
    pub season: String,
    pub org_code: String,
    pub conferences_in_use: bool,
    pub divisions_in_use: bool,
    pub sport: LeagueSportDto,
    pub sort_order: i32,
    pub active: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LeagueSeasonDateInfoDto {
    pub season_id: String,
    pub pre_season_start_date: NaiveDate,
    pub pre_season_end_date: NaiveDate,
    pub season_start_date: NaiveDate,
    pub spring_start_date: NaiveDate,
    pub spring_end_date: NaiveDate,
    pub regular_season_start_date: NaiveDate,
    pub last_date_1st_half: NaiveDate,
    pub all_star_date: NaiveDate,
    pub first_date_2nd_half: NaiveDate,
    pub regular_season_end_date: NaiveDate,
    pub post_season_start_date: NaiveDate,
    pub post_season_end_date: NaiveDate,
    pub season_end_date: NaiveDate,
    pub offseason_start_date: NaiveDate,
    pub offseason_end_date: NaiveDate,
    pub season_level_gameday_type: String,
    pub game_level_gameday_type: String,
    pub qualifier_plate_appearances: f64,
    pub qualifier_outs_pitched: f64,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LeagueSportDto {
    pub id: i32,
    pub link: String,
}
