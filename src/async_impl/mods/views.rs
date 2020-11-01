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

use crate::async_impl::SlackWebRequestSender;
pub use crate::mod_types::views_types::*;

/// Open a view for a user.
///
/// Wraps https://api.slack.com/methods/views.open

pub async fn open<R>(client: &R, request: &OpenRequest) -> Result<OpenResponse, OpenError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        Some(("trigger_id", request.trigger_id.to_string())),
        Some(("view", request.view.to_string())),
    ];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/views.open");
    client
        .get(&url, &params[..])
        .await
        .map_err(OpenError::Client)
        .and_then(|result| {
            serde_json::from_str::<OpenResponse>(&result)
                .map_err(|e| OpenError::MalformedResponse(result, e))
        })
}
/// Publish a static view for a User.
///
/// Wraps https://api.slack.com/methods/views.publish

pub async fn publish<R>(
    client: &R,
    request: &PublishRequest,
) -> Result<PublishResponse, PublishError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        request.hash.as_ref().map(|hash| ("hash", hash.to_string())),
        Some(("user_id", request.user_id.to_string())),
        Some(("view", request.view.to_string())),
    ];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/views.publish");
    client
        .get(&url, &params[..])
        .await
        .map_err(PublishError::Client)
        .and_then(|result| {
            serde_json::from_str::<PublishResponse>(&result)
                .map_err(|e| PublishError::MalformedResponse(result, e))
        })
}
/// Push a view onto the stack of a root view.
///
/// Wraps https://api.slack.com/methods/views.push

pub async fn push<R>(client: &R, request: &PushRequest) -> Result<PushResponse, PushError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        Some(("trigger_id", request.trigger_id.to_string())),
        Some(("view", request.view.to_string())),
    ];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/views.push");
    client
        .get(&url, &params[..])
        .await
        .map_err(PushError::Client)
        .and_then(|result| {
            serde_json::from_str::<PushResponse>(&result)
                .map_err(|e| PushError::MalformedResponse(result, e))
        })
}
/// Update an existing view.
///
/// Wraps https://api.slack.com/methods/views.update

pub async fn update<R>(
    client: &R,
    request: &UpdateRequest,
) -> Result<UpdateResponse, UpdateError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        request
            .external_id
            .as_ref()
            .map(|external_id| ("external_id", external_id.to_string())),
        request.hash.as_ref().map(|hash| ("hash", hash.to_string())),
        request.view.as_ref().map(|view| ("view", view.to_string())),
        request
            .view_id
            .as_ref()
            .map(|view_id| ("view_id", view_id.to_string())),
    ];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/views.update");
    client
        .get(&url, &params[..])
        .await
        .map_err(UpdateError::Client)
        .and_then(|result| {
            serde_json::from_str::<UpdateResponse>(&result)
                .map_err(|e| UpdateError::MalformedResponse(result, e))
        })
}