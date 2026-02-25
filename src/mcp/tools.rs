use crate::mcp::types::*;
use maplit::hashmap;
use reqwest::Client;
use rpc_router::{
    Handler, HandlerResult, IntoHandlerError, RouterBuilder, RpcParams,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;

/// register all tools to the router
pub fn register_tools(router_builder: RouterBuilder) -> RouterBuilder {
    router_builder
        .append_dyn("tools/list", tools_list.into_dyn())
        .append_dyn("create_op_return", create_op_return.into_dyn())
        .append_dyn("check_invoice_status", check_invoice_status.into_dyn())
}

pub async fn tools_list(_request: Option<ListToolsRequest>) -> HandlerResult<ListToolsResult> {
    let response = ListToolsResult {
		tools: vec![Tool {
			name: "create_op_return".to_string(),
			description: Some("Creates a lightning invoice, when this lightning invoice is paid, OP_RETURN Bot will create a bitcoin transaction that has an OP_RETURN to embed the given message into the bitcoin blockchain.".to_string()),
			input_schema: ToolInputSchema {
				type_name: "object".to_string(),
				properties: hashmap! {
                    "message".to_string() => ToolInputSchemaProperty {
                        type_name: Some("string".to_owned()),
                        description: Some("Message to be included in the OP_RETURN output".to_owned()),
                        enum_values: None,
                    }
                },
				required: vec!["message".to_string()],
			},
		},
		Tool {
			name: "check_invoice_status".to_string(),
			description: Some("Checks the payment status of a lightning invoice. Returns the bitcoin transaction ID if the invoice has been paid and the transaction broadcast, 'null' if paid but not yet broadcast, or an error if the invoice has not been paid.".to_string()),
			input_schema: ToolInputSchema {
				type_name: "object".to_string(),
				properties: hashmap! {
                    "invoice".to_string() => ToolInputSchemaProperty {
                        type_name: Some("string".to_owned()),
                        description: Some("The lightning invoice string (e.g. lnbc...) or the payment hash (rHash hex) to check".to_owned()),
                        enum_values: None,
                    }
                },
				required: vec!["invoice".to_string()],
			},
		}],
		next_cursor: None,
	};
    Ok(response)
}

#[derive(Deserialize, Serialize, RpcParams)]
pub struct CreateOpReturnRequest {
    pub message: String,
}

pub async fn create_op_return(request: CreateOpReturnRequest) -> HandlerResult<CallToolResult> {
	let mut params = HashMap::new();
	params.insert("message", request.message.as_str());
	let client = Client::new();
	let res = client
		.post("https://opreturnbot.com/api/create")
		.form(&params)
		.send()
		.await
		.map_err(|_| json!({"code": -32603, "message": "Internal error"}).into_handler_error())?
		.text()
		.await
		.map_err(|_| json!({"code": -32603, "message": "Internal error"}).into_handler_error())?;

	Ok(CallToolResult {
		is_error: !res.starts_with("lnbc"),
		content: vec![CallToolResultContent::Text { text: res }],
	})
}

#[derive(Deserialize, Serialize, RpcParams)]
pub struct CheckInvoiceStatusRequest {
    pub invoice: String,
}

pub async fn check_invoice_status(request: CheckInvoiceStatusRequest) -> HandlerResult<CallToolResult> {
	let client = Client::new();
	let url = format!("https://opreturnbot.com/api/status/{}", request.invoice);
	let res = client
		.get(&url)
		.send()
		.await
		.map_err(|_| json!({"code": -32603, "message": "Internal error"}).into_handler_error())?;

	let status = res.status();
	let body = res
		.text()
		.await
		.map_err(|_| json!({"code": -32603, "message": "Internal error"}).into_handler_error())?;

	Ok(CallToolResult {
		is_error: !status.is_success(),
		content: vec![CallToolResultContent::Text { text: body }],
	})
}
