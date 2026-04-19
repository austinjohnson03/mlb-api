use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TeamDto {
    all_star_status: String,
    id: i32,
    name: String,
    link: String,
    venue: VenueDto,
    team_code: String,
    file_code: String,
    abbreviation: String,
    team_name: String,
    location_name: String,
    first_year_of_play: String,
    league: LeagueDto,
    sport: SportDto,
    short_name: String,
    parent_org_name: String,
    parent_org_id: i32,
    franchise_name: String,
    club_name: String,
    active: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TeamVenueDto {
    id: i32,
    name: String,
    link: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TeamLeagueDto {
    id: i32,
    name: String,
    link: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TeamSportDto {
    id: i32,
    link: String,
    name: String,
}
