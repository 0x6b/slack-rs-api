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

pub use crate::mod_types::conversations_types::*;
use crate::sync::SlackWebRequestSender;

/// Archives a conversation.
///
/// Wraps https://api.slack.com/methods/conversations.archive

pub fn archive<R>(
    client: &R,
    request: &ArchiveRequest,
) -> Result<ArchiveResponse, ArchiveError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![request
        .channel
        .as_ref()
        .map(|channel| ("channel", channel.to_string()))];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/conversations.archive");
    client
        .get(&url, &params[..])
        .map_err(ArchiveError::Client)
        .and_then(|result| {
            serde_json::from_str::<ArchiveResponse>(&result)
                .map_err(|e| ArchiveError::MalformedResponse(result, e))
        })
}
/// Closes a direct message or multi-person direct message.
///
/// Wraps https://api.slack.com/methods/conversations.close

pub fn close<R>(client: &R, request: &CloseRequest) -> Result<CloseResponse, CloseError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![request
        .channel
        .as_ref()
        .map(|channel| ("channel", channel.to_string()))];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/conversations.close");
    client
        .get(&url, &params[..])
        .map_err(CloseError::Client)
        .and_then(|result| {
            serde_json::from_str::<CloseResponse>(&result)
                .map_err(|e| CloseError::MalformedResponse(result, e))
        })
}
/// Initiates a public or private channel-based conversation
///
/// Wraps https://api.slack.com/methods/conversations.create

pub fn create<R>(
    client: &R,
    request: &CreateRequest,
) -> Result<CreateResponse, CreateError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        request
            .is_private
            .as_ref()
            .map(|is_private| ("is_private", is_private.to_string())),
        request.name.as_ref().map(|name| ("name", name.to_string())),
    ];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/conversations.create");
    client
        .get(&url, &params[..])
        .map_err(CreateError::Client)
        .and_then(|result| {
            serde_json::from_str::<CreateResponse>(&result)
                .map_err(|e| CreateError::MalformedResponse(result, e))
        })
}
/// Fetches a conversation's history of messages and events.
///
/// Wraps https://api.slack.com/methods/conversations.history

pub fn history<R>(
    client: &R,
    request: &HistoryRequest,
) -> Result<HistoryResponse, HistoryError<R::Error>>
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
            .inclusive
            .as_ref()
            .map(|inclusive| ("inclusive", inclusive.to_string())),
        request
            .latest
            .as_ref()
            .map(|latest| ("latest", latest.to_string())),
        request
            .limit
            .as_ref()
            .map(|limit| ("limit", limit.to_string())),
        request
            .oldest
            .as_ref()
            .map(|oldest| ("oldest", oldest.to_string())),
    ];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/conversations.history");
    client
        .get(&url, &params[..])
        .map_err(HistoryError::Client)
        .and_then(|result| {
            serde_json::from_str::<HistoryResponse>(&result)
                .map_err(|e| HistoryError::MalformedResponse(result, e))
        })
}
/// Retrieve information about a conversation.
///
/// Wraps https://api.slack.com/methods/conversations.info

pub fn info<R>(client: &R, request: &InfoRequest) -> Result<InfoResponse, InfoError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        request
            .channel
            .as_ref()
            .map(|channel| ("channel", channel.to_string())),
        request
            .include_locale
            .as_ref()
            .map(|include_locale| ("include_locale", include_locale.to_string())),
        request
            .include_num_members
            .as_ref()
            .map(|include_num_members| ("include_num_members", include_num_members.to_string())),
    ];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/conversations.info");
    client
        .get(&url, &params[..])
        .map_err(InfoError::Client)
        .and_then(|result| {
            serde_json::from_str::<InfoResponse>(&result)
                .map_err(|e| InfoError::MalformedResponse(result, e))
        })
}
/// Invites users to a channel.
///
/// Wraps https://api.slack.com/methods/conversations.invite

pub fn invite<R>(
    client: &R,
    request: &InviteRequest,
) -> Result<InviteResponse, InviteError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        request
            .channel
            .as_ref()
            .map(|channel| ("channel", channel.to_string())),
        request
            .users
            .as_ref()
            .map(|users| ("users", users.to_string())),
    ];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/conversations.invite");
    client
        .get(&url, &params[..])
        .map_err(InviteError::Client)
        .and_then(|result| {
            serde_json::from_str::<InviteResponse>(&result)
                .map_err(|e| InviteError::MalformedResponse(result, e))
        })
}
/// Joins an existing conversation.
///
/// Wraps https://api.slack.com/methods/conversations.join

pub fn join<R>(client: &R, request: &JoinRequest) -> Result<JoinResponse, JoinError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![request
        .channel
        .as_ref()
        .map(|channel| ("channel", channel.to_string()))];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/conversations.join");
    client
        .get(&url, &params[..])
        .map_err(JoinError::Client)
        .and_then(|result| {
            serde_json::from_str::<JoinResponse>(&result)
                .map_err(|e| JoinError::MalformedResponse(result, e))
        })
}
/// Removes a user from a conversation.
///
/// Wraps https://api.slack.com/methods/conversations.kick

pub fn kick<R>(client: &R, request: &KickRequest) -> Result<KickResponse, KickError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        request
            .channel
            .as_ref()
            .map(|channel| ("channel", channel.to_string())),
        request.user.as_ref().map(|user| ("user", user.to_string())),
    ];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/conversations.kick");
    client
        .get(&url, &params[..])
        .map_err(KickError::Client)
        .and_then(|result| {
            serde_json::from_str::<KickResponse>(&result)
                .map_err(|e| KickError::MalformedResponse(result, e))
        })
}
/// Leaves a conversation.
///
/// Wraps https://api.slack.com/methods/conversations.leave

pub fn leave<R>(client: &R, request: &LeaveRequest) -> Result<LeaveResponse, LeaveError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![request
        .channel
        .as_ref()
        .map(|channel| ("channel", channel.to_string()))];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/conversations.leave");
    client
        .get(&url, &params[..])
        .map_err(LeaveError::Client)
        .and_then(|result| {
            serde_json::from_str::<LeaveResponse>(&result)
                .map_err(|e| LeaveError::MalformedResponse(result, e))
        })
}
/// Lists all channels in a Slack team.
///
/// Wraps https://api.slack.com/methods/conversations.list

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
            .exclude_archived
            .as_ref()
            .map(|exclude_archived| ("exclude_archived", exclude_archived.to_string())),
        request
            .limit
            .as_ref()
            .map(|limit| ("limit", limit.to_string())),
        request
            .types
            .as_ref()
            .map(|types| ("types", types.to_string())),
    ];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/conversations.list");
    client
        .get(&url, &params[..])
        .map_err(ListError::Client)
        .and_then(|result| {
            serde_json::from_str::<ListResponse>(&result)
                .map_err(|e| ListError::MalformedResponse(result, e))
        })
}
/// Sets the read cursor in a channel.
///
/// Wraps https://api.slack.com/methods/conversations.mark

pub fn mark<R>(client: &R, request: &MarkRequest) -> Result<MarkResponse, MarkError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        request
            .channel
            .as_ref()
            .map(|channel| ("channel", channel.to_string())),
        request.ts.as_ref().map(|ts| ("ts", ts.to_string())),
    ];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/conversations.mark");
    client
        .get(&url, &params[..])
        .map_err(MarkError::Client)
        .and_then(|result| {
            serde_json::from_str::<MarkResponse>(&result)
                .map_err(|e| MarkError::MalformedResponse(result, e))
        })
}
/// Retrieve members of a conversation.
///
/// Wraps https://api.slack.com/methods/conversations.members

pub fn members<R>(
    client: &R,
    request: &MembersRequest,
) -> Result<MembersResponse, MembersError<R::Error>>
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
    ];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/conversations.members");
    client
        .get(&url, &params[..])
        .map_err(MembersError::Client)
        .and_then(|result| {
            serde_json::from_str::<MembersResponse>(&result)
                .map_err(|e| MembersError::MalformedResponse(result, e))
        })
}
/// Opens or resumes a direct message or multi-person direct message.
///
/// Wraps https://api.slack.com/methods/conversations.open

pub fn open<R>(client: &R, request: &OpenRequest) -> Result<OpenResponse, OpenError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        request
            .channel
            .as_ref()
            .map(|channel| ("channel", channel.to_string())),
        request
            .return_im
            .as_ref()
            .map(|return_im| ("return_im", return_im.to_string())),
        request
            .users
            .as_ref()
            .map(|users| ("users", users.to_string())),
    ];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/conversations.open");
    client
        .get(&url, &params[..])
        .map_err(OpenError::Client)
        .and_then(|result| {
            serde_json::from_str::<OpenResponse>(&result)
                .map_err(|e| OpenError::MalformedResponse(result, e))
        })
}
/// Renames a conversation.
///
/// Wraps https://api.slack.com/methods/conversations.rename

pub fn rename<R>(
    client: &R,
    request: &RenameRequest,
) -> Result<RenameResponse, RenameError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        request
            .channel
            .as_ref()
            .map(|channel| ("channel", channel.to_string())),
        request.name.as_ref().map(|name| ("name", name.to_string())),
    ];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/conversations.rename");
    client
        .get(&url, &params[..])
        .map_err(RenameError::Client)
        .and_then(|result| {
            serde_json::from_str::<RenameResponse>(&result)
                .map_err(|e| RenameError::MalformedResponse(result, e))
        })
}
/// Retrieve a thread of messages posted to a conversation
///
/// Wraps https://api.slack.com/methods/conversations.replies

pub fn replies<R>(
    client: &R,
    request: &RepliesRequest,
) -> Result<RepliesResponse, RepliesError<R::Error>>
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
            .inclusive
            .as_ref()
            .map(|inclusive| ("inclusive", inclusive.to_string())),
        request
            .latest
            .as_ref()
            .map(|latest| ("latest", latest.to_string())),
        request
            .limit
            .as_ref()
            .map(|limit| ("limit", limit.to_string())),
        request
            .oldest
            .as_ref()
            .map(|oldest| ("oldest", oldest.to_string())),
        request.ts.as_ref().map(|ts| ("ts", ts.to_string())),
    ];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/conversations.replies");
    client
        .get(&url, &params[..])
        .map_err(RepliesError::Client)
        .and_then(|result| {
            serde_json::from_str::<RepliesResponse>(&result)
                .map_err(|e| RepliesError::MalformedResponse(result, e))
        })
}
/// Sets the purpose for a conversation.
///
/// Wraps https://api.slack.com/methods/conversations.setPurpose

pub fn set_purpose<R>(
    client: &R,
    request: &SetPurposeRequest,
) -> Result<SetPurposeResponse, SetPurposeError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        request
            .channel
            .as_ref()
            .map(|channel| ("channel", channel.to_string())),
        request
            .purpose
            .as_ref()
            .map(|purpose| ("purpose", purpose.to_string())),
    ];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/conversations.setPurpose");
    client
        .get(&url, &params[..])
        .map_err(SetPurposeError::Client)
        .and_then(|result| {
            serde_json::from_str::<SetPurposeResponse>(&result)
                .map_err(|e| SetPurposeError::MalformedResponse(result, e))
        })
}
/// Sets the topic for a conversation.
///
/// Wraps https://api.slack.com/methods/conversations.setTopic

pub fn set_topic<R>(
    client: &R,
    request: &SetTopicRequest,
) -> Result<SetTopicResponse, SetTopicError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        request
            .channel
            .as_ref()
            .map(|channel| ("channel", channel.to_string())),
        request
            .topic
            .as_ref()
            .map(|topic| ("topic", topic.to_string())),
    ];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/conversations.setTopic");
    client
        .get(&url, &params[..])
        .map_err(SetTopicError::Client)
        .and_then(|result| {
            serde_json::from_str::<SetTopicResponse>(&result)
                .map_err(|e| SetTopicError::MalformedResponse(result, e))
        })
}
/// Reverses conversation archival.
///
/// Wraps https://api.slack.com/methods/conversations.unarchive

pub fn unarchive<R>(
    client: &R,
    request: &UnarchiveRequest,
) -> Result<UnarchiveResponse, UnarchiveError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![request
        .channel
        .as_ref()
        .map(|channel| ("channel", channel.to_string()))];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/conversations.unarchive");
    client
        .get(&url, &params[..])
        .map_err(UnarchiveError::Client)
        .and_then(|result| {
            serde_json::from_str::<UnarchiveResponse>(&result)
                .map_err(|e| UnarchiveError::MalformedResponse(result, e))
        })
}