use crate::{queues::*, FlowTasks};
use colink::*;
use std::sync::Arc;
use tracing::debug;

struct Scheduler {
    cl: CoLink,
}

impl Scheduler {
    async fn _operator(
        &self,
        queue_name: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        let mut subscriber = self.cl.new_subscriber(queue_name).await?;
        println!("scheduler started.");
        loop {
            let data = subscriber.get_next().await?;
            debug!("Received [{}]", String::from_utf8_lossy(&data));
            let message: SubscriptionMessage = prost::Message::decode(&*data)?;
            if message.change_type != "delete" {
                let task_id: Task = prost::Message::decode(&*message.payload).unwrap();
                let res = self
                    .cl
                    .read_entry(&format!("_internal:tasks:{}", task_id.task_id))
                    .await?;
                let task: Task = prost::Message::decode(&*res).unwrap();
                if task.protocol_name != "coflows_dispatch" {
                    continue;
                }
                let json_str = String::from_utf8_lossy(&task.protocol_param);
                let flow_tasks: FlowTasks = serde_json::from_str(&json_str)?;
                let mut queues = QUEUES.lock().await;
                let queue = queues.get_mut(&flow_tasks.flow_id).unwrap();
                for _ in 0..flow_tasks.message_ids.len() {
                    queue.pop_front().unwrap(); // TODO check message_id == pop_front?
                }
                let mut message_ids = vec![];
                if !queue.is_empty() {
                    #[allow(clippy::get_first)] // TODO push more than one messages
                    let id = queue.get(0).unwrap().clone();
                    message_ids.push(id);
                }
                drop(queues);
                if !message_ids.is_empty() {
                    let participants = vec![Participant {
                        user_id: self.cl.get_user_id()?,
                        role: "local".to_string(),
                    }];
                    self.cl
                        .run_task(
                            "coflows_dispatch",
                            serde_json::to_string(&FlowTasks {
                                flow_id: flow_tasks.flow_id,
                                message_ids,
                            })?
                            .as_bytes(),
                            &participants,
                            false,
                        )
                        .await?;
                }
            }
        }
    }

    async fn operator(&self, queue_name: &str) -> Result<(), String> {
        match self._operator(queue_name).await {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }
}

pub struct SchedulerLauncher;
#[colink::async_trait]
impl ProtocolEntry for SchedulerLauncher {
    async fn start(
        &self,
        cl: CoLink,
        _param: Vec<u8>,
        _participants: Vec<Participant>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        let scheduler = Arc::new(Scheduler { cl: cl.clone() });
        let finished_task_queue_name = String::from_utf8_lossy(
            &cl.read_or_wait("coflows_scheduler:finished_task_queue_name")
                .await?,
        )
        .to_string();
        let operator = {
            let queue_name = finished_task_queue_name.clone();
            let scheduler = scheduler.clone();
            tokio::spawn(async move { scheduler.operator(&queue_name).await })
        };
        operator.await??;
        cl.unsubscribe(&finished_task_queue_name).await?;
        Ok(())
    }
}
