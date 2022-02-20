/*
 * This file is part of CycloneDX Rust Cargo.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::{
    external_models::{normalized_string::NormalizedString, uri::Uri},
    models,
    utilities::{convert_optional, convert_optional_vec, convert_vec},
    xml::{to_xml_write_error, write_simple_tag, ToInnerXml, ToXml},
};
use serde::{Deserialize, Serialize};
use xml::writer::XmlEvent;

use crate::specs::v1_3::{
    external_reference::ExternalReferences, license::Licenses, organization::OrganizationalEntity,
    property::Properties,
};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(transparent)]
pub(crate) struct Services(Vec<Service>);

impl From<models::Services> for Services {
    fn from(other: models::Services) -> Self {
        Services(convert_vec(other.0))
    }
}

impl From<Services> for models::Services {
    fn from(other: Services) -> Self {
        models::Services(convert_vec(other.0))
    }
}

const SERVICES_TAG: &str = "services";

impl ToXml for Services {
    fn write_xml_element<W: std::io::Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
    ) -> Result<(), crate::errors::XmlWriteError> {
        writer
            .write(XmlEvent::start_element(SERVICES_TAG))
            .map_err(to_xml_write_error(SERVICES_TAG))?;

        for service in &self.0 {
            service.write_xml_element(writer)?;
        }

        writer
            .write(XmlEvent::end_element())
            .map_err(to_xml_write_error(SERVICES_TAG))?;
        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Service {
    #[serde(rename = "bom-ref")]
    bom_ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<OrganizationalEntity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    group: Option<String>,
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    endpoints: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    authenticated: Option<bool>,
    #[serde(rename = "x-trust-boundary", skip_serializing_if = "Option::is_none")]
    x_trust_boundary: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Vec<DataClassification>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    licenses: Option<Licenses>,
    #[serde(skip_serializing_if = "Option::is_none")]
    external_references: Option<ExternalReferences>,
    #[serde(skip_serializing_if = "Option::is_none")]
    properties: Option<Properties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    services: Option<Services>,
}

impl From<models::Service> for Service {
    fn from(other: models::Service) -> Self {
        Self {
            bom_ref: other.bom_ref,
            provider: convert_optional(other.provider),
            group: other.group.map(|g| g.to_string()),
            name: other.name.to_string(),
            version: other.version.map(|v| v.to_string()),
            description: other.description.map(|d| d.to_string()),
            endpoints: other
                .endpoints
                .map(|endpoints| endpoints.into_iter().map(|e| e.to_string()).collect()),
            authenticated: other.authenticated,
            x_trust_boundary: other.x_trust_boundary,
            data: convert_optional_vec(other.data),
            licenses: convert_optional(other.licenses),
            external_references: convert_optional(other.external_references),
            properties: convert_optional(other.properties),
            services: convert_optional(other.services),
        }
    }
}

impl From<Service> for models::Service {
    fn from(other: Service) -> Self {
        Self {
            bom_ref: other.bom_ref,
            provider: convert_optional(other.provider),
            group: other.group.map(NormalizedString::new_unchecked),
            name: NormalizedString::new_unchecked(other.name),
            version: other.version.map(NormalizedString::new_unchecked),
            description: other.description.map(NormalizedString::new_unchecked),
            endpoints: other
                .endpoints
                .map(|endpoints| endpoints.into_iter().map(Uri).collect()),
            authenticated: other.authenticated,
            x_trust_boundary: other.x_trust_boundary,
            data: convert_optional_vec(other.data),
            licenses: convert_optional(other.licenses),
            external_references: convert_optional(other.external_references),
            properties: convert_optional(other.properties),
            services: convert_optional(other.services),
        }
    }
}

const SERVICE_TAG: &str = "service";
const BOM_REF_ATTR: &str = "bom-ref";
const PROVIDER_TAG: &str = "provider";
const GROUP_TAG: &str = "group";
const NAME_TAG: &str = "name";
const VERSION_TAG: &str = "version";
const DESCRIPTION_TAG: &str = "description";
const ENDPOINTS_TAG: &str = "endpoints";
const ENDPOINT_TAG: &str = "endpoint";
const AUTHENTICATED_TAG: &str = "authenticated";
const X_TRUST_BOUNDARY_TAG: &str = "x-trust-boundary";
const DATA_TAG: &str = "data";

impl ToXml for Service {
    fn write_xml_element<W: std::io::Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
    ) -> Result<(), crate::errors::XmlWriteError> {
        let mut service_start_tag = XmlEvent::start_element(SERVICE_TAG);

        if let Some(bom_ref) = &self.bom_ref {
            service_start_tag = service_start_tag.attr(BOM_REF_ATTR, bom_ref);
        }

        writer
            .write(service_start_tag)
            .map_err(to_xml_write_error(SERVICE_TAG))?;

        if let Some(provider) = &self.provider {
            provider.write_xml_named_element(writer, PROVIDER_TAG)?;
        }

        if let Some(group) = &self.group {
            write_simple_tag(writer, GROUP_TAG, group)?;
        }

        write_simple_tag(writer, NAME_TAG, &self.name)?;

        if let Some(version) = &self.version {
            write_simple_tag(writer, VERSION_TAG, version)?;
        }

        if let Some(description) = &self.description {
            write_simple_tag(writer, DESCRIPTION_TAG, description)?;
        }

        if let Some(endpoints) = &self.endpoints {
            writer
                .write(XmlEvent::start_element(ENDPOINTS_TAG))
                .map_err(to_xml_write_error(ENDPOINTS_TAG))?;
            for endpoint in endpoints {
                write_simple_tag(writer, ENDPOINT_TAG, endpoint)?;
            }
            writer
                .write(XmlEvent::end_element())
                .map_err(to_xml_write_error(ENDPOINTS_TAG))?;
        }

        if let Some(authenticated) = &self.authenticated {
            write_simple_tag(writer, AUTHENTICATED_TAG, &format!("{}", authenticated))?;
        }

        if let Some(x_trust_boundary) = &self.x_trust_boundary {
            write_simple_tag(
                writer,
                X_TRUST_BOUNDARY_TAG,
                &format!("{}", x_trust_boundary),
            )?;
        }

        if let Some(data) = &self.data {
            writer
                .write(XmlEvent::start_element(DATA_TAG))
                .map_err(to_xml_write_error(DATA_TAG))?;
            for d in data {
                d.write_xml_element(writer)?;
            }
            writer
                .write(XmlEvent::end_element())
                .map_err(to_xml_write_error(DATA_TAG))?;
        }

        if let Some(licenses) = &self.licenses {
            licenses.write_xml_element(writer)?;
        }

        if let Some(external_references) = &self.external_references {
            external_references.write_xml_element(writer)?;
        }

        if let Some(properties) = &self.properties {
            properties.write_xml_element(writer)?;
        }

        if let Some(services) = &self.services {
            services.write_xml_element(writer)?;
        }

        writer
            .write(XmlEvent::end_element())
            .map_err(to_xml_write_error(SERVICE_TAG))?;

        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
struct DataClassification {
    flow: String,
    classification: String,
}

impl From<models::DataClassification> for DataClassification {
    fn from(other: models::DataClassification) -> Self {
        Self {
            flow: other.flow.to_string(),
            classification: other.classification.to_string(),
        }
    }
}

impl From<DataClassification> for models::DataClassification {
    fn from(other: DataClassification) -> Self {
        Self {
            flow: models::DataFlowType::new_unchecked(&other.flow),
            classification: NormalizedString::new_unchecked(other.classification),
        }
    }
}

const CLASSIFICATION_TAG: &str = "classification";
const FLOW_ATTR: &str = "flow";

impl ToXml for DataClassification {
    fn write_xml_element<W: std::io::Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
    ) -> Result<(), crate::errors::XmlWriteError> {
        writer
            .write(XmlEvent::start_element(CLASSIFICATION_TAG).attr(FLOW_ATTR, &self.flow))
            .map_err(to_xml_write_error(CLASSIFICATION_TAG))?;

        writer
            .write(XmlEvent::characters(&self.classification))
            .map_err(to_xml_write_error(CLASSIFICATION_TAG))?;

        writer
            .write(XmlEvent::end_element())
            .map_err(to_xml_write_error(CLASSIFICATION_TAG))?;

        Ok(())
    }
}

#[cfg(test)]
pub(crate) mod test {
    use super::*;
    use crate::{
        specs::v1_3::{
            external_reference::test::{
                corresponding_external_references, example_external_references,
            },
            license::test::{corresponding_licenses, example_licenses},
            organization::test::{corresponding_entity, example_entity},
            property::test::{corresponding_properties, example_properties},
        },
        xml::test::write_element_to_string,
    };

    pub(crate) fn example_services() -> Services {
        Services(vec![example_service()])
    }

    pub(crate) fn corresponding_services() -> models::Services {
        models::Services(vec![corresponding_service()])
    }

    pub(crate) fn example_service() -> Service {
        Service {
            bom_ref: Some("bom-ref".to_string()),
            provider: Some(example_entity()),
            group: Some("group".to_string()),
            name: "name".to_string(),
            version: Some("version".to_string()),
            description: Some("description".to_string()),
            endpoints: Some(vec!["endpoint".to_string()]),
            authenticated: Some(true),
            x_trust_boundary: Some(true),
            data: Some(vec![example_data_classification()]),
            licenses: Some(example_licenses()),
            external_references: Some(example_external_references()),
            properties: Some(example_properties()),
            services: Some(Services(vec![])),
        }
    }

    pub(crate) fn corresponding_service() -> models::Service {
        models::Service {
            bom_ref: Some("bom-ref".to_string()),
            provider: Some(corresponding_entity()),
            group: Some(NormalizedString::new_unchecked("group".to_string())),
            name: NormalizedString::new_unchecked("name".to_string()),
            version: Some(NormalizedString::new_unchecked("version".to_string())),
            description: Some(NormalizedString::new_unchecked("description".to_string())),
            endpoints: Some(vec![Uri("endpoint".to_string())]),
            authenticated: Some(true),
            x_trust_boundary: Some(true),
            data: Some(vec![corresponding_data_classification()]),
            licenses: Some(corresponding_licenses()),
            external_references: Some(corresponding_external_references()),
            properties: Some(corresponding_properties()),
            services: Some(models::Services(vec![])),
        }
    }

    fn example_data_classification() -> DataClassification {
        DataClassification {
            flow: "flow".to_string(),
            classification: "classification".to_string(),
        }
    }

    fn corresponding_data_classification() -> models::DataClassification {
        models::DataClassification {
            flow: models::DataFlowType::UnknownDataFlow("flow".to_string()),
            classification: NormalizedString::new_unchecked("classification".to_string()),
        }
    }

    #[test]
    fn it_should_write_xml_full() {
        let xml_output = write_element_to_string(example_services());
        insta::assert_snapshot!(xml_output);
    }
}
