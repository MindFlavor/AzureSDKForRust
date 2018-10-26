use azure::core::errors::{check_status_extract_headers_and_body, AzureError};
use azure::core::lease::LeaseId;
use azure::core::{
    ClientRequestIdOption, ClientRequestIdSupport, ClientRequired, ContainerNameRequired, ContainerNameSupport, LeaseIdOption,
    LeaseIdSupport, TimeoutOption, TimeoutSupport,
};
use azure::core::{No, ToAssign, Yes};
use azure::storage::client::Client;
use futures::future::{done, ok, Future};
use hyper::{Method, StatusCode};
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct DeleteBuilder<'a, ContainerNameSet> {
    p_container_name: PhantomData<ContainerNameSet>,
    client: &'a Client,
    container_name: Option<&'a str>,
    timeout: Option<u64>,
    client_request_id: Option<&'a str>,
    lease_id: Option<&'a LeaseId>,
}

impl<'a, ContainerNameSet> ClientRequired<'a> for DeleteBuilder<'a, ContainerNameSet> {
    fn client(&self) -> &'a Client {
        self.client
    }
}

impl<'a, ContainerNameSet> ClientRequestIdOption<'a> for DeleteBuilder<'a, ContainerNameSet>
where
    ContainerNameSet: ToAssign,
{
    fn client_request_id(&self) -> Option<&'a str> {
        self.client_request_id
    }
}

impl<'a, ContainerNameSet> ClientRequestIdSupport<'a> for DeleteBuilder<'a, ContainerNameSet>
where
    ContainerNameSet: ToAssign,
{
    type O = DeleteBuilder<'a, ContainerNameSet>;

    fn with_client_request_id(self, client_request_id: &'a str) -> Self {
        DeleteBuilder {
            p_container_name: PhantomData {},
            client: self.client,
            container_name: self.container_name,
            timeout: self.timeout,
            client_request_id: Some(client_request_id),
            lease_id: self.lease_id,
        }
    }
}

impl<'a, ContainerNameSet> LeaseIdOption<'a> for DeleteBuilder<'a, ContainerNameSet>
where
    ContainerNameSet: ToAssign,
{
    fn lease_id(&self) -> Option<&'a LeaseId> {
        self.lease_id
    }
}

impl<'a, ContainerNameSet> LeaseIdSupport<'a> for DeleteBuilder<'a, ContainerNameSet>
where
    ContainerNameSet: ToAssign,
{
    type O = DeleteBuilder<'a, ContainerNameSet>;

    fn with_lease_id(self, lease_id: &'a LeaseId) -> Self::O {
        DeleteBuilder {
            p_container_name: PhantomData {},
            client: self.client,
            container_name: self.container_name,
            timeout: self.timeout,
            client_request_id: self.client_request_id,
            lease_id: Some(lease_id),
        }
    }
}

impl<'a, ContainerNameSet> TimeoutOption for DeleteBuilder<'a, ContainerNameSet>
where
    ContainerNameSet: ToAssign,
{
    fn timeout(&self) -> Option<u64> {
        self.timeout
    }
}

impl<'a, ContainerNameSet> TimeoutSupport for DeleteBuilder<'a, ContainerNameSet>
where
    ContainerNameSet: ToAssign,
{
    type O = DeleteBuilder<'a, ContainerNameSet>;

    fn with_timeout(self, timeout: u64) -> Self::O {
        DeleteBuilder {
            p_container_name: PhantomData {},
            client: self.client,
            container_name: self.container_name,
            timeout: Some(timeout),
            client_request_id: self.client_request_id,
            lease_id: self.lease_id,
        }
    }
}

impl<'a, ContainerNameSet> ContainerNameSupport<'a> for DeleteBuilder<'a, ContainerNameSet>
where
    ContainerNameSet: ToAssign,
{
    type O = DeleteBuilder<'a, Yes>;

    fn with_container_name(self, t: &'a str) -> Self::O {
        DeleteBuilder {
            p_container_name: PhantomData {},
            client: self.client,
            container_name: Some(t),
            timeout: self.timeout,
            client_request_id: self.client_request_id,
            lease_id: self.lease_id,
        }
    }
}

impl<'a> ContainerNameRequired<'a> for DeleteBuilder<'a, Yes> {
    fn container_name(&self) -> &'a str {
        self.container_name.unwrap()
    }
}

impl<'a> DeleteBuilder<'a, No> {
    pub fn new(client: &'a Client) -> DeleteBuilder<'a, No> {
        DeleteBuilder {
            p_container_name: PhantomData {},
            client,
            container_name: None,
            timeout: None,
            client_request_id: None,
            lease_id: None,
        }
    }
}

impl<'a> DeleteBuilder<'a, Yes> {
    pub fn finalize(self) -> impl Future<Item = (), Error = AzureError> {
        let mut uri = format!(
            "{}/{}?restype=container",
            self.client().uri_builder().blob_uri(),
            self.container_name()
        );

        if let Some(nm) = TimeoutOption::to_uri_parameter(&self) {
            uri = format!("{}&{}", uri, nm);
        }

        let req = self.client().perform_request(
            &uri,
            &Method::DELETE,
            |ref mut request| {
                ClientRequestIdOption::add_header(&self, request);
                LeaseIdOption::add_header(&self, request);
            },
            Some(&[]),
        );

        done(req).from_err().and_then(move |future_response| {
            check_status_extract_headers_and_body(future_response, StatusCode::ACCEPTED).and_then(|_| ok(()))
        })
    }
}

#[cfg(test)]
mod test {
    use azure::storage::client::Account;
    use super::*;

    #[test]
    fn alloc() {
        let client = Client::new(Account::Azure { account: "a".to_string(), key: "b".to_string() }).unwrap();
        let del = DeleteBuilder::new(&client).with_container_name("ciccio");
        println!("container_name == {}", del.container_name());
        // this would fail as Client was not set
        //del.finalize();
    }
}
