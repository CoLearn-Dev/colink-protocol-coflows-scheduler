use colink::*;

pub struct Init;
#[colink::async_trait]
impl ProtocolEntry for Init {
    async fn start(
        &self,
        cl: CoLink,
        _param: Vec<u8>,
        _participants: Vec<Participant>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        let task_queue_name = cl
            .subscribe("_internal:tasks:status:finished:latest", None)
            .await?;
        cl.update_entry(
            "coflows_scheduler:finished_task_queue_name",
            task_queue_name.as_bytes(),
        )
        .await?;
        let participants = vec![Participant {
            user_id: cl.get_user_id()?,
            role: "local".to_string(),
        }];
        cl.run_task(
            "coflows_scheduler",
            Default::default(),
            &participants,
            false,
        )
        .await?;
        Ok(())
    }
}
