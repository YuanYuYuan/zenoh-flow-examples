use async_trait::async_trait;
use zenoh_flow::prelude::*;
use async_std::sync::Arc;
use zenoh::{publication::Publisher, prelude::r#async::*};

mod config;

use config::Config;

struct PubSink {
    input: Input,
    session: Arc<Session>,
}

// struct WTF<'a> {
//     session: Arc<zenoh::Session>,
//     publisher: Publisher<'a>,
// }

impl PubSink {
    async fn new() -> Result<Self> {
        let session = zenoh::open(zenoh::config::peer()).res().await?;
        Ok(Self {
            // session: Arc::new(session),
            publisher
        })
    }
}

// #[async_trait]
// impl<'a> Node for PubSink<'a> {
//     async fn iteration(&self) -> Result<()> {
//         // if let Ok(data) = self.input.recv_async().await {

//         // }
//         Ok(())
//     }
// }

struct PubSinkFactory;

#[async_trait]
impl SinkFactoryTrait for PubSinkFactory {
    async fn new_sink(
        &self,
        _context: &mut Context,
        configuration: &Option<Configuration>,
        _inputs: Inputs,
    ) -> Result<Option<Arc<dyn Node>>> {
        let session = zenoh::open(zenoh::config::peer()).res().await?;
        let input =
        // let publisher = session.declare_publisher("key/expression").res().await?;
        let wtf = PubSink {
            // input: inputs,
            session
        };

        Ok(Some(Arc::new(wtf)))
        todo!()
    }
}
