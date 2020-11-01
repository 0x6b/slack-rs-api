//=============================================================================
//
//                    WARNING: This file is AUTO-GENERATED
//
// Do not make changes directly to this file.
//
// If you would like to make a change to the library, please update the schema
// definitions at https://github.com/slack-rs/slack-api-schemas
//
// If you would like to make a change how the library was generated,
// please edit https://github.com/slack-rs/slack-rs-api/tree/master/codegen
//
//=============================================================================

#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

pub mod admins;
pub mod owners;
pub mod settings;

pub use crate::mod_types::admin::teams::*;
use crate::sync::SlackWebRequestSender;

/// Create an Enterprise team.
///
/// Wraps https://api.slack.com/methods/admin.teams.create

pub fn create<R>(
    client: &R,
    request: &CreateRequest,
) -> Result<CreateResponse, CreateError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        request
            .team_description
            .as_ref()
            .map(|team_description| ("team_description", team_description.to_string())),
        request
            .team_discoverability
            .as_ref()
            .map(|team_discoverability| ("team_discoverability", team_discoverability.to_string())),
        Some(("team_domain", request.team_domain.to_string())),
        Some(("team_name", request.team_name.to_string())),
    ];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/admin.teams.create");
    client
        .get(&url, &params[..])
        .map_err(CreateError::Client)
        .and_then(|result| {
            serde_json::from_str::<CreateResponse>(&result)
                .map_err(|e| CreateError::MalformedResponse(result, e))
        })
}
/// List all teams on an Enterprise organization
///
/// Wraps https://api.slack.com/methods/admin.teams.list

pub fn list<R>(client: &R, request: &ListRequest) -> Result<ListResponse, ListError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        request
            .cursor
            .as_ref()
            .map(|cursor| ("cursor", cursor.to_string())),
        request
            .limit
            .as_ref()
            .map(|limit| ("limit", limit.to_string())),
    ];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/admin.teams.list");
    client
        .get(&url, &params[..])
        .map_err(ListError::Client)
        .and_then(|result| {
            serde_json::from_str::<ListResponse>(&result)
                .map_err(|e| ListError::MalformedResponse(result, e))
        })
}