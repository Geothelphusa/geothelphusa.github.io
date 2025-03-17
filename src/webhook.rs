// `build.rs` で生成された `webhook.rs` を `include!` で埋め込む
include!(concat!(env!("OUT_DIR"), "/webhook.rs"));
