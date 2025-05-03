use crate::mcp::types::*;
use rpc_router::{HandlerError, HandlerResult, IntoHandlerError};
use serde_json::json;

pub async fn prompts_list(
    _request: Option<ListPromptsRequest>,
) -> HandlerResult<ListPromptsResult> {
    let response = ListPromptsResult {
		next_cursor: None,
		prompts: vec![
			Prompt {
				name: "create_op_return".to_string(),
				description: Some("Creates a lightning invoice, when this lightning invoice is paid, OP_RETURN Bot will create a bitcoin transaction that has an OP_RETURN to embed the given message into the bitcoin blockchain.".to_string()),
				arguments: Some(vec![PromptArgument {
					name: "message".to_string(),
					description: Some("Message to be included in the OP_RETURN output".to_string()),
					required: Some(true),
				}]),
			},
			Prompt {
				name: "get_opreturn_message".to_string(),
				description: Some("Takes a given bitcoin transaction id and looks up the OP_RETURN message associated with it.".to_string()),
				arguments: Some(vec![PromptArgument {
					name: "txid".to_string(),
					description: Some("The transaction id".to_string()),
					required: Some(true),
				}]),
			},
		],
	};
    Ok(response)
}

pub async fn prompts_get(request: GetPromptRequest) -> HandlerResult<PromptResult> {
    let list = prompts_list(None).await?;

    match list.prompts.into_iter().find(|p| p.name == request.name) {
        Some(p) => {
            let res = PromptResult {
                description: p.description.unwrap(),
                messages: Some(vec![PromptMessage {
                    role: "user".to_string(),
                    content: PromptMessageContent {
                        type_name: "text".to_string(),
                        text: format!(
                            "Create a OP_RETURN with this message {}",
                            request.arguments.unwrap()["message"].as_str().unwrap()
                        ),
                    },
                }]),
            };
            Ok(res)
        }
        None => Err(json!({"code": -32602, "message": "Prompt not found"}).into_handler_error()),
    }
}
