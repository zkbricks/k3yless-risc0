use jwt_core::CustomClaims;
use jwt_methods::VALIDATOR_ID;
use jwt_validator::prove_token_validation;

fn main() {
    let claims = CustomClaims {
        subject: "Hello, world!".to_string(),
    };

    let (receipt, _) = prove_token_validation(&claims);

    receipt
        .verify(VALIDATOR_ID)
        .expect("Proof of validation should successfully verify");
}
