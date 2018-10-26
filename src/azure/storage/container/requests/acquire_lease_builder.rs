use azure::core::errors::{check_status_extract_headers_and_body, AzureError};
use azure::core::headers::LEASE_ACTION;
use azure::core::lease::LeaseId;
use azure::core::{
    ClientRequestIdOption, ClientRequestIdSupport, ClientRequired, ContainerNameRequired, ContainerNameSupport, LeaseDurationRequired,
    LeaseDurationSupport, LeaseIdOption, LeaseIdSupport, ProposedLeaseIdOption, ProposedLeaseIdSupport, TimeoutOption, TimeoutSupport,
};
use azure::core::{No, ToAssign, Yes};
use azure::storage::client::Client;
use azure::storage::container::responses::AcquireLeaseResponse;
use futures::future::{done, Future};
use hyper::{Method, StatusCode};
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct AcquireLeaseBuilder<'a, ContainerNameSet, LeaseDurationSet>
where
    ContainerNameSet: ToAssign,
    LeaseDurationSet: ToAssign,
{
    client: &'a Client,
    p_container_name: PhantomData<ContainerNameSet>,
    p_lease_duration: PhantomData<LeaseDurationSet>,
    container_name: Option<&'a str>,
    client_request_id: Option<&'a str>,
    timeout: Option<u64>,
    lease_id: Option<&'a LeaseId>,
    lease_duration: Option<i8>,
    proposed_lease_id: Option<&'a LeaseId>,
}

impl<'a> AcquireLeaseBuilder<'a, No, No> {
    pub(crate) fn new(client: &'a Client) -> AcquireLeaseBuilder<'a, No, No> {
        AcquireLeaseBuilder {
            client,
            p_container_name: PhantomData {},
            container_name: None,
            p_lease_duration: PhantomData {},
            lease_duration: None,
            client_request_id: None,
            timeout: None,
            lease_id: None,
            proposed_lease_id: None,
        }
    }
}

impl<'a, ContainerNameSet, LeaseDurationSet> ClientRequired<'a> for AcquireLeaseBuilder<'a, ContainerNameSet, LeaseDurationSet>
where
    ContainerNameSet: ToAssign,
    LeaseDurationSet: ToAssign,
{
    fn client(&self) -> &'a Client {
        self.client
    }
}

impl<'a, LeaseDurationSet> ContainerNameRequired<'a> for AcquireLeaseBuilder<'a, Yes, LeaseDurationSet>
where
    LeaseDurationSet: ToAssign,
{
    fn container_name(&self) -> &'a str {
        self.container_name.unwrap()
    }
}

impl<'a, ContainerNameSet, LeaseDurationSet> ClientRequestIdOption<'a> for AcquireLeaseBuilder<'a, ContainerNameSet, LeaseDurationSet>
where
    ContainerNameSet: ToAssign,
    LeaseDurationSet: ToAssign,
{
    fn client_request_id(&self) -> Option<&'a str> {
        self.client_request_id
    }
}

impl<'a, ContainerNameSet, LeaseDurationSet> TimeoutOption for AcquireLeaseBuilder<'a, ContainerNameSet, LeaseDurationSet>
where
    ContainerNameSet: ToAssign,
    LeaseDurationSet: ToAssign,
{
    fn timeout(&self) -> Option<u64> {
        self.timeout
    }
}

impl<'a, ContainerNameSet, LeaseDurationSet> LeaseIdOption<'a> for AcquireLeaseBuilder<'a, ContainerNameSet, LeaseDurationSet>
where
    ContainerNameSet: ToAssign,
    LeaseDurationSet: ToAssign,
{
    fn lease_id(&self) -> Option<&'a LeaseId> {
        self.lease_id
    }
}

impl<'a, ContainerNameSet> LeaseDurationRequired for AcquireLeaseBuilder<'a, ContainerNameSet, Yes>
where
    ContainerNameSet: ToAssign,
{
    fn lease_duration(&self) -> i8 {
        self.lease_duration.unwrap()
    }
}

impl<'a, ContainerNameSet, LeaseDurationSet> ProposedLeaseIdOption<'a> for AcquireLeaseBuilder<'a, ContainerNameSet, LeaseDurationSet>
where
    ContainerNameSet: ToAssign,
    LeaseDurationSet: ToAssign,
{
    fn proposed_lease_id(&self) -> Option<&'a LeaseId> {
        self.proposed_lease_id
    }
}

impl<'a, ContainerNameSet, LeaseDurationSet> ContainerNameSupport<'a> for AcquireLeaseBuilder<'a, ContainerNameSet, LeaseDurationSet>
where
    ContainerNameSet: ToAssign,
    LeaseDurationSet: ToAssign,
{
    type O = AcquireLeaseBuilder<'a, Yes, LeaseDurationSet>;

    fn with_container_name(self, container_name: &'a str) -> Self::O {
        AcquireLeaseBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_lease_duration: PhantomData {},
            container_name: Some(container_name),
            client_request_id: self.client_request_id,
            timeout: self.timeout,
            lease_id: self.lease_id,
            lease_duration: self.lease_duration,
            proposed_lease_id: self.proposed_lease_id,
        }
    }
}

impl<'a, ContainerNameSet, LeaseDurationSet> ClientRequestIdSupport<'a> for AcquireLeaseBuilder<'a, ContainerNameSet, LeaseDurationSet>
where
    ContainerNameSet: ToAssign,
    LeaseDurationSet: ToAssign,
{
    type O = AcquireLeaseBuilder<'a, ContainerNameSet, LeaseDurationSet>;

    fn with_client_request_id(self, client_request_id: &'a str) -> Self::O {
        AcquireLeaseBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_lease_duration: PhantomData {},
            container_name: self.container_name,
            client_request_id: Some(client_request_id),
            timeout: self.timeout,
            lease_id: self.lease_id,
            lease_duration: self.lease_duration,
            proposed_lease_id: self.proposed_lease_id,
        }
    }
}

impl<'a, ContainerNameSet, LeaseDurationSet> TimeoutSupport for AcquireLeaseBuilder<'a, ContainerNameSet, LeaseDurationSet>
where
    ContainerNameSet: ToAssign,
    LeaseDurationSet: ToAssign,
{
    type O = AcquireLeaseBuilder<'a, ContainerNameSet, LeaseDurationSet>;

    fn with_timeout(self, timeout: u64) -> Self::O {
        AcquireLeaseBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_lease_duration: PhantomData {},
            container_name: self.container_name,
            client_request_id: self.client_request_id,
            timeout: Some(timeout),
            lease_id: self.lease_id,
            lease_duration: self.lease_duration,
            proposed_lease_id: self.proposed_lease_id,
        }
    }
}

impl<'a, ContainerNameSet, LeaseDurationSet> LeaseIdSupport<'a> for AcquireLeaseBuilder<'a, ContainerNameSet, LeaseDurationSet>
where
    ContainerNameSet: ToAssign,
    LeaseDurationSet: ToAssign,
{
    type O = AcquireLeaseBuilder<'a, ContainerNameSet, LeaseDurationSet>;

    fn with_lease_id(self, lease_id: &'a LeaseId) -> Self::O {
        AcquireLeaseBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_lease_duration: PhantomData {},
            container_name: self.container_name,
            client_request_id: self.client_request_id,
            timeout: self.timeout,
            lease_id: Some(lease_id),
            lease_duration: self.lease_duration,
            proposed_lease_id: self.proposed_lease_id,
        }
    }
}

impl<'a, ContainerNameSet, LeaseDurationSet> LeaseDurationSupport for AcquireLeaseBuilder<'a, ContainerNameSet, LeaseDurationSet>
where
    ContainerNameSet: ToAssign,
    LeaseDurationSet: ToAssign,
{
    type O = AcquireLeaseBuilder<'a, ContainerNameSet, Yes>;

    fn with_lease_duration(self, lease_duration: i8) -> Self::O {
        AcquireLeaseBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_lease_duration: PhantomData {},
            container_name: self.container_name,
            client_request_id: self.client_request_id,
            timeout: self.timeout,
            lease_id: self.lease_id,
            lease_duration: Some(lease_duration),
            proposed_lease_id: self.proposed_lease_id,
        }
    }
}

impl<'a, ContainerNameSet, LeaseDurationSet> ProposedLeaseIdSupport<'a> for AcquireLeaseBuilder<'a, ContainerNameSet, LeaseDurationSet>
where
    ContainerNameSet: ToAssign,
    LeaseDurationSet: ToAssign,
{
    type O = AcquireLeaseBuilder<'a, ContainerNameSet, LeaseDurationSet>;

    fn with_proposed_lease_id(self, proposed_lease_id: &'a LeaseId) -> Self::O {
        AcquireLeaseBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_lease_duration: PhantomData {},
            container_name: self.container_name,
            client_request_id: self.client_request_id,
            timeout: self.timeout,
            lease_id: self.lease_id,
            lease_duration: self.lease_duration,
            proposed_lease_id: Some(proposed_lease_id),
        }
    }
}

// methods callable regardless
impl<'a, ContainerNameSet, LeaseDurationSet> AcquireLeaseBuilder<'a, ContainerNameSet, LeaseDurationSet>
where
    ContainerNameSet: ToAssign,
    LeaseDurationSet: ToAssign,
{}

impl<'a> AcquireLeaseBuilder<'a, Yes, Yes> {
    pub fn finalize(self) -> impl Future<Item = AcquireLeaseResponse, Error = AzureError> {
        let mut uri = format!(
            "{}/{}?comp=lease&restype=container",
            self.client().uri_builder().blob_uri(),
            self.container_name()
        );

        if let Some(nm) = TimeoutOption::to_uri_parameter(&self) {
            uri = format!("{}&{}", uri, nm);
        }

        let req = self.client().perform_request(
            &uri,
            &Method::PUT,
            |ref mut request| {
                ClientRequestIdOption::add_header(&self, request);
                LeaseIdOption::add_header(&self, request);
                request.header(LEASE_ACTION, "acquire");
                LeaseDurationRequired::add_header(&self, request);
                ProposedLeaseIdOption::add_header(&self, request);
            },
            Some(&[]),
        );

        done(req)
            .from_err()
            .and_then(move |future_response| check_status_extract_headers_and_body(future_response, StatusCode::CREATED))
            .and_then(|(headers, _body)| done(AcquireLeaseResponse::from_headers(&headers)))
    }
}
