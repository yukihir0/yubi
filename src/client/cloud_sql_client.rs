use anyhow::Result;
use async_trait::async_trait;
use googapis::{
    google::cloud::sql::v1::{
        database_instance, sql_instances_service_client::SqlInstancesServiceClient,
        SqlInstancesGetRequest,
    },
    CERTIFICATES,
};
use gouth::Token;
use tonic::{
    metadata::MetadataValue,
    transport::{Certificate, Channel, ClientTlsConfig},
    Request,
};

#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg_attr(test, automock)]
#[async_trait]
pub trait CloudSqlClientTrait {
    async fn fetch_sql_instance_status(
        &self,
        project: &String,
        instance: &String,
    ) -> Result<database_instance::SqlInstanceState>;
}

pub struct CloudSqlClient {}

impl CloudSqlClient {
    pub fn new() -> CloudSqlClient {
        CloudSqlClient {}
    }
}

#[async_trait]
impl CloudSqlClientTrait for CloudSqlClient {
    async fn fetch_sql_instance_status(
        &self,
        project: &String,
        instance: &String,
    ) -> Result<database_instance::SqlInstanceState> {
        let token = Token::new().map_err(|e| {
            let msg = format!("{}", e);
            anyhow::Error::new(e).context(msg)
        })?;

        let tls_config = ClientTlsConfig::new()
            .ca_certificate(Certificate::from_pem(CERTIFICATES))
            .domain_name("sqladmin.googleapis.com");

        let channel = Channel::from_static("https://sqladmin.googleapis.com")
            .tls_config(tls_config)?
            .connect()
            .await
            .map_err(|e| {
                let msg = format!("{}", e);
                anyhow::Error::new(e).context(msg)
            })?;

        let mut client =
            SqlInstancesServiceClient::with_interceptor(channel, move |mut request: Request<()>| {
                let token = &*token.header_value().unwrap();
                let meta = MetadataValue::from_str(token).unwrap();
                request.metadata_mut().insert("authorization", meta);
                Ok(request)
            });

        let response = client
            .get(Request::new(SqlInstancesGetRequest {
                project: project.to_string(),
                instance: instance.to_string(),
                ..Default::default()
            }))
            .await
            .map_err(|e| {
                let msg = format!("{}", e.message());
                anyhow::Error::new(e).context(msg)
            })?;

        let state = match response.into_inner().state {
            0 => database_instance::SqlInstanceState::Unspecified,
            1 => database_instance::SqlInstanceState::Runnable,
            2 => database_instance::SqlInstanceState::Suspended,
            3 => database_instance::SqlInstanceState::PendingDelete,
            4 => database_instance::SqlInstanceState::PendingCreate,
            5 => database_instance::SqlInstanceState::Maintenance,
            6 => database_instance::SqlInstanceState::Failed,
            7 => database_instance::SqlInstanceState::OnlineMaintenance,
            _ => panic!("none state"),
        };

        Ok(state)
    }
}
