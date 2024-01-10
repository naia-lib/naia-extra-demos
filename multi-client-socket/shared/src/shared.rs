use naia_socket_shared::{LinkConditionerConfig, SocketConfig};

pub const PING_MSG_A: &str = "PING A";
pub const PONG_MSG_A: &str = "PONG A";

pub fn shared_config_a() -> SocketConfig {
    //let link_condition = None;
    let link_condition = Some(LinkConditionerConfig::average_condition());
    //    let link_condition = Some(LinkConditionerConfig {
    //        incoming_latency: 500,
    //        incoming_jitter: 1,
    //        incoming_loss: 0.0,
    //        incoming_corruption: 0.0
    //    });

    SocketConfig::new(link_condition, None)
}
