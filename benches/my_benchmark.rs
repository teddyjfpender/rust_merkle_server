//use std::str::FromStr;

use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use rand::{self, Rng};
use reqwest::blocking::Client;
use mina_signer::{create_legacy, Keypair, Signer};
use serde_json;
use o1_utils::field_helpers::FieldHelpers;
use rust_merkle_server::models::mutation::{Mutation, SignatureJSON, SignedMutation};
struct ApiRequest {
    signed_mutation: SignedMutation,
}

fn generate_requests() -> Vec<ApiRequest> {
    (0..10).map(|_| {
        let key_pair = Keypair::rand(&mut rand::rngs::OsRng).unwrap();
        let public_key = key_pair.public.into_address();
        let balance = rand::thread_rng().gen_range(0..100);
        
        let mutation = Mutation { public_key: public_key.clone(), balance };
        let mut ctx = create_legacy::<Mutation>(0);
        let signature = Signer::sign(&mut ctx, &key_pair, &mutation);
        
        let signature_json = SignatureJSON {
            r: signature.rx.to_biguint().to_string(),
            s: signature.s.to_biguint().to_string(),
        };
        let signed_mutation = SignedMutation {
            mutation: mutation.clone(),
            signature: signature_json,
        };
        // For API requests we need to convert the signature `r` to a string 
        // For verification we need to convert the string of `r` back to a BaseField
        /*let convert = BaseField::from_bytes(&signed_mutation.signature.r.as_bytes());
        println!("convert original {:?}", convert);
        let r_as_string = convert.unwrap().to_biguint().to_string();
        //print!("convert signature `r`: {:?} \n\n", r_as_string);
        let r_as_biguint = BigUint::from_str(&r_as_string).unwrap();

        let r_as_basefield = BaseField::from_biguint(&r_as_biguint);
        println!("r_as_basefield: {:?}\n\n", r_as_basefield);

        // then same for scalar
        
        let convert = ScalarField::from_bytes(&signed_mutation.signature.s.as_bytes());
        println!("convert original {:?}", convert);
        let s_as_string = convert.unwrap().to_biguint().to_string();
        print!("convert signature `s`: {:?} \n\n", s_as_string);
        let s_as_biguint = BigUint::from_str(&s_as_string).unwrap();

        let s_as_scalarfield = ScalarField::from_biguint(&s_as_biguint);
        println!("s_as_scalarfield: {:?}\n\n", s_as_scalarfield);
        */
        
        ApiRequest {
            signed_mutation,
        }
    }).collect()
}


fn benchmark_mutation(c: &mut Criterion) {
    let mut group = c.benchmark_group("Mutation Benchmarks");
    group.sample_size(2000);
    group.warm_up_time(std::time::Duration::from_secs(2));

    let client = Client::new();
    let requests = generate_requests();

    // perform health check
    let response = client.get("http://127.0.0.1:8000/health")
        .send()
        .unwrap();

    assert_eq!(response.status().as_u16(), 200);

    group.bench_function(BenchmarkId::new("mutate_balance", "v1"), |b| {
        b.iter(|| {
            for request in &requests {
                // Serialize SignedMutation to JSON string for the API request
                let body = serde_json::to_string(&request.signed_mutation).unwrap();

                let response = client.post("http://127.0.0.1:8000/mutate")
                    .header("Content-Type", "application/json")
                    .body(body)
                    .send()
                    .unwrap();

                // Ensure the request was successful
                assert_eq!(response.status().as_u16(), 200);

                // Optionally, check the response body too
                let response_body = response.text().unwrap();
                assert_eq!(response_body, "Mutation accepted");
            }
        })
    });

    group.finish();
}

criterion_group!(benches, benchmark_mutation);
criterion_main!(benches);