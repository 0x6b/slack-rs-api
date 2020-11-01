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

pub use crate::mod_types::files::remote_types::*;
use crate::sync::SlackWebRequestSender;

/// Adds a file from a remote service
///
/// Wraps https://api.slack.com/methods/files.remote.add

pub fn add<R>(client: &R, request: &AddRequest) -> Result<AddResponse, AddError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        request
            .external_id
            .as_ref()
            .map(|external_id| ("external_id", external_id.to_string())),
        request
            .external_url
            .as_ref()
            .map(|external_url| ("external_url", external_url.to_string())),
        request
            .filetype
            .as_ref()
            .map(|filetype| ("filetype", filetype.to_string())),
        request
            .indexable_file_contents
            .as_ref()
            .map(|indexable_file_contents| {
                (
                    "indexable_file_contents",
                    indexable_file_contents.to_string(),
                )
            }),
        request
            .preview_image
            .as_ref()
            .map(|preview_image| ("preview_image", preview_image.to_string())),
        request
            .title
            .as_ref()
            .map(|title| ("title", title.to_string())),
        request
            .token
            .as_ref()
            .map(|token| ("token", token.to_string())),
    ];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/files.remote.add");
    client
        .get(&url, &params[..])
        .map_err(AddError::Client)
        .and_then(|result| {
            serde_json::from_str::<AddResponse>(&result)
                .map_err(|e| AddError::MalformedResponse(result, e))
        })
}
/// Retrieve information about a remote file added to Slack
///
/// Wraps https://api.slack.com/methods/files.remote.info

pub fn info<R>(client: &R, request: &InfoRequest) -> Result<InfoResponse, InfoError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        request
            .external_id
            .as_ref()
            .map(|external_id| ("external_id", external_id.to_string())),
        request.file.as_ref().map(|file| ("file", file.to_string())),
    ];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/files.remote.info");
    client
        .get(&url, &params[..])
        .map_err(InfoError::Client)
        .and_then(|result| {
            serde_json::from_str::<InfoResponse>(&result)
                .map_err(|e| InfoError::MalformedResponse(result, e))
        })
}
/// Retrieve information about a remote file added to Slack
///
/// Wraps https://api.slack.com/methods/files.remote.list

pub fn list<R>(client: &R, request: &ListRequest) -> Result<ListResponse, ListError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        request
            .channel
            .as_ref()
            .map(|channel| ("channel", channel.to_string())),
        request
            .cursor
            .as_ref()
            .map(|cursor| ("cursor", cursor.to_string())),
        request
            .limit
            .as_ref()
            .map(|limit| ("limit", limit.to_string())),
        request
            .ts_from
            .as_ref()
            .map(|ts_from| ("ts_from", ts_from.to_string())),
        request
            .ts_to
            .as_ref()
            .map(|ts_to| ("ts_to", ts_to.to_string())),
    ];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/files.remote.list");
    client
        .get(&url, &params[..])
        .map_err(ListError::Client)
        .and_then(|result| {
            serde_json::from_str::<ListResponse>(&result)
                .map_err(|e| ListError::MalformedResponse(result, e))
        })
}
/// Remove a remote file.
///
/// Wraps https://api.slack.com/methods/files.remote.remove

pub fn remove<R>(
    client: &R,
    request: &RemoveRequest,
) -> Result<RemoveResponse, RemoveError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        request
            .external_id
            .as_ref()
            .map(|external_id| ("external_id", external_id.to_string())),
        request.file.as_ref().map(|file| ("file", file.to_string())),
        request
            .token
            .as_ref()
            .map(|token| ("token", token.to_string())),
    ];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/files.remote.remove");
    client
        .get(&url, &params[..])
        .map_err(RemoveError::Client)
        .and_then(|result| {
            serde_json::from_str::<RemoveResponse>(&result)
                .map_err(|e| RemoveError::MalformedResponse(result, e))
        })
}
/// Share a remote file into a channel.
///
/// Wraps https://api.slack.com/methods/files.remote.share

pub fn share<R>(client: &R, request: &ShareRequest) -> Result<ShareResponse, ShareError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        request
            .channels
            .as_ref()
            .map(|channels| ("channels", channels.to_string())),
        request
            .external_id
            .as_ref()
            .map(|external_id| ("external_id", external_id.to_string())),
        request.file.as_ref().map(|file| ("file", file.to_string())),
    ];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/files.remote.share");
    client
        .get(&url, &params[..])
        .map_err(ShareError::Client)
        .and_then(|result| {
            serde_json::from_str::<ShareResponse>(&result)
                .map_err(|e| ShareError::MalformedResponse(result, e))
        })
}
/// Updates an existing remote file.
///
/// Wraps https://api.slack.com/methods/files.remote.update

pub fn update<R>(
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
        request
            .external_url
            .as_ref()
            .map(|external_url| ("external_url", external_url.to_string())),
        request.file.as_ref().map(|file| ("file", file.to_string())),
        request
            .filetype
            .as_ref()
            .map(|filetype| ("filetype", filetype.to_string())),
        request
            .indexable_file_contents
            .as_ref()
            .map(|indexable_file_contents| {
                (
                    "indexable_file_contents",
                    indexable_file_contents.to_string(),
                )
            }),
        request
            .preview_image
            .as_ref()
            .map(|preview_image| ("preview_image", preview_image.to_string())),
        request
            .title
            .as_ref()
            .map(|title| ("title", title.to_string())),
        request
            .token
            .as_ref()
            .map(|token| ("token", token.to_string())),
    ];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/files.remote.update");
    client
        .get(&url, &params[..])
        .map_err(UpdateError::Client)
        .and_then(|result| {
            serde_json::from_str::<UpdateResponse>(&result)
                .map_err(|e| UpdateError::MalformedResponse(result, e))
        })
}