//! The credentials Claude Code 2.1.322 keeps from every process it
//! starts (161): its hooks run in the environment its child-environment
//! builder gives, which drops each of its own list of credential
//! variables wherever one of them is set, and the memory service's token
//! with its address, whatever else the environment holds. A start's
//! record names none of them, since the SessionStart hook cannot see
//! them. Every fragment of the release this rests on is in
//! [`fragments`], which a test of the check-tools layer finds byte for
//! byte in the release the tree pins.

/// The credential variables 2.1.322 keeps from the processes it starts,
/// its own list in its order with the file-descriptor ones it spreads in
/// at the third place, then the memory service's token.
pub const WITHHELD: [&str; 20] = [
    "CLAUDE_CODE_OAUTH_TOKEN",
    "CLAUDE_CODE_SESSIONS_OAUTH_TOKEN",
    "CLAUDE_CODE_OAUTH_TOKEN_FILE_DESCRIPTOR",
    "CLAUDE_CODE_GATEWAY_TOKEN_FILE_DESCRIPTOR",
    "CLAUDE_CODE_API_KEY_FILE_DESCRIPTOR",
    "CLAUDE_CODE_WEBSOCKET_AUTH_FILE_DESCRIPTOR",
    "CCR_AGENT_PROXY_TOKEN_FILE_DESCRIPTOR",
    "CLAUDE_CODE_ARTIFACTS_API_TOKEN",
    "CLAUDE_CODE_SLACK_TAG_TOKEN",
    "CLAUDE_CODE_HFI_BEARER_TOKEN",
    "CLAUDE_BRIDGE_OAUTH_TOKEN",
    "CLAUDE_TRUSTED_DEVICE_TOKEN",
    "AGENT_PROXY_AUTH_TOKEN",
    "CLAUDE_CODE_MCP_SERVE_AUTH_TOKEN",
    "CLAUDE_BG_AUTH_SNAPSHOT_PATH",
    "CLAUDE_BG_SOCKET_TOKENS_PATH",
    "CLAUDE_BG_RV_AUTH",
    "CLAUDE_BG_PTY_AUTH",
    "CLAUDE_BG_CLAIM_AUTH",
    "CLAUDE_CODE_MEMORY_API_TOKEN",
];

/// How many of [`WITHHELD`] the release's list holds before it spreads in
/// its list of file-descriptor credentials.
const BEFORE: usize = 2;

/// How many of [`WITHHELD`], from the third, the release's list of
/// file-descriptor credentials holds.
const DESCRIPTORS: usize = 5;

/// The list `names` as the release's program writes one.
fn list(names: &[&str]) -> String {
    names
        .iter()
        .map(|name| format!("\"{name}\""))
        .collect::<Vec<_>>()
        .join(",")
}

/// The source fragments of 2.1.322 the list rests on, each named and as
/// the release's program holds it: its list of the file-descriptor
/// credentials, its list of credentials, the memory service's pair, the
/// set it tests the environment against, the names its builder drops,
/// the test that drops the memory service's pair, the drop itself, and
/// the hook runner's environment built on the builder's.
pub fn fragments() -> Vec<(&'static str, String)> {
    let (own, memory) = WITHHELD.split_at(WITHHELD.len() - 1);
    let descriptors = &own[BEFORE..BEFORE + DESCRIPTORS];
    let rest = &own[BEFORE + DESCRIPTORS..];
    vec![
        (
            "the list of the file-descriptor credentials",
            format!("Hne=[{}]", list(descriptors)),
        ),
        (
            "the list of the credentials kept from children",
            format!("uKn=[{},...Hne,{}]", list(&own[..BEFORE]), list(rest)),
        ),
        (
            "the memory service's address and token",
            format!("ls=[\"CLAUDE_CODE_MEMORY_API_BASE_URL\",{}]", list(memory)),
        ),
        (
            "the set the environment is tested against",
            "var K=new Set(uKn.map((n)=>n.toUpperCase()))".to_string(),
        ),
        (
            "the names the child environment drops",
            "return[...uKn.filter((r)=>!e(r)),...Object.keys(n).filter((r)=>Rnt(r.toUpperCase()))"
                .to_string(),
        ),
        (
            "the test that drops the memory service's pair",
            "return EOo(e)||Cqe(e)||".to_string(),
        ),
        (
            "the drop from the child environment",
            "if(Ve.has(o.toUpperCase())||Rnt(o.toUpperCase())||K.has(o.toUpperCase())&&!h(o,d)||"
                .to_string(),
        ),
        (
            "the hook runner's environment",
            "{...os()},...He,...Xpt(y),CLAUDE_PROJECT_DIR:".to_string(),
        ),
        (
            "the environment an organization's hook inherits, the same builder's",
            "inherited:{...os()}".to_string(),
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_lists_are_written_as_the_release_writes_them() {
        let fragments = fragments();
        assert_eq!(
            fragments[0].1,
            "Hne=[\"CLAUDE_CODE_OAUTH_TOKEN_FILE_DESCRIPTOR\",\"CLAUDE_CODE_GATEWAY_TOKEN_FILE_DESCRIPTOR\",\"CLAUDE_CODE_API_KEY_FILE_DESCRIPTOR\",\"CLAUDE_CODE_WEBSOCKET_AUTH_FILE_DESCRIPTOR\",\"CCR_AGENT_PROXY_TOKEN_FILE_DESCRIPTOR\"]"
        );
        assert!(
            fragments[1].1.starts_with(
                "uKn=[\"CLAUDE_CODE_OAUTH_TOKEN\",\"CLAUDE_CODE_SESSIONS_OAUTH_TOKEN\",...Hne,\"CLAUDE_CODE_ARTIFACTS_API_TOKEN\","
            ),
            "{}",
            fragments[1].1
        );
        assert!(
            fragments[1].1.ends_with(",\"CLAUDE_BG_CLAIM_AUTH\"]"),
            "{}",
            fragments[1].1
        );
        assert_eq!(
            fragments[2].1,
            "ls=[\"CLAUDE_CODE_MEMORY_API_BASE_URL\",\"CLAUDE_CODE_MEMORY_API_TOKEN\"]"
        );
    }
}
