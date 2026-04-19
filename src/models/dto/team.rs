use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TeamDto {
    pub all_star_status: String,
    pub id: i32,
    pub name: String,
    pub link: String,
    pub venue: TeamVenueDto,
    pub team_code: String,
    pub file_code: String,
    pub abbreviation: String,
    pub team_name: String,
    pub location_name: String,
    pub first_year_of_play: String,
    pub league: TeamLeagueDto,
    pub sport: TeamSportDto,
    pub short_name: String,
    pub parent_org_name: String,
    pub parent_org_id: i32,
    pub franchise_name: String,
    pub club_name: String,
    pub active: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TeamVenueDto {
    pub id: i32,
    pub name: String,
    pub link: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TeamLeagueDto {
    pub id: i32,
    pub name: String,
    pub link: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TeamSportDto {
    pub id: i32,
    pub link: String,
    pub name: String,
}
