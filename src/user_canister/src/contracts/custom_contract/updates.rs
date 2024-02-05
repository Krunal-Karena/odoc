use candid::Error::Custom;
use ic_cdk::caller;
use ic_cdk_macros::update;

use crate::{CPayment, PaymentStatus, Wallet};
use crate::CustomContract;
use crate::user_history::UserHistory;


// #[update]
// fn release_c_payment(c_payment: CPayment) -> Result<(), String> {
//     if self.promises.contains(&c_payment) {
//         c_payment.clone().pay();
//         self.promises.retain(|payment| payment.id != c_payment.id);
//         self.payments.push(c_payment);
//         self.clone().save()?;
//         Ok(())
//     } else {
//         Err("Payment not found".to_string())
//     }
// }

#[update]
fn confirmed_c_payment(c_payment: CPayment) -> Result<(), String> {
    if let Some(mut contract) = CustomContract::get_for_user(c_payment.contract_id, c_payment.sender) {
        contract.promises = contract.promises.iter_mut().map(|payment| {
            if payment.id == c_payment.id && payment.receiver == caller() && payment.status == PaymentStatus::None {
                payment.status = PaymentStatus::Confirmed;
                let wallet = Wallet::get(c_payment.sender);
                wallet.add_dept(payment.amount.clone(), payment.id.clone());
                let mut user_history = UserHistory::get(c_payment.sender);
                user_history.promise_payment(payment.clone());
                user_history.save()
            }
            payment.clone()
        }).collect();

        contract.save()?;
        Ok(())
    } else {
        Err("Contract not found".to_string())
    }
}


#[update]
fn confirmed_cancellation(c_payment: CPayment) -> Result<(), String> {
    if let Some(mut contract) = CustomContract::get_for_user(c_payment.contract_id, c_payment.sender) {
        contract.promises = contract.promises.iter_mut().map(|payment| {
            if payment.id == c_payment.id && payment.receiver == caller() &&( payment.status == PaymentStatus::Canceled  || payment.status == PaymentStatus::RequestCancellation) {
                payment.status = PaymentStatus::ConfirmedCancellation;
                let wallet = Wallet::get(c_payment.sender);
                wallet.remove_dept(payment.id.clone());

                let mut user_history = UserHistory::get(c_payment.sender);
                user_history.confirm_cancellation(payment.clone());
                user_history.save()

            }
            payment.clone()
        }).collect();

        contract.save()?;
        Ok(())
    } else {
        Err("Contract not found".to_string())
    }
}


#[update]
fn approve_heigh_conform(c_payment: CPayment) -> Result<(), String> {
    if let Some(mut contract) = CustomContract::get_for_user(c_payment.contract_id, c_payment.sender) {
        contract.promises = contract.promises.iter_mut().map(|payment| {
            if payment.id == c_payment.id && payment.receiver == caller() && payment.status == PaymentStatus::HeighConformed {
                payment.status = PaymentStatus::ApproveHeighConformed;
                let wallet = Wallet::get(c_payment.sender);
                wallet.remove_dept(payment.id.clone());
                let mut user_history = UserHistory::get(c_payment.sender);
                user_history.promise_payment(payment.clone());
            }
            payment.clone()
        }).collect();

        contract.save()?;
        Ok(())
    } else {
        Err("Contract not found".to_string())
    }
}

#[update]
fn object_on_cancel(c_payment: CPayment, reason: String) -> Result<(), String> {
    if let Some(mut contract) = CustomContract::get_for_user(c_payment.contract_id, c_payment.sender) {
        contract.promises = contract.promises.iter_mut().map(|payment| {
            if payment.id == c_payment.id && payment.receiver == caller() && payment.status == PaymentStatus::Canceled {
                payment.status = PaymentStatus::Objected(reason.clone());
                  let wallet = Wallet::get(c_payment.sender);
                wallet.add_dept(payment.amount.clone(), payment.id.clone());

                let mut user_history = UserHistory::get(c_payment.sender);
                user_history.cancel_payment(payment.clone());
                user_history.save()
            }
            payment.clone()
        }).collect();

        contract.save()?;
        Ok(())
    } else {
        Err("Contract not found".to_string())
    }
}

#[update]
fn delete_custom_contract(id: String) -> Result<(), String> {
    let contract = CustomContract::get(id);
    if let Some(contract) = contract {
        return contract.delete();
    }
    Err("Not found".to_string())
}

