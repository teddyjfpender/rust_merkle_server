use std::str::FromStr;

use crate::models::{app_state::AppState, mutation::{SignedMutation, Mutation}};
use actix_web::{web, HttpResponse, Responder};
use mina_signer::{create_legacy, Signer, PubKey, BaseField, ScalarField, Signature};
use o1_utils::field_helpers::FieldHelpers;
use log::info;
use num_bigint::BigUint;

/*
// we can manually create a mutation and signature to test the API
curl -X POST http://127.0.0.1:8000/mutate \
-H "Content-Type: application/json" \
-d "{\"mutation\":{\"public_key\":\"B62qpDiVzRVFfbou8HceV5hdrVFNKWFXqHCouTnZtbxDtPVKBJbRxkx\",\"balance\":63},\"signature\":{\"r\":\"4763190888861912447828341346847260784994725016138374145789859678727541493286\",\"s\":\"4849312444676475516258499247907171411621146611730788623357703154010267203643\"}}" 
 */
fn parse_public_key(address: &str) -> Result<PubKey, HttpResponse> {
    PubKey::from_address(address).map_err(|e| {
        info!("Error parsing public key: {}. PublicKey: {}", e, address);
        HttpResponse::BadRequest().body("Failed to parse public key")
    })
}

fn convert_signature_component(component: &str) -> Result<BigUint, HttpResponse> {
    BigUint::from_str(component).map_err(|e| {
        info!("Error parsing component of signature: {}. Component: {}", e, component);
        HttpResponse::BadRequest().body("Failed to parse component of signature")
    })
}

fn to_base_field(value: &BigUint) -> Result<BaseField, HttpResponse> {
    BaseField::from_biguint(value).map_err(|e| {
        info!("Error converting component to BaseField: {}. Component: {}", e, value);
        HttpResponse::BadRequest().body("Failed to convert to BaseField")
    })
}

fn to_scalar_field(value: &BigUint) -> Result<ScalarField, HttpResponse> {
    ScalarField::from_biguint(value).map_err(|e| {
        info!("Error converting component to ScalarField: {}. Component: {}", e, value);
        HttpResponse::BadRequest().body("Failed to convert to ScalarField")
    })
}

pub async fn mutate_map(data: web::Data<AppState>, signed_mutation: web::Json<SignedMutation>) -> impl Responder {
    let mut ctx = create_legacy::<Mutation>(0);

    let public_key = match parse_public_key(&signed_mutation.mutation.public_key) {
        Ok(pk) => pk,
        Err(response) => return response
    };

    let rx_biguint = match convert_signature_component(&signed_mutation.signature.r) {
        Ok(val) => val,
        Err(response) => return response
    };

    let rx = match to_base_field(&rx_biguint) {
        Ok(val) => val,
        Err(response) => return response
    };

    let s_biguint = match convert_signature_component(&signed_mutation.signature.s) {
        Ok(val) => val,
        Err(response) => return response
    };

    let s = match to_scalar_field(&s_biguint) {
        Ok(val) => val,
        Err(response) => return response
    };

    let mutation = Mutation {
        public_key: signed_mutation.mutation.public_key.clone(),
        balance: signed_mutation.mutation.balance,
    };

    let signature = Signature::new(rx, s);
    if !ctx.verify(&signature, &public_key, &mutation) {
        info!("Verification failed for PublicKey: {}", signed_mutation.mutation.public_key);
        return HttpResponse::BadRequest().body("Invalid signature for provided mutation");
    }

    data.merkle_map.insert(signed_mutation.mutation.public_key.clone(), signed_mutation.mutation.balance);

    HttpResponse::Ok().body("Mutation accepted")
}
