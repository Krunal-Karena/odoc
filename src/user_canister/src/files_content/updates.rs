use std::collections::HashMap;
use ic_cdk::caller;

use ic_cdk_macros::update;

use crate::{PaymentContract, SharesContract, StoredContract};
use crate::files::FileNode;
use crate::files_content::ContentNode;
use crate::storage_schema::{ContentId, ContentTree, ContractId, FileId};

#[update]
fn content_updates(file_id: FileId, content_parent_id: Option<ContentId>, new_text: String) -> Result<String, String> {
    if FileNode::get(&file_id).is_none() {
        return Err("No such file with this id.".to_string());
    }
    let parent_id: ContentId = match content_parent_id {
        Some(id) => id,
        None => ContentNode::new(file_id.clone(), None, String::from(""), String::from(""), None).unwrap().id
    };
    let updated_content = ContentNode::new(file_id, Some(parent_id), String::from(""), new_text, None);
    Ok(format!("Content created successfully. Content ID: {}", updated_content.unwrap().id))
}

// #[update]
// fn save_one_file(
//     file: FileNode,
//     content_tree: HashMap<FileId, ContentTree>,
//     contracts: Vec<StoredContract>,
// ) -> Result<(), String> {
//     // check permissions
//     if caller().to_string() != file.author {
//         if let Some(share_id) = file.share_id {
//             let share_file = ShareFile::get_file(share_id);
//         };
//     }
//     Ok(())
// }

#[update]
fn multi_updates(
    files: Vec<FileNode>,
    content_trees: Vec<HashMap<FileId, ContentTree>>,
    contracts: Vec<StoredContract>,
    delete_contracts: Vec<ContractId>,
) -> Result<String, String> {
    let mut messages = "".to_string();
    // Update file names and parents or create
    for file in files {
        file.save()?;
    }


    // Update payment contracts
    // Note PaymentContract can be updated only if caller() == author/creator of the contract
    PaymentContract::update_payment_contracts(contracts.clone())?;


    // Update shares contracts
    SharesContract::update_shares_contracts(contracts)?;


    // Update FILE_CONTENTS
    for update in content_trees {
        for (file_id, content_tree) in update {
            ContentNode::update_file_contents(file_id, content_tree);
        }
    };

    for contract_id in delete_contracts {
        let payment = PaymentContract::get(caller(), contract_id.clone())?;
        payment.delete()?;
    }

    messages.push_str("Updates applied successfully.");
    Ok(messages)
}
