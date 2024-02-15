use colink_protocol_coflows_scheduler::coflows_push::{Initiator, Receiver};
use colink_protocol_coflows_scheduler::coflows_scheduler::SchedulerLauncher;
use colink_protocol_coflows_scheduler::init::Init;

colink::protocol_start!(
    ("coflows_scheduler:@init", Init),
    ("coflows_scheduler:local", SchedulerLauncher),
    ("coflows_push:initiator", Initiator),
    ("coflows_push:receiver", Receiver)
);
