use crate::{queues::*, FlowTask, FlowTasks};
use colink::*;
use std::collections::VecDeque;

pub struct Initiator;
#[colink::async_trait]
impl ProtocolEntry for Initiator {
    async fn start(
        &self,
        cl: CoLink,
        param: Vec<u8>,
        participants: Vec<Participant>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        let json_str = String::from_utf8_lossy(&param);
        let flow_task: FlowTask = serde_json::from_str(&json_str)?;
        let msg = cl.read_entry(&flow_task.message_id).await?;
        cl.send_variable("msg", &msg, &[participants[1].clone()])
            .await?;
        Ok(())
    }
}

pub struct Receiver;
#[colink::async_trait]
impl ProtocolEntry for Receiver {
    async fn start(
        &self,
        cl: CoLink,
        param: Vec<u8>,
        participants: Vec<Participant>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        let json_str = String::from_utf8_lossy(&param);
        let flow_task: FlowTask = serde_json::from_str(&json_str)?;
        let message_id = if participants.len() == 1 {
            flow_task.message_id
        } else {
            let msg = cl.recv_variable("msg", &participants[0]).await?;
            let message_id = flow_task.message_id;
            cl.create_entry(&message_id, &msg).await?;
            message_id
        };
        let mut queues = QUEUES.lock().await;
        if !queues.contains_key(&flow_task.flow_id) {
            let mut dispatch_point = "coflows_dispatch".to_string();
            if let Ok(res) = cl
                .read_entry(&format!("instance_metadata:{}", flow_task.flow_id))
                .await
            {
                let json_str = String::from_utf8_lossy(&res);
                let data: serde_json::Value = serde_json::from_str(&json_str)?;
                let flow_type = data["flow_type"].as_str().unwrap();
                let user_id = data["user_id"].as_str().unwrap();
                if let Ok(res) = cl
                    .read_entry(&format!("flows:{}:default_dispatch_point", flow_type))
                    .await
                {
                    dispatch_point = String::from_utf8_lossy(&res).to_string();
                }
                if let Ok(res) = cl
                    .read_entry(&format!(
                        "flows:{}:mounts:{}:{}:dispatch_point_override",
                        flow_type, user_id, flow_task.flow_id
                    ))
                    .await
                {
                    dispatch_point = String::from_utf8_lossy(&res).to_string();
                }
            }
            queues.insert(
                flow_task.flow_id.clone(),
                (dispatch_point.clone(), VecDeque::new()),
            );
        }
        let (dispatch_point, queue) = queues.get_mut(&flow_task.flow_id).unwrap();
        let dispatch_point = dispatch_point.clone();
        if queue.is_empty() {
            queue.push_back(message_id.clone());
            drop(queues);
            let message_ids = vec![message_id.clone()];
            let participants = vec![Participant {
                user_id: cl.get_user_id()?,
                role: "local".to_string(),
            }];
            cl.run_task(
                &dispatch_point,
                serde_json::to_string(&FlowTasks {
                    flow_id: flow_task.flow_id,
                    message_ids,
                })?
                .as_bytes(),
                &participants,
                false,
            )
            .await?;
        } else {
            queue.push_back(message_id);
            drop(queues);
        }
        Ok(())
    }
}
