use jot_proto::judge::v1::{
    JudgeRequest, Language, ResourceLimits, judge_service_client::JudgeServiceClient,
    judge_service_server::JudgeServiceServer,
};
use prost::Message;

#[test]
fn public_v1_types_round_trip() {
    let request = JudgeRequest {
        request_id: 42,
        language: Language::Cpp.into(),
        source_code: "int main() {}".into(),
        limits: Some(ResourceLimits {
            compile_time_ms: 1000,
            instruction_count: 1_000_000,
            memory_bytes: 64 * 1024 * 1024,
            build_artifact_bytes: 1024 * 1024,
        }),
    };

    let decoded = JudgeRequest::decode(request.encode_to_vec().as_slice()).unwrap();
    assert_eq!(decoded, request);

    let _client_type = std::any::type_name::<JudgeServiceClient<tonic::transport::Channel>>();
    let _server_type = std::any::type_name::<JudgeServiceServer<NoService>>();
}

struct NoService;
