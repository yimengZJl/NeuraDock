use tauri::State;
use crate::application::ResultExt;


use crate::application::dtos::ProviderNodeDto;
use crate::presentation::state::AppState;

#[tauri::command]
#[specta::specta]
pub async fn get_provider_nodes(
    provider_id: String,
    state: State<'_, AppState>,
) -> Result<Vec<ProviderNodeDto>, String> {
    let provider_id_obj = neuradock_domain::shared::ProviderId::from_string(&provider_id);
    let provider = state
        .provider_repo
        .find_by_id(&provider_id_obj)
        .await
        .to_string_err()?
        .ok_or_else(|| format!("Provider {} not found", provider_id))?;

    let mut nodes = vec![ProviderNodeDto {
        id: provider_id.clone(),
        name: provider.name().to_string(),
        base_url: provider.domain().to_string(),
    }];

    // Add custom nodes
    let custom_nodes = state
        .custom_node_repo
        .find_by_provider(&provider_id_obj)
        .await
        .to_string_err()?;

    for custom_node in custom_nodes {
        nodes.push(ProviderNodeDto {
            id: format!("custom_{}", custom_node.id().value()),
            name: custom_node.name().to_string(),
            base_url: custom_node.base_url().to_string(),
        });
    }

    Ok(nodes)
}

#[tauri::command]
#[specta::specta]
pub async fn add_custom_node(
    provider_id: String,
    name: String,
    base_url: String,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let provider_id_obj = neuradock_domain::shared::ProviderId::from_string(&provider_id);
    let node = neuradock_domain::custom_node::CustomProviderNode::create(
        provider_id_obj,
        name.clone(),
        base_url.clone(),
    );

    state
        .custom_node_repo
        .create(&node)
        .await
        .to_string_err()?;

    Ok(format!("Custom node '{}' added successfully", name))
}

#[tauri::command]
#[specta::specta]
pub async fn delete_custom_node(
    node_id: i64,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let id = neuradock_domain::custom_node::CustomNodeId::new(node_id);

    state
        .custom_node_repo
        .delete(&id)
        .await
        .to_string_err()?;

    Ok("Custom node deleted successfully".to_string())
}
