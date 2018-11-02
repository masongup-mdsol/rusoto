// =================================================================
//
//                           * WARNING *
//
//                    This file is generated!
//
//  Changes made to this file will be overwritten. If changes are
//  required to the generated code, the service_crategen project
//  must be updated to generate the changes.
//
// =================================================================

use std::error::Error;
use std::fmt;
use std::io;

#[allow(warnings)]
use futures::future;
use futures::Future;
use rusoto_core::region;
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::{Client, RusotoFuture};

use rusoto_core::credential::{CredentialsError, ProvideAwsCredentials};
use rusoto_core::request::HttpDispatchError;

use rusoto_core::param::{Params, ServiceParams};
use rusoto_core::signature::SignedRequest;
use rusoto_core::xmlerror::*;
use rusoto_core::xmlutil::{
    characters, end_element, find_start_element, peek_at_name, skip_tree, start_element,
};
use rusoto_core::xmlutil::{Next, Peek, XmlParseError, XmlResponse};
use serde_urlencoded;
use std::str::FromStr;
use xml::reader::ParserConfig;
use xml::reader::XmlEvent;
use xml::EventReader;

enum DeserializerNext {
    Close,
    Skip,
    Element(String),
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct AddRoleToDBClusterMessage {
    /// <p>The name of the DB cluster to associate the IAM role with.</p>
    pub db_cluster_identifier: String,
    /// <p>The Amazon Resource Name (ARN) of the IAM role to associate with the Neptune DB cluster, for example <code>arn:aws:iam::123456789012:role/NeptuneAccessRole</code>.</p>
    pub role_arn: String,
}

/// Serialize `AddRoleToDBClusterMessage` contents to a `SignedRequest`.
struct AddRoleToDBClusterMessageSerializer;
impl AddRoleToDBClusterMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &AddRoleToDBClusterMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "DBClusterIdentifier"),
            &obj.db_cluster_identifier,
        );
        params.put(&format!("{}{}", prefix, "RoleArn"), &obj.role_arn);
    }
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct AddSourceIdentifierToSubscriptionMessage {
    /// <p><p>The identifier of the event source to be added.</p> <p>Constraints:</p> <ul> <li> <p>If the source type is a DB instance, then a <code>DBInstanceIdentifier</code> must be supplied.</p> </li> <li> <p>If the source type is a DB security group, a <code>DBSecurityGroupName</code> must be supplied.</p> </li> <li> <p>If the source type is a DB parameter group, a <code>DBParameterGroupName</code> must be supplied.</p> </li> <li> <p>If the source type is a DB snapshot, a <code>DBSnapshotIdentifier</code> must be supplied.</p> </li> </ul></p>
    pub source_identifier: String,
    /// <p>The name of the event notification subscription you want to add a source identifier to.</p>
    pub subscription_name: String,
}

/// Serialize `AddSourceIdentifierToSubscriptionMessage` contents to a `SignedRequest`.
struct AddSourceIdentifierToSubscriptionMessageSerializer;
impl AddSourceIdentifierToSubscriptionMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &AddSourceIdentifierToSubscriptionMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "SourceIdentifier"),
            &obj.source_identifier,
        );
        params.put(
            &format!("{}{}", prefix, "SubscriptionName"),
            &obj.subscription_name,
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct AddSourceIdentifierToSubscriptionResult {
    pub event_subscription: Option<EventSubscription>,
}

struct AddSourceIdentifierToSubscriptionResultDeserializer;
impl AddSourceIdentifierToSubscriptionResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<AddSourceIdentifierToSubscriptionResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = AddSourceIdentifierToSubscriptionResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "EventSubscription" => {
                        obj.event_subscription = Some(try!(
                            EventSubscriptionDeserializer::deserialize("EventSubscription", stack)
                        ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct AddTagsToResourceMessage {
    /// <p>The Amazon Neptune resource that the tags are added to. This value is an Amazon Resource Name (ARN). For information about creating an ARN, see <a href="http://docs.aws.amazon.com/neptune/latest/UserGuide/tagging.ARN.html#tagging.ARN.Constructing"> Constructing an Amazon Resource Name (ARN)</a>.</p>
    pub resource_name: String,
    /// <p>The tags to be assigned to the Amazon Neptune resource.</p>
    pub tags: Vec<Tag>,
}

/// Serialize `AddTagsToResourceMessage` contents to a `SignedRequest`.
struct AddTagsToResourceMessageSerializer;
impl AddTagsToResourceMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &AddTagsToResourceMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(&format!("{}{}", prefix, "ResourceName"), &obj.resource_name);
        TagListSerializer::serialize(params, &format!("{}{}", prefix, "Tag"), &obj.tags);
    }
}

struct ApplyMethodDeserializer;
impl ApplyMethodDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ApplyPendingMaintenanceActionMessage {
    /// <p>The pending maintenance action to apply to this resource.</p> <p>Valid values: <code>system-update</code>, <code>db-upgrade</code> </p>
    pub apply_action: String,
    /// <p><p>A value that specifies the type of opt-in request, or undoes an opt-in request. An opt-in request of type <code>immediate</code> can&#39;t be undone.</p> <p>Valid values:</p> <ul> <li> <p> <code>immediate</code> - Apply the maintenance action immediately.</p> </li> <li> <p> <code>next-maintenance</code> - Apply the maintenance action during the next maintenance window for the resource.</p> </li> <li> <p> <code>undo-opt-in</code> - Cancel any existing <code>next-maintenance</code> opt-in requests.</p> </li> </ul></p>
    pub opt_in_type: String,
    /// <p>The Amazon Resource Name (ARN) of the resource that the pending maintenance action applies to. For information about creating an ARN, see <a href="http://docs.aws.amazon.com/neptune/latest/UserGuide/tagging.ARN.html#tagging.ARN.Constructing"> Constructing an Amazon Resource Name (ARN)</a>.</p>
    pub resource_identifier: String,
}

/// Serialize `ApplyPendingMaintenanceActionMessage` contents to a `SignedRequest`.
struct ApplyPendingMaintenanceActionMessageSerializer;
impl ApplyPendingMaintenanceActionMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ApplyPendingMaintenanceActionMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(&format!("{}{}", prefix, "ApplyAction"), &obj.apply_action);
        params.put(&format!("{}{}", prefix, "OptInType"), &obj.opt_in_type);
        params.put(
            &format!("{}{}", prefix, "ResourceIdentifier"),
            &obj.resource_identifier,
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct ApplyPendingMaintenanceActionResult {
    pub resource_pending_maintenance_actions: Option<ResourcePendingMaintenanceActions>,
}

struct ApplyPendingMaintenanceActionResultDeserializer;
impl ApplyPendingMaintenanceActionResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ApplyPendingMaintenanceActionResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ApplyPendingMaintenanceActionResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "ResourcePendingMaintenanceActions" => {
                        obj.resource_pending_maintenance_actions = Some(try!(
                            ResourcePendingMaintenanceActionsDeserializer::deserialize(
                                "ResourcePendingMaintenanceActions",
                                stack
                            )
                        ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct AttributeValueListDeserializer;
impl AttributeValueListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "AttributeValue" {
                        obj.push(try!(StringDeserializer::deserialize(
                            "AttributeValue",
                            stack
                        )));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}

/// Serialize `AttributeValueList` contents to a `SignedRequest`.
struct AttributeValueListSerializer;
impl AttributeValueListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

/// <p><p>Contains Availability Zone information.</p> <p> This data type is used as an element in the following data type:</p> <ul> <li> <p> <a>OrderableDBInstanceOption</a> </p> </li> </ul></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct AvailabilityZone {
    /// <p>The name of the availability zone.</p>
    pub name: Option<String>,
}

struct AvailabilityZoneDeserializer;
impl AvailabilityZoneDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<AvailabilityZone, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = AvailabilityZone::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Name" => {
                        obj.name = Some(try!(StringDeserializer::deserialize("Name", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct AvailabilityZoneListDeserializer;
impl AvailabilityZoneListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<AvailabilityZone>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "AvailabilityZone" {
                        obj.push(try!(AvailabilityZoneDeserializer::deserialize(
                            "AvailabilityZone",
                            stack
                        )));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}
struct AvailabilityZonesDeserializer;
impl AvailabilityZonesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "AvailabilityZone" {
                        obj.push(try!(StringDeserializer::deserialize(
                            "AvailabilityZone",
                            stack
                        )));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}

/// Serialize `AvailabilityZones` contents to a `SignedRequest`.
struct AvailabilityZonesSerializer;
impl AvailabilityZonesSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

struct BooleanDeserializer;
impl BooleanDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<bool, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = bool::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct BooleanOptionalDeserializer;
impl BooleanOptionalDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<bool, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = bool::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p> This data type is used as a response element in the action <a>DescribeDBEngineVersions</a>. </p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CharacterSet {
    /// <p>The description of the character set.</p>
    pub character_set_description: Option<String>,
    /// <p>The name of the character set.</p>
    pub character_set_name: Option<String>,
}

struct CharacterSetDeserializer;
impl CharacterSetDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CharacterSet, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = CharacterSet::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "CharacterSetDescription" => {
                        obj.character_set_description = Some(try!(
                            StringDeserializer::deserialize("CharacterSetDescription", stack)
                        ));
                    }
                    "CharacterSetName" => {
                        obj.character_set_name = Some(try!(StringDeserializer::deserialize(
                            "CharacterSetName",
                            stack
                        )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>The configuration setting for the log types to be enabled for export to CloudWatch Logs for a specific DB instance or DB cluster.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CloudwatchLogsExportConfiguration {
    /// <p>The list of log types to disable.</p>
    pub disable_log_types: Option<Vec<String>>,
    /// <p>The list of log types to enable.</p>
    pub enable_log_types: Option<Vec<String>>,
}

/// Serialize `CloudwatchLogsExportConfiguration` contents to a `SignedRequest`.
struct CloudwatchLogsExportConfigurationSerializer;
impl CloudwatchLogsExportConfigurationSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &CloudwatchLogsExportConfiguration) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.disable_log_types {
            LogTypeListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "DisableLogTypes"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.enable_log_types {
            LogTypeListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "EnableLogTypes"),
                field_value,
            );
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct CopyDBClusterParameterGroupMessage {
    /// <p><p>The identifier or Amazon Resource Name (ARN) for the source DB cluster parameter group. For information about creating an ARN, see <a href="http://docs.aws.amazon.com/neptune/latest/UserGuide/tagging.ARN.html#tagging.ARN.Constructing"> Constructing an Amazon Resource Name (ARN)</a>. </p> <p>Constraints:</p> <ul> <li> <p>Must specify a valid DB cluster parameter group.</p> </li> <li> <p>If the source DB cluster parameter group is in the same AWS Region as the copy, specify a valid DB parameter group identifier, for example <code>my-db-cluster-param-group</code>, or a valid ARN.</p> </li> <li> <p>If the source DB parameter group is in a different AWS Region than the copy, specify a valid DB cluster parameter group ARN, for example <code>arn:aws:rds:us-east-1:123456789012:cluster-pg:custom-cluster-group1</code>.</p> </li> </ul></p>
    pub source_db_cluster_parameter_group_identifier: String,
    pub tags: Option<Vec<Tag>>,
    /// <p>A description for the copied DB cluster parameter group.</p>
    pub target_db_cluster_parameter_group_description: String,
    /// <p>The identifier for the copied DB cluster parameter group.</p> <p>Constraints:</p> <ul> <li> <p>Cannot be null, empty, or blank</p> </li> <li> <p>Must contain from 1 to 255 letters, numbers, or hyphens</p> </li> <li> <p>First character must be a letter</p> </li> <li> <p>Cannot end with a hyphen or contain two consecutive hyphens</p> </li> </ul> <p>Example: <code>my-cluster-param-group1</code> </p>
    pub target_db_cluster_parameter_group_identifier: String,
}

/// Serialize `CopyDBClusterParameterGroupMessage` contents to a `SignedRequest`.
struct CopyDBClusterParameterGroupMessageSerializer;
impl CopyDBClusterParameterGroupMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &CopyDBClusterParameterGroupMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "SourceDBClusterParameterGroupIdentifier"),
            &obj.source_db_cluster_parameter_group_identifier,
        );
        if let Some(ref field_value) = obj.tags {
            TagListSerializer::serialize(params, &format!("{}{}", prefix, "Tag"), field_value);
        }
        params.put(
            &format!("{}{}", prefix, "TargetDBClusterParameterGroupDescription"),
            &obj.target_db_cluster_parameter_group_description,
        );
        params.put(
            &format!("{}{}", prefix, "TargetDBClusterParameterGroupIdentifier"),
            &obj.target_db_cluster_parameter_group_identifier,
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct CopyDBClusterParameterGroupResult {
    pub db_cluster_parameter_group: Option<DBClusterParameterGroup>,
}

struct CopyDBClusterParameterGroupResultDeserializer;
impl CopyDBClusterParameterGroupResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CopyDBClusterParameterGroupResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = CopyDBClusterParameterGroupResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "DBClusterParameterGroup" => {
                        obj.db_cluster_parameter_group =
                            Some(try!(DBClusterParameterGroupDeserializer::deserialize(
                                "DBClusterParameterGroup",
                                stack
                            )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CopyDBClusterSnapshotMessage {
    /// <p>True to copy all tags from the source DB cluster snapshot to the target DB cluster snapshot, and otherwise false. The default is false.</p>
    pub copy_tags: Option<bool>,
    /// <p>The AWS AWS KMS key ID for an encrypted DB cluster snapshot. The KMS key ID is the Amazon Resource Name (ARN), KMS key identifier, or the KMS key alias for the KMS encryption key. </p> <p>If you copy an unencrypted DB cluster snapshot and specify a value for the <code>KmsKeyId</code> parameter, Amazon Neptune encrypts the target DB cluster snapshot using the specified KMS encryption key. </p> <p>If you copy an encrypted DB cluster snapshot from your AWS account, you can specify a value for <code>KmsKeyId</code> to encrypt the copy with a new KMS encryption key. If you don't specify a value for <code>KmsKeyId</code>, then the copy of the DB cluster snapshot is encrypted with the same KMS key as the source DB cluster snapshot. </p> <p>If you copy an encrypted DB cluster snapshot that is shared from another AWS account, then you must specify a value for <code>KmsKeyId</code>. </p> <p>To copy an encrypted DB cluster snapshot to another AWS Region, you must set <code>KmsKeyId</code> to the KMS key ID you want to use to encrypt the copy of the DB cluster snapshot in the destination AWS Region. KMS encryption keys are specific to the AWS Region that they are created in, and you can't use encryption keys from one AWS Region in another AWS Region.</p>
    pub kms_key_id: Option<String>,
    /// <p>The URL that contains a Signature Version 4 signed request for the <code>CopyDBClusterSnapshot</code> API action in the AWS Region that contains the source DB cluster snapshot to copy. The <code>PreSignedUrl</code> parameter must be used when copying an encrypted DB cluster snapshot from another AWS Region.</p> <p>The pre-signed URL must be a valid request for the <code>CopyDBSClusterSnapshot</code> API action that can be executed in the source AWS Region that contains the encrypted DB cluster snapshot to be copied. The pre-signed URL request must contain the following parameter values:</p> <ul> <li> <p> <code>KmsKeyId</code> - The AWS KMS key identifier for the key to use to encrypt the copy of the DB cluster snapshot in the destination AWS Region. This is the same identifier for both the <code>CopyDBClusterSnapshot</code> action that is called in the destination AWS Region, and the action contained in the pre-signed URL.</p> </li> <li> <p> <code>DestinationRegion</code> - The name of the AWS Region that the DB cluster snapshot will be created in.</p> </li> <li> <p> <code>SourceDBClusterSnapshotIdentifier</code> - The DB cluster snapshot identifier for the encrypted DB cluster snapshot to be copied. This identifier must be in the Amazon Resource Name (ARN) format for the source AWS Region. For example, if you are copying an encrypted DB cluster snapshot from the us-west-2 AWS Region, then your <code>SourceDBClusterSnapshotIdentifier</code> looks like the following example: <code>arn:aws:rds:us-west-2:123456789012:cluster-snapshot:neptune-cluster1-snapshot-20161115</code>.</p> </li> </ul> <p>To learn how to generate a Signature Version 4 signed request, see <a href="http://docs.aws.amazon.com/AmazonS3/latest/API/sigv4-query-string-auth.html"> Authenticating Requests: Using Query Parameters (AWS Signature Version 4)</a> and <a href="http://docs.aws.amazon.com/general/latest/gr/signature-version-4.html"> Signature Version 4 Signing Process</a>.</p>
    pub pre_signed_url: Option<String>,
    /// <p>The identifier of the DB cluster snapshot to copy. This parameter is not case-sensitive.</p> <p>You can't copy an encrypted, shared DB cluster snapshot from one AWS Region to another.</p> <p>Constraints:</p> <ul> <li> <p>Must specify a valid system snapshot in the "available" state.</p> </li> <li> <p>If the source snapshot is in the same AWS Region as the copy, specify a valid DB snapshot identifier.</p> </li> <li> <p>If the source snapshot is in a different AWS Region than the copy, specify a valid DB cluster snapshot ARN. </p> </li> </ul> <p>Example: <code>my-cluster-snapshot1</code> </p>
    pub source_db_cluster_snapshot_identifier: String,
    pub tags: Option<Vec<Tag>>,
    /// <p>The identifier of the new DB cluster snapshot to create from the source DB cluster snapshot. This parameter is not case-sensitive.</p> <p>Constraints:</p> <ul> <li> <p>Must contain from 1 to 63 letters, numbers, or hyphens.</p> </li> <li> <p>First character must be a letter.</p> </li> <li> <p>Cannot end with a hyphen or contain two consecutive hyphens.</p> </li> </ul> <p>Example: <code>my-cluster-snapshot2</code> </p>
    pub target_db_cluster_snapshot_identifier: String,
}

/// Serialize `CopyDBClusterSnapshotMessage` contents to a `SignedRequest`.
struct CopyDBClusterSnapshotMessageSerializer;
impl CopyDBClusterSnapshotMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &CopyDBClusterSnapshotMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.copy_tags {
            params.put(
                &format!("{}{}", prefix, "CopyTags"),
                &field_value.to_string(),
            );
        }
        if let Some(ref field_value) = obj.kms_key_id {
            params.put(&format!("{}{}", prefix, "KmsKeyId"), &field_value);
        }
        if let Some(ref field_value) = obj.pre_signed_url {
            params.put(&format!("{}{}", prefix, "PreSignedUrl"), &field_value);
        }
        params.put(
            &format!("{}{}", prefix, "SourceDBClusterSnapshotIdentifier"),
            &obj.source_db_cluster_snapshot_identifier,
        );
        if let Some(ref field_value) = obj.tags {
            TagListSerializer::serialize(params, &format!("{}{}", prefix, "Tag"), field_value);
        }
        params.put(
            &format!("{}{}", prefix, "TargetDBClusterSnapshotIdentifier"),
            &obj.target_db_cluster_snapshot_identifier,
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct CopyDBClusterSnapshotResult {
    pub db_cluster_snapshot: Option<DBClusterSnapshot>,
}

struct CopyDBClusterSnapshotResultDeserializer;
impl CopyDBClusterSnapshotResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CopyDBClusterSnapshotResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = CopyDBClusterSnapshotResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "DBClusterSnapshot" => {
                        obj.db_cluster_snapshot = Some(try!(
                            DBClusterSnapshotDeserializer::deserialize("DBClusterSnapshot", stack)
                        ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CopyDBParameterGroupMessage {
    /// <p><p> The identifier or ARN for the source DB parameter group. For information about creating an ARN, see <a href="http://docs.aws.amazon.com/neptune/latest/UserGuide/tagging.ARN.html#tagging.ARN.Constructing"> Constructing an Amazon Resource Name (ARN)</a>. </p> <p>Constraints:</p> <ul> <li> <p>Must specify a valid DB parameter group.</p> </li> <li> <p>Must specify a valid DB parameter group identifier, for example <code>my-db-param-group</code>, or a valid ARN.</p> </li> </ul></p>
    pub source_db_parameter_group_identifier: String,
    pub tags: Option<Vec<Tag>>,
    /// <p>A description for the copied DB parameter group.</p>
    pub target_db_parameter_group_description: String,
    /// <p>The identifier for the copied DB parameter group.</p> <p>Constraints:</p> <ul> <li> <p>Cannot be null, empty, or blank</p> </li> <li> <p>Must contain from 1 to 255 letters, numbers, or hyphens</p> </li> <li> <p>First character must be a letter</p> </li> <li> <p>Cannot end with a hyphen or contain two consecutive hyphens</p> </li> </ul> <p>Example: <code>my-db-parameter-group</code> </p>
    pub target_db_parameter_group_identifier: String,
}

/// Serialize `CopyDBParameterGroupMessage` contents to a `SignedRequest`.
struct CopyDBParameterGroupMessageSerializer;
impl CopyDBParameterGroupMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &CopyDBParameterGroupMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "SourceDBParameterGroupIdentifier"),
            &obj.source_db_parameter_group_identifier,
        );
        if let Some(ref field_value) = obj.tags {
            TagListSerializer::serialize(params, &format!("{}{}", prefix, "Tag"), field_value);
        }
        params.put(
            &format!("{}{}", prefix, "TargetDBParameterGroupDescription"),
            &obj.target_db_parameter_group_description,
        );
        params.put(
            &format!("{}{}", prefix, "TargetDBParameterGroupIdentifier"),
            &obj.target_db_parameter_group_identifier,
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct CopyDBParameterGroupResult {
    pub db_parameter_group: Option<DBParameterGroup>,
}

struct CopyDBParameterGroupResultDeserializer;
impl CopyDBParameterGroupResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CopyDBParameterGroupResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = CopyDBParameterGroupResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "DBParameterGroup" => {
                        obj.db_parameter_group = Some(try!(
                            DBParameterGroupDeserializer::deserialize("DBParameterGroup", stack)
                        ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateDBClusterMessage {
    /// <p>A list of EC2 Availability Zones that instances in the DB cluster can be created in. </p>
    pub availability_zones: Option<Vec<String>>,
    /// <p><p>The number of days for which automated backups are retained. You must specify a minimum value of 1.</p> <p>Default: 1</p> <p>Constraints:</p> <ul> <li> <p>Must be a value from 1 to 35</p> </li> </ul></p>
    pub backup_retention_period: Option<i64>,
    /// <p>A value that indicates that the DB cluster should be associated with the specified CharacterSet.</p>
    pub character_set_name: Option<String>,
    /// <p>The DB cluster identifier. This parameter is stored as a lowercase string.</p> <p>Constraints:</p> <ul> <li> <p>Must contain from 1 to 63 letters, numbers, or hyphens.</p> </li> <li> <p>First character must be a letter.</p> </li> <li> <p>Cannot end with a hyphen or contain two consecutive hyphens.</p> </li> </ul> <p>Example: <code>my-cluster1</code> </p>
    pub db_cluster_identifier: String,
    /// <p><p> The name of the DB cluster parameter group to associate with this DB cluster. If this argument is omitted, the default is used. </p> <p>Constraints:</p> <ul> <li> <p>If supplied, must match the name of an existing DBClusterParameterGroup.</p> </li> </ul></p>
    pub db_cluster_parameter_group_name: Option<String>,
    /// <p>A DB subnet group to associate with this DB cluster.</p> <p>Constraints: Must match the name of an existing DBSubnetGroup. Must not be default.</p> <p>Example: <code>mySubnetgroup</code> </p>
    pub db_subnet_group_name: Option<String>,
    /// <p>The name for your database of up to 64 alpha-numeric characters. If you do not provide a name, Amazon Neptune will not create a database in the DB cluster you are creating.</p>
    pub database_name: Option<String>,
    /// <p>True to enable mapping of AWS Identity and Access Management (IAM) accounts to database accounts, and otherwise false.</p> <p>Default: <code>false</code> </p>
    pub enable_iam_database_authentication: Option<bool>,
    /// <p>The name of the database engine to be used for this DB cluster.</p> <p>Valid Values: <code>neptune</code> </p>
    pub engine: String,
    /// <p>The version number of the database engine to use.</p> <p>Example: <code>1.0.1</code> </p>
    pub engine_version: Option<String>,
    /// <p>The AWS KMS key identifier for an encrypted DB cluster.</p> <p>The KMS key identifier is the Amazon Resource Name (ARN) for the KMS encryption key. If you are creating a DB cluster with the same AWS account that owns the KMS encryption key used to encrypt the new DB cluster, then you can use the KMS key alias instead of the ARN for the KMS encryption key.</p> <p>If an encryption key is not specified in <code>KmsKeyId</code>:</p> <ul> <li> <p>If <code>ReplicationSourceIdentifier</code> identifies an encrypted source, then Amazon Neptune will use the encryption key used to encrypt the source. Otherwise, Amazon Neptune will use your default encryption key. </p> </li> <li> <p>If the <code>StorageEncrypted</code> parameter is true and <code>ReplicationSourceIdentifier</code> is not specified, then Amazon Neptune will use your default encryption key.</p> </li> </ul> <p>AWS KMS creates the default encryption key for your AWS account. Your AWS account has a different default encryption key for each AWS Region.</p> <p>If you create a Read Replica of an encrypted DB cluster in another AWS Region, you must set <code>KmsKeyId</code> to a KMS key ID that is valid in the destination AWS Region. This key is used to encrypt the Read Replica in that AWS Region.</p>
    pub kms_key_id: Option<String>,
    /// <p>The password for the master database user. This password can contain any printable ASCII character except "/", """, or "@".</p> <p>Constraints: Must contain from 8 to 41 characters.</p>
    pub master_user_password: Option<String>,
    /// <p><p>The name of the master user for the DB cluster.</p> <p>Constraints:</p> <ul> <li> <p>Must be 1 to 16 letters or numbers.</p> </li> <li> <p>First character must be a letter.</p> </li> <li> <p>Cannot be a reserved word for the chosen database engine.</p> </li> </ul></p>
    pub master_username: Option<String>,
    /// <p>A value that indicates that the DB cluster should be associated with the specified option group.</p> <p>Permanent options can't be removed from an option group. The option group can't be removed from a DB cluster once it is associated with a DB cluster.</p>
    pub option_group_name: Option<String>,
    /// <p>The port number on which the instances in the DB cluster accept connections.</p> <p> Default: <code>8182</code> </p>
    pub port: Option<i64>,
    /// <p>A URL that contains a Signature Version 4 signed request for the <code>CreateDBCluster</code> action to be called in the source AWS Region where the DB cluster is replicated from. You only need to specify <code>PreSignedUrl</code> when you are performing cross-region replication from an encrypted DB cluster.</p> <p>The pre-signed URL must be a valid request for the <code>CreateDBCluster</code> API action that can be executed in the source AWS Region that contains the encrypted DB cluster to be copied.</p> <p>The pre-signed URL request must contain the following parameter values:</p> <ul> <li> <p> <code>KmsKeyId</code> - The AWS KMS key identifier for the key to use to encrypt the copy of the DB cluster in the destination AWS Region. This should refer to the same KMS key for both the <code>CreateDBCluster</code> action that is called in the destination AWS Region, and the action contained in the pre-signed URL.</p> </li> <li> <p> <code>DestinationRegion</code> - The name of the AWS Region that Read Replica will be created in.</p> </li> <li> <p> <code>ReplicationSourceIdentifier</code> - The DB cluster identifier for the encrypted DB cluster to be copied. This identifier must be in the Amazon Resource Name (ARN) format for the source AWS Region. For example, if you are copying an encrypted DB cluster from the us-west-2 AWS Region, then your <code>ReplicationSourceIdentifier</code> would look like Example: <code>arn:aws:rds:us-west-2:123456789012:cluster:neptune-cluster1</code>.</p> </li> </ul> <p>To learn how to generate a Signature Version 4 signed request, see <a href="http://docs.aws.amazon.com/AmazonS3/latest/API/sigv4-query-string-auth.html"> Authenticating Requests: Using Query Parameters (AWS Signature Version 4)</a> and <a href="http://docs.aws.amazon.com/general/latest/gr/signature-version-4.html"> Signature Version 4 Signing Process</a>.</p>
    pub pre_signed_url: Option<String>,
    /// <p><p>The daily time range during which automated backups are created if automated backups are enabled using the <code>BackupRetentionPeriod</code> parameter. </p> <p>The default is a 30-minute window selected at random from an 8-hour block of time for each AWS Region. To see the time blocks available, see <a href="http://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/AdjustingTheMaintenanceWindow.html"> Adjusting the Preferred Maintenance Window</a> in the <i>Amazon Neptune User Guide.</i> </p> <p>Constraints:</p> <ul> <li> <p>Must be in the format <code>hh24:mi-hh24:mi</code>.</p> </li> <li> <p>Must be in Universal Coordinated Time (UTC).</p> </li> <li> <p>Must not conflict with the preferred maintenance window.</p> </li> <li> <p>Must be at least 30 minutes.</p> </li> </ul></p>
    pub preferred_backup_window: Option<String>,
    /// <p>The weekly time range during which system maintenance can occur, in Universal Coordinated Time (UTC).</p> <p>Format: <code>ddd:hh24:mi-ddd:hh24:mi</code> </p> <p>The default is a 30-minute window selected at random from an 8-hour block of time for each AWS Region, occurring on a random day of the week. To see the time blocks available, see <a href="http://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/AdjustingTheMaintenanceWindow.html"> Adjusting the Preferred Maintenance Window</a> in the <i>Amazon Neptune User Guide.</i> </p> <p>Valid Days: Mon, Tue, Wed, Thu, Fri, Sat, Sun.</p> <p>Constraints: Minimum 30-minute window.</p>
    pub preferred_maintenance_window: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the source DB instance or DB cluster if this DB cluster is created as a Read Replica.</p>
    pub replication_source_identifier: Option<String>,
    /// <p>Specifies whether the DB cluster is encrypted.</p>
    pub storage_encrypted: Option<bool>,
    pub tags: Option<Vec<Tag>>,
    /// <p>A list of EC2 VPC security groups to associate with this DB cluster.</p>
    pub vpc_security_group_ids: Option<Vec<String>>,
}

/// Serialize `CreateDBClusterMessage` contents to a `SignedRequest`.
struct CreateDBClusterMessageSerializer;
impl CreateDBClusterMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &CreateDBClusterMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.availability_zones {
            AvailabilityZonesSerializer::serialize(
                params,
                &format!("{}{}", prefix, "AvailabilityZone"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.backup_retention_period {
            params.put(
                &format!("{}{}", prefix, "BackupRetentionPeriod"),
                &field_value.to_string(),
            );
        }
        if let Some(ref field_value) = obj.character_set_name {
            params.put(&format!("{}{}", prefix, "CharacterSetName"), &field_value);
        }
        params.put(
            &format!("{}{}", prefix, "DBClusterIdentifier"),
            &obj.db_cluster_identifier,
        );
        if let Some(ref field_value) = obj.db_cluster_parameter_group_name {
            params.put(
                &format!("{}{}", prefix, "DBClusterParameterGroupName"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.db_subnet_group_name {
            params.put(&format!("{}{}", prefix, "DBSubnetGroupName"), &field_value);
        }
        if let Some(ref field_value) = obj.database_name {
            params.put(&format!("{}{}", prefix, "DatabaseName"), &field_value);
        }
        if let Some(ref field_value) = obj.enable_iam_database_authentication {
            params.put(
                &format!("{}{}", prefix, "EnableIAMDatabaseAuthentication"),
                &field_value.to_string(),
            );
        }
        params.put(&format!("{}{}", prefix, "Engine"), &obj.engine);
        if let Some(ref field_value) = obj.engine_version {
            params.put(&format!("{}{}", prefix, "EngineVersion"), &field_value);
        }
        if let Some(ref field_value) = obj.kms_key_id {
            params.put(&format!("{}{}", prefix, "KmsKeyId"), &field_value);
        }
        if let Some(ref field_value) = obj.master_user_password {
            params.put(&format!("{}{}", prefix, "MasterUserPassword"), &field_value);
        }
        if let Some(ref field_value) = obj.master_username {
            params.put(&format!("{}{}", prefix, "MasterUsername"), &field_value);
        }
        if let Some(ref field_value) = obj.option_group_name {
            params.put(&format!("{}{}", prefix, "OptionGroupName"), &field_value);
        }
        if let Some(ref field_value) = obj.port {
            params.put(&format!("{}{}", prefix, "Port"), &field_value.to_string());
        }
        if let Some(ref field_value) = obj.pre_signed_url {
            params.put(&format!("{}{}", prefix, "PreSignedUrl"), &field_value);
        }
        if let Some(ref field_value) = obj.preferred_backup_window {
            params.put(
                &format!("{}{}", prefix, "PreferredBackupWindow"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.preferred_maintenance_window {
            params.put(
                &format!("{}{}", prefix, "PreferredMaintenanceWindow"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.replication_source_identifier {
            params.put(
                &format!("{}{}", prefix, "ReplicationSourceIdentifier"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.storage_encrypted {
            params.put(
                &format!("{}{}", prefix, "StorageEncrypted"),
                &field_value.to_string(),
            );
        }
        if let Some(ref field_value) = obj.tags {
            TagListSerializer::serialize(params, &format!("{}{}", prefix, "Tag"), field_value);
        }
        if let Some(ref field_value) = obj.vpc_security_group_ids {
            VpcSecurityGroupIdListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "VpcSecurityGroupId"),
                field_value,
            );
        }
    }
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateDBClusterParameterGroupMessage {
    /// <p><p>The name of the DB cluster parameter group.</p> <p>Constraints:</p> <ul> <li> <p>Must match the name of an existing DBClusterParameterGroup.</p> </li> </ul> <note> <p>This value is stored as a lowercase string.</p> </note></p>
    pub db_cluster_parameter_group_name: String,
    /// <p>The DB cluster parameter group family name. A DB cluster parameter group can be associated with one and only one DB cluster parameter group family, and can be applied only to a DB cluster running a database engine and engine version compatible with that DB cluster parameter group family.</p>
    pub db_parameter_group_family: String,
    /// <p>The description for the DB cluster parameter group.</p>
    pub description: String,
    pub tags: Option<Vec<Tag>>,
}

/// Serialize `CreateDBClusterParameterGroupMessage` contents to a `SignedRequest`.
struct CreateDBClusterParameterGroupMessageSerializer;
impl CreateDBClusterParameterGroupMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &CreateDBClusterParameterGroupMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "DBClusterParameterGroupName"),
            &obj.db_cluster_parameter_group_name,
        );
        params.put(
            &format!("{}{}", prefix, "DBParameterGroupFamily"),
            &obj.db_parameter_group_family,
        );
        params.put(&format!("{}{}", prefix, "Description"), &obj.description);
        if let Some(ref field_value) = obj.tags {
            TagListSerializer::serialize(params, &format!("{}{}", prefix, "Tag"), field_value);
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateDBClusterParameterGroupResult {
    pub db_cluster_parameter_group: Option<DBClusterParameterGroup>,
}

struct CreateDBClusterParameterGroupResultDeserializer;
impl CreateDBClusterParameterGroupResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateDBClusterParameterGroupResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = CreateDBClusterParameterGroupResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "DBClusterParameterGroup" => {
                        obj.db_cluster_parameter_group =
                            Some(try!(DBClusterParameterGroupDeserializer::deserialize(
                                "DBClusterParameterGroup",
                                stack
                            )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateDBClusterResult {
    pub db_cluster: Option<DBCluster>,
}

struct CreateDBClusterResultDeserializer;
impl CreateDBClusterResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateDBClusterResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = CreateDBClusterResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "DBCluster" => {
                        obj.db_cluster =
                            Some(try!(DBClusterDeserializer::deserialize("DBCluster", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateDBClusterSnapshotMessage {
    /// <p>The identifier of the DB cluster to create a snapshot for. This parameter is not case-sensitive.</p> <p>Constraints:</p> <ul> <li> <p>Must match the identifier of an existing DBCluster.</p> </li> </ul> <p>Example: <code>my-cluster1</code> </p>
    pub db_cluster_identifier: String,
    /// <p>The identifier of the DB cluster snapshot. This parameter is stored as a lowercase string.</p> <p>Constraints:</p> <ul> <li> <p>Must contain from 1 to 63 letters, numbers, or hyphens.</p> </li> <li> <p>First character must be a letter.</p> </li> <li> <p>Cannot end with a hyphen or contain two consecutive hyphens.</p> </li> </ul> <p>Example: <code>my-cluster1-snapshot1</code> </p>
    pub db_cluster_snapshot_identifier: String,
    /// <p>The tags to be assigned to the DB cluster snapshot.</p>
    pub tags: Option<Vec<Tag>>,
}

/// Serialize `CreateDBClusterSnapshotMessage` contents to a `SignedRequest`.
struct CreateDBClusterSnapshotMessageSerializer;
impl CreateDBClusterSnapshotMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &CreateDBClusterSnapshotMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "DBClusterIdentifier"),
            &obj.db_cluster_identifier,
        );
        params.put(
            &format!("{}{}", prefix, "DBClusterSnapshotIdentifier"),
            &obj.db_cluster_snapshot_identifier,
        );
        if let Some(ref field_value) = obj.tags {
            TagListSerializer::serialize(params, &format!("{}{}", prefix, "Tag"), field_value);
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateDBClusterSnapshotResult {
    pub db_cluster_snapshot: Option<DBClusterSnapshot>,
}

struct CreateDBClusterSnapshotResultDeserializer;
impl CreateDBClusterSnapshotResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateDBClusterSnapshotResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = CreateDBClusterSnapshotResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "DBClusterSnapshot" => {
                        obj.db_cluster_snapshot = Some(try!(
                            DBClusterSnapshotDeserializer::deserialize("DBClusterSnapshot", stack)
                        ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateDBInstanceMessage {
    /// <p>The amount of storage (in gibibytes) to allocate for the DB instance.</p> <p>Type: Integer</p> <p>Not applicable. Neptune cluster volumes automatically grow as the amount of data in your database increases, though you are only charged for the space that you use in a Neptune cluster volume.</p>
    pub allocated_storage: Option<i64>,
    /// <p>Indicates that minor engine upgrades are applied automatically to the DB instance during the maintenance window.</p> <p>Default: <code>true</code> </p>
    pub auto_minor_version_upgrade: Option<bool>,
    /// <p> The EC2 Availability Zone that the DB instance is created in. </p> <p>Default: A random, system-chosen Availability Zone in the endpoint's AWS Region.</p> <p> Example: <code>us-east-1d</code> </p> <p> Constraint: The AvailabilityZone parameter can't be specified if the MultiAZ parameter is set to <code>true</code>. The specified Availability Zone must be in the same AWS Region as the current endpoint. </p>
    pub availability_zone: Option<String>,
    /// <p><p>The number of days for which automated backups are retained.</p> <p>Not applicable. The retention period for automated backups is managed by the DB cluster. For more information, see <a>CreateDBCluster</a>.</p> <p>Default: 1</p> <p>Constraints:</p> <ul> <li> <p>Must be a value from 0 to 35</p> </li> <li> <p>Cannot be set to 0 if the DB instance is a source to Read Replicas</p> </li> </ul></p>
    pub backup_retention_period: Option<i64>,
    /// <p>Indicates that the DB instance should be associated with the specified CharacterSet.</p> <p>Not applicable. The character set is managed by the DB cluster. For more information, see <a>CreateDBCluster</a>.</p>
    pub character_set_name: Option<String>,
    /// <p>True to copy all tags from the DB instance to snapshots of the DB instance, and otherwise false. The default is false.</p>
    pub copy_tags_to_snapshot: Option<bool>,
    /// <p>The identifier of the DB cluster that the instance will belong to.</p> <p>For information on creating a DB cluster, see <a>CreateDBCluster</a>.</p> <p>Type: String</p>
    pub db_cluster_identifier: Option<String>,
    /// <p>The compute and memory capacity of the DB instance, for example, <code>db.m4.large</code>. Not all DB instance classes are available in all AWS Regions. </p>
    pub db_instance_class: String,
    /// <p>The DB instance identifier. This parameter is stored as a lowercase string.</p> <p>Constraints:</p> <ul> <li> <p>Must contain from 1 to 63 letters, numbers, or hyphens.</p> </li> <li> <p>First character must be a letter.</p> </li> <li> <p>Cannot end with a hyphen or contain two consecutive hyphens.</p> </li> </ul> <p>Example: <code>mydbinstance</code> </p>
    pub db_instance_identifier: String,
    /// <p>The database name. </p> <p>Type: String</p>
    pub db_name: Option<String>,
    /// <p><p>The name of the DB parameter group to associate with this DB instance. If this argument is omitted, the default DBParameterGroup for the specified engine is used.</p> <p>Constraints:</p> <ul> <li> <p>Must be 1 to 255 letters, numbers, or hyphens.</p> </li> <li> <p>First character must be a letter</p> </li> <li> <p>Cannot end with a hyphen or contain two consecutive hyphens</p> </li> </ul></p>
    pub db_parameter_group_name: Option<String>,
    /// <p>A list of DB security groups to associate with this DB instance.</p> <p>Default: The default DB security group for the database engine.</p>
    pub db_security_groups: Option<Vec<String>>,
    /// <p>A DB subnet group to associate with this DB instance.</p> <p>If there is no DB subnet group, then it is a non-VPC DB instance.</p>
    pub db_subnet_group_name: Option<String>,
    /// <p>Specify the Active Directory Domain to create the instance in.</p>
    pub domain: Option<String>,
    /// <p>Specify the name of the IAM role to be used when making API calls to the Directory Service.</p>
    pub domain_iam_role_name: Option<String>,
    /// <p>The list of log types that need to be enabled for exporting to CloudWatch Logs.</p>
    pub enable_cloudwatch_logs_exports: Option<Vec<String>>,
    /// <p>True to enable AWS Identity and Access Management (IAM) authentication for Neptune.</p> <p>Default: <code>false</code> </p>
    pub enable_iam_database_authentication: Option<bool>,
    /// <p>True to enable Performance Insights for the DB instance, and otherwise false. </p>
    pub enable_performance_insights: Option<bool>,
    /// <p>The name of the database engine to be used for this instance. </p> <p>Valid Values: <code>neptune</code> </p>
    pub engine: String,
    /// <p>The version number of the database engine to use.</p>
    pub engine_version: Option<String>,
    /// <p>The amount of Provisioned IOPS (input/output operations per second) to be initially allocated for the DB instance. </p>
    pub iops: Option<i64>,
    /// <p>The AWS KMS key identifier for an encrypted DB instance.</p> <p>The KMS key identifier is the Amazon Resource Name (ARN) for the KMS encryption key. If you are creating a DB instance with the same AWS account that owns the KMS encryption key used to encrypt the new DB instance, then you can use the KMS key alias instead of the ARN for the KM encryption key.</p> <p>Not applicable. The KMS key identifier is managed by the DB cluster. For more information, see <a>CreateDBCluster</a>.</p> <p>If the <code>StorageEncrypted</code> parameter is true, and you do not specify a value for the <code>KmsKeyId</code> parameter, then Amazon Neptune will use your default encryption key. AWS KMS creates the default encryption key for your AWS account. Your AWS account has a different default encryption key for each AWS Region.</p>
    pub kms_key_id: Option<String>,
    /// <p>License model information for this DB instance.</p> <p> Valid values: <code>license-included</code> | <code>bring-your-own-license</code> | <code>general-public-license</code> </p>
    pub license_model: Option<String>,
    /// <p>The password for the master user. The password can include any printable ASCII character except "/", """, or "@".</p> <p> Not used. </p>
    pub master_user_password: Option<String>,
    /// <p>The name for the master user. Not used.</p>
    pub master_username: Option<String>,
    /// <p>The interval, in seconds, between points when Enhanced Monitoring metrics are collected for the DB instance. To disable collecting Enhanced Monitoring metrics, specify 0. The default is 0.</p> <p>If <code>MonitoringRoleArn</code> is specified, then you must also set <code>MonitoringInterval</code> to a value other than 0.</p> <p>Valid Values: <code>0, 1, 5, 10, 15, 30, 60</code> </p>
    pub monitoring_interval: Option<i64>,
    /// <p>The ARN for the IAM role that permits Neptune to send enhanced monitoring metrics to Amazon CloudWatch Logs. For example, <code>arn:aws:iam:123456789012:role/emaccess</code>.</p> <p>If <code>MonitoringInterval</code> is set to a value other than 0, then you must supply a <code>MonitoringRoleArn</code> value.</p>
    pub monitoring_role_arn: Option<String>,
    /// <p>Specifies if the DB instance is a Multi-AZ deployment. You can't set the AvailabilityZone parameter if the MultiAZ parameter is set to true.</p>
    pub multi_az: Option<bool>,
    /// <p>Indicates that the DB instance should be associated with the specified option group.</p> <p>Permanent options, such as the TDE option for Oracle Advanced Security TDE, can't be removed from an option group, and that option group can't be removed from a DB instance once it is associated with a DB instance</p>
    pub option_group_name: Option<String>,
    /// <p>The AWS KMS key identifier for encryption of Performance Insights data. The KMS key ID is the Amazon Resource Name (ARN), KMS key identifier, or the KMS key alias for the KMS encryption key.</p>
    pub performance_insights_kms_key_id: Option<String>,
    /// <p>The port number on which the database accepts connections.</p> <p>Not applicable. The port is managed by the DB cluster. For more information, see <a>CreateDBCluster</a>.</p> <p> Default: <code>8182</code> </p> <p>Type: Integer</p>
    pub port: Option<i64>,
    /// <p> The daily time range during which automated backups are created. </p> <p>Not applicable. The daily time range for creating automated backups is managed by the DB cluster. For more information, see <a>CreateDBCluster</a>.</p>
    pub preferred_backup_window: Option<String>,
    /// <p>The time range each week during which system maintenance can occur, in Universal Coordinated Time (UTC). </p> <p> Format: <code>ddd:hh24:mi-ddd:hh24:mi</code> </p> <p>The default is a 30-minute window selected at random from an 8-hour block of time for each AWS Region, occurring on a random day of the week. </p> <p>Valid Days: Mon, Tue, Wed, Thu, Fri, Sat, Sun.</p> <p>Constraints: Minimum 30-minute window.</p>
    pub preferred_maintenance_window: Option<String>,
    /// <p>A value that specifies the order in which an Read Replica is promoted to the primary instance after a failure of the existing primary instance. </p> <p>Default: 1</p> <p>Valid Values: 0 - 15</p>
    pub promotion_tier: Option<i64>,
    /// <p>Specifies whether the DB instance is encrypted.</p> <p>Not applicable. The encryption for DB instances is managed by the DB cluster. For more information, see <a>CreateDBCluster</a>.</p> <p>Default: false</p>
    pub storage_encrypted: Option<bool>,
    /// <p>Specifies the storage type to be associated with the DB instance.</p> <p>Not applicable. Storage is managed by the DB Cluster.</p>
    pub storage_type: Option<String>,
    pub tags: Option<Vec<Tag>>,
    /// <p>The ARN from the key store with which to associate the instance for TDE encryption.</p>
    pub tde_credential_arn: Option<String>,
    /// <p>The password for the given ARN from the key store in order to access the device.</p>
    pub tde_credential_password: Option<String>,
    /// <p>The time zone of the DB instance. </p>
    pub timezone: Option<String>,
    /// <p>A list of EC2 VPC security groups to associate with this DB instance.</p> <p>Not applicable. The associated list of EC2 VPC security groups is managed by the DB cluster. For more information, see <a>CreateDBCluster</a>.</p> <p>Default: The default EC2 VPC security group for the DB subnet group's VPC.</p>
    pub vpc_security_group_ids: Option<Vec<String>>,
}

/// Serialize `CreateDBInstanceMessage` contents to a `SignedRequest`.
struct CreateDBInstanceMessageSerializer;
impl CreateDBInstanceMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &CreateDBInstanceMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.allocated_storage {
            params.put(
                &format!("{}{}", prefix, "AllocatedStorage"),
                &field_value.to_string(),
            );
        }
        if let Some(ref field_value) = obj.auto_minor_version_upgrade {
            params.put(
                &format!("{}{}", prefix, "AutoMinorVersionUpgrade"),
                &field_value.to_string(),
            );
        }
        if let Some(ref field_value) = obj.availability_zone {
            params.put(&format!("{}{}", prefix, "AvailabilityZone"), &field_value);
        }
        if let Some(ref field_value) = obj.backup_retention_period {
            params.put(
                &format!("{}{}", prefix, "BackupRetentionPeriod"),
                &field_value.to_string(),
            );
        }
        if let Some(ref field_value) = obj.character_set_name {
            params.put(&format!("{}{}", prefix, "CharacterSetName"), &field_value);
        }
        if let Some(ref field_value) = obj.copy_tags_to_snapshot {
            params.put(
                &format!("{}{}", prefix, "CopyTagsToSnapshot"),
                &field_value.to_string(),
            );
        }
        if let Some(ref field_value) = obj.db_cluster_identifier {
            params.put(
                &format!("{}{}", prefix, "DBClusterIdentifier"),
                &field_value,
            );
        }
        params.put(
            &format!("{}{}", prefix, "DBInstanceClass"),
            &obj.db_instance_class,
        );
        params.put(
            &format!("{}{}", prefix, "DBInstanceIdentifier"),
            &obj.db_instance_identifier,
        );
        if let Some(ref field_value) = obj.db_name {
            params.put(&format!("{}{}", prefix, "DBName"), &field_value);
        }
        if let Some(ref field_value) = obj.db_parameter_group_name {
            params.put(
                &format!("{}{}", prefix, "DBParameterGroupName"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.db_security_groups {
            DBSecurityGroupNameListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "DBSecurityGroupName"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.db_subnet_group_name {
            params.put(&format!("{}{}", prefix, "DBSubnetGroupName"), &field_value);
        }
        if let Some(ref field_value) = obj.domain {
            params.put(&format!("{}{}", prefix, "Domain"), &field_value);
        }
        if let Some(ref field_value) = obj.domain_iam_role_name {
            params.put(&format!("{}{}", prefix, "DomainIAMRoleName"), &field_value);
        }
        if let Some(ref field_value) = obj.enable_cloudwatch_logs_exports {
            LogTypeListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "EnableCloudwatchLogsExports"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.enable_iam_database_authentication {
            params.put(
                &format!("{}{}", prefix, "EnableIAMDatabaseAuthentication"),
                &field_value.to_string(),
            );
        }
        if let Some(ref field_value) = obj.enable_performance_insights {
            params.put(
                &format!("{}{}", prefix, "EnablePerformanceInsights"),
                &field_value.to_string(),
            );
        }
        params.put(&format!("{}{}", prefix, "Engine"), &obj.engine);
        if let Some(ref field_value) = obj.engine_version {
            params.put(&format!("{}{}", prefix, "EngineVersion"), &field_value);
        }
        if let Some(ref field_value) = obj.iops {
            params.put(&format!("{}{}", prefix, "Iops"), &field_value.to_string());
        }
        if let Some(ref field_value) = obj.kms_key_id {
            params.put(&format!("{}{}", prefix, "KmsKeyId"), &field_value);
        }
        if let Some(ref field_value) = obj.license_model {
            params.put(&format!("{}{}", prefix, "LicenseModel"), &field_value);
        }
        if let Some(ref field_value) = obj.master_user_password {
            params.put(&format!("{}{}", prefix, "MasterUserPassword"), &field_value);
        }
        if let Some(ref field_value) = obj.master_username {
            params.put(&format!("{}{}", prefix, "MasterUsername"), &field_value);
        }
        if let Some(ref field_value) = obj.monitoring_interval {
            params.put(
                &format!("{}{}", prefix, "MonitoringInterval"),
                &field_value.to_string(),
            );
        }
        if let Some(ref field_value) = obj.monitoring_role_arn {
            params.put(&format!("{}{}", prefix, "MonitoringRoleArn"), &field_value);
        }
        if let Some(ref field_value) = obj.multi_az {
            params.put(
                &format!("{}{}", prefix, "MultiAZ"),
                &field_value.to_string(),
            );
        }
        if let Some(ref field_value) = obj.option_group_name {
            params.put(&format!("{}{}", prefix, "OptionGroupName"), &field_value);
        }
        if let Some(ref field_value) = obj.performance_insights_kms_key_id {
            params.put(
                &format!("{}{}", prefix, "PerformanceInsightsKMSKeyId"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.port {
            params.put(&format!("{}{}", prefix, "Port"), &field_value.to_string());
        }
        if let Some(ref field_value) = obj.preferred_backup_window {
            params.put(
                &format!("{}{}", prefix, "PreferredBackupWindow"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.preferred_maintenance_window {
            params.put(
                &format!("{}{}", prefix, "PreferredMaintenanceWindow"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.promotion_tier {
            params.put(
                &format!("{}{}", prefix, "PromotionTier"),
                &field_value.to_string(),
            );
        }

        if let Some(ref field_value) = obj.storage_encrypted {
            params.put(
                &format!("{}{}", prefix, "StorageEncrypted"),
                &field_value.to_string(),
            );
        }
        if let Some(ref field_value) = obj.storage_type {
            params.put(&format!("{}{}", prefix, "StorageType"), &field_value);
        }
        if let Some(ref field_value) = obj.tags {
            TagListSerializer::serialize(params, &format!("{}{}", prefix, "Tag"), field_value);
        }
        if let Some(ref field_value) = obj.tde_credential_arn {
            params.put(&format!("{}{}", prefix, "TdeCredentialArn"), &field_value);
        }
        if let Some(ref field_value) = obj.tde_credential_password {
            params.put(
                &format!("{}{}", prefix, "TdeCredentialPassword"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.timezone {
            params.put(&format!("{}{}", prefix, "Timezone"), &field_value);
        }
        if let Some(ref field_value) = obj.vpc_security_group_ids {
            VpcSecurityGroupIdListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "VpcSecurityGroupId"),
                field_value,
            );
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateDBInstanceResult {
    pub db_instance: Option<DBInstance>,
}

struct CreateDBInstanceResultDeserializer;
impl CreateDBInstanceResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateDBInstanceResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = CreateDBInstanceResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "DBInstance" => {
                        obj.db_instance = Some(try!(DBInstanceDeserializer::deserialize(
                            "DBInstance",
                            stack
                        )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateDBParameterGroupMessage {
    /// <p>The DB parameter group family name. A DB parameter group can be associated with one and only one DB parameter group family, and can be applied only to a DB instance running a database engine and engine version compatible with that DB parameter group family.</p>
    pub db_parameter_group_family: String,
    /// <p><p>The name of the DB parameter group.</p> <p>Constraints:</p> <ul> <li> <p>Must be 1 to 255 letters, numbers, or hyphens.</p> </li> <li> <p>First character must be a letter</p> </li> <li> <p>Cannot end with a hyphen or contain two consecutive hyphens</p> </li> </ul> <note> <p>This value is stored as a lowercase string.</p> </note></p>
    pub db_parameter_group_name: String,
    /// <p>The description for the DB parameter group.</p>
    pub description: String,
    pub tags: Option<Vec<Tag>>,
}

/// Serialize `CreateDBParameterGroupMessage` contents to a `SignedRequest`.
struct CreateDBParameterGroupMessageSerializer;
impl CreateDBParameterGroupMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &CreateDBParameterGroupMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "DBParameterGroupFamily"),
            &obj.db_parameter_group_family,
        );
        params.put(
            &format!("{}{}", prefix, "DBParameterGroupName"),
            &obj.db_parameter_group_name,
        );
        params.put(&format!("{}{}", prefix, "Description"), &obj.description);
        if let Some(ref field_value) = obj.tags {
            TagListSerializer::serialize(params, &format!("{}{}", prefix, "Tag"), field_value);
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateDBParameterGroupResult {
    pub db_parameter_group: Option<DBParameterGroup>,
}

struct CreateDBParameterGroupResultDeserializer;
impl CreateDBParameterGroupResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateDBParameterGroupResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = CreateDBParameterGroupResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "DBParameterGroup" => {
                        obj.db_parameter_group = Some(try!(
                            DBParameterGroupDeserializer::deserialize("DBParameterGroup", stack)
                        ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateDBSubnetGroupMessage {
    /// <p>The description for the DB subnet group.</p>
    pub db_subnet_group_description: String,
    /// <p>The name for the DB subnet group. This value is stored as a lowercase string.</p> <p>Constraints: Must contain no more than 255 letters, numbers, periods, underscores, spaces, or hyphens. Must not be default.</p> <p>Example: <code>mySubnetgroup</code> </p>
    pub db_subnet_group_name: String,
    /// <p>The EC2 Subnet IDs for the DB subnet group.</p>
    pub subnet_ids: Vec<String>,
    pub tags: Option<Vec<Tag>>,
}

/// Serialize `CreateDBSubnetGroupMessage` contents to a `SignedRequest`.
struct CreateDBSubnetGroupMessageSerializer;
impl CreateDBSubnetGroupMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &CreateDBSubnetGroupMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "DBSubnetGroupDescription"),
            &obj.db_subnet_group_description,
        );
        params.put(
            &format!("{}{}", prefix, "DBSubnetGroupName"),
            &obj.db_subnet_group_name,
        );
        SubnetIdentifierListSerializer::serialize(
            params,
            &format!("{}{}", prefix, "SubnetIdentifier"),
            &obj.subnet_ids,
        );
        if let Some(ref field_value) = obj.tags {
            TagListSerializer::serialize(params, &format!("{}{}", prefix, "Tag"), field_value);
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateDBSubnetGroupResult {
    pub db_subnet_group: Option<DBSubnetGroup>,
}

struct CreateDBSubnetGroupResultDeserializer;
impl CreateDBSubnetGroupResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateDBSubnetGroupResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = CreateDBSubnetGroupResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "DBSubnetGroup" => {
                        obj.db_subnet_group = Some(try!(DBSubnetGroupDeserializer::deserialize(
                            "DBSubnetGroup",
                            stack
                        )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateEventSubscriptionMessage {
    /// <p> A Boolean value; set to <b>true</b> to activate the subscription, set to <b>false</b> to create the subscription but not active it. </p>
    pub enabled: Option<bool>,
    /// <p> A list of event categories for a SourceType that you want to subscribe to. You can see a list of the categories for a given SourceType by using the <b>DescribeEventCategories</b> action. </p>
    pub event_categories: Option<Vec<String>>,
    /// <p>The Amazon Resource Name (ARN) of the SNS topic created for event notification. The ARN is created by Amazon SNS when you create a topic and subscribe to it.</p>
    pub sns_topic_arn: String,
    /// <p><p>The list of identifiers of the event sources for which events are returned. If not specified, then all sources are included in the response. An identifier must begin with a letter and must contain only ASCII letters, digits, and hyphens; it can&#39;t end with a hyphen or contain two consecutive hyphens.</p> <p>Constraints:</p> <ul> <li> <p>If SourceIds are supplied, SourceType must also be provided.</p> </li> <li> <p>If the source type is a DB instance, then a <code>DBInstanceIdentifier</code> must be supplied.</p> </li> <li> <p>If the source type is a DB security group, a <code>DBSecurityGroupName</code> must be supplied.</p> </li> <li> <p>If the source type is a DB parameter group, a <code>DBParameterGroupName</code> must be supplied.</p> </li> <li> <p>If the source type is a DB snapshot, a <code>DBSnapshotIdentifier</code> must be supplied.</p> </li> </ul></p>
    pub source_ids: Option<Vec<String>>,
    /// <p>The type of source that is generating the events. For example, if you want to be notified of events generated by a DB instance, you would set this parameter to db-instance. if this value is not specified, all events are returned.</p> <p>Valid values: <code>db-instance</code> | <code>db-cluster</code> | <code>db-parameter-group</code> | <code>db-security-group</code> | <code>db-snapshot</code> | <code>db-cluster-snapshot</code> </p>
    pub source_type: Option<String>,
    /// <p>The name of the subscription.</p> <p>Constraints: The name must be less than 255 characters.</p>
    pub subscription_name: String,
    pub tags: Option<Vec<Tag>>,
}

/// Serialize `CreateEventSubscriptionMessage` contents to a `SignedRequest`.
struct CreateEventSubscriptionMessageSerializer;
impl CreateEventSubscriptionMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &CreateEventSubscriptionMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.enabled {
            params.put(
                &format!("{}{}", prefix, "Enabled"),
                &field_value.to_string(),
            );
        }
        if let Some(ref field_value) = obj.event_categories {
            EventCategoriesListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "EventCategory"),
                field_value,
            );
        }
        params.put(&format!("{}{}", prefix, "SnsTopicArn"), &obj.sns_topic_arn);
        if let Some(ref field_value) = obj.source_ids {
            SourceIdsListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "SourceId"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.source_type {
            params.put(&format!("{}{}", prefix, "SourceType"), &field_value);
        }
        params.put(
            &format!("{}{}", prefix, "SubscriptionName"),
            &obj.subscription_name,
        );
        if let Some(ref field_value) = obj.tags {
            TagListSerializer::serialize(params, &format!("{}{}", prefix, "Tag"), field_value);
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateEventSubscriptionResult {
    pub event_subscription: Option<EventSubscription>,
}

struct CreateEventSubscriptionResultDeserializer;
impl CreateEventSubscriptionResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateEventSubscriptionResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = CreateEventSubscriptionResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "EventSubscription" => {
                        obj.event_subscription = Some(try!(
                            EventSubscriptionDeserializer::deserialize("EventSubscription", stack)
                        ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Contains the details of an Amazon Neptune DB cluster. </p> <p>This data type is used as a response element in the <a>DescribeDBClusters</a> action. </p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DBCluster {
    /// <p> <code>AllocatedStorage</code> always returns 1, because Neptune DB cluster storage size is not fixed, but instead automatically adjusts as needed.</p>
    pub allocated_storage: Option<i64>,
    /// <p>Provides a list of the AWS Identity and Access Management (IAM) roles that are associated with the DB cluster. IAM roles that are associated with a DB cluster grant permission for the DB cluster to access other AWS services on your behalf.</p>
    pub associated_roles: Option<Vec<DBClusterRole>>,
    /// <p>Provides the list of EC2 Availability Zones that instances in the DB cluster can be created in.</p>
    pub availability_zones: Option<Vec<String>>,
    /// <p>Specifies the number of days for which automatic DB snapshots are retained.</p>
    pub backup_retention_period: Option<i64>,
    /// <p>If present, specifies the name of the character set that this cluster is associated with.</p>
    pub character_set_name: Option<String>,
    /// <p>Identifies the clone group to which the DB cluster is associated.</p>
    pub clone_group_id: Option<String>,
    /// <p>Specifies the time when the DB cluster was created, in Universal Coordinated Time (UTC).</p>
    pub cluster_create_time: Option<String>,
    /// <p>The Amazon Resource Name (ARN) for the DB cluster.</p>
    pub db_cluster_arn: Option<String>,
    /// <p>Contains a user-supplied DB cluster identifier. This identifier is the unique key that identifies a DB cluster.</p>
    pub db_cluster_identifier: Option<String>,
    /// <p>Provides the list of instances that make up the DB cluster.</p>
    pub db_cluster_members: Option<Vec<DBClusterMember>>,
    /// <p>Provides the list of option group memberships for this DB cluster.</p>
    pub db_cluster_option_group_memberships: Option<Vec<DBClusterOptionGroupStatus>>,
    /// <p>Specifies the name of the DB cluster parameter group for the DB cluster.</p>
    pub db_cluster_parameter_group: Option<String>,
    /// <p>Specifies information on the subnet group associated with the DB cluster, including the name, description, and subnets in the subnet group.</p>
    pub db_subnet_group: Option<String>,
    /// <p>Contains the name of the initial database of this DB cluster that was provided at create time, if one was specified when the DB cluster was created. This same name is returned for the life of the DB cluster.</p>
    pub database_name: Option<String>,
    /// <p>The AWS Region-unique, immutable identifier for the DB cluster. This identifier is found in AWS CloudTrail log entries whenever the AWS KMS key for the DB cluster is accessed.</p>
    pub db_cluster_resource_id: Option<String>,
    /// <p>Specifies the earliest time to which a database can be restored with point-in-time restore.</p>
    pub earliest_restorable_time: Option<String>,
    /// <p>Specifies the connection endpoint for the primary instance of the DB cluster.</p>
    pub endpoint: Option<String>,
    /// <p>Provides the name of the database engine to be used for this DB cluster.</p>
    pub engine: Option<String>,
    /// <p>Indicates the database engine version.</p>
    pub engine_version: Option<String>,
    /// <p>Specifies the ID that Amazon Route 53 assigns when you create a hosted zone.</p>
    pub hosted_zone_id: Option<String>,
    /// <p>True if mapping of AWS Identity and Access Management (IAM) accounts to database accounts is enabled, and otherwise false.</p>
    pub iam_database_authentication_enabled: Option<bool>,
    /// <p>If <code>StorageEncrypted</code> is true, the AWS KMS key identifier for the encrypted DB cluster.</p>
    pub kms_key_id: Option<String>,
    /// <p>Specifies the latest time to which a database can be restored with point-in-time restore.</p>
    pub latest_restorable_time: Option<String>,
    /// <p>Contains the master username for the DB cluster.</p>
    pub master_username: Option<String>,
    /// <p>Specifies whether the DB cluster has instances in multiple Availability Zones.</p>
    pub multi_az: Option<bool>,
    /// <p>Specifies the progress of the operation as a percentage.</p>
    pub percent_progress: Option<String>,
    /// <p>Specifies the port that the database engine is listening on.</p>
    pub port: Option<i64>,
    /// <p>Specifies the daily time range during which automated backups are created if automated backups are enabled, as determined by the <code>BackupRetentionPeriod</code>. </p>
    pub preferred_backup_window: Option<String>,
    /// <p>Specifies the weekly time range during which system maintenance can occur, in Universal Coordinated Time (UTC).</p>
    pub preferred_maintenance_window: Option<String>,
    /// <p>Contains one or more identifiers of the Read Replicas associated with this DB cluster.</p>
    pub read_replica_identifiers: Option<Vec<String>>,
    /// <p>The reader endpoint for the DB cluster. The reader endpoint for a DB cluster load-balances connections across the Read Replicas that are available in a DB cluster. As clients request new connections to the reader endpoint, Neptune distributes the connection requests among the Read Replicas in the DB cluster. This functionality can help balance your read workload across multiple Read Replicas in your DB cluster. </p> <p>If a failover occurs, and the Read Replica that you are connected to is promoted to be the primary instance, your connection is dropped. To continue sending your read workload to other Read Replicas in the cluster, you can then reconnect to the reader endpoint.</p>
    pub reader_endpoint: Option<String>,
    /// <p>Contains the identifier of the source DB cluster if this DB cluster is a Read Replica.</p>
    pub replication_source_identifier: Option<String>,
    /// <p>Specifies the current state of this DB cluster.</p>
    pub status: Option<String>,
    /// <p>Specifies whether the DB cluster is encrypted.</p>
    pub storage_encrypted: Option<bool>,
    /// <p>Provides a list of VPC security groups that the DB cluster belongs to.</p>
    pub vpc_security_groups: Option<Vec<VpcSecurityGroupMembership>>,
}

struct DBClusterDeserializer;
impl DBClusterDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DBCluster, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DBCluster::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "AllocatedStorage" => {
                        obj.allocated_storage = Some(try!(
                            IntegerOptionalDeserializer::deserialize("AllocatedStorage", stack)
                        ));
                    }
                    "AssociatedRoles" => {
                        obj.associated_roles = Some(try!(DBClusterRolesDeserializer::deserialize(
                            "AssociatedRoles",
                            stack
                        )));
                    }
                    "AvailabilityZones" => {
                        obj.availability_zones = Some(try!(
                            AvailabilityZonesDeserializer::deserialize("AvailabilityZones", stack)
                        ));
                    }
                    "BackupRetentionPeriod" => {
                        obj.backup_retention_period =
                            Some(try!(IntegerOptionalDeserializer::deserialize(
                                "BackupRetentionPeriod",
                                stack
                            )));
                    }
                    "CharacterSetName" => {
                        obj.character_set_name = Some(try!(StringDeserializer::deserialize(
                            "CharacterSetName",
                            stack
                        )));
                    }
                    "CloneGroupId" => {
                        obj.clone_group_id =
                            Some(try!(StringDeserializer::deserialize("CloneGroupId", stack)));
                    }
                    "ClusterCreateTime" => {
                        obj.cluster_create_time = Some(try!(TStampDeserializer::deserialize(
                            "ClusterCreateTime",
                            stack
                        )));
                    }
                    "DBClusterArn" => {
                        obj.db_cluster_arn =
                            Some(try!(StringDeserializer::deserialize("DBClusterArn", stack)));
                    }
                    "DBClusterIdentifier" => {
                        obj.db_cluster_identifier = Some(try!(StringDeserializer::deserialize(
                            "DBClusterIdentifier",
                            stack
                        )));
                    }
                    "DBClusterMembers" => {
                        obj.db_cluster_members = Some(try!(
                            DBClusterMemberListDeserializer::deserialize("DBClusterMembers", stack)
                        ));
                    }
                    "DBClusterOptionGroupMemberships" => {
                        obj.db_cluster_option_group_memberships = Some(try!(
                            DBClusterOptionGroupMembershipsDeserializer::deserialize(
                                "DBClusterOptionGroupMemberships",
                                stack
                            )
                        ));
                    }
                    "DBClusterParameterGroup" => {
                        obj.db_cluster_parameter_group = Some(try!(
                            StringDeserializer::deserialize("DBClusterParameterGroup", stack)
                        ));
                    }
                    "DBSubnetGroup" => {
                        obj.db_subnet_group = Some(try!(StringDeserializer::deserialize(
                            "DBSubnetGroup",
                            stack
                        )));
                    }
                    "DatabaseName" => {
                        obj.database_name =
                            Some(try!(StringDeserializer::deserialize("DatabaseName", stack)));
                    }
                    "DbClusterResourceId" => {
                        obj.db_cluster_resource_id = Some(try!(StringDeserializer::deserialize(
                            "DbClusterResourceId",
                            stack
                        )));
                    }
                    "EarliestRestorableTime" => {
                        obj.earliest_restorable_time = Some(try!(TStampDeserializer::deserialize(
                            "EarliestRestorableTime",
                            stack
                        )));
                    }
                    "Endpoint" => {
                        obj.endpoint =
                            Some(try!(StringDeserializer::deserialize("Endpoint", stack)));
                    }
                    "Engine" => {
                        obj.engine = Some(try!(StringDeserializer::deserialize("Engine", stack)));
                    }
                    "EngineVersion" => {
                        obj.engine_version = Some(try!(StringDeserializer::deserialize(
                            "EngineVersion",
                            stack
                        )));
                    }
                    "HostedZoneId" => {
                        obj.hosted_zone_id =
                            Some(try!(StringDeserializer::deserialize("HostedZoneId", stack)));
                    }
                    "IAMDatabaseAuthenticationEnabled" => {
                        obj.iam_database_authentication_enabled =
                            Some(try!(BooleanDeserializer::deserialize(
                                "IAMDatabaseAuthenticationEnabled",
                                stack
                            )));
                    }
                    "KmsKeyId" => {
                        obj.kms_key_id =
                            Some(try!(StringDeserializer::deserialize("KmsKeyId", stack)));
                    }
                    "LatestRestorableTime" => {
                        obj.latest_restorable_time = Some(try!(TStampDeserializer::deserialize(
                            "LatestRestorableTime",
                            stack
                        )));
                    }
                    "MasterUsername" => {
                        obj.master_username = Some(try!(StringDeserializer::deserialize(
                            "MasterUsername",
                            stack
                        )));
                    }
                    "MultiAZ" => {
                        obj.multi_az =
                            Some(try!(BooleanDeserializer::deserialize("MultiAZ", stack)));
                    }
                    "PercentProgress" => {
                        obj.percent_progress = Some(try!(StringDeserializer::deserialize(
                            "PercentProgress",
                            stack
                        )));
                    }
                    "Port" => {
                        obj.port = Some(try!(IntegerOptionalDeserializer::deserialize(
                            "Port", stack
                        )));
                    }
                    "PreferredBackupWindow" => {
                        obj.preferred_backup_window = Some(try!(StringDeserializer::deserialize(
                            "PreferredBackupWindow",
                            stack
                        )));
                    }
                    "PreferredMaintenanceWindow" => {
                        obj.preferred_maintenance_window = Some(try!(
                            StringDeserializer::deserialize("PreferredMaintenanceWindow", stack)
                        ));
                    }
                    "ReadReplicaIdentifiers" => {
                        obj.read_replica_identifiers =
                            Some(try!(ReadReplicaIdentifierListDeserializer::deserialize(
                                "ReadReplicaIdentifiers",
                                stack
                            )));
                    }
                    "ReaderEndpoint" => {
                        obj.reader_endpoint = Some(try!(StringDeserializer::deserialize(
                            "ReaderEndpoint",
                            stack
                        )));
                    }
                    "ReplicationSourceIdentifier" => {
                        obj.replication_source_identifier = Some(try!(
                            StringDeserializer::deserialize("ReplicationSourceIdentifier", stack)
                        ));
                    }
                    "Status" => {
                        obj.status = Some(try!(StringDeserializer::deserialize("Status", stack)));
                    }
                    "StorageEncrypted" => {
                        obj.storage_encrypted = Some(try!(BooleanDeserializer::deserialize(
                            "StorageEncrypted",
                            stack
                        )));
                    }
                    "VpcSecurityGroups" => {
                        obj.vpc_security_groups = Some(try!(
                            VpcSecurityGroupMembershipListDeserializer::deserialize(
                                "VpcSecurityGroups",
                                stack
                            )
                        ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct DBClusterListDeserializer;
impl DBClusterListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<DBCluster>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "DBCluster" {
                        obj.push(try!(DBClusterDeserializer::deserialize("DBCluster", stack)));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}
/// <p>Contains information about an instance that is part of a DB cluster.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DBClusterMember {
    /// <p>Specifies the status of the DB cluster parameter group for this member of the DB cluster.</p>
    pub db_cluster_parameter_group_status: Option<String>,
    /// <p>Specifies the instance identifier for this member of the DB cluster.</p>
    pub db_instance_identifier: Option<String>,
    /// <p>Value that is <code>true</code> if the cluster member is the primary instance for the DB cluster and <code>false</code> otherwise.</p>
    pub is_cluster_writer: Option<bool>,
    /// <p>A value that specifies the order in which a Read Replica is promoted to the primary instance after a failure of the existing primary instance. </p>
    pub promotion_tier: Option<i64>,
}

struct DBClusterMemberDeserializer;
impl DBClusterMemberDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DBClusterMember, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DBClusterMember::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "DBClusterParameterGroupStatus" => {
                        obj.db_cluster_parameter_group_status = Some(try!(
                            StringDeserializer::deserialize("DBClusterParameterGroupStatus", stack)
                        ));
                    }
                    "DBInstanceIdentifier" => {
                        obj.db_instance_identifier = Some(try!(StringDeserializer::deserialize(
                            "DBInstanceIdentifier",
                            stack
                        )));
                    }
                    "IsClusterWriter" => {
                        obj.is_cluster_writer = Some(try!(BooleanDeserializer::deserialize(
                            "IsClusterWriter",
                            stack
                        )));
                    }
                    "PromotionTier" => {
                        obj.promotion_tier = Some(try!(IntegerOptionalDeserializer::deserialize(
                            "PromotionTier",
                            stack
                        )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct DBClusterMemberListDeserializer;
impl DBClusterMemberListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<DBClusterMember>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "DBClusterMember" {
                        obj.push(try!(DBClusterMemberDeserializer::deserialize(
                            "DBClusterMember",
                            stack
                        )));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}
/// <p>Contains the result of a successful invocation of the <a>DescribeDBClusters</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DBClusterMessage {
    /// <p>Contains a list of DB clusters for the user.</p>
    pub db_clusters: Option<Vec<DBCluster>>,
    /// <p>A pagination token that can be used in a subsequent DescribeDBClusters request.</p>
    pub marker: Option<String>,
}

struct DBClusterMessageDeserializer;
impl DBClusterMessageDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DBClusterMessage, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DBClusterMessage::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "DBClusters" => {
                        obj.db_clusters = Some(try!(DBClusterListDeserializer::deserialize(
                            "DBClusters",
                            stack
                        )));
                    }
                    "Marker" => {
                        obj.marker = Some(try!(StringDeserializer::deserialize("Marker", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct DBClusterOptionGroupMembershipsDeserializer;
impl DBClusterOptionGroupMembershipsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<DBClusterOptionGroupStatus>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "DBClusterOptionGroup" {
                        obj.push(try!(DBClusterOptionGroupStatusDeserializer::deserialize(
                            "DBClusterOptionGroup",
                            stack
                        )));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}
/// <p>Contains status information for a DB cluster option group.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DBClusterOptionGroupStatus {
    /// <p>Specifies the name of the DB cluster option group.</p>
    pub db_cluster_option_group_name: Option<String>,
    /// <p>Specifies the status of the DB cluster option group.</p>
    pub status: Option<String>,
}

struct DBClusterOptionGroupStatusDeserializer;
impl DBClusterOptionGroupStatusDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DBClusterOptionGroupStatus, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DBClusterOptionGroupStatus::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "DBClusterOptionGroupName" => {
                        obj.db_cluster_option_group_name = Some(try!(
                            StringDeserializer::deserialize("DBClusterOptionGroupName", stack)
                        ));
                    }
                    "Status" => {
                        obj.status = Some(try!(StringDeserializer::deserialize("Status", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Contains the details of an Amazon Neptune DB cluster parameter group. </p> <p>This data type is used as a response element in the <a>DescribeDBClusterParameterGroups</a> action. </p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DBClusterParameterGroup {
    /// <p>The Amazon Resource Name (ARN) for the DB cluster parameter group.</p>
    pub db_cluster_parameter_group_arn: Option<String>,
    /// <p>Provides the name of the DB cluster parameter group.</p>
    pub db_cluster_parameter_group_name: Option<String>,
    /// <p>Provides the name of the DB parameter group family that this DB cluster parameter group is compatible with.</p>
    pub db_parameter_group_family: Option<String>,
    /// <p>Provides the customer-specified description for this DB cluster parameter group.</p>
    pub description: Option<String>,
}

struct DBClusterParameterGroupDeserializer;
impl DBClusterParameterGroupDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DBClusterParameterGroup, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DBClusterParameterGroup::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "DBClusterParameterGroupArn" => {
                        obj.db_cluster_parameter_group_arn = Some(try!(
                            StringDeserializer::deserialize("DBClusterParameterGroupArn", stack)
                        ));
                    }
                    "DBClusterParameterGroupName" => {
                        obj.db_cluster_parameter_group_name = Some(try!(
                            StringDeserializer::deserialize("DBClusterParameterGroupName", stack)
                        ));
                    }
                    "DBParameterGroupFamily" => {
                        obj.db_parameter_group_family = Some(try!(
                            StringDeserializer::deserialize("DBParameterGroupFamily", stack)
                        ));
                    }
                    "Description" => {
                        obj.description =
                            Some(try!(StringDeserializer::deserialize("Description", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Provides details about a DB cluster parameter group including the parameters in the DB cluster parameter group.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DBClusterParameterGroupDetails {
    /// <p> An optional pagination token provided by a previous DescribeDBClusterParameters request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code> . </p>
    pub marker: Option<String>,
    /// <p>Provides a list of parameters for the DB cluster parameter group.</p>
    pub parameters: Option<Vec<Parameter>>,
}

struct DBClusterParameterGroupDetailsDeserializer;
impl DBClusterParameterGroupDetailsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DBClusterParameterGroupDetails, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DBClusterParameterGroupDetails::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Marker" => {
                        obj.marker = Some(try!(StringDeserializer::deserialize("Marker", stack)));
                    }
                    "Parameters" => {
                        obj.parameters = Some(try!(ParametersListDeserializer::deserialize(
                            "Parameters",
                            stack
                        )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct DBClusterParameterGroupListDeserializer;
impl DBClusterParameterGroupListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<DBClusterParameterGroup>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "DBClusterParameterGroup" {
                        obj.push(try!(DBClusterParameterGroupDeserializer::deserialize(
                            "DBClusterParameterGroup",
                            stack
                        )));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}
/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DBClusterParameterGroupNameMessage {
    /// <p><p>The name of the DB cluster parameter group.</p> <p>Constraints:</p> <ul> <li> <p>Must be 1 to 255 letters or numbers.</p> </li> <li> <p>First character must be a letter</p> </li> <li> <p>Cannot end with a hyphen or contain two consecutive hyphens</p> </li> </ul> <note> <p>This value is stored as a lowercase string.</p> </note></p>
    pub db_cluster_parameter_group_name: Option<String>,
}

struct DBClusterParameterGroupNameMessageDeserializer;
impl DBClusterParameterGroupNameMessageDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DBClusterParameterGroupNameMessage, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DBClusterParameterGroupNameMessage::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "DBClusterParameterGroupName" => {
                        obj.db_cluster_parameter_group_name = Some(try!(
                            StringDeserializer::deserialize("DBClusterParameterGroupName", stack)
                        ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DBClusterParameterGroupsMessage {
    /// <p>A list of DB cluster parameter groups.</p>
    pub db_cluster_parameter_groups: Option<Vec<DBClusterParameterGroup>>,
    /// <p> An optional pagination token provided by a previous <code>DescribeDBClusterParameterGroups</code> request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>. </p>
    pub marker: Option<String>,
}

struct DBClusterParameterGroupsMessageDeserializer;
impl DBClusterParameterGroupsMessageDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DBClusterParameterGroupsMessage, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DBClusterParameterGroupsMessage::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "DBClusterParameterGroups" => {
                        obj.db_cluster_parameter_groups =
                            Some(try!(DBClusterParameterGroupListDeserializer::deserialize(
                                "DBClusterParameterGroups",
                                stack
                            )));
                    }
                    "Marker" => {
                        obj.marker = Some(try!(StringDeserializer::deserialize("Marker", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Describes an AWS Identity and Access Management (IAM) role that is associated with a DB cluster.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DBClusterRole {
    /// <p>The Amazon Resource Name (ARN) of the IAM role that is associated with the DB cluster.</p>
    pub role_arn: Option<String>,
    /// <p><p>Describes the state of association between the IAM role and the DB cluster. The Status property returns one of the following values:</p> <ul> <li> <p> <code>ACTIVE</code> - the IAM role ARN is associated with the DB cluster and can be used to access other AWS services on your behalf.</p> </li> <li> <p> <code>PENDING</code> - the IAM role ARN is being associated with the DB cluster.</p> </li> <li> <p> <code>INVALID</code> - the IAM role ARN is associated with the DB cluster, but the DB cluster is unable to assume the IAM role in order to access other AWS services on your behalf.</p> </li> </ul></p>
    pub status: Option<String>,
}

struct DBClusterRoleDeserializer;
impl DBClusterRoleDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DBClusterRole, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DBClusterRole::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "RoleArn" => {
                        obj.role_arn =
                            Some(try!(StringDeserializer::deserialize("RoleArn", stack)));
                    }
                    "Status" => {
                        obj.status = Some(try!(StringDeserializer::deserialize("Status", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct DBClusterRolesDeserializer;
impl DBClusterRolesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<DBClusterRole>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "DBClusterRole" {
                        obj.push(try!(DBClusterRoleDeserializer::deserialize(
                            "DBClusterRole",
                            stack
                        )));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}
/// <p>Contains the details for an Amazon Neptune DB cluster snapshot </p> <p>This data type is used as a response element in the <a>DescribeDBClusterSnapshots</a> action. </p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DBClusterSnapshot {
    /// <p>Specifies the allocated storage size in gibibytes (GiB).</p>
    pub allocated_storage: Option<i64>,
    /// <p>Provides the list of EC2 Availability Zones that instances in the DB cluster snapshot can be restored in.</p>
    pub availability_zones: Option<Vec<String>>,
    /// <p>Specifies the time when the DB cluster was created, in Universal Coordinated Time (UTC).</p>
    pub cluster_create_time: Option<String>,
    /// <p>Specifies the DB cluster identifier of the DB cluster that this DB cluster snapshot was created from.</p>
    pub db_cluster_identifier: Option<String>,
    /// <p>The Amazon Resource Name (ARN) for the DB cluster snapshot.</p>
    pub db_cluster_snapshot_arn: Option<String>,
    /// <p>Specifies the identifier for the DB cluster snapshot.</p>
    pub db_cluster_snapshot_identifier: Option<String>,
    /// <p>Specifies the name of the database engine.</p>
    pub engine: Option<String>,
    /// <p>Provides the version of the database engine for this DB cluster snapshot.</p>
    pub engine_version: Option<String>,
    /// <p>True if mapping of AWS Identity and Access Management (IAM) accounts to database accounts is enabled, and otherwise false.</p>
    pub iam_database_authentication_enabled: Option<bool>,
    /// <p>If <code>StorageEncrypted</code> is true, the AWS KMS key identifier for the encrypted DB cluster snapshot.</p>
    pub kms_key_id: Option<String>,
    /// <p>Provides the license model information for this DB cluster snapshot.</p>
    pub license_model: Option<String>,
    /// <p>Provides the master username for the DB cluster snapshot.</p>
    pub master_username: Option<String>,
    /// <p>Specifies the percentage of the estimated data that has been transferred.</p>
    pub percent_progress: Option<i64>,
    /// <p>Specifies the port that the DB cluster was listening on at the time of the snapshot.</p>
    pub port: Option<i64>,
    /// <p>Provides the time when the snapshot was taken, in Universal Coordinated Time (UTC).</p>
    pub snapshot_create_time: Option<String>,
    /// <p>Provides the type of the DB cluster snapshot.</p>
    pub snapshot_type: Option<String>,
    /// <p>If the DB cluster snapshot was copied from a source DB cluster snapshot, the Amazon Resource Name (ARN) for the source DB cluster snapshot, otherwise, a null value.</p>
    pub source_db_cluster_snapshot_arn: Option<String>,
    /// <p>Specifies the status of this DB cluster snapshot.</p>
    pub status: Option<String>,
    /// <p>Specifies whether the DB cluster snapshot is encrypted.</p>
    pub storage_encrypted: Option<bool>,
    /// <p>Provides the VPC ID associated with the DB cluster snapshot.</p>
    pub vpc_id: Option<String>,
}

struct DBClusterSnapshotDeserializer;
impl DBClusterSnapshotDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DBClusterSnapshot, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DBClusterSnapshot::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "AllocatedStorage" => {
                        obj.allocated_storage = Some(try!(IntegerDeserializer::deserialize(
                            "AllocatedStorage",
                            stack
                        )));
                    }
                    "AvailabilityZones" => {
                        obj.availability_zones = Some(try!(
                            AvailabilityZonesDeserializer::deserialize("AvailabilityZones", stack)
                        ));
                    }
                    "ClusterCreateTime" => {
                        obj.cluster_create_time = Some(try!(TStampDeserializer::deserialize(
                            "ClusterCreateTime",
                            stack
                        )));
                    }
                    "DBClusterIdentifier" => {
                        obj.db_cluster_identifier = Some(try!(StringDeserializer::deserialize(
                            "DBClusterIdentifier",
                            stack
                        )));
                    }
                    "DBClusterSnapshotArn" => {
                        obj.db_cluster_snapshot_arn = Some(try!(StringDeserializer::deserialize(
                            "DBClusterSnapshotArn",
                            stack
                        )));
                    }
                    "DBClusterSnapshotIdentifier" => {
                        obj.db_cluster_snapshot_identifier = Some(try!(
                            StringDeserializer::deserialize("DBClusterSnapshotIdentifier", stack)
                        ));
                    }
                    "Engine" => {
                        obj.engine = Some(try!(StringDeserializer::deserialize("Engine", stack)));
                    }
                    "EngineVersion" => {
                        obj.engine_version = Some(try!(StringDeserializer::deserialize(
                            "EngineVersion",
                            stack
                        )));
                    }
                    "IAMDatabaseAuthenticationEnabled" => {
                        obj.iam_database_authentication_enabled =
                            Some(try!(BooleanDeserializer::deserialize(
                                "IAMDatabaseAuthenticationEnabled",
                                stack
                            )));
                    }
                    "KmsKeyId" => {
                        obj.kms_key_id =
                            Some(try!(StringDeserializer::deserialize("KmsKeyId", stack)));
                    }
                    "LicenseModel" => {
                        obj.license_model =
                            Some(try!(StringDeserializer::deserialize("LicenseModel", stack)));
                    }
                    "MasterUsername" => {
                        obj.master_username = Some(try!(StringDeserializer::deserialize(
                            "MasterUsername",
                            stack
                        )));
                    }
                    "PercentProgress" => {
                        obj.percent_progress = Some(try!(IntegerDeserializer::deserialize(
                            "PercentProgress",
                            stack
                        )));
                    }
                    "Port" => {
                        obj.port = Some(try!(IntegerDeserializer::deserialize("Port", stack)));
                    }
                    "SnapshotCreateTime" => {
                        obj.snapshot_create_time = Some(try!(TStampDeserializer::deserialize(
                            "SnapshotCreateTime",
                            stack
                        )));
                    }
                    "SnapshotType" => {
                        obj.snapshot_type =
                            Some(try!(StringDeserializer::deserialize("SnapshotType", stack)));
                    }
                    "SourceDBClusterSnapshotArn" => {
                        obj.source_db_cluster_snapshot_arn = Some(try!(
                            StringDeserializer::deserialize("SourceDBClusterSnapshotArn", stack)
                        ));
                    }
                    "Status" => {
                        obj.status = Some(try!(StringDeserializer::deserialize("Status", stack)));
                    }
                    "StorageEncrypted" => {
                        obj.storage_encrypted = Some(try!(BooleanDeserializer::deserialize(
                            "StorageEncrypted",
                            stack
                        )));
                    }
                    "VpcId" => {
                        obj.vpc_id = Some(try!(StringDeserializer::deserialize("VpcId", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Contains the name and values of a manual DB cluster snapshot attribute.</p> <p>Manual DB cluster snapshot attributes are used to authorize other AWS accounts to restore a manual DB cluster snapshot. For more information, see the <a>ModifyDBClusterSnapshotAttribute</a> API action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DBClusterSnapshotAttribute {
    /// <p>The name of the manual DB cluster snapshot attribute.</p> <p>The attribute named <code>restore</code> refers to the list of AWS accounts that have permission to copy or restore the manual DB cluster snapshot. For more information, see the <a>ModifyDBClusterSnapshotAttribute</a> API action.</p>
    pub attribute_name: Option<String>,
    /// <p>The value(s) for the manual DB cluster snapshot attribute.</p> <p>If the <code>AttributeName</code> field is set to <code>restore</code>, then this element returns a list of IDs of the AWS accounts that are authorized to copy or restore the manual DB cluster snapshot. If a value of <code>all</code> is in the list, then the manual DB cluster snapshot is public and available for any AWS account to copy or restore.</p>
    pub attribute_values: Option<Vec<String>>,
}

struct DBClusterSnapshotAttributeDeserializer;
impl DBClusterSnapshotAttributeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DBClusterSnapshotAttribute, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DBClusterSnapshotAttribute::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "AttributeName" => {
                        obj.attribute_name = Some(try!(StringDeserializer::deserialize(
                            "AttributeName",
                            stack
                        )));
                    }
                    "AttributeValues" => {
                        obj.attribute_values = Some(try!(
                            AttributeValueListDeserializer::deserialize("AttributeValues", stack)
                        ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct DBClusterSnapshotAttributeListDeserializer;
impl DBClusterSnapshotAttributeListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<DBClusterSnapshotAttribute>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "DBClusterSnapshotAttribute" {
                        obj.push(try!(DBClusterSnapshotAttributeDeserializer::deserialize(
                            "DBClusterSnapshotAttribute",
                            stack
                        )));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}
/// <p>Contains the results of a successful call to the <a>DescribeDBClusterSnapshotAttributes</a> API action.</p> <p>Manual DB cluster snapshot attributes are used to authorize other AWS accounts to copy or restore a manual DB cluster snapshot. For more information, see the <a>ModifyDBClusterSnapshotAttribute</a> API action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DBClusterSnapshotAttributesResult {
    /// <p>The list of attributes and values for the manual DB cluster snapshot.</p>
    pub db_cluster_snapshot_attributes: Option<Vec<DBClusterSnapshotAttribute>>,
    /// <p>The identifier of the manual DB cluster snapshot that the attributes apply to.</p>
    pub db_cluster_snapshot_identifier: Option<String>,
}

struct DBClusterSnapshotAttributesResultDeserializer;
impl DBClusterSnapshotAttributesResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DBClusterSnapshotAttributesResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DBClusterSnapshotAttributesResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "DBClusterSnapshotAttributes" => {
                        obj.db_cluster_snapshot_attributes = Some(try!(
                            DBClusterSnapshotAttributeListDeserializer::deserialize(
                                "DBClusterSnapshotAttributes",
                                stack
                            )
                        ));
                    }
                    "DBClusterSnapshotIdentifier" => {
                        obj.db_cluster_snapshot_identifier = Some(try!(
                            StringDeserializer::deserialize("DBClusterSnapshotIdentifier", stack)
                        ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct DBClusterSnapshotListDeserializer;
impl DBClusterSnapshotListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<DBClusterSnapshot>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "DBClusterSnapshot" {
                        obj.push(try!(DBClusterSnapshotDeserializer::deserialize(
                            "DBClusterSnapshot",
                            stack
                        )));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}
/// <p> Provides a list of DB cluster snapshots for the user as the result of a call to the <a>DescribeDBClusterSnapshots</a> action. </p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DBClusterSnapshotMessage {
    /// <p>Provides a list of DB cluster snapshots for the user.</p>
    pub db_cluster_snapshots: Option<Vec<DBClusterSnapshot>>,
    /// <p> An optional pagination token provided by a previous <a>DescribeDBClusterSnapshots</a> request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>. </p>
    pub marker: Option<String>,
}

struct DBClusterSnapshotMessageDeserializer;
impl DBClusterSnapshotMessageDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DBClusterSnapshotMessage, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DBClusterSnapshotMessage::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "DBClusterSnapshots" => {
                        obj.db_cluster_snapshots =
                            Some(try!(DBClusterSnapshotListDeserializer::deserialize(
                                "DBClusterSnapshots",
                                stack
                            )));
                    }
                    "Marker" => {
                        obj.marker = Some(try!(StringDeserializer::deserialize("Marker", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p> This data type is used as a response element in the action <a>DescribeDBEngineVersions</a>. </p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DBEngineVersion {
    /// <p>The description of the database engine.</p>
    pub db_engine_description: Option<String>,
    /// <p>The description of the database engine version.</p>
    pub db_engine_version_description: Option<String>,
    /// <p>The name of the DB parameter group family for the database engine.</p>
    pub db_parameter_group_family: Option<String>,
    /// <p> The default character set for new instances of this engine version, if the <code>CharacterSetName</code> parameter of the CreateDBInstance API is not specified. </p>
    pub default_character_set: Option<CharacterSet>,
    /// <p>The name of the database engine.</p>
    pub engine: Option<String>,
    /// <p>The version number of the database engine.</p>
    pub engine_version: Option<String>,
    /// <p>The types of logs that the database engine has available for export to CloudWatch Logs.</p>
    pub exportable_log_types: Option<Vec<String>>,
    /// <p> A list of the character sets supported by this engine for the <code>CharacterSetName</code> parameter of the <code>CreateDBInstance</code> action. </p>
    pub supported_character_sets: Option<Vec<CharacterSet>>,
    /// <p>A list of the time zones supported by this engine for the <code>Timezone</code> parameter of the <code>CreateDBInstance</code> action. </p>
    pub supported_timezones: Option<Vec<Timezone>>,
    /// <p>A value that indicates whether the engine version supports exporting the log types specified by ExportableLogTypes to CloudWatch Logs.</p>
    pub supports_log_exports_to_cloudwatch_logs: Option<bool>,
    /// <p>Indicates whether the database engine version supports read replicas.</p>
    pub supports_read_replica: Option<bool>,
    /// <p>A list of engine versions that this database engine version can be upgraded to.</p>
    pub valid_upgrade_target: Option<Vec<UpgradeTarget>>,
}

struct DBEngineVersionDeserializer;
impl DBEngineVersionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DBEngineVersion, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DBEngineVersion::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "DBEngineDescription" => {
                        obj.db_engine_description = Some(try!(StringDeserializer::deserialize(
                            "DBEngineDescription",
                            stack
                        )));
                    }
                    "DBEngineVersionDescription" => {
                        obj.db_engine_version_description = Some(try!(
                            StringDeserializer::deserialize("DBEngineVersionDescription", stack)
                        ));
                    }
                    "DBParameterGroupFamily" => {
                        obj.db_parameter_group_family = Some(try!(
                            StringDeserializer::deserialize("DBParameterGroupFamily", stack)
                        ));
                    }
                    "DefaultCharacterSet" => {
                        obj.default_character_set = Some(try!(
                            CharacterSetDeserializer::deserialize("DefaultCharacterSet", stack)
                        ));
                    }
                    "Engine" => {
                        obj.engine = Some(try!(StringDeserializer::deserialize("Engine", stack)));
                    }
                    "EngineVersion" => {
                        obj.engine_version = Some(try!(StringDeserializer::deserialize(
                            "EngineVersion",
                            stack
                        )));
                    }
                    "ExportableLogTypes" => {
                        obj.exportable_log_types = Some(try!(
                            LogTypeListDeserializer::deserialize("ExportableLogTypes", stack)
                        ));
                    }
                    "SupportedCharacterSets" => {
                        obj.supported_character_sets =
                            Some(try!(SupportedCharacterSetsListDeserializer::deserialize(
                                "SupportedCharacterSets",
                                stack
                            )));
                    }
                    "SupportedTimezones" => {
                        obj.supported_timezones =
                            Some(try!(SupportedTimezonesListDeserializer::deserialize(
                                "SupportedTimezones",
                                stack
                            )));
                    }
                    "SupportsLogExportsToCloudwatchLogs" => {
                        obj.supports_log_exports_to_cloudwatch_logs =
                            Some(try!(BooleanDeserializer::deserialize(
                                "SupportsLogExportsToCloudwatchLogs",
                                stack
                            )));
                    }
                    "SupportsReadReplica" => {
                        obj.supports_read_replica = Some(try!(BooleanDeserializer::deserialize(
                            "SupportsReadReplica",
                            stack
                        )));
                    }
                    "ValidUpgradeTarget" => {
                        obj.valid_upgrade_target =
                            Some(try!(ValidUpgradeTargetListDeserializer::deserialize(
                                "ValidUpgradeTarget",
                                stack
                            )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct DBEngineVersionListDeserializer;
impl DBEngineVersionListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<DBEngineVersion>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "DBEngineVersion" {
                        obj.push(try!(DBEngineVersionDeserializer::deserialize(
                            "DBEngineVersion",
                            stack
                        )));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}
/// <p> Contains the result of a successful invocation of the <a>DescribeDBEngineVersions</a> action. </p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DBEngineVersionMessage {
    /// <p> A list of <code>DBEngineVersion</code> elements. </p>
    pub db_engine_versions: Option<Vec<DBEngineVersion>>,
    /// <p> An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>. </p>
    pub marker: Option<String>,
}

struct DBEngineVersionMessageDeserializer;
impl DBEngineVersionMessageDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DBEngineVersionMessage, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DBEngineVersionMessage::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "DBEngineVersions" => {
                        obj.db_engine_versions = Some(try!(
                            DBEngineVersionListDeserializer::deserialize("DBEngineVersions", stack)
                        ));
                    }
                    "Marker" => {
                        obj.marker = Some(try!(StringDeserializer::deserialize("Marker", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Contains the details of an Amazon Neptune DB instance. </p> <p>This data type is used as a response element in the <a>DescribeDBInstances</a> action. </p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DBInstance {
    /// <p>Specifies the allocated storage size specified in gibibytes.</p>
    pub allocated_storage: Option<i64>,
    /// <p>Indicates that minor version patches are applied automatically.</p>
    pub auto_minor_version_upgrade: Option<bool>,
    /// <p>Specifies the name of the Availability Zone the DB instance is located in.</p>
    pub availability_zone: Option<String>,
    /// <p>Specifies the number of days for which automatic DB snapshots are retained.</p>
    pub backup_retention_period: Option<i64>,
    /// <p>The identifier of the CA certificate for this DB instance.</p>
    pub ca_certificate_identifier: Option<String>,
    /// <p>If present, specifies the name of the character set that this instance is associated with.</p>
    pub character_set_name: Option<String>,
    /// <p>Specifies whether tags are copied from the DB instance to snapshots of the DB instance.</p>
    pub copy_tags_to_snapshot: Option<bool>,
    /// <p>If the DB instance is a member of a DB cluster, contains the name of the DB cluster that the DB instance is a member of.</p>
    pub db_cluster_identifier: Option<String>,
    /// <p>The Amazon Resource Name (ARN) for the DB instance.</p>
    pub db_instance_arn: Option<String>,
    /// <p>Contains the name of the compute and memory capacity class of the DB instance.</p>
    pub db_instance_class: Option<String>,
    /// <p>Contains a user-supplied database identifier. This identifier is the unique key that identifies a DB instance.</p>
    pub db_instance_identifier: Option<String>,
    /// <p>Specifies the current state of this database.</p>
    pub db_instance_status: Option<String>,
    /// <p>The database name.</p>
    pub db_name: Option<String>,
    /// <p>Provides the list of DB parameter groups applied to this DB instance.</p>
    pub db_parameter_groups: Option<Vec<DBParameterGroupStatus>>,
    /// <p> Provides List of DB security group elements containing only <code>DBSecurityGroup.Name</code> and <code>DBSecurityGroup.Status</code> subelements. </p>
    pub db_security_groups: Option<Vec<DBSecurityGroupMembership>>,
    /// <p>Specifies information on the subnet group associated with the DB instance, including the name, description, and subnets in the subnet group.</p>
    pub db_subnet_group: Option<DBSubnetGroup>,
    /// <p>Specifies the port that the DB instance listens on. If the DB instance is part of a DB cluster, this can be a different port than the DB cluster port.</p>
    pub db_instance_port: Option<i64>,
    /// <p>The AWS Region-unique, immutable identifier for the DB instance. This identifier is found in AWS CloudTrail log entries whenever the AWS KMS key for the DB instance is accessed.</p>
    pub dbi_resource_id: Option<String>,
    /// <p>Not supported</p>
    pub domain_memberships: Option<Vec<DomainMembership>>,
    /// <p>A list of log types that this DB instance is configured to export to CloudWatch Logs.</p>
    pub enabled_cloudwatch_logs_exports: Option<Vec<String>>,
    /// <p>Specifies the connection endpoint.</p>
    pub endpoint: Option<Endpoint>,
    /// <p>Provides the name of the database engine to be used for this DB instance.</p>
    pub engine: Option<String>,
    /// <p>Indicates the database engine version.</p>
    pub engine_version: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the Amazon CloudWatch Logs log stream that receives the Enhanced Monitoring metrics data for the DB instance.</p>
    pub enhanced_monitoring_resource_arn: Option<String>,
    /// <p>True if AWS Identity and Access Management (IAM) authentication is enabled, and otherwise false.</p>
    pub iam_database_authentication_enabled: Option<bool>,
    /// <p>Provides the date and time the DB instance was created.</p>
    pub instance_create_time: Option<String>,
    /// <p>Specifies the Provisioned IOPS (I/O operations per second) value.</p>
    pub iops: Option<i64>,
    /// <p> If <code>StorageEncrypted</code> is true, the AWS KMS key identifier for the encrypted DB instance. </p>
    pub kms_key_id: Option<String>,
    /// <p>Specifies the latest time to which a database can be restored with point-in-time restore.</p>
    pub latest_restorable_time: Option<String>,
    /// <p>License model information for this DB instance.</p>
    pub license_model: Option<String>,
    /// <p>Contains the master username for the DB instance.</p>
    pub master_username: Option<String>,
    /// <p>The interval, in seconds, between points when Enhanced Monitoring metrics are collected for the DB instance.</p>
    pub monitoring_interval: Option<i64>,
    /// <p>The ARN for the IAM role that permits Neptune to send Enhanced Monitoring metrics to Amazon CloudWatch Logs.</p>
    pub monitoring_role_arn: Option<String>,
    /// <p>Specifies if the DB instance is a Multi-AZ deployment.</p>
    pub multi_az: Option<bool>,
    /// <p>Provides the list of option group memberships for this DB instance.</p>
    pub option_group_memberships: Option<Vec<OptionGroupMembership>>,
    /// <p>Specifies that changes to the DB instance are pending. This element is only included when changes are pending. Specific changes are identified by subelements.</p>
    pub pending_modified_values: Option<PendingModifiedValues>,
    /// <p>True if Performance Insights is enabled for the DB instance, and otherwise false.</p>
    pub performance_insights_enabled: Option<bool>,
    /// <p>The AWS KMS key identifier for encryption of Performance Insights data. The KMS key ID is the Amazon Resource Name (ARN), KMS key identifier, or the KMS key alias for the KMS encryption key.</p>
    pub performance_insights_kms_key_id: Option<String>,
    /// <p> Specifies the daily time range during which automated backups are created if automated backups are enabled, as determined by the <code>BackupRetentionPeriod</code>. </p>
    pub preferred_backup_window: Option<String>,
    /// <p>Specifies the weekly time range during which system maintenance can occur, in Universal Coordinated Time (UTC).</p>
    pub preferred_maintenance_window: Option<String>,
    /// <p>A value that specifies the order in which a Read Replica is promoted to the primary instance after a failure of the existing primary instance. </p>
    pub promotion_tier: Option<i64>,
    /// <p>Contains one or more identifiers of DB clusters that are Read Replicas of this DB instance.</p>
    pub read_replica_db_cluster_identifiers: Option<Vec<String>>,
    /// <p>Contains one or more identifiers of the Read Replicas associated with this DB instance.</p>
    pub read_replica_db_instance_identifiers: Option<Vec<String>>,
    /// <p>Contains the identifier of the source DB instance if this DB instance is a Read Replica.</p>
    pub read_replica_source_db_instance_identifier: Option<String>,
    /// <p>If present, specifies the name of the secondary Availability Zone for a DB instance with multi-AZ support.</p>
    pub secondary_availability_zone: Option<String>,
    /// <p>The status of a Read Replica. If the instance is not a Read Replica, this is blank.</p>
    pub status_infos: Option<Vec<DBInstanceStatusInfo>>,
    /// <p>Specifies whether the DB instance is encrypted.</p>
    pub storage_encrypted: Option<bool>,
    /// <p>Specifies the storage type associated with DB instance.</p>
    pub storage_type: Option<String>,
    /// <p>The ARN from the key store with which the instance is associated for TDE encryption.</p>
    pub tde_credential_arn: Option<String>,
    /// <p>Not supported. </p>
    pub timezone: Option<String>,
    /// <p>Provides a list of VPC security group elements that the DB instance belongs to.</p>
    pub vpc_security_groups: Option<Vec<VpcSecurityGroupMembership>>,
}

struct DBInstanceDeserializer;
impl DBInstanceDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DBInstance, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DBInstance::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "AllocatedStorage" => {
                        obj.allocated_storage = Some(try!(IntegerDeserializer::deserialize(
                            "AllocatedStorage",
                            stack
                        )));
                    }
                    "AutoMinorVersionUpgrade" => {
                        obj.auto_minor_version_upgrade = Some(try!(
                            BooleanDeserializer::deserialize("AutoMinorVersionUpgrade", stack)
                        ));
                    }
                    "AvailabilityZone" => {
                        obj.availability_zone = Some(try!(StringDeserializer::deserialize(
                            "AvailabilityZone",
                            stack
                        )));
                    }
                    "BackupRetentionPeriod" => {
                        obj.backup_retention_period = Some(try!(IntegerDeserializer::deserialize(
                            "BackupRetentionPeriod",
                            stack
                        )));
                    }
                    "CACertificateIdentifier" => {
                        obj.ca_certificate_identifier = Some(try!(
                            StringDeserializer::deserialize("CACertificateIdentifier", stack)
                        ));
                    }
                    "CharacterSetName" => {
                        obj.character_set_name = Some(try!(StringDeserializer::deserialize(
                            "CharacterSetName",
                            stack
                        )));
                    }
                    "CopyTagsToSnapshot" => {
                        obj.copy_tags_to_snapshot = Some(try!(BooleanDeserializer::deserialize(
                            "CopyTagsToSnapshot",
                            stack
                        )));
                    }
                    "DBClusterIdentifier" => {
                        obj.db_cluster_identifier = Some(try!(StringDeserializer::deserialize(
                            "DBClusterIdentifier",
                            stack
                        )));
                    }
                    "DBInstanceArn" => {
                        obj.db_instance_arn = Some(try!(StringDeserializer::deserialize(
                            "DBInstanceArn",
                            stack
                        )));
                    }
                    "DBInstanceClass" => {
                        obj.db_instance_class = Some(try!(StringDeserializer::deserialize(
                            "DBInstanceClass",
                            stack
                        )));
                    }
                    "DBInstanceIdentifier" => {
                        obj.db_instance_identifier = Some(try!(StringDeserializer::deserialize(
                            "DBInstanceIdentifier",
                            stack
                        )));
                    }
                    "DBInstanceStatus" => {
                        obj.db_instance_status = Some(try!(StringDeserializer::deserialize(
                            "DBInstanceStatus",
                            stack
                        )));
                    }
                    "DBName" => {
                        obj.db_name = Some(try!(StringDeserializer::deserialize("DBName", stack)));
                    }
                    "DBParameterGroups" => {
                        obj.db_parameter_groups =
                            Some(try!(DBParameterGroupStatusListDeserializer::deserialize(
                                "DBParameterGroups",
                                stack
                            )));
                    }
                    "DBSecurityGroups" => {
                        obj.db_security_groups = Some(try!(
                            DBSecurityGroupMembershipListDeserializer::deserialize(
                                "DBSecurityGroups",
                                stack
                            )
                        ));
                    }
                    "DBSubnetGroup" => {
                        obj.db_subnet_group = Some(try!(DBSubnetGroupDeserializer::deserialize(
                            "DBSubnetGroup",
                            stack
                        )));
                    }
                    "DbInstancePort" => {
                        obj.db_instance_port = Some(try!(IntegerDeserializer::deserialize(
                            "DbInstancePort",
                            stack
                        )));
                    }
                    "DbiResourceId" => {
                        obj.dbi_resource_id = Some(try!(StringDeserializer::deserialize(
                            "DbiResourceId",
                            stack
                        )));
                    }
                    "DomainMemberships" => {
                        obj.domain_memberships =
                            Some(try!(DomainMembershipListDeserializer::deserialize(
                                "DomainMemberships",
                                stack
                            )));
                    }
                    "EnabledCloudwatchLogsExports" => {
                        obj.enabled_cloudwatch_logs_exports =
                            Some(try!(LogTypeListDeserializer::deserialize(
                                "EnabledCloudwatchLogsExports",
                                stack
                            )));
                    }
                    "Endpoint" => {
                        obj.endpoint =
                            Some(try!(EndpointDeserializer::deserialize("Endpoint", stack)));
                    }
                    "Engine" => {
                        obj.engine = Some(try!(StringDeserializer::deserialize("Engine", stack)));
                    }
                    "EngineVersion" => {
                        obj.engine_version = Some(try!(StringDeserializer::deserialize(
                            "EngineVersion",
                            stack
                        )));
                    }
                    "EnhancedMonitoringResourceArn" => {
                        obj.enhanced_monitoring_resource_arn = Some(try!(
                            StringDeserializer::deserialize("EnhancedMonitoringResourceArn", stack)
                        ));
                    }
                    "IAMDatabaseAuthenticationEnabled" => {
                        obj.iam_database_authentication_enabled =
                            Some(try!(BooleanDeserializer::deserialize(
                                "IAMDatabaseAuthenticationEnabled",
                                stack
                            )));
                    }
                    "InstanceCreateTime" => {
                        obj.instance_create_time = Some(try!(TStampDeserializer::deserialize(
                            "InstanceCreateTime",
                            stack
                        )));
                    }
                    "Iops" => {
                        obj.iops = Some(try!(IntegerOptionalDeserializer::deserialize(
                            "Iops", stack
                        )));
                    }
                    "KmsKeyId" => {
                        obj.kms_key_id =
                            Some(try!(StringDeserializer::deserialize("KmsKeyId", stack)));
                    }
                    "LatestRestorableTime" => {
                        obj.latest_restorable_time = Some(try!(TStampDeserializer::deserialize(
                            "LatestRestorableTime",
                            stack
                        )));
                    }
                    "LicenseModel" => {
                        obj.license_model =
                            Some(try!(StringDeserializer::deserialize("LicenseModel", stack)));
                    }
                    "MasterUsername" => {
                        obj.master_username = Some(try!(StringDeserializer::deserialize(
                            "MasterUsername",
                            stack
                        )));
                    }
                    "MonitoringInterval" => {
                        obj.monitoring_interval = Some(try!(
                            IntegerOptionalDeserializer::deserialize("MonitoringInterval", stack)
                        ));
                    }
                    "MonitoringRoleArn" => {
                        obj.monitoring_role_arn = Some(try!(StringDeserializer::deserialize(
                            "MonitoringRoleArn",
                            stack
                        )));
                    }
                    "MultiAZ" => {
                        obj.multi_az =
                            Some(try!(BooleanDeserializer::deserialize("MultiAZ", stack)));
                    }
                    "OptionGroupMemberships" => {
                        obj.option_group_memberships =
                            Some(try!(OptionGroupMembershipListDeserializer::deserialize(
                                "OptionGroupMemberships",
                                stack
                            )));
                    }
                    "PendingModifiedValues" => {
                        obj.pending_modified_values =
                            Some(try!(PendingModifiedValuesDeserializer::deserialize(
                                "PendingModifiedValues",
                                stack
                            )));
                    }
                    "PerformanceInsightsEnabled" => {
                        obj.performance_insights_enabled =
                            Some(try!(BooleanOptionalDeserializer::deserialize(
                                "PerformanceInsightsEnabled",
                                stack
                            )));
                    }
                    "PerformanceInsightsKMSKeyId" => {
                        obj.performance_insights_kms_key_id = Some(try!(
                            StringDeserializer::deserialize("PerformanceInsightsKMSKeyId", stack)
                        ));
                    }
                    "PreferredBackupWindow" => {
                        obj.preferred_backup_window = Some(try!(StringDeserializer::deserialize(
                            "PreferredBackupWindow",
                            stack
                        )));
                    }
                    "PreferredMaintenanceWindow" => {
                        obj.preferred_maintenance_window = Some(try!(
                            StringDeserializer::deserialize("PreferredMaintenanceWindow", stack)
                        ));
                    }
                    "PromotionTier" => {
                        obj.promotion_tier = Some(try!(IntegerOptionalDeserializer::deserialize(
                            "PromotionTier",
                            stack
                        )));
                    }
                    "ReadReplicaDBClusterIdentifiers" => {
                        obj.read_replica_db_cluster_identifiers = Some(try!(
                            ReadReplicaDBClusterIdentifierListDeserializer::deserialize(
                                "ReadReplicaDBClusterIdentifiers",
                                stack
                            )
                        ));
                    }
                    "ReadReplicaDBInstanceIdentifiers" => {
                        obj.read_replica_db_instance_identifiers = Some(try!(
                            ReadReplicaDBInstanceIdentifierListDeserializer::deserialize(
                                "ReadReplicaDBInstanceIdentifiers",
                                stack
                            )
                        ));
                    }
                    "ReadReplicaSourceDBInstanceIdentifier" => {
                        obj.read_replica_source_db_instance_identifier =
                            Some(try!(StringDeserializer::deserialize(
                                "ReadReplicaSourceDBInstanceIdentifier",
                                stack
                            )));
                    }
                    "SecondaryAvailabilityZone" => {
                        obj.secondary_availability_zone = Some(try!(
                            StringDeserializer::deserialize("SecondaryAvailabilityZone", stack)
                        ));
                    }
                    "StatusInfos" => {
                        obj.status_infos = Some(try!(
                            DBInstanceStatusInfoListDeserializer::deserialize("StatusInfos", stack)
                        ));
                    }
                    "StorageEncrypted" => {
                        obj.storage_encrypted = Some(try!(BooleanDeserializer::deserialize(
                            "StorageEncrypted",
                            stack
                        )));
                    }
                    "StorageType" => {
                        obj.storage_type =
                            Some(try!(StringDeserializer::deserialize("StorageType", stack)));
                    }
                    "TdeCredentialArn" => {
                        obj.tde_credential_arn = Some(try!(StringDeserializer::deserialize(
                            "TdeCredentialArn",
                            stack
                        )));
                    }
                    "Timezone" => {
                        obj.timezone =
                            Some(try!(StringDeserializer::deserialize("Timezone", stack)));
                    }
                    "VpcSecurityGroups" => {
                        obj.vpc_security_groups = Some(try!(
                            VpcSecurityGroupMembershipListDeserializer::deserialize(
                                "VpcSecurityGroups",
                                stack
                            )
                        ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct DBInstanceListDeserializer;
impl DBInstanceListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<DBInstance>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "DBInstance" {
                        obj.push(try!(DBInstanceDeserializer::deserialize(
                            "DBInstance",
                            stack
                        )));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}
/// <p> Contains the result of a successful invocation of the <a>DescribeDBInstances</a> action. </p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DBInstanceMessage {
    /// <p> A list of <a>DBInstance</a> instances. </p>
    pub db_instances: Option<Vec<DBInstance>>,
    /// <p> An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code> . </p>
    pub marker: Option<String>,
}

struct DBInstanceMessageDeserializer;
impl DBInstanceMessageDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DBInstanceMessage, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DBInstanceMessage::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "DBInstances" => {
                        obj.db_instances = Some(try!(DBInstanceListDeserializer::deserialize(
                            "DBInstances",
                            stack
                        )));
                    }
                    "Marker" => {
                        obj.marker = Some(try!(StringDeserializer::deserialize("Marker", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Provides a list of status information for a DB instance.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DBInstanceStatusInfo {
    /// <p>Details of the error if there is an error for the instance. If the instance is not in an error state, this value is blank.</p>
    pub message: Option<String>,
    /// <p>Boolean value that is true if the instance is operating normally, or false if the instance is in an error state.</p>
    pub normal: Option<bool>,
    /// <p>Status of the DB instance. For a StatusType of read replica, the values can be replicating, error, stopped, or terminated.</p>
    pub status: Option<String>,
    /// <p>This value is currently "read replication."</p>
    pub status_type: Option<String>,
}

struct DBInstanceStatusInfoDeserializer;
impl DBInstanceStatusInfoDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DBInstanceStatusInfo, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DBInstanceStatusInfo::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Message" => {
                        obj.message = Some(try!(StringDeserializer::deserialize("Message", stack)));
                    }
                    "Normal" => {
                        obj.normal = Some(try!(BooleanDeserializer::deserialize("Normal", stack)));
                    }
                    "Status" => {
                        obj.status = Some(try!(StringDeserializer::deserialize("Status", stack)));
                    }
                    "StatusType" => {
                        obj.status_type =
                            Some(try!(StringDeserializer::deserialize("StatusType", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct DBInstanceStatusInfoListDeserializer;
impl DBInstanceStatusInfoListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<DBInstanceStatusInfo>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "DBInstanceStatusInfo" {
                        obj.push(try!(DBInstanceStatusInfoDeserializer::deserialize(
                            "DBInstanceStatusInfo",
                            stack
                        )));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}
/// <p>Contains the details of an Amazon Neptune DB parameter group. </p> <p>This data type is used as a response element in the <a>DescribeDBParameterGroups</a> action. </p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DBParameterGroup {
    /// <p>The Amazon Resource Name (ARN) for the DB parameter group.</p>
    pub db_parameter_group_arn: Option<String>,
    /// <p>Provides the name of the DB parameter group family that this DB parameter group is compatible with.</p>
    pub db_parameter_group_family: Option<String>,
    /// <p>Provides the name of the DB parameter group.</p>
    pub db_parameter_group_name: Option<String>,
    /// <p>Provides the customer-specified description for this DB parameter group.</p>
    pub description: Option<String>,
}

struct DBParameterGroupDeserializer;
impl DBParameterGroupDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DBParameterGroup, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DBParameterGroup::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "DBParameterGroupArn" => {
                            obj.db_parameter_group_arn = Some(try!(
                                StringDeserializer::deserialize("DBParameterGroupArn", stack)
                            ));
                        }
                        "DBParameterGroupFamily" => {
                            obj.db_parameter_group_family = Some(try!(
                                StringDeserializer::deserialize("DBParameterGroupFamily", stack)
                            ));
                        }
                        "DBParameterGroupName" => {
                            obj.db_parameter_group_name = Some(try!(
                                StringDeserializer::deserialize("DBParameterGroupName", stack)
                            ));
                        }
                        "Description" => {
                            obj.description =
                                Some(try!(StringDeserializer::deserialize("Description", stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p> Contains the result of a successful invocation of the <a>DescribeDBParameters</a> action. </p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DBParameterGroupDetails {
    /// <p> An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>. </p>
    pub marker: Option<String>,
    /// <p> A list of <a>Parameter</a> values. </p>
    pub parameters: Option<Vec<Parameter>>,
}

struct DBParameterGroupDetailsDeserializer;
impl DBParameterGroupDetailsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DBParameterGroupDetails, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DBParameterGroupDetails::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Marker" => {
                        obj.marker = Some(try!(StringDeserializer::deserialize("Marker", stack)));
                    }
                    "Parameters" => {
                        obj.parameters = Some(try!(ParametersListDeserializer::deserialize(
                            "Parameters",
                            stack
                        )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct DBParameterGroupListDeserializer;
impl DBParameterGroupListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<DBParameterGroup>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "DBParameterGroup" {
                        obj.push(try!(DBParameterGroupDeserializer::deserialize(
                            "DBParameterGroup",
                            stack
                        )));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}
/// <p> Contains the result of a successful invocation of the <a>ModifyDBParameterGroup</a> or <a>ResetDBParameterGroup</a> action. </p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DBParameterGroupNameMessage {
    /// <p>Provides the name of the DB parameter group.</p>
    pub db_parameter_group_name: Option<String>,
}

struct DBParameterGroupNameMessageDeserializer;
impl DBParameterGroupNameMessageDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DBParameterGroupNameMessage, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DBParameterGroupNameMessage::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "DBParameterGroupName" => {
                        obj.db_parameter_group_name = Some(try!(StringDeserializer::deserialize(
                            "DBParameterGroupName",
                            stack
                        )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p><p>The status of the DB parameter group.</p> <p>This data type is used as a response element in the following actions:</p> <ul> <li> <p> <a>CreateDBInstance</a> </p> </li> <li> <p> <a>DeleteDBInstance</a> </p> </li> <li> <p> <a>ModifyDBInstance</a> </p> </li> <li> <p> <a>RebootDBInstance</a> </p> </li> </ul></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DBParameterGroupStatus {
    /// <p>The name of the DP parameter group.</p>
    pub db_parameter_group_name: Option<String>,
    /// <p>The status of parameter updates.</p>
    pub parameter_apply_status: Option<String>,
}

struct DBParameterGroupStatusDeserializer;
impl DBParameterGroupStatusDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DBParameterGroupStatus, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DBParameterGroupStatus::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "DBParameterGroupName" => {
                            obj.db_parameter_group_name = Some(try!(
                                StringDeserializer::deserialize("DBParameterGroupName", stack)
                            ));
                        }
                        "ParameterApplyStatus" => {
                            obj.parameter_apply_status = Some(try!(
                                StringDeserializer::deserialize("ParameterApplyStatus", stack)
                            ));
                        }
                        _ => skip_tree(stack),
                    }
                }
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct DBParameterGroupStatusListDeserializer;
impl DBParameterGroupStatusListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<DBParameterGroupStatus>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "DBParameterGroup" {
                        obj.push(try!(DBParameterGroupStatusDeserializer::deserialize(
                            "DBParameterGroup",
                            stack
                        )));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}
/// <p> Contains the result of a successful invocation of the <a>DescribeDBParameterGroups</a> action. </p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DBParameterGroupsMessage {
    /// <p> A list of <a>DBParameterGroup</a> instances. </p>
    pub db_parameter_groups: Option<Vec<DBParameterGroup>>,
    /// <p> An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>. </p>
    pub marker: Option<String>,
}

struct DBParameterGroupsMessageDeserializer;
impl DBParameterGroupsMessageDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DBParameterGroupsMessage, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DBParameterGroupsMessage::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "DBParameterGroups" => {
                        obj.db_parameter_groups =
                            Some(try!(DBParameterGroupListDeserializer::deserialize(
                                "DBParameterGroups",
                                stack
                            )));
                    }
                    "Marker" => {
                        obj.marker = Some(try!(StringDeserializer::deserialize("Marker", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p><p>This data type is used as a response element in the following actions:</p> <ul> <li> <p> <a>ModifyDBInstance</a> </p> </li> <li> <p> <a>RebootDBInstance</a> </p> </li> </ul></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DBSecurityGroupMembership {
    /// <p>The name of the DB security group.</p>
    pub db_security_group_name: Option<String>,
    /// <p>The status of the DB security group.</p>
    pub status: Option<String>,
}

struct DBSecurityGroupMembershipDeserializer;
impl DBSecurityGroupMembershipDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DBSecurityGroupMembership, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DBSecurityGroupMembership::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "DBSecurityGroupName" => {
                        obj.db_security_group_name = Some(try!(StringDeserializer::deserialize(
                            "DBSecurityGroupName",
                            stack
                        )));
                    }
                    "Status" => {
                        obj.status = Some(try!(StringDeserializer::deserialize("Status", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct DBSecurityGroupMembershipListDeserializer;
impl DBSecurityGroupMembershipListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<DBSecurityGroupMembership>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "DBSecurityGroup" {
                        obj.push(try!(DBSecurityGroupMembershipDeserializer::deserialize(
                            "DBSecurityGroup",
                            stack
                        )));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}

/// Serialize `DBSecurityGroupNameList` contents to a `SignedRequest`.
struct DBSecurityGroupNameListSerializer;
impl DBSecurityGroupNameListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

/// <p>Contains the details of an Amazon Neptune DB subnet group. </p> <p>This data type is used as a response element in the <a>DescribeDBSubnetGroups</a> action. </p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DBSubnetGroup {
    /// <p>The Amazon Resource Name (ARN) for the DB subnet group.</p>
    pub db_subnet_group_arn: Option<String>,
    /// <p>Provides the description of the DB subnet group.</p>
    pub db_subnet_group_description: Option<String>,
    /// <p>The name of the DB subnet group.</p>
    pub db_subnet_group_name: Option<String>,
    /// <p>Provides the status of the DB subnet group.</p>
    pub subnet_group_status: Option<String>,
    /// <p> Contains a list of <a>Subnet</a> elements. </p>
    pub subnets: Option<Vec<Subnet>>,
    /// <p>Provides the VpcId of the DB subnet group.</p>
    pub vpc_id: Option<String>,
}

struct DBSubnetGroupDeserializer;
impl DBSubnetGroupDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DBSubnetGroup, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DBSubnetGroup::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "DBSubnetGroupArn" => {
                        obj.db_subnet_group_arn = Some(try!(StringDeserializer::deserialize(
                            "DBSubnetGroupArn",
                            stack
                        )));
                    }
                    "DBSubnetGroupDescription" => {
                        obj.db_subnet_group_description = Some(try!(
                            StringDeserializer::deserialize("DBSubnetGroupDescription", stack)
                        ));
                    }
                    "DBSubnetGroupName" => {
                        obj.db_subnet_group_name = Some(try!(StringDeserializer::deserialize(
                            "DBSubnetGroupName",
                            stack
                        )));
                    }
                    "SubnetGroupStatus" => {
                        obj.subnet_group_status = Some(try!(StringDeserializer::deserialize(
                            "SubnetGroupStatus",
                            stack
                        )));
                    }
                    "Subnets" => {
                        obj.subnets =
                            Some(try!(SubnetListDeserializer::deserialize("Subnets", stack)));
                    }
                    "VpcId" => {
                        obj.vpc_id = Some(try!(StringDeserializer::deserialize("VpcId", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p> Contains the result of a successful invocation of the <a>DescribeDBSubnetGroups</a> action. </p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DBSubnetGroupMessage {
    /// <p> A list of <a>DBSubnetGroup</a> instances. </p>
    pub db_subnet_groups: Option<Vec<DBSubnetGroup>>,
    /// <p> An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>. </p>
    pub marker: Option<String>,
}

struct DBSubnetGroupMessageDeserializer;
impl DBSubnetGroupMessageDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DBSubnetGroupMessage, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DBSubnetGroupMessage::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "DBSubnetGroups" => {
                        obj.db_subnet_groups = Some(try!(DBSubnetGroupsDeserializer::deserialize(
                            "DBSubnetGroups",
                            stack
                        )));
                    }
                    "Marker" => {
                        obj.marker = Some(try!(StringDeserializer::deserialize("Marker", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct DBSubnetGroupsDeserializer;
impl DBSubnetGroupsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<DBSubnetGroup>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "DBSubnetGroup" {
                        obj.push(try!(DBSubnetGroupDeserializer::deserialize(
                            "DBSubnetGroup",
                            stack
                        )));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}
/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteDBClusterMessage {
    /// <p><p>The DB cluster identifier for the DB cluster to be deleted. This parameter isn&#39;t case-sensitive.</p> <p>Constraints:</p> <ul> <li> <p>Must match an existing DBClusterIdentifier.</p> </li> </ul></p>
    pub db_cluster_identifier: String,
    /// <p><p> The DB cluster snapshot identifier of the new DB cluster snapshot created when <code>SkipFinalSnapshot</code> is set to <code>false</code>. </p> <note> <p> Specifying this parameter and also setting the <code>SkipFinalShapshot</code> parameter to true results in an error. </p> </note> <p>Constraints:</p> <ul> <li> <p>Must be 1 to 255 letters, numbers, or hyphens.</p> </li> <li> <p>First character must be a letter</p> </li> <li> <p>Cannot end with a hyphen or contain two consecutive hyphens</p> </li> </ul></p>
    pub final_db_snapshot_identifier: Option<String>,
    /// <p> Determines whether a final DB cluster snapshot is created before the DB cluster is deleted. If <code>true</code> is specified, no DB cluster snapshot is created. If <code>false</code> is specified, a DB cluster snapshot is created before the DB cluster is deleted. </p> <note> <p>You must specify a <code>FinalDBSnapshotIdentifier</code> parameter if <code>SkipFinalSnapshot</code> is <code>false</code>.</p> </note> <p>Default: <code>false</code> </p>
    pub skip_final_snapshot: Option<bool>,
}

/// Serialize `DeleteDBClusterMessage` contents to a `SignedRequest`.
struct DeleteDBClusterMessageSerializer;
impl DeleteDBClusterMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DeleteDBClusterMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "DBClusterIdentifier"),
            &obj.db_cluster_identifier,
        );
        if let Some(ref field_value) = obj.final_db_snapshot_identifier {
            params.put(
                &format!("{}{}", prefix, "FinalDBSnapshotIdentifier"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.skip_final_snapshot {
            params.put(
                &format!("{}{}", prefix, "SkipFinalSnapshot"),
                &field_value.to_string(),
            );
        }
    }
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteDBClusterParameterGroupMessage {
    /// <p><p>The name of the DB cluster parameter group.</p> <p>Constraints:</p> <ul> <li> <p>Must be the name of an existing DB cluster parameter group.</p> </li> <li> <p>You can&#39;t delete a default DB cluster parameter group.</p> </li> <li> <p>Cannot be associated with any DB clusters.</p> </li> </ul></p>
    pub db_cluster_parameter_group_name: String,
}

/// Serialize `DeleteDBClusterParameterGroupMessage` contents to a `SignedRequest`.
struct DeleteDBClusterParameterGroupMessageSerializer;
impl DeleteDBClusterParameterGroupMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DeleteDBClusterParameterGroupMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "DBClusterParameterGroupName"),
            &obj.db_cluster_parameter_group_name,
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteDBClusterResult {
    pub db_cluster: Option<DBCluster>,
}

struct DeleteDBClusterResultDeserializer;
impl DeleteDBClusterResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteDBClusterResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DeleteDBClusterResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "DBCluster" => {
                        obj.db_cluster =
                            Some(try!(DBClusterDeserializer::deserialize("DBCluster", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteDBClusterSnapshotMessage {
    /// <p>The identifier of the DB cluster snapshot to delete.</p> <p>Constraints: Must be the name of an existing DB cluster snapshot in the <code>available</code> state.</p>
    pub db_cluster_snapshot_identifier: String,
}

/// Serialize `DeleteDBClusterSnapshotMessage` contents to a `SignedRequest`.
struct DeleteDBClusterSnapshotMessageSerializer;
impl DeleteDBClusterSnapshotMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DeleteDBClusterSnapshotMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "DBClusterSnapshotIdentifier"),
            &obj.db_cluster_snapshot_identifier,
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteDBClusterSnapshotResult {
    pub db_cluster_snapshot: Option<DBClusterSnapshot>,
}

struct DeleteDBClusterSnapshotResultDeserializer;
impl DeleteDBClusterSnapshotResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteDBClusterSnapshotResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DeleteDBClusterSnapshotResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "DBClusterSnapshot" => {
                        obj.db_cluster_snapshot = Some(try!(
                            DBClusterSnapshotDeserializer::deserialize("DBClusterSnapshot", stack)
                        ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteDBInstanceMessage {
    /// <p><p>The DB instance identifier for the DB instance to be deleted. This parameter isn&#39;t case-sensitive.</p> <p>Constraints:</p> <ul> <li> <p>Must match the name of an existing DB instance.</p> </li> </ul></p>
    pub db_instance_identifier: String,
    /// <p><p> The DBSnapshotIdentifier of the new DBSnapshot created when SkipFinalSnapshot is set to <code>false</code>. </p> <note> <p>Specifying this parameter and also setting the SkipFinalShapshot parameter to true results in an error.</p> </note> <p>Constraints:</p> <ul> <li> <p>Must be 1 to 255 letters or numbers.</p> </li> <li> <p>First character must be a letter</p> </li> <li> <p>Cannot end with a hyphen or contain two consecutive hyphens</p> </li> <li> <p>Cannot be specified when deleting a Read Replica.</p> </li> </ul></p>
    pub final_db_snapshot_identifier: Option<String>,
    /// <p> Determines whether a final DB snapshot is created before the DB instance is deleted. If <code>true</code> is specified, no DBSnapshot is created. If <code>false</code> is specified, a DB snapshot is created before the DB instance is deleted. </p> <p>Note that when a DB instance is in a failure state and has a status of 'failed', 'incompatible-restore', or 'incompatible-network', it can only be deleted when the SkipFinalSnapshot parameter is set to "true".</p> <p>Specify <code>true</code> when deleting a Read Replica.</p> <note> <p>The FinalDBSnapshotIdentifier parameter must be specified if SkipFinalSnapshot is <code>false</code>.</p> </note> <p>Default: <code>false</code> </p>
    pub skip_final_snapshot: Option<bool>,
}

/// Serialize `DeleteDBInstanceMessage` contents to a `SignedRequest`.
struct DeleteDBInstanceMessageSerializer;
impl DeleteDBInstanceMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DeleteDBInstanceMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "DBInstanceIdentifier"),
            &obj.db_instance_identifier,
        );
        if let Some(ref field_value) = obj.final_db_snapshot_identifier {
            params.put(
                &format!("{}{}", prefix, "FinalDBSnapshotIdentifier"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.skip_final_snapshot {
            params.put(
                &format!("{}{}", prefix, "SkipFinalSnapshot"),
                &field_value.to_string(),
            );
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteDBInstanceResult {
    pub db_instance: Option<DBInstance>,
}

struct DeleteDBInstanceResultDeserializer;
impl DeleteDBInstanceResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteDBInstanceResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DeleteDBInstanceResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "DBInstance" => {
                        obj.db_instance = Some(try!(DBInstanceDeserializer::deserialize(
                            "DBInstance",
                            stack
                        )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteDBParameterGroupMessage {
    /// <p><p>The name of the DB parameter group.</p> <p>Constraints:</p> <ul> <li> <p>Must be the name of an existing DB parameter group</p> </li> <li> <p>You can&#39;t delete a default DB parameter group</p> </li> <li> <p>Cannot be associated with any DB instances</p> </li> </ul></p>
    pub db_parameter_group_name: String,
}

/// Serialize `DeleteDBParameterGroupMessage` contents to a `SignedRequest`.
struct DeleteDBParameterGroupMessageSerializer;
impl DeleteDBParameterGroupMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DeleteDBParameterGroupMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "DBParameterGroupName"),
            &obj.db_parameter_group_name,
        );
    }
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteDBSubnetGroupMessage {
    /// <p>The name of the database subnet group to delete.</p> <note> <p>You can't delete the default subnet group.</p> </note> <p>Constraints:</p> <p>Constraints: Must match the name of an existing DBSubnetGroup. Must not be default.</p> <p>Example: <code>mySubnetgroup</code> </p>
    pub db_subnet_group_name: String,
}

/// Serialize `DeleteDBSubnetGroupMessage` contents to a `SignedRequest`.
struct DeleteDBSubnetGroupMessageSerializer;
impl DeleteDBSubnetGroupMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DeleteDBSubnetGroupMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "DBSubnetGroupName"),
            &obj.db_subnet_group_name,
        );
    }
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteEventSubscriptionMessage {
    /// <p>The name of the event notification subscription you want to delete.</p>
    pub subscription_name: String,
}

/// Serialize `DeleteEventSubscriptionMessage` contents to a `SignedRequest`.
struct DeleteEventSubscriptionMessageSerializer;
impl DeleteEventSubscriptionMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DeleteEventSubscriptionMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "SubscriptionName"),
            &obj.subscription_name,
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteEventSubscriptionResult {
    pub event_subscription: Option<EventSubscription>,
}

struct DeleteEventSubscriptionResultDeserializer;
impl DeleteEventSubscriptionResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteEventSubscriptionResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DeleteEventSubscriptionResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "EventSubscription" => {
                        obj.event_subscription = Some(try!(
                            EventSubscriptionDeserializer::deserialize("EventSubscription", stack)
                        ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeDBClusterParameterGroupsMessage {
    /// <p><p>The name of a specific DB cluster parameter group to return details for.</p> <p>Constraints:</p> <ul> <li> <p>If supplied, must match the name of an existing DBClusterParameterGroup.</p> </li> </ul></p>
    pub db_cluster_parameter_group_name: Option<String>,
    /// <p>This parameter is not currently supported.</p>
    pub filters: Option<Vec<Filter>>,
    /// <p> An optional pagination token provided by a previous <code>DescribeDBClusterParameterGroups</code> request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>. </p>
    pub marker: Option<String>,
    /// <p> The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token called a marker is included in the response so that the remaining results can be retrieved. </p> <p>Default: 100</p> <p>Constraints: Minimum 20, maximum 100.</p>
    pub max_records: Option<i64>,
}

/// Serialize `DescribeDBClusterParameterGroupsMessage` contents to a `SignedRequest`.
struct DescribeDBClusterParameterGroupsMessageSerializer;
impl DescribeDBClusterParameterGroupsMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeDBClusterParameterGroupsMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.db_cluster_parameter_group_name {
            params.put(
                &format!("{}{}", prefix, "DBClusterParameterGroupName"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.filters {
            FilterListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Filter"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.marker {
            params.put(&format!("{}{}", prefix, "Marker"), &field_value);
        }
        if let Some(ref field_value) = obj.max_records {
            params.put(
                &format!("{}{}", prefix, "MaxRecords"),
                &field_value.to_string(),
            );
        }
    }
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeDBClusterParametersMessage {
    /// <p><p>The name of a specific DB cluster parameter group to return parameter details for.</p> <p>Constraints:</p> <ul> <li> <p>If supplied, must match the name of an existing DBClusterParameterGroup.</p> </li> </ul></p>
    pub db_cluster_parameter_group_name: String,
    /// <p>This parameter is not currently supported.</p>
    pub filters: Option<Vec<Filter>>,
    /// <p> An optional pagination token provided by a previous <code>DescribeDBClusterParameters</code> request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>. </p>
    pub marker: Option<String>,
    /// <p> The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token called a marker is included in the response so that the remaining results can be retrieved. </p> <p>Default: 100</p> <p>Constraints: Minimum 20, maximum 100.</p>
    pub max_records: Option<i64>,
    /// <p> A value that indicates to return only parameters for a specific source. Parameter sources can be <code>engine</code>, <code>service</code>, or <code>customer</code>. </p>
    pub source: Option<String>,
}

/// Serialize `DescribeDBClusterParametersMessage` contents to a `SignedRequest`.
struct DescribeDBClusterParametersMessageSerializer;
impl DescribeDBClusterParametersMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeDBClusterParametersMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "DBClusterParameterGroupName"),
            &obj.db_cluster_parameter_group_name,
        );
        if let Some(ref field_value) = obj.filters {
            FilterListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Filter"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.marker {
            params.put(&format!("{}{}", prefix, "Marker"), &field_value);
        }
        if let Some(ref field_value) = obj.max_records {
            params.put(
                &format!("{}{}", prefix, "MaxRecords"),
                &field_value.to_string(),
            );
        }
        if let Some(ref field_value) = obj.source {
            params.put(&format!("{}{}", prefix, "Source"), &field_value);
        }
    }
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeDBClusterSnapshotAttributesMessage {
    /// <p>The identifier for the DB cluster snapshot to describe the attributes for.</p>
    pub db_cluster_snapshot_identifier: String,
}

/// Serialize `DescribeDBClusterSnapshotAttributesMessage` contents to a `SignedRequest`.
struct DescribeDBClusterSnapshotAttributesMessageSerializer;
impl DescribeDBClusterSnapshotAttributesMessageSerializer {
    fn serialize(
        params: &mut Params,
        name: &str,
        obj: &DescribeDBClusterSnapshotAttributesMessage,
    ) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "DBClusterSnapshotIdentifier"),
            &obj.db_cluster_snapshot_identifier,
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeDBClusterSnapshotAttributesResult {
    pub db_cluster_snapshot_attributes_result: Option<DBClusterSnapshotAttributesResult>,
}

struct DescribeDBClusterSnapshotAttributesResultDeserializer;
impl DescribeDBClusterSnapshotAttributesResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeDBClusterSnapshotAttributesResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DescribeDBClusterSnapshotAttributesResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "DBClusterSnapshotAttributesResult" => {
                        obj.db_cluster_snapshot_attributes_result = Some(try!(
                            DBClusterSnapshotAttributesResultDeserializer::deserialize(
                                "DBClusterSnapshotAttributesResult",
                                stack
                            )
                        ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeDBClusterSnapshotsMessage {
    /// <p><p>The ID of the DB cluster to retrieve the list of DB cluster snapshots for. This parameter can&#39;t be used in conjunction with the <code>DBClusterSnapshotIdentifier</code> parameter. This parameter is not case-sensitive. </p> <p>Constraints:</p> <ul> <li> <p>If supplied, must match the identifier of an existing DBCluster.</p> </li> </ul></p>
    pub db_cluster_identifier: Option<String>,
    /// <p><p>A specific DB cluster snapshot identifier to describe. This parameter can&#39;t be used in conjunction with the <code>DBClusterIdentifier</code> parameter. This value is stored as a lowercase string. </p> <p>Constraints:</p> <ul> <li> <p>If supplied, must match the identifier of an existing DBClusterSnapshot.</p> </li> <li> <p>If this identifier is for an automated snapshot, the <code>SnapshotType</code> parameter must also be specified.</p> </li> </ul></p>
    pub db_cluster_snapshot_identifier: Option<String>,
    /// <p>This parameter is not currently supported.</p>
    pub filters: Option<Vec<Filter>>,
    /// <p>True to include manual DB cluster snapshots that are public and can be copied or restored by any AWS account, and otherwise false. The default is <code>false</code>. The default is false.</p> <p>You can share a manual DB cluster snapshot as public by using the <a>ModifyDBClusterSnapshotAttribute</a> API action.</p>
    pub include_public: Option<bool>,
    /// <p>True to include shared manual DB cluster snapshots from other AWS accounts that this AWS account has been given permission to copy or restore, and otherwise false. The default is <code>false</code>.</p> <p>You can give an AWS account permission to restore a manual DB cluster snapshot from another AWS account by the <a>ModifyDBClusterSnapshotAttribute</a> API action.</p>
    pub include_shared: Option<bool>,
    /// <p>An optional pagination token provided by a previous <code>DescribeDBClusterSnapshots</code> request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>. </p>
    pub marker: Option<String>,
    /// <p>The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token called a marker is included in the response so that the remaining results can be retrieved. </p> <p>Default: 100</p> <p>Constraints: Minimum 20, maximum 100.</p>
    pub max_records: Option<i64>,
    /// <p>The type of DB cluster snapshots to be returned. You can specify one of the following values:</p> <ul> <li> <p> <code>automated</code> - Return all DB cluster snapshots that have been automatically taken by Amazon Neptune for my AWS account.</p> </li> <li> <p> <code>manual</code> - Return all DB cluster snapshots that have been taken by my AWS account.</p> </li> <li> <p> <code>shared</code> - Return all manual DB cluster snapshots that have been shared to my AWS account.</p> </li> <li> <p> <code>public</code> - Return all DB cluster snapshots that have been marked as public.</p> </li> </ul> <p>If you don't specify a <code>SnapshotType</code> value, then both automated and manual DB cluster snapshots are returned. You can include shared DB cluster snapshots with these results by setting the <code>IncludeShared</code> parameter to <code>true</code>. You can include public DB cluster snapshots with these results by setting the <code>IncludePublic</code> parameter to <code>true</code>.</p> <p>The <code>IncludeShared</code> and <code>IncludePublic</code> parameters don't apply for <code>SnapshotType</code> values of <code>manual</code> or <code>automated</code>. The <code>IncludePublic</code> parameter doesn't apply when <code>SnapshotType</code> is set to <code>shared</code>. The <code>IncludeShared</code> parameter doesn't apply when <code>SnapshotType</code> is set to <code>public</code>.</p>
    pub snapshot_type: Option<String>,
}

/// Serialize `DescribeDBClusterSnapshotsMessage` contents to a `SignedRequest`.
struct DescribeDBClusterSnapshotsMessageSerializer;
impl DescribeDBClusterSnapshotsMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeDBClusterSnapshotsMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.db_cluster_identifier {
            params.put(
                &format!("{}{}", prefix, "DBClusterIdentifier"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.db_cluster_snapshot_identifier {
            params.put(
                &format!("{}{}", prefix, "DBClusterSnapshotIdentifier"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.filters {
            FilterListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Filter"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.include_public {
            params.put(
                &format!("{}{}", prefix, "IncludePublic"),
                &field_value.to_string(),
            );
        }
        if let Some(ref field_value) = obj.include_shared {
            params.put(
                &format!("{}{}", prefix, "IncludeShared"),
                &field_value.to_string(),
            );
        }
        if let Some(ref field_value) = obj.marker {
            params.put(&format!("{}{}", prefix, "Marker"), &field_value);
        }
        if let Some(ref field_value) = obj.max_records {
            params.put(
                &format!("{}{}", prefix, "MaxRecords"),
                &field_value.to_string(),
            );
        }
        if let Some(ref field_value) = obj.snapshot_type {
            params.put(&format!("{}{}", prefix, "SnapshotType"), &field_value);
        }
    }
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeDBClustersMessage {
    /// <p><p>The user-supplied DB cluster identifier. If this parameter is specified, information from only the specific DB cluster is returned. This parameter isn&#39;t case-sensitive.</p> <p>Constraints:</p> <ul> <li> <p>If supplied, must match an existing DBClusterIdentifier.</p> </li> </ul></p>
    pub db_cluster_identifier: Option<String>,
    /// <p><p>A filter that specifies one or more DB clusters to describe.</p> <p>Supported filters:</p> <ul> <li> <p> <code>db-cluster-id</code> - Accepts DB cluster identifiers and DB cluster Amazon Resource Names (ARNs). The results list will only include information about the DB clusters identified by these ARNs.</p> </li> </ul></p>
    pub filters: Option<Vec<Filter>>,
    /// <p>An optional pagination token provided by a previous <a>DescribeDBClusters</a> request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>. </p>
    pub marker: Option<String>,
    /// <p>The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token called a marker is included in the response so that the remaining results can be retrieved. </p> <p>Default: 100</p> <p>Constraints: Minimum 20, maximum 100.</p>
    pub max_records: Option<i64>,
}

/// Serialize `DescribeDBClustersMessage` contents to a `SignedRequest`.
struct DescribeDBClustersMessageSerializer;
impl DescribeDBClustersMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeDBClustersMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.db_cluster_identifier {
            params.put(
                &format!("{}{}", prefix, "DBClusterIdentifier"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.filters {
            FilterListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Filter"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.marker {
            params.put(&format!("{}{}", prefix, "Marker"), &field_value);
        }
        if let Some(ref field_value) = obj.max_records {
            params.put(
                &format!("{}{}", prefix, "MaxRecords"),
                &field_value.to_string(),
            );
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeDBEngineVersionsMessage {
    /// <p><p>The name of a specific DB parameter group family to return details for.</p> <p>Constraints:</p> <ul> <li> <p>If supplied, must match an existing DBParameterGroupFamily.</p> </li> </ul></p>
    pub db_parameter_group_family: Option<String>,
    /// <p>Indicates that only the default version of the specified engine or engine and major version combination is returned.</p>
    pub default_only: Option<bool>,
    /// <p>The database engine to return.</p>
    pub engine: Option<String>,
    /// <p>The database engine version to return.</p> <p>Example: <code>5.1.49</code> </p>
    pub engine_version: Option<String>,
    /// <p>Not currently supported.</p>
    pub filters: Option<Vec<Filter>>,
    /// <p>If this parameter is specified and the requested engine supports the <code>CharacterSetName</code> parameter for <code>CreateDBInstance</code>, the response includes a list of supported character sets for each engine version. </p>
    pub list_supported_character_sets: Option<bool>,
    /// <p>If this parameter is specified and the requested engine supports the <code>TimeZone</code> parameter for <code>CreateDBInstance</code>, the response includes a list of supported time zones for each engine version. </p>
    pub list_supported_timezones: Option<bool>,
    /// <p> An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>. </p>
    pub marker: Option<String>,
    /// <p> The maximum number of records to include in the response. If more than the <code>MaxRecords</code> value is available, a pagination token called a marker is included in the response so that the following results can be retrieved. </p> <p>Default: 100</p> <p>Constraints: Minimum 20, maximum 100.</p>
    pub max_records: Option<i64>,
}

/// Serialize `DescribeDBEngineVersionsMessage` contents to a `SignedRequest`.
struct DescribeDBEngineVersionsMessageSerializer;
impl DescribeDBEngineVersionsMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeDBEngineVersionsMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.db_parameter_group_family {
            params.put(
                &format!("{}{}", prefix, "DBParameterGroupFamily"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.default_only {
            params.put(
                &format!("{}{}", prefix, "DefaultOnly"),
                &field_value.to_string(),
            );
        }
        if let Some(ref field_value) = obj.engine {
            params.put(&format!("{}{}", prefix, "Engine"), &field_value);
        }
        if let Some(ref field_value) = obj.engine_version {
            params.put(&format!("{}{}", prefix, "EngineVersion"), &field_value);
        }
        if let Some(ref field_value) = obj.filters {
            FilterListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Filter"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.list_supported_character_sets {
            params.put(
                &format!("{}{}", prefix, "ListSupportedCharacterSets"),
                &field_value.to_string(),
            );
        }
        if let Some(ref field_value) = obj.list_supported_timezones {
            params.put(
                &format!("{}{}", prefix, "ListSupportedTimezones"),
                &field_value.to_string(),
            );
        }
        if let Some(ref field_value) = obj.marker {
            params.put(&format!("{}{}", prefix, "Marker"), &field_value);
        }
        if let Some(ref field_value) = obj.max_records {
            params.put(
                &format!("{}{}", prefix, "MaxRecords"),
                &field_value.to_string(),
            );
        }
    }
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeDBInstancesMessage {
    /// <p><p>The user-supplied instance identifier. If this parameter is specified, information from only the specific DB instance is returned. This parameter isn&#39;t case-sensitive.</p> <p>Constraints:</p> <ul> <li> <p>If supplied, must match the identifier of an existing DBInstance.</p> </li> </ul></p>
    pub db_instance_identifier: Option<String>,
    /// <p><p>A filter that specifies one or more DB instances to describe.</p> <p>Supported filters:</p> <ul> <li> <p> <code>db-cluster-id</code> - Accepts DB cluster identifiers and DB cluster Amazon Resource Names (ARNs). The results list will only include information about the DB instances associated with the DB clusters identified by these ARNs.</p> </li> <li> <p> <code>db-instance-id</code> - Accepts DB instance identifiers and DB instance Amazon Resource Names (ARNs). The results list will only include information about the DB instances identified by these ARNs.</p> </li> </ul></p>
    pub filters: Option<Vec<Filter>>,
    /// <p> An optional pagination token provided by a previous <code>DescribeDBInstances</code> request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>. </p>
    pub marker: Option<String>,
    /// <p> The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token called a marker is included in the response so that the remaining results can be retrieved. </p> <p>Default: 100</p> <p>Constraints: Minimum 20, maximum 100.</p>
    pub max_records: Option<i64>,
}

/// Serialize `DescribeDBInstancesMessage` contents to a `SignedRequest`.
struct DescribeDBInstancesMessageSerializer;
impl DescribeDBInstancesMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeDBInstancesMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.db_instance_identifier {
            params.put(
                &format!("{}{}", prefix, "DBInstanceIdentifier"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.filters {
            FilterListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Filter"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.marker {
            params.put(&format!("{}{}", prefix, "Marker"), &field_value);
        }
        if let Some(ref field_value) = obj.max_records {
            params.put(
                &format!("{}{}", prefix, "MaxRecords"),
                &field_value.to_string(),
            );
        }
    }
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeDBParameterGroupsMessage {
    /// <p><p>The name of a specific DB parameter group to return details for.</p> <p>Constraints:</p> <ul> <li> <p>If supplied, must match the name of an existing DBClusterParameterGroup.</p> </li> </ul></p>
    pub db_parameter_group_name: Option<String>,
    /// <p>This parameter is not currently supported.</p>
    pub filters: Option<Vec<Filter>>,
    /// <p> An optional pagination token provided by a previous <code>DescribeDBParameterGroups</code> request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>. </p>
    pub marker: Option<String>,
    /// <p> The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token called a marker is included in the response so that the remaining results can be retrieved. </p> <p>Default: 100</p> <p>Constraints: Minimum 20, maximum 100.</p>
    pub max_records: Option<i64>,
}

/// Serialize `DescribeDBParameterGroupsMessage` contents to a `SignedRequest`.
struct DescribeDBParameterGroupsMessageSerializer;
impl DescribeDBParameterGroupsMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeDBParameterGroupsMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.db_parameter_group_name {
            params.put(
                &format!("{}{}", prefix, "DBParameterGroupName"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.filters {
            FilterListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Filter"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.marker {
            params.put(&format!("{}{}", prefix, "Marker"), &field_value);
        }
        if let Some(ref field_value) = obj.max_records {
            params.put(
                &format!("{}{}", prefix, "MaxRecords"),
                &field_value.to_string(),
            );
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeDBParametersMessage {
    /// <p><p>The name of a specific DB parameter group to return details for.</p> <p>Constraints:</p> <ul> <li> <p>If supplied, must match the name of an existing DBParameterGroup.</p> </li> </ul></p>
    pub db_parameter_group_name: String,
    /// <p>This parameter is not currently supported.</p>
    pub filters: Option<Vec<Filter>>,
    /// <p> An optional pagination token provided by a previous <code>DescribeDBParameters</code> request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>. </p>
    pub marker: Option<String>,
    /// <p> The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token called a marker is included in the response so that the remaining results can be retrieved. </p> <p>Default: 100</p> <p>Constraints: Minimum 20, maximum 100.</p>
    pub max_records: Option<i64>,
    /// <p>The parameter types to return.</p> <p>Default: All parameter types returned</p> <p>Valid Values: <code>user | system | engine-default</code> </p>
    pub source: Option<String>,
}

/// Serialize `DescribeDBParametersMessage` contents to a `SignedRequest`.
struct DescribeDBParametersMessageSerializer;
impl DescribeDBParametersMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeDBParametersMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "DBParameterGroupName"),
            &obj.db_parameter_group_name,
        );
        if let Some(ref field_value) = obj.filters {
            FilterListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Filter"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.marker {
            params.put(&format!("{}{}", prefix, "Marker"), &field_value);
        }
        if let Some(ref field_value) = obj.max_records {
            params.put(
                &format!("{}{}", prefix, "MaxRecords"),
                &field_value.to_string(),
            );
        }
        if let Some(ref field_value) = obj.source {
            params.put(&format!("{}{}", prefix, "Source"), &field_value);
        }
    }
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeDBSubnetGroupsMessage {
    /// <p>The name of the DB subnet group to return details for.</p>
    pub db_subnet_group_name: Option<String>,
    /// <p>This parameter is not currently supported.</p>
    pub filters: Option<Vec<Filter>>,
    /// <p> An optional pagination token provided by a previous DescribeDBSubnetGroups request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>. </p>
    pub marker: Option<String>,
    /// <p> The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token called a marker is included in the response so that the remaining results can be retrieved. </p> <p>Default: 100</p> <p>Constraints: Minimum 20, maximum 100.</p>
    pub max_records: Option<i64>,
}

/// Serialize `DescribeDBSubnetGroupsMessage` contents to a `SignedRequest`.
struct DescribeDBSubnetGroupsMessageSerializer;
impl DescribeDBSubnetGroupsMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeDBSubnetGroupsMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.db_subnet_group_name {
            params.put(&format!("{}{}", prefix, "DBSubnetGroupName"), &field_value);
        }
        if let Some(ref field_value) = obj.filters {
            FilterListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Filter"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.marker {
            params.put(&format!("{}{}", prefix, "Marker"), &field_value);
        }
        if let Some(ref field_value) = obj.max_records {
            params.put(
                &format!("{}{}", prefix, "MaxRecords"),
                &field_value.to_string(),
            );
        }
    }
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeEngineDefaultClusterParametersMessage {
    /// <p>The name of the DB cluster parameter group family to return engine parameter information for.</p>
    pub db_parameter_group_family: String,
    /// <p>This parameter is not currently supported.</p>
    pub filters: Option<Vec<Filter>>,
    /// <p> An optional pagination token provided by a previous <code>DescribeEngineDefaultClusterParameters</code> request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>. </p>
    pub marker: Option<String>,
    /// <p> The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token called a marker is included in the response so that the remaining results can be retrieved. </p> <p>Default: 100</p> <p>Constraints: Minimum 20, maximum 100.</p>
    pub max_records: Option<i64>,
}

/// Serialize `DescribeEngineDefaultClusterParametersMessage` contents to a `SignedRequest`.
struct DescribeEngineDefaultClusterParametersMessageSerializer;
impl DescribeEngineDefaultClusterParametersMessageSerializer {
    fn serialize(
        params: &mut Params,
        name: &str,
        obj: &DescribeEngineDefaultClusterParametersMessage,
    ) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "DBParameterGroupFamily"),
            &obj.db_parameter_group_family,
        );
        if let Some(ref field_value) = obj.filters {
            FilterListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Filter"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.marker {
            params.put(&format!("{}{}", prefix, "Marker"), &field_value);
        }
        if let Some(ref field_value) = obj.max_records {
            params.put(
                &format!("{}{}", prefix, "MaxRecords"),
                &field_value.to_string(),
            );
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeEngineDefaultClusterParametersResult {
    pub engine_defaults: Option<EngineDefaults>,
}

struct DescribeEngineDefaultClusterParametersResultDeserializer;
impl DescribeEngineDefaultClusterParametersResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeEngineDefaultClusterParametersResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DescribeEngineDefaultClusterParametersResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "EngineDefaults" => {
                        obj.engine_defaults = Some(try!(EngineDefaultsDeserializer::deserialize(
                            "EngineDefaults",
                            stack
                        )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeEngineDefaultParametersMessage {
    /// <p>The name of the DB parameter group family.</p>
    pub db_parameter_group_family: String,
    /// <p>Not currently supported.</p>
    pub filters: Option<Vec<Filter>>,
    /// <p> An optional pagination token provided by a previous <code>DescribeEngineDefaultParameters</code> request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>. </p>
    pub marker: Option<String>,
    /// <p> The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token called a marker is included in the response so that the remaining results can be retrieved. </p> <p>Default: 100</p> <p>Constraints: Minimum 20, maximum 100.</p>
    pub max_records: Option<i64>,
}

/// Serialize `DescribeEngineDefaultParametersMessage` contents to a `SignedRequest`.
struct DescribeEngineDefaultParametersMessageSerializer;
impl DescribeEngineDefaultParametersMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeEngineDefaultParametersMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "DBParameterGroupFamily"),
            &obj.db_parameter_group_family,
        );
        if let Some(ref field_value) = obj.filters {
            FilterListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Filter"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.marker {
            params.put(&format!("{}{}", prefix, "Marker"), &field_value);
        }
        if let Some(ref field_value) = obj.max_records {
            params.put(
                &format!("{}{}", prefix, "MaxRecords"),
                &field_value.to_string(),
            );
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeEngineDefaultParametersResult {
    pub engine_defaults: Option<EngineDefaults>,
}

struct DescribeEngineDefaultParametersResultDeserializer;
impl DescribeEngineDefaultParametersResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeEngineDefaultParametersResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DescribeEngineDefaultParametersResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "EngineDefaults" => {
                        obj.engine_defaults = Some(try!(EngineDefaultsDeserializer::deserialize(
                            "EngineDefaults",
                            stack
                        )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeEventCategoriesMessage {
    /// <p>This parameter is not currently supported.</p>
    pub filters: Option<Vec<Filter>>,
    /// <p>The type of source that is generating the events.</p> <p>Valid values: db-instance | db-parameter-group | db-security-group | db-snapshot</p>
    pub source_type: Option<String>,
}

/// Serialize `DescribeEventCategoriesMessage` contents to a `SignedRequest`.
struct DescribeEventCategoriesMessageSerializer;
impl DescribeEventCategoriesMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeEventCategoriesMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.filters {
            FilterListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Filter"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.source_type {
            params.put(&format!("{}{}", prefix, "SourceType"), &field_value);
        }
    }
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeEventSubscriptionsMessage {
    /// <p>This parameter is not currently supported.</p>
    pub filters: Option<Vec<Filter>>,
    /// <p> An optional pagination token provided by a previous DescribeOrderableDBInstanceOptions request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code> . </p>
    pub marker: Option<String>,
    /// <p> The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token called a marker is included in the response so that the remaining results can be retrieved. </p> <p>Default: 100</p> <p>Constraints: Minimum 20, maximum 100.</p>
    pub max_records: Option<i64>,
    /// <p>The name of the event notification subscription you want to describe.</p>
    pub subscription_name: Option<String>,
}

/// Serialize `DescribeEventSubscriptionsMessage` contents to a `SignedRequest`.
struct DescribeEventSubscriptionsMessageSerializer;
impl DescribeEventSubscriptionsMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeEventSubscriptionsMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.filters {
            FilterListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Filter"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.marker {
            params.put(&format!("{}{}", prefix, "Marker"), &field_value);
        }
        if let Some(ref field_value) = obj.max_records {
            params.put(
                &format!("{}{}", prefix, "MaxRecords"),
                &field_value.to_string(),
            );
        }
        if let Some(ref field_value) = obj.subscription_name {
            params.put(&format!("{}{}", prefix, "SubscriptionName"), &field_value);
        }
    }
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeEventsMessage {
    /// <p>The number of minutes to retrieve events for.</p> <p>Default: 60</p>
    pub duration: Option<i64>,
    /// <p> The end of the time interval for which to retrieve events, specified in ISO 8601 format. For more information about ISO 8601, go to the <a href="http://en.wikipedia.org/wiki/ISO_8601">ISO8601 Wikipedia page.</a> </p> <p>Example: 2009-07-08T18:00Z</p>
    pub end_time: Option<String>,
    /// <p>A list of event categories that trigger notifications for a event notification subscription.</p>
    pub event_categories: Option<Vec<String>>,
    /// <p>This parameter is not currently supported.</p>
    pub filters: Option<Vec<Filter>>,
    /// <p> An optional pagination token provided by a previous DescribeEvents request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>. </p>
    pub marker: Option<String>,
    /// <p> The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token called a marker is included in the response so that the remaining results can be retrieved. </p> <p>Default: 100</p> <p>Constraints: Minimum 20, maximum 100.</p>
    pub max_records: Option<i64>,
    /// <p><p>The identifier of the event source for which events are returned. If not specified, then all sources are included in the response.</p> <p>Constraints:</p> <ul> <li> <p>If SourceIdentifier is supplied, SourceType must also be provided.</p> </li> <li> <p>If the source type is <code>DBInstance</code>, then a <code>DBInstanceIdentifier</code> must be supplied.</p> </li> <li> <p>If the source type is <code>DBSecurityGroup</code>, a <code>DBSecurityGroupName</code> must be supplied.</p> </li> <li> <p>If the source type is <code>DBParameterGroup</code>, a <code>DBParameterGroupName</code> must be supplied.</p> </li> <li> <p>If the source type is <code>DBSnapshot</code>, a <code>DBSnapshotIdentifier</code> must be supplied.</p> </li> <li> <p>Cannot end with a hyphen or contain two consecutive hyphens.</p> </li> </ul></p>
    pub source_identifier: Option<String>,
    /// <p>The event source to retrieve events for. If no value is specified, all events are returned.</p>
    pub source_type: Option<String>,
    /// <p> The beginning of the time interval to retrieve events for, specified in ISO 8601 format. For more information about ISO 8601, go to the <a href="http://en.wikipedia.org/wiki/ISO_8601">ISO8601 Wikipedia page.</a> </p> <p>Example: 2009-07-08T18:00Z</p>
    pub start_time: Option<String>,
}

/// Serialize `DescribeEventsMessage` contents to a `SignedRequest`.
struct DescribeEventsMessageSerializer;
impl DescribeEventsMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeEventsMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.duration {
            params.put(
                &format!("{}{}", prefix, "Duration"),
                &field_value.to_string(),
            );
        }
        if let Some(ref field_value) = obj.end_time {
            params.put(&format!("{}{}", prefix, "EndTime"), &field_value);
        }
        if let Some(ref field_value) = obj.event_categories {
            EventCategoriesListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "EventCategory"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.filters {
            FilterListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Filter"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.marker {
            params.put(&format!("{}{}", prefix, "Marker"), &field_value);
        }
        if let Some(ref field_value) = obj.max_records {
            params.put(
                &format!("{}{}", prefix, "MaxRecords"),
                &field_value.to_string(),
            );
        }
        if let Some(ref field_value) = obj.source_identifier {
            params.put(&format!("{}{}", prefix, "SourceIdentifier"), &field_value);
        }
        if let Some(ref field_value) = obj.source_type {
            params.put(&format!("{}{}", prefix, "SourceType"), &field_value);
        }
        if let Some(ref field_value) = obj.start_time {
            params.put(&format!("{}{}", prefix, "StartTime"), &field_value);
        }
    }
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeOrderableDBInstanceOptionsMessage {
    /// <p>The DB instance class filter value. Specify this parameter to show only the available offerings matching the specified DB instance class.</p>
    pub db_instance_class: Option<String>,
    /// <p>The name of the engine to retrieve DB instance options for.</p>
    pub engine: String,
    /// <p>The engine version filter value. Specify this parameter to show only the available offerings matching the specified engine version.</p>
    pub engine_version: Option<String>,
    /// <p>This parameter is not currently supported.</p>
    pub filters: Option<Vec<Filter>>,
    /// <p>The license model filter value. Specify this parameter to show only the available offerings matching the specified license model.</p>
    pub license_model: Option<String>,
    /// <p> An optional pagination token provided by a previous DescribeOrderableDBInstanceOptions request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code> . </p>
    pub marker: Option<String>,
    /// <p> The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token called a marker is included in the response so that the remaining results can be retrieved. </p> <p>Default: 100</p> <p>Constraints: Minimum 20, maximum 100.</p>
    pub max_records: Option<i64>,
    /// <p>The VPC filter value. Specify this parameter to show only the available VPC or non-VPC offerings.</p>
    pub vpc: Option<bool>,
}

/// Serialize `DescribeOrderableDBInstanceOptionsMessage` contents to a `SignedRequest`.
struct DescribeOrderableDBInstanceOptionsMessageSerializer;
impl DescribeOrderableDBInstanceOptionsMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeOrderableDBInstanceOptionsMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.db_instance_class {
            params.put(&format!("{}{}", prefix, "DBInstanceClass"), &field_value);
        }
        params.put(&format!("{}{}", prefix, "Engine"), &obj.engine);
        if let Some(ref field_value) = obj.engine_version {
            params.put(&format!("{}{}", prefix, "EngineVersion"), &field_value);
        }
        if let Some(ref field_value) = obj.filters {
            FilterListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Filter"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.license_model {
            params.put(&format!("{}{}", prefix, "LicenseModel"), &field_value);
        }
        if let Some(ref field_value) = obj.marker {
            params.put(&format!("{}{}", prefix, "Marker"), &field_value);
        }
        if let Some(ref field_value) = obj.max_records {
            params.put(
                &format!("{}{}", prefix, "MaxRecords"),
                &field_value.to_string(),
            );
        }
        if let Some(ref field_value) = obj.vpc {
            params.put(&format!("{}{}", prefix, "Vpc"), &field_value.to_string());
        }
    }
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribePendingMaintenanceActionsMessage {
    /// <p><p>A filter that specifies one or more resources to return pending maintenance actions for.</p> <p>Supported filters:</p> <ul> <li> <p> <code>db-cluster-id</code> - Accepts DB cluster identifiers and DB cluster Amazon Resource Names (ARNs). The results list will only include pending maintenance actions for the DB clusters identified by these ARNs.</p> </li> <li> <p> <code>db-instance-id</code> - Accepts DB instance identifiers and DB instance ARNs. The results list will only include pending maintenance actions for the DB instances identified by these ARNs.</p> </li> </ul></p>
    pub filters: Option<Vec<Filter>>,
    /// <p> An optional pagination token provided by a previous <code>DescribePendingMaintenanceActions</code> request. If this parameter is specified, the response includes only records beyond the marker, up to a number of records specified by <code>MaxRecords</code>. </p>
    pub marker: Option<String>,
    /// <p> The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token called a marker is included in the response so that the remaining results can be retrieved. </p> <p>Default: 100</p> <p>Constraints: Minimum 20, maximum 100.</p>
    pub max_records: Option<i64>,
    /// <p>The ARN of a resource to return pending maintenance actions for.</p>
    pub resource_identifier: Option<String>,
}

/// Serialize `DescribePendingMaintenanceActionsMessage` contents to a `SignedRequest`.
struct DescribePendingMaintenanceActionsMessageSerializer;
impl DescribePendingMaintenanceActionsMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribePendingMaintenanceActionsMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.filters {
            FilterListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Filter"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.marker {
            params.put(&format!("{}{}", prefix, "Marker"), &field_value);
        }
        if let Some(ref field_value) = obj.max_records {
            params.put(
                &format!("{}{}", prefix, "MaxRecords"),
                &field_value.to_string(),
            );
        }
        if let Some(ref field_value) = obj.resource_identifier {
            params.put(&format!("{}{}", prefix, "ResourceIdentifier"), &field_value);
        }
    }
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeValidDBInstanceModificationsMessage {
    /// <p>The customer identifier or the ARN of your DB instance. </p>
    pub db_instance_identifier: String,
}

/// Serialize `DescribeValidDBInstanceModificationsMessage` contents to a `SignedRequest`.
struct DescribeValidDBInstanceModificationsMessageSerializer;
impl DescribeValidDBInstanceModificationsMessageSerializer {
    fn serialize(
        params: &mut Params,
        name: &str,
        obj: &DescribeValidDBInstanceModificationsMessage,
    ) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "DBInstanceIdentifier"),
            &obj.db_instance_identifier,
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeValidDBInstanceModificationsResult {
    pub valid_db_instance_modifications_message: Option<ValidDBInstanceModificationsMessage>,
}

struct DescribeValidDBInstanceModificationsResultDeserializer;
impl DescribeValidDBInstanceModificationsResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeValidDBInstanceModificationsResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DescribeValidDBInstanceModificationsResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "ValidDBInstanceModificationsMessage" => {
                        obj.valid_db_instance_modifications_message = Some(try!(
                            ValidDBInstanceModificationsMessageDeserializer::deserialize(
                                "ValidDBInstanceModificationsMessage",
                                stack
                            )
                        ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>An Active Directory Domain membership record associated with the DB instance.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DomainMembership {
    /// <p>The identifier of the Active Directory Domain.</p>
    pub domain: Option<String>,
    /// <p>The fully qualified domain name of the Active Directory Domain.</p>
    pub fqdn: Option<String>,
    /// <p>The name of the IAM role to be used when making API calls to the Directory Service.</p>
    pub iam_role_name: Option<String>,
    /// <p>The status of the DB instance's Active Directory Domain membership, such as joined, pending-join, failed etc).</p>
    pub status: Option<String>,
}

struct DomainMembershipDeserializer;
impl DomainMembershipDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DomainMembership, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DomainMembership::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Domain" => {
                        obj.domain = Some(try!(StringDeserializer::deserialize("Domain", stack)));
                    }
                    "FQDN" => {
                        obj.fqdn = Some(try!(StringDeserializer::deserialize("FQDN", stack)));
                    }
                    "IAMRoleName" => {
                        obj.iam_role_name =
                            Some(try!(StringDeserializer::deserialize("IAMRoleName", stack)));
                    }
                    "Status" => {
                        obj.status = Some(try!(StringDeserializer::deserialize("Status", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct DomainMembershipListDeserializer;
impl DomainMembershipListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<DomainMembership>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "DomainMembership" {
                        obj.push(try!(DomainMembershipDeserializer::deserialize(
                            "DomainMembership",
                            stack
                        )));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}
struct DoubleDeserializer;
impl DoubleDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<f64, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = f64::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct DoubleOptionalDeserializer;
impl DoubleOptionalDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<f64, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = f64::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>A range of double values.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DoubleRange {
    /// <p>The minimum value in the range.</p>
    pub from: Option<f64>,
    /// <p>The maximum value in the range.</p>
    pub to: Option<f64>,
}

struct DoubleRangeDeserializer;
impl DoubleRangeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DoubleRange, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DoubleRange::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "From" => {
                        obj.from = Some(try!(DoubleDeserializer::deserialize("From", stack)));
                    }
                    "To" => {
                        obj.to = Some(try!(DoubleDeserializer::deserialize("To", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct DoubleRangeListDeserializer;
impl DoubleRangeListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<DoubleRange>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "DoubleRange" {
                        obj.push(try!(DoubleRangeDeserializer::deserialize(
                            "DoubleRange",
                            stack
                        )));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}
/// <p><p>This data type is used as a response element in the following actions:</p> <ul> <li> <p> <a>CreateDBInstance</a> </p> </li> <li> <p> <a>DescribeDBInstances</a> </p> </li> <li> <p> <a>DeleteDBInstance</a> </p> </li> </ul></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Endpoint {
    /// <p>Specifies the DNS address of the DB instance.</p>
    pub address: Option<String>,
    /// <p>Specifies the ID that Amazon Route 53 assigns when you create a hosted zone.</p>
    pub hosted_zone_id: Option<String>,
    /// <p>Specifies the port that the database engine is listening on.</p>
    pub port: Option<i64>,
}

struct EndpointDeserializer;
impl EndpointDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Endpoint, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = Endpoint::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Address" => {
                        obj.address = Some(try!(StringDeserializer::deserialize("Address", stack)));
                    }
                    "HostedZoneId" => {
                        obj.hosted_zone_id =
                            Some(try!(StringDeserializer::deserialize("HostedZoneId", stack)));
                    }
                    "Port" => {
                        obj.port = Some(try!(IntegerDeserializer::deserialize("Port", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p> Contains the result of a successful invocation of the <a>DescribeEngineDefaultParameters</a> action. </p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct EngineDefaults {
    /// <p>Specifies the name of the DB parameter group family that the engine default parameters apply to.</p>
    pub db_parameter_group_family: Option<String>,
    /// <p> An optional pagination token provided by a previous EngineDefaults request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code> . </p>
    pub marker: Option<String>,
    /// <p>Contains a list of engine default parameters.</p>
    pub parameters: Option<Vec<Parameter>>,
}

struct EngineDefaultsDeserializer;
impl EngineDefaultsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<EngineDefaults, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = EngineDefaults::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "DBParameterGroupFamily" => {
                        obj.db_parameter_group_family = Some(try!(
                            StringDeserializer::deserialize("DBParameterGroupFamily", stack)
                        ));
                    }
                    "Marker" => {
                        obj.marker = Some(try!(StringDeserializer::deserialize("Marker", stack)));
                    }
                    "Parameters" => {
                        obj.parameters = Some(try!(ParametersListDeserializer::deserialize(
                            "Parameters",
                            stack
                        )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p> This data type is used as a response element in the <a>DescribeEvents</a> action. </p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Event {
    /// <p>Specifies the date and time of the event.</p>
    pub date: Option<String>,
    /// <p>Specifies the category for the event.</p>
    pub event_categories: Option<Vec<String>>,
    /// <p>Provides the text of this event.</p>
    pub message: Option<String>,
    /// <p>The Amazon Resource Name (ARN) for the event.</p>
    pub source_arn: Option<String>,
    /// <p>Provides the identifier for the source of the event.</p>
    pub source_identifier: Option<String>,
    /// <p>Specifies the source type for this event.</p>
    pub source_type: Option<String>,
}

struct EventDeserializer;
impl EventDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Event, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = Event::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Date" => {
                        obj.date = Some(try!(TStampDeserializer::deserialize("Date", stack)));
                    }
                    "EventCategories" => {
                        obj.event_categories = Some(try!(
                            EventCategoriesListDeserializer::deserialize("EventCategories", stack)
                        ));
                    }
                    "Message" => {
                        obj.message = Some(try!(StringDeserializer::deserialize("Message", stack)));
                    }
                    "SourceArn" => {
                        obj.source_arn =
                            Some(try!(StringDeserializer::deserialize("SourceArn", stack)));
                    }
                    "SourceIdentifier" => {
                        obj.source_identifier = Some(try!(StringDeserializer::deserialize(
                            "SourceIdentifier",
                            stack
                        )));
                    }
                    "SourceType" => {
                        obj.source_type = Some(try!(SourceTypeDeserializer::deserialize(
                            "SourceType",
                            stack
                        )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct EventCategoriesListDeserializer;
impl EventCategoriesListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "EventCategory" {
                        obj.push(try!(StringDeserializer::deserialize(
                            "EventCategory",
                            stack
                        )));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}

/// Serialize `EventCategoriesList` contents to a `SignedRequest`.
struct EventCategoriesListSerializer;
impl EventCategoriesListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

/// <p>Contains the results of a successful invocation of the <a>DescribeEventCategories</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct EventCategoriesMap {
    /// <p>The event categories for the specified source type</p>
    pub event_categories: Option<Vec<String>>,
    /// <p>The source type that the returned categories belong to</p>
    pub source_type: Option<String>,
}

struct EventCategoriesMapDeserializer;
impl EventCategoriesMapDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<EventCategoriesMap, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = EventCategoriesMap::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "EventCategories" => {
                        obj.event_categories = Some(try!(
                            EventCategoriesListDeserializer::deserialize("EventCategories", stack)
                        ));
                    }
                    "SourceType" => {
                        obj.source_type =
                            Some(try!(StringDeserializer::deserialize("SourceType", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct EventCategoriesMapListDeserializer;
impl EventCategoriesMapListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<EventCategoriesMap>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "EventCategoriesMap" {
                        obj.push(try!(EventCategoriesMapDeserializer::deserialize(
                            "EventCategoriesMap",
                            stack
                        )));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}
/// <p>Data returned from the <b>DescribeEventCategories</b> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct EventCategoriesMessage {
    /// <p>A list of EventCategoriesMap data types.</p>
    pub event_categories_map_list: Option<Vec<EventCategoriesMap>>,
}

struct EventCategoriesMessageDeserializer;
impl EventCategoriesMessageDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<EventCategoriesMessage, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = EventCategoriesMessage::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "EventCategoriesMapList" => {
                        obj.event_categories_map_list =
                            Some(try!(EventCategoriesMapListDeserializer::deserialize(
                                "EventCategoriesMapList",
                                stack
                            )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct EventListDeserializer;
impl EventListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<Event>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "Event" {
                        obj.push(try!(EventDeserializer::deserialize("Event", stack)));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}
/// <p>Contains the results of a successful invocation of the <a>DescribeEventSubscriptions</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct EventSubscription {
    /// <p>The event notification subscription Id.</p>
    pub cust_subscription_id: Option<String>,
    /// <p>The AWS customer account associated with the event notification subscription.</p>
    pub customer_aws_id: Option<String>,
    /// <p>A Boolean value indicating if the subscription is enabled. True indicates the subscription is enabled.</p>
    pub enabled: Option<bool>,
    /// <p>A list of event categories for the event notification subscription.</p>
    pub event_categories_list: Option<Vec<String>>,
    /// <p>The Amazon Resource Name (ARN) for the event subscription.</p>
    pub event_subscription_arn: Option<String>,
    /// <p>The topic ARN of the event notification subscription.</p>
    pub sns_topic_arn: Option<String>,
    /// <p>A list of source IDs for the event notification subscription.</p>
    pub source_ids_list: Option<Vec<String>>,
    /// <p>The source type for the event notification subscription.</p>
    pub source_type: Option<String>,
    /// <p>The status of the event notification subscription.</p> <p>Constraints:</p> <p>Can be one of the following: creating | modifying | deleting | active | no-permission | topic-not-exist</p> <p>The status "no-permission" indicates that Neptune no longer has permission to post to the SNS topic. The status "topic-not-exist" indicates that the topic was deleted after the subscription was created.</p>
    pub status: Option<String>,
    /// <p>The time the event notification subscription was created.</p>
    pub subscription_creation_time: Option<String>,
}

struct EventSubscriptionDeserializer;
impl EventSubscriptionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<EventSubscription, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = EventSubscription::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "CustSubscriptionId" => {
                        obj.cust_subscription_id = Some(try!(StringDeserializer::deserialize(
                            "CustSubscriptionId",
                            stack
                        )));
                    }
                    "CustomerAwsId" => {
                        obj.customer_aws_id = Some(try!(StringDeserializer::deserialize(
                            "CustomerAwsId",
                            stack
                        )));
                    }
                    "Enabled" => {
                        obj.enabled =
                            Some(try!(BooleanDeserializer::deserialize("Enabled", stack)));
                    }
                    "EventCategoriesList" => {
                        obj.event_categories_list =
                            Some(try!(EventCategoriesListDeserializer::deserialize(
                                "EventCategoriesList",
                                stack
                            )));
                    }
                    "EventSubscriptionArn" => {
                        obj.event_subscription_arn = Some(try!(StringDeserializer::deserialize(
                            "EventSubscriptionArn",
                            stack
                        )));
                    }
                    "SnsTopicArn" => {
                        obj.sns_topic_arn =
                            Some(try!(StringDeserializer::deserialize("SnsTopicArn", stack)));
                    }
                    "SourceIdsList" => {
                        obj.source_ids_list = Some(try!(SourceIdsListDeserializer::deserialize(
                            "SourceIdsList",
                            stack
                        )));
                    }
                    "SourceType" => {
                        obj.source_type =
                            Some(try!(StringDeserializer::deserialize("SourceType", stack)));
                    }
                    "Status" => {
                        obj.status = Some(try!(StringDeserializer::deserialize("Status", stack)));
                    }
                    "SubscriptionCreationTime" => {
                        obj.subscription_creation_time = Some(try!(
                            StringDeserializer::deserialize("SubscriptionCreationTime", stack)
                        ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct EventSubscriptionsListDeserializer;
impl EventSubscriptionsListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<EventSubscription>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "EventSubscription" {
                        obj.push(try!(EventSubscriptionDeserializer::deserialize(
                            "EventSubscription",
                            stack
                        )));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}
/// <p>Data returned by the <b>DescribeEventSubscriptions</b> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct EventSubscriptionsMessage {
    /// <p>A list of EventSubscriptions data types.</p>
    pub event_subscriptions_list: Option<Vec<EventSubscription>>,
    /// <p> An optional pagination token provided by a previous DescribeOrderableDBInstanceOptions request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>. </p>
    pub marker: Option<String>,
}

struct EventSubscriptionsMessageDeserializer;
impl EventSubscriptionsMessageDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<EventSubscriptionsMessage, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = EventSubscriptionsMessage::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "EventSubscriptionsList" => {
                        obj.event_subscriptions_list =
                            Some(try!(EventSubscriptionsListDeserializer::deserialize(
                                "EventSubscriptionsList",
                                stack
                            )));
                    }
                    "Marker" => {
                        obj.marker = Some(try!(StringDeserializer::deserialize("Marker", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p> Contains the result of a successful invocation of the <a>DescribeEvents</a> action. </p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct EventsMessage {
    /// <p> A list of <a>Event</a> instances. </p>
    pub events: Option<Vec<Event>>,
    /// <p> An optional pagination token provided by a previous Events request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code> . </p>
    pub marker: Option<String>,
}

struct EventsMessageDeserializer;
impl EventsMessageDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<EventsMessage, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = EventsMessage::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Events" => {
                        obj.events =
                            Some(try!(EventListDeserializer::deserialize("Events", stack)));
                    }
                    "Marker" => {
                        obj.marker = Some(try!(StringDeserializer::deserialize("Marker", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct FailoverDBClusterMessage {
    /// <p><p>A DB cluster identifier to force a failover for. This parameter is not case-sensitive.</p> <p>Constraints:</p> <ul> <li> <p>Must match the identifier of an existing DBCluster.</p> </li> </ul></p>
    pub db_cluster_identifier: Option<String>,
    /// <p>The name of the instance to promote to the primary instance.</p> <p>You must specify the instance identifier for an Read Replica in the DB cluster. For example, <code>mydbcluster-replica1</code>.</p>
    pub target_db_instance_identifier: Option<String>,
}

/// Serialize `FailoverDBClusterMessage` contents to a `SignedRequest`.
struct FailoverDBClusterMessageSerializer;
impl FailoverDBClusterMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &FailoverDBClusterMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.db_cluster_identifier {
            params.put(
                &format!("{}{}", prefix, "DBClusterIdentifier"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.target_db_instance_identifier {
            params.put(
                &format!("{}{}", prefix, "TargetDBInstanceIdentifier"),
                &field_value,
            );
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct FailoverDBClusterResult {
    pub db_cluster: Option<DBCluster>,
}

struct FailoverDBClusterResultDeserializer;
impl FailoverDBClusterResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<FailoverDBClusterResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = FailoverDBClusterResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "DBCluster" => {
                        obj.db_cluster =
                            Some(try!(DBClusterDeserializer::deserialize("DBCluster", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>This type is not currently supported.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Filter {
    /// <p>This parameter is not currently supported.</p>
    pub name: String,
    /// <p>This parameter is not currently supported.</p>
    pub values: Vec<String>,
}

/// Serialize `Filter` contents to a `SignedRequest`.
struct FilterSerializer;
impl FilterSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Filter) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(&format!("{}{}", prefix, "Name"), &obj.name);
        FilterValueListSerializer::serialize(
            params,
            &format!("{}{}", prefix, "Value"),
            &obj.values,
        );
    }
}

/// Serialize `FilterList` contents to a `SignedRequest`.
struct FilterListSerializer;
impl FilterListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<Filter>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            FilterSerializer::serialize(params, &key, obj);
        }
    }
}

/// Serialize `FilterValueList` contents to a `SignedRequest`.
struct FilterValueListSerializer;
impl FilterValueListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

struct IntegerDeserializer;
impl IntegerDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<i64, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = i64::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct IntegerOptionalDeserializer;
impl IntegerOptionalDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<i64, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = i64::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

/// Serialize `KeyList` contents to a `SignedRequest`.
struct KeyListSerializer;
impl KeyListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListTagsForResourceMessage {
    /// <p>This parameter is not currently supported.</p>
    pub filters: Option<Vec<Filter>>,
    /// <p>The Amazon Neptune resource with tags to be listed. This value is an Amazon Resource Name (ARN). For information about creating an ARN, see <a href="http://docs.aws.amazon.com/neptune/latest/UserGuide/tagging.ARN.html#tagging.ARN.Constructing"> Constructing an Amazon Resource Name (ARN)</a>.</p>
    pub resource_name: String,
}

/// Serialize `ListTagsForResourceMessage` contents to a `SignedRequest`.
struct ListTagsForResourceMessageSerializer;
impl ListTagsForResourceMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ListTagsForResourceMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.filters {
            FilterListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Filter"),
                field_value,
            );
        }
        params.put(&format!("{}{}", prefix, "ResourceName"), &obj.resource_name);
    }
}

struct LogTypeListDeserializer;
impl LogTypeListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "member" {
                        obj.push(try!(StringDeserializer::deserialize("member", stack)));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}

/// Serialize `LogTypeList` contents to a `SignedRequest`.
struct LogTypeListSerializer;
impl LogTypeListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ModifyDBClusterMessage {
    /// <p>A value that specifies whether the modifications in this request and any pending modifications are asynchronously applied as soon as possible, regardless of the <code>PreferredMaintenanceWindow</code> setting for the DB cluster. If this parameter is set to <code>false</code>, changes to the DB cluster are applied during the next maintenance window.</p> <p>The <code>ApplyImmediately</code> parameter only affects the <code>NewDBClusterIdentifier</code> and <code>MasterUserPassword</code> values. If you set the <code>ApplyImmediately</code> parameter value to false, then changes to the <code>NewDBClusterIdentifier</code> and <code>MasterUserPassword</code> values are applied during the next maintenance window. All other changes are applied immediately, regardless of the value of the <code>ApplyImmediately</code> parameter.</p> <p>Default: <code>false</code> </p>
    pub apply_immediately: Option<bool>,
    /// <p><p>The number of days for which automated backups are retained. You must specify a minimum value of 1.</p> <p>Default: 1</p> <p>Constraints:</p> <ul> <li> <p>Must be a value from 1 to 35</p> </li> </ul></p>
    pub backup_retention_period: Option<i64>,
    /// <p><p>The DB cluster identifier for the cluster being modified. This parameter is not case-sensitive.</p> <p>Constraints:</p> <ul> <li> <p>Must match the identifier of an existing DBCluster.</p> </li> </ul></p>
    pub db_cluster_identifier: String,
    /// <p>The name of the DB cluster parameter group to use for the DB cluster.</p>
    pub db_cluster_parameter_group_name: Option<String>,
    /// <p>True to enable mapping of AWS Identity and Access Management (IAM) accounts to database accounts, and otherwise false.</p> <p>Default: <code>false</code> </p>
    pub enable_iam_database_authentication: Option<bool>,
    /// <p>The version number of the database engine to which you want to upgrade. Changing this parameter results in an outage. The change is applied during the next maintenance window unless the ApplyImmediately parameter is set to true.</p> <p>For a list of valid engine versions, see <a>CreateDBInstance</a>, or call <a>DescribeDBEngineVersions</a>.</p>
    pub engine_version: Option<String>,
    /// <p>The new password for the master database user. This password can contain any printable ASCII character except "/", """, or "@".</p> <p>Constraints: Must contain from 8 to 41 characters.</p>
    pub master_user_password: Option<String>,
    /// <p>The new DB cluster identifier for the DB cluster when renaming a DB cluster. This value is stored as a lowercase string.</p> <p>Constraints:</p> <ul> <li> <p>Must contain from 1 to 63 letters, numbers, or hyphens</p> </li> <li> <p>The first character must be a letter</p> </li> <li> <p>Cannot end with a hyphen or contain two consecutive hyphens</p> </li> </ul> <p>Example: <code>my-cluster2</code> </p>
    pub new_db_cluster_identifier: Option<String>,
    /// <p>A value that indicates that the DB cluster should be associated with the specified option group. Changing this parameter doesn't result in an outage except in the following case, and the change is applied during the next maintenance window unless the <code>ApplyImmediately</code> parameter is set to <code>true</code> for this request. If the parameter change results in an option group that enables OEM, this change can cause a brief (sub-second) period during which new connections are rejected but existing connections are not interrupted. </p> <p>Permanent options can't be removed from an option group. The option group can't be removed from a DB cluster once it is associated with a DB cluster.</p>
    pub option_group_name: Option<String>,
    /// <p>The port number on which the DB cluster accepts connections.</p> <p>Constraints: Value must be <code>1150-65535</code> </p> <p>Default: The same port as the original DB cluster.</p>
    pub port: Option<i64>,
    /// <p><p>The daily time range during which automated backups are created if automated backups are enabled, using the <code>BackupRetentionPeriod</code> parameter. </p> <p>The default is a 30-minute window selected at random from an 8-hour block of time for each AWS Region. </p> <p>Constraints:</p> <ul> <li> <p>Must be in the format <code>hh24:mi-hh24:mi</code>.</p> </li> <li> <p>Must be in Universal Coordinated Time (UTC).</p> </li> <li> <p>Must not conflict with the preferred maintenance window.</p> </li> <li> <p>Must be at least 30 minutes.</p> </li> </ul></p>
    pub preferred_backup_window: Option<String>,
    /// <p>The weekly time range during which system maintenance can occur, in Universal Coordinated Time (UTC).</p> <p>Format: <code>ddd:hh24:mi-ddd:hh24:mi</code> </p> <p>The default is a 30-minute window selected at random from an 8-hour block of time for each AWS Region, occurring on a random day of the week. </p> <p>Valid Days: Mon, Tue, Wed, Thu, Fri, Sat, Sun.</p> <p>Constraints: Minimum 30-minute window.</p>
    pub preferred_maintenance_window: Option<String>,
    /// <p>A list of VPC security groups that the DB cluster will belong to.</p>
    pub vpc_security_group_ids: Option<Vec<String>>,
}

/// Serialize `ModifyDBClusterMessage` contents to a `SignedRequest`.
struct ModifyDBClusterMessageSerializer;
impl ModifyDBClusterMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ModifyDBClusterMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.apply_immediately {
            params.put(
                &format!("{}{}", prefix, "ApplyImmediately"),
                &field_value.to_string(),
            );
        }
        if let Some(ref field_value) = obj.backup_retention_period {
            params.put(
                &format!("{}{}", prefix, "BackupRetentionPeriod"),
                &field_value.to_string(),
            );
        }
        params.put(
            &format!("{}{}", prefix, "DBClusterIdentifier"),
            &obj.db_cluster_identifier,
        );
        if let Some(ref field_value) = obj.db_cluster_parameter_group_name {
            params.put(
                &format!("{}{}", prefix, "DBClusterParameterGroupName"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.enable_iam_database_authentication {
            params.put(
                &format!("{}{}", prefix, "EnableIAMDatabaseAuthentication"),
                &field_value.to_string(),
            );
        }
        if let Some(ref field_value) = obj.engine_version {
            params.put(&format!("{}{}", prefix, "EngineVersion"), &field_value);
        }
        if let Some(ref field_value) = obj.master_user_password {
            params.put(&format!("{}{}", prefix, "MasterUserPassword"), &field_value);
        }
        if let Some(ref field_value) = obj.new_db_cluster_identifier {
            params.put(
                &format!("{}{}", prefix, "NewDBClusterIdentifier"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.option_group_name {
            params.put(&format!("{}{}", prefix, "OptionGroupName"), &field_value);
        }
        if let Some(ref field_value) = obj.port {
            params.put(&format!("{}{}", prefix, "Port"), &field_value.to_string());
        }
        if let Some(ref field_value) = obj.preferred_backup_window {
            params.put(
                &format!("{}{}", prefix, "PreferredBackupWindow"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.preferred_maintenance_window {
            params.put(
                &format!("{}{}", prefix, "PreferredMaintenanceWindow"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.vpc_security_group_ids {
            VpcSecurityGroupIdListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "VpcSecurityGroupId"),
                field_value,
            );
        }
    }
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ModifyDBClusterParameterGroupMessage {
    /// <p>The name of the DB cluster parameter group to modify.</p>
    pub db_cluster_parameter_group_name: String,
    /// <p>A list of parameters in the DB cluster parameter group to modify.</p>
    pub parameters: Vec<Parameter>,
}

/// Serialize `ModifyDBClusterParameterGroupMessage` contents to a `SignedRequest`.
struct ModifyDBClusterParameterGroupMessageSerializer;
impl ModifyDBClusterParameterGroupMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ModifyDBClusterParameterGroupMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "DBClusterParameterGroupName"),
            &obj.db_cluster_parameter_group_name,
        );
        ParametersListSerializer::serialize(
            params,
            &format!("{}{}", prefix, "Parameter"),
            &obj.parameters,
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct ModifyDBClusterResult {
    pub db_cluster: Option<DBCluster>,
}

struct ModifyDBClusterResultDeserializer;
impl ModifyDBClusterResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ModifyDBClusterResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ModifyDBClusterResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "DBCluster" => {
                        obj.db_cluster =
                            Some(try!(DBClusterDeserializer::deserialize("DBCluster", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ModifyDBClusterSnapshotAttributeMessage {
    /// <p>The name of the DB cluster snapshot attribute to modify.</p> <p>To manage authorization for other AWS accounts to copy or restore a manual DB cluster snapshot, set this value to <code>restore</code>.</p>
    pub attribute_name: String,
    /// <p>The identifier for the DB cluster snapshot to modify the attributes for.</p>
    pub db_cluster_snapshot_identifier: String,
    /// <p>A list of DB cluster snapshot attributes to add to the attribute specified by <code>AttributeName</code>.</p> <p>To authorize other AWS accounts to copy or restore a manual DB cluster snapshot, set this list to include one or more AWS account IDs, or <code>all</code> to make the manual DB cluster snapshot restorable by any AWS account. Do not add the <code>all</code> value for any manual DB cluster snapshots that contain private information that you don't want available to all AWS accounts.</p>
    pub values_to_add: Option<Vec<String>>,
    /// <p>A list of DB cluster snapshot attributes to remove from the attribute specified by <code>AttributeName</code>.</p> <p>To remove authorization for other AWS accounts to copy or restore a manual DB cluster snapshot, set this list to include one or more AWS account identifiers, or <code>all</code> to remove authorization for any AWS account to copy or restore the DB cluster snapshot. If you specify <code>all</code>, an AWS account whose account ID is explicitly added to the <code>restore</code> attribute can still copy or restore a manual DB cluster snapshot.</p>
    pub values_to_remove: Option<Vec<String>>,
}

/// Serialize `ModifyDBClusterSnapshotAttributeMessage` contents to a `SignedRequest`.
struct ModifyDBClusterSnapshotAttributeMessageSerializer;
impl ModifyDBClusterSnapshotAttributeMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ModifyDBClusterSnapshotAttributeMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "AttributeName"),
            &obj.attribute_name,
        );
        params.put(
            &format!("{}{}", prefix, "DBClusterSnapshotIdentifier"),
            &obj.db_cluster_snapshot_identifier,
        );
        if let Some(ref field_value) = obj.values_to_add {
            AttributeValueListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "AttributeValue"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.values_to_remove {
            AttributeValueListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "AttributeValue"),
                field_value,
            );
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct ModifyDBClusterSnapshotAttributeResult {
    pub db_cluster_snapshot_attributes_result: Option<DBClusterSnapshotAttributesResult>,
}

struct ModifyDBClusterSnapshotAttributeResultDeserializer;
impl ModifyDBClusterSnapshotAttributeResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ModifyDBClusterSnapshotAttributeResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ModifyDBClusterSnapshotAttributeResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "DBClusterSnapshotAttributesResult" => {
                        obj.db_cluster_snapshot_attributes_result = Some(try!(
                            DBClusterSnapshotAttributesResultDeserializer::deserialize(
                                "DBClusterSnapshotAttributesResult",
                                stack
                            )
                        ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ModifyDBInstanceMessage {
    /// <p>The new amount of storage (in gibibytes) to allocate for the DB instance. </p> <p>Not applicable. Storage is managed by the DB Cluster.</p>
    pub allocated_storage: Option<i64>,
    /// <p>Indicates that major version upgrades are allowed. Changing this parameter doesn't result in an outage and the change is asynchronously applied as soon as possible.</p> <p>Constraints: This parameter must be set to true when specifying a value for the EngineVersion parameter that is a different major version than the DB instance's current version.</p>
    pub allow_major_version_upgrade: Option<bool>,
    /// <p>Specifies whether the modifications in this request and any pending modifications are asynchronously applied as soon as possible, regardless of the <code>PreferredMaintenanceWindow</code> setting for the DB instance. </p> <p> If this parameter is set to <code>false</code>, changes to the DB instance are applied during the next maintenance window. Some parameter changes can cause an outage and are applied on the next call to <a>RebootDBInstance</a>, or the next failure reboot. </p> <p>Default: <code>false</code> </p>
    pub apply_immediately: Option<bool>,
    /// <p> Indicates that minor version upgrades are applied automatically to the DB instance during the maintenance window. Changing this parameter doesn't result in an outage except in the following case and the change is asynchronously applied as soon as possible. An outage will result if this parameter is set to <code>true</code> during the maintenance window, and a newer minor version is available, and Neptune has enabled auto patching for that engine version. </p>
    pub auto_minor_version_upgrade: Option<bool>,
    /// <p>The number of days to retain automated backups. Setting this parameter to a positive number enables backups. Setting this parameter to 0 disables automated backups.</p> <p>Not applicable. The retention period for automated backups is managed by the DB cluster. For more information, see <a>ModifyDBCluster</a>.</p> <p>Default: Uses existing setting</p>
    pub backup_retention_period: Option<i64>,
    /// <p>Indicates the certificate that needs to be associated with the instance.</p>
    pub ca_certificate_identifier: Option<String>,
    /// <p>The configuration setting for the log types to be enabled for export to CloudWatch Logs for a specific DB instance or DB cluster.</p>
    pub cloudwatch_logs_export_configuration: Option<CloudwatchLogsExportConfiguration>,
    /// <p>True to copy all tags from the DB instance to snapshots of the DB instance, and otherwise false. The default is false.</p>
    pub copy_tags_to_snapshot: Option<bool>,
    /// <p>The new compute and memory capacity of the DB instance, for example, <code>db.m4.large</code>. Not all DB instance classes are available in all AWS Regions. </p> <p>If you modify the DB instance class, an outage occurs during the change. The change is applied during the next maintenance window, unless <code>ApplyImmediately</code> is specified as <code>true</code> for this request. </p> <p>Default: Uses existing setting</p>
    pub db_instance_class: Option<String>,
    /// <p><p>The DB instance identifier. This value is stored as a lowercase string.</p> <p>Constraints:</p> <ul> <li> <p>Must match the identifier of an existing DBInstance.</p> </li> </ul></p>
    pub db_instance_identifier: String,
    /// <p>The name of the DB parameter group to apply to the DB instance. Changing this setting doesn't result in an outage. The parameter group name itself is changed immediately, but the actual parameter changes are not applied until you reboot the instance without failover. The db instance will NOT be rebooted automatically and the parameter changes will NOT be applied during the next maintenance window.</p> <p>Default: Uses existing setting</p> <p>Constraints: The DB parameter group must be in the same DB parameter group family as this DB instance.</p>
    pub db_parameter_group_name: Option<String>,
    /// <p>The port number on which the database accepts connections.</p> <p>The value of the <code>DBPortNumber</code> parameter must not match any of the port values specified for options in the option group for the DB instance.</p> <p>Your database will restart when you change the <code>DBPortNumber</code> value regardless of the value of the <code>ApplyImmediately</code> parameter.</p> <p> Default: <code>8182</code> </p>
    pub db_port_number: Option<i64>,
    /// <p><p>A list of DB security groups to authorize on this DB instance. Changing this setting doesn&#39;t result in an outage and the change is asynchronously applied as soon as possible.</p> <p>Constraints:</p> <ul> <li> <p>If supplied, must match existing DBSecurityGroups.</p> </li> </ul></p>
    pub db_security_groups: Option<Vec<String>>,
    /// <p>The new DB subnet group for the DB instance. You can use this parameter to move your DB instance to a different VPC. </p> <p>Changing the subnet group causes an outage during the change. The change is applied during the next maintenance window, unless you specify <code>true</code> for the <code>ApplyImmediately</code> parameter. </p> <p>Constraints: If supplied, must match the name of an existing DBSubnetGroup.</p> <p>Example: <code>mySubnetGroup</code> </p>
    pub db_subnet_group_name: Option<String>,
    /// <p>Not supported. </p>
    pub domain: Option<String>,
    /// <p>Not supported</p>
    pub domain_iam_role_name: Option<String>,
    /// <p>True to enable mapping of AWS Identity and Access Management (IAM) accounts to database accounts, and otherwise false.</p> <p>You can enable IAM database authentication for the following database engines</p> <p>Not applicable. Mapping AWS IAM accounts to database accounts is managed by the DB cluster. For more information, see <a>ModifyDBCluster</a>.</p> <p>Default: <code>false</code> </p>
    pub enable_iam_database_authentication: Option<bool>,
    /// <p>True to enable Performance Insights for the DB instance, and otherwise false.</p>
    pub enable_performance_insights: Option<bool>,
    /// <p> The version number of the database engine to upgrade to. Changing this parameter results in an outage and the change is applied during the next maintenance window unless the <code>ApplyImmediately</code> parameter is set to <code>true</code> for this request. </p> <p>For major version upgrades, if a nondefault DB parameter group is currently in use, a new DB parameter group in the DB parameter group family for the new engine version must be specified. The new DB parameter group can be the default for that DB parameter group family.</p>
    pub engine_version: Option<String>,
    /// <p>The new Provisioned IOPS (I/O operations per second) value for the instance. </p> <p>Changing this setting doesn't result in an outage and the change is applied during the next maintenance window unless the <code>ApplyImmediately</code> parameter is set to <code>true</code> for this request.</p> <p>Default: Uses existing setting</p>
    pub iops: Option<i64>,
    /// <p>The license model for the DB instance.</p> <p>Valid values: <code>license-included</code> | <code>bring-your-own-license</code> | <code>general-public-license</code> </p>
    pub license_model: Option<String>,
    /// <p>The new password for the master user. The password can include any printable ASCII character except "/", """, or "@".</p> <p>Not applicable. </p> <p>Default: Uses existing setting</p>
    pub master_user_password: Option<String>,
    /// <p>The interval, in seconds, between points when Enhanced Monitoring metrics are collected for the DB instance. To disable collecting Enhanced Monitoring metrics, specify 0. The default is 0.</p> <p>If <code>MonitoringRoleArn</code> is specified, then you must also set <code>MonitoringInterval</code> to a value other than 0.</p> <p>Valid Values: <code>0, 1, 5, 10, 15, 30, 60</code> </p>
    pub monitoring_interval: Option<i64>,
    /// <p>The ARN for the IAM role that permits Neptune to send enhanced monitoring metrics to Amazon CloudWatch Logs. For example, <code>arn:aws:iam:123456789012:role/emaccess</code>. </p> <p>If <code>MonitoringInterval</code> is set to a value other than 0, then you must supply a <code>MonitoringRoleArn</code> value.</p>
    pub monitoring_role_arn: Option<String>,
    /// <p>Specifies if the DB instance is a Multi-AZ deployment. Changing this parameter doesn't result in an outage and the change is applied during the next maintenance window unless the <code>ApplyImmediately</code> parameter is set to <code>true</code> for this request. </p>
    pub multi_az: Option<bool>,
    /// <p> The new DB instance identifier for the DB instance when renaming a DB instance. When you change the DB instance identifier, an instance reboot will occur immediately if you set <code>Apply Immediately</code> to true, or will occur during the next maintenance window if <code>Apply Immediately</code> to false. This value is stored as a lowercase string. </p> <p>Constraints:</p> <ul> <li> <p>Must contain from 1 to 63 letters, numbers, or hyphens.</p> </li> <li> <p>The first character must be a letter.</p> </li> <li> <p>Cannot end with a hyphen or contain two consecutive hyphens.</p> </li> </ul> <p>Example: <code>mydbinstance</code> </p>
    pub new_db_instance_identifier: Option<String>,
    /// <p> Indicates that the DB instance should be associated with the specified option group. Changing this parameter doesn't result in an outage except in the following case and the change is applied during the next maintenance window unless the <code>ApplyImmediately</code> parameter is set to <code>true</code> for this request. If the parameter change results in an option group that enables OEM, this change can cause a brief (sub-second) period during which new connections are rejected but existing connections are not interrupted. </p> <p>Permanent options, such as the TDE option for Oracle Advanced Security TDE, can't be removed from an option group, and that option group can't be removed from a DB instance once it is associated with a DB instance</p>
    pub option_group_name: Option<String>,
    /// <p>The AWS KMS key identifier for encryption of Performance Insights data. The KMS key ID is the Amazon Resource Name (ARN), KMS key identifier, or the KMS key alias for the KMS encryption key.</p>
    pub performance_insights_kms_key_id: Option<String>,
    /// <p><p> The daily time range during which automated backups are created if automated backups are enabled. </p> <p>Not applicable. The daily time range for creating automated backups is managed by the DB cluster. For more information, see <a>ModifyDBCluster</a>.</p> <p>Constraints:</p> <ul> <li> <p>Must be in the format hh24:mi-hh24:mi</p> </li> <li> <p>Must be in Universal Time Coordinated (UTC)</p> </li> <li> <p>Must not conflict with the preferred maintenance window</p> </li> <li> <p>Must be at least 30 minutes</p> </li> </ul></p>
    pub preferred_backup_window: Option<String>,
    /// <p>The weekly time range (in UTC) during which system maintenance can occur, which might result in an outage. Changing this parameter doesn't result in an outage, except in the following situation, and the change is asynchronously applied as soon as possible. If there are pending actions that cause a reboot, and the maintenance window is changed to include the current time, then changing this parameter will cause a reboot of the DB instance. If moving this window to the current time, there must be at least 30 minutes between the current time and end of the window to ensure pending changes are applied.</p> <p>Default: Uses existing setting</p> <p>Format: ddd:hh24:mi-ddd:hh24:mi</p> <p>Valid Days: Mon | Tue | Wed | Thu | Fri | Sat | Sun</p> <p>Constraints: Must be at least 30 minutes</p>
    pub preferred_maintenance_window: Option<String>,
    /// <p>A value that specifies the order in which a Read Replica is promoted to the primary instance after a failure of the existing primary instance. </p> <p>Default: 1</p> <p>Valid Values: 0 - 15</p>
    pub promotion_tier: Option<i64>,
    /// <p>Specifies the storage type to be associated with the DB instance. </p> <p>If you specify Provisioned IOPS (<code>io1</code>), you must also include a value for the <code>Iops</code> parameter. </p> <p>If you choose to migrate your DB instance from using standard storage to using Provisioned IOPS, or from using Provisioned IOPS to using standard storage, the process can take time. The duration of the migration depends on several factors such as database load, storage size, storage type (standard or Provisioned IOPS), amount of IOPS provisioned (if any), and the number of prior scale storage operations. Typical migration times are under 24 hours, but the process can take up to several days in some cases. During the migration, the DB instance is available for use, but might experience performance degradation. While the migration takes place, nightly backups for the instance are suspended. No other Amazon Neptune operations can take place for the instance, including modifying the instance, rebooting the instance, deleting the instance, creating a Read Replica for the instance, and creating a DB snapshot of the instance. </p> <p> Valid values: <code>standard | gp2 | io1</code> </p> <p>Default: <code>io1</code> if the <code>Iops</code> parameter is specified, otherwise <code>standard</code> </p>
    pub storage_type: Option<String>,
    /// <p>The ARN from the key store with which to associate the instance for TDE encryption.</p>
    pub tde_credential_arn: Option<String>,
    /// <p>The password for the given ARN from the key store in order to access the device.</p>
    pub tde_credential_password: Option<String>,
    /// <p><p>A list of EC2 VPC security groups to authorize on this DB instance. This change is asynchronously applied as soon as possible.</p> <p>Not applicable. The associated list of EC2 VPC security groups is managed by the DB cluster. For more information, see <a>ModifyDBCluster</a>.</p> <p>Constraints:</p> <ul> <li> <p>If supplied, must match existing VpcSecurityGroupIds.</p> </li> </ul></p>
    pub vpc_security_group_ids: Option<Vec<String>>,
}

/// Serialize `ModifyDBInstanceMessage` contents to a `SignedRequest`.
struct ModifyDBInstanceMessageSerializer;
impl ModifyDBInstanceMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ModifyDBInstanceMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.allocated_storage {
            params.put(
                &format!("{}{}", prefix, "AllocatedStorage"),
                &field_value.to_string(),
            );
        }
        if let Some(ref field_value) = obj.allow_major_version_upgrade {
            params.put(
                &format!("{}{}", prefix, "AllowMajorVersionUpgrade"),
                &field_value.to_string(),
            );
        }
        if let Some(ref field_value) = obj.apply_immediately {
            params.put(
                &format!("{}{}", prefix, "ApplyImmediately"),
                &field_value.to_string(),
            );
        }
        if let Some(ref field_value) = obj.auto_minor_version_upgrade {
            params.put(
                &format!("{}{}", prefix, "AutoMinorVersionUpgrade"),
                &field_value.to_string(),
            );
        }
        if let Some(ref field_value) = obj.backup_retention_period {
            params.put(
                &format!("{}{}", prefix, "BackupRetentionPeriod"),
                &field_value.to_string(),
            );
        }
        if let Some(ref field_value) = obj.ca_certificate_identifier {
            params.put(
                &format!("{}{}", prefix, "CACertificateIdentifier"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.cloudwatch_logs_export_configuration {
            CloudwatchLogsExportConfigurationSerializer::serialize(
                params,
                &format!("{}{}", prefix, "CloudwatchLogsExportConfiguration"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.copy_tags_to_snapshot {
            params.put(
                &format!("{}{}", prefix, "CopyTagsToSnapshot"),
                &field_value.to_string(),
            );
        }
        if let Some(ref field_value) = obj.db_instance_class {
            params.put(&format!("{}{}", prefix, "DBInstanceClass"), &field_value);
        }
        params.put(
            &format!("{}{}", prefix, "DBInstanceIdentifier"),
            &obj.db_instance_identifier,
        );
        if let Some(ref field_value) = obj.db_parameter_group_name {
            params.put(
                &format!("{}{}", prefix, "DBParameterGroupName"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.db_port_number {
            params.put(
                &format!("{}{}", prefix, "DBPortNumber"),
                &field_value.to_string(),
            );
        }
        if let Some(ref field_value) = obj.db_security_groups {
            DBSecurityGroupNameListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "DBSecurityGroupName"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.db_subnet_group_name {
            params.put(&format!("{}{}", prefix, "DBSubnetGroupName"), &field_value);
        }
        if let Some(ref field_value) = obj.domain {
            params.put(&format!("{}{}", prefix, "Domain"), &field_value);
        }
        if let Some(ref field_value) = obj.domain_iam_role_name {
            params.put(&format!("{}{}", prefix, "DomainIAMRoleName"), &field_value);
        }
        if let Some(ref field_value) = obj.enable_iam_database_authentication {
            params.put(
                &format!("{}{}", prefix, "EnableIAMDatabaseAuthentication"),
                &field_value.to_string(),
            );
        }
        if let Some(ref field_value) = obj.enable_performance_insights {
            params.put(
                &format!("{}{}", prefix, "EnablePerformanceInsights"),
                &field_value.to_string(),
            );
        }
        if let Some(ref field_value) = obj.engine_version {
            params.put(&format!("{}{}", prefix, "EngineVersion"), &field_value);
        }
        if let Some(ref field_value) = obj.iops {
            params.put(&format!("{}{}", prefix, "Iops"), &field_value.to_string());
        }
        if let Some(ref field_value) = obj.license_model {
            params.put(&format!("{}{}", prefix, "LicenseModel"), &field_value);
        }
        if let Some(ref field_value) = obj.master_user_password {
            params.put(&format!("{}{}", prefix, "MasterUserPassword"), &field_value);
        }
        if let Some(ref field_value) = obj.monitoring_interval {
            params.put(
                &format!("{}{}", prefix, "MonitoringInterval"),
                &field_value.to_string(),
            );
        }
        if let Some(ref field_value) = obj.monitoring_role_arn {
            params.put(&format!("{}{}", prefix, "MonitoringRoleArn"), &field_value);
        }
        if let Some(ref field_value) = obj.multi_az {
            params.put(
                &format!("{}{}", prefix, "MultiAZ"),
                &field_value.to_string(),
            );
        }
        if let Some(ref field_value) = obj.new_db_instance_identifier {
            params.put(
                &format!("{}{}", prefix, "NewDBInstanceIdentifier"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.option_group_name {
            params.put(&format!("{}{}", prefix, "OptionGroupName"), &field_value);
        }
        if let Some(ref field_value) = obj.performance_insights_kms_key_id {
            params.put(
                &format!("{}{}", prefix, "PerformanceInsightsKMSKeyId"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.preferred_backup_window {
            params.put(
                &format!("{}{}", prefix, "PreferredBackupWindow"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.preferred_maintenance_window {
            params.put(
                &format!("{}{}", prefix, "PreferredMaintenanceWindow"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.promotion_tier {
            params.put(
                &format!("{}{}", prefix, "PromotionTier"),
                &field_value.to_string(),
            );
        }

        if let Some(ref field_value) = obj.storage_type {
            params.put(&format!("{}{}", prefix, "StorageType"), &field_value);
        }
        if let Some(ref field_value) = obj.tde_credential_arn {
            params.put(&format!("{}{}", prefix, "TdeCredentialArn"), &field_value);
        }
        if let Some(ref field_value) = obj.tde_credential_password {
            params.put(
                &format!("{}{}", prefix, "TdeCredentialPassword"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.vpc_security_group_ids {
            VpcSecurityGroupIdListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "VpcSecurityGroupId"),
                field_value,
            );
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct ModifyDBInstanceResult {
    pub db_instance: Option<DBInstance>,
}

struct ModifyDBInstanceResultDeserializer;
impl ModifyDBInstanceResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ModifyDBInstanceResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ModifyDBInstanceResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "DBInstance" => {
                        obj.db_instance = Some(try!(DBInstanceDeserializer::deserialize(
                            "DBInstance",
                            stack
                        )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ModifyDBParameterGroupMessage {
    /// <p><p>The name of the DB parameter group.</p> <p>Constraints:</p> <ul> <li> <p>If supplied, must match the name of an existing DBParameterGroup.</p> </li> </ul></p>
    pub db_parameter_group_name: String,
    /// <p><p>An array of parameter names, values, and the apply method for the parameter update. At least one parameter name, value, and apply method must be supplied; subsequent arguments are optional. A maximum of 20 parameters can be modified in a single request.</p> <p>Valid Values (for the application method): <code>immediate | pending-reboot</code> </p> <note> <p>You can use the immediate value with dynamic parameters only. You can use the pending-reboot value for both dynamic and static parameters, and changes are applied when you reboot the DB instance without failover.</p> </note></p>
    pub parameters: Vec<Parameter>,
}

/// Serialize `ModifyDBParameterGroupMessage` contents to a `SignedRequest`.
struct ModifyDBParameterGroupMessageSerializer;
impl ModifyDBParameterGroupMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ModifyDBParameterGroupMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "DBParameterGroupName"),
            &obj.db_parameter_group_name,
        );
        ParametersListSerializer::serialize(
            params,
            &format!("{}{}", prefix, "Parameter"),
            &obj.parameters,
        );
    }
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ModifyDBSubnetGroupMessage {
    /// <p>The description for the DB subnet group.</p>
    pub db_subnet_group_description: Option<String>,
    /// <p>The name for the DB subnet group. This value is stored as a lowercase string. You can't modify the default subnet group. </p> <p>Constraints: Must match the name of an existing DBSubnetGroup. Must not be default.</p> <p>Example: <code>mySubnetgroup</code> </p>
    pub db_subnet_group_name: String,
    /// <p>The EC2 subnet IDs for the DB subnet group.</p>
    pub subnet_ids: Vec<String>,
}

/// Serialize `ModifyDBSubnetGroupMessage` contents to a `SignedRequest`.
struct ModifyDBSubnetGroupMessageSerializer;
impl ModifyDBSubnetGroupMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ModifyDBSubnetGroupMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.db_subnet_group_description {
            params.put(
                &format!("{}{}", prefix, "DBSubnetGroupDescription"),
                &field_value,
            );
        }
        params.put(
            &format!("{}{}", prefix, "DBSubnetGroupName"),
            &obj.db_subnet_group_name,
        );
        SubnetIdentifierListSerializer::serialize(
            params,
            &format!("{}{}", prefix, "SubnetIdentifier"),
            &obj.subnet_ids,
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct ModifyDBSubnetGroupResult {
    pub db_subnet_group: Option<DBSubnetGroup>,
}

struct ModifyDBSubnetGroupResultDeserializer;
impl ModifyDBSubnetGroupResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ModifyDBSubnetGroupResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ModifyDBSubnetGroupResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "DBSubnetGroup" => {
                        obj.db_subnet_group = Some(try!(DBSubnetGroupDeserializer::deserialize(
                            "DBSubnetGroup",
                            stack
                        )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ModifyEventSubscriptionMessage {
    /// <p> A Boolean value; set to <b>true</b> to activate the subscription. </p>
    pub enabled: Option<bool>,
    /// <p> A list of event categories for a SourceType that you want to subscribe to. You can see a list of the categories for a given SourceType by using the <b>DescribeEventCategories</b> action. </p>
    pub event_categories: Option<Vec<String>>,
    /// <p>The Amazon Resource Name (ARN) of the SNS topic created for event notification. The ARN is created by Amazon SNS when you create a topic and subscribe to it.</p>
    pub sns_topic_arn: Option<String>,
    /// <p>The type of source that is generating the events. For example, if you want to be notified of events generated by a DB instance, you would set this parameter to db-instance. if this value is not specified, all events are returned.</p> <p>Valid values: db-instance | db-parameter-group | db-security-group | db-snapshot</p>
    pub source_type: Option<String>,
    /// <p>The name of the event notification subscription.</p>
    pub subscription_name: String,
}

/// Serialize `ModifyEventSubscriptionMessage` contents to a `SignedRequest`.
struct ModifyEventSubscriptionMessageSerializer;
impl ModifyEventSubscriptionMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ModifyEventSubscriptionMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.enabled {
            params.put(
                &format!("{}{}", prefix, "Enabled"),
                &field_value.to_string(),
            );
        }
        if let Some(ref field_value) = obj.event_categories {
            EventCategoriesListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "EventCategory"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.sns_topic_arn {
            params.put(&format!("{}{}", prefix, "SnsTopicArn"), &field_value);
        }
        if let Some(ref field_value) = obj.source_type {
            params.put(&format!("{}{}", prefix, "SourceType"), &field_value);
        }
        params.put(
            &format!("{}{}", prefix, "SubscriptionName"),
            &obj.subscription_name,
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct ModifyEventSubscriptionResult {
    pub event_subscription: Option<EventSubscription>,
}

struct ModifyEventSubscriptionResultDeserializer;
impl ModifyEventSubscriptionResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ModifyEventSubscriptionResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ModifyEventSubscriptionResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "EventSubscription" => {
                        obj.event_subscription = Some(try!(
                            EventSubscriptionDeserializer::deserialize("EventSubscription", stack)
                        ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Provides information on the option groups the DB instance is a member of.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct OptionGroupMembership {
    /// <p>The name of the option group that the instance belongs to.</p>
    pub option_group_name: Option<String>,
    /// <p>The status of the DB instance's option group membership. Valid values are: <code>in-sync</code>, <code>pending-apply</code>, <code>pending-removal</code>, <code>pending-maintenance-apply</code>, <code>pending-maintenance-removal</code>, <code>applying</code>, <code>removing</code>, and <code>failed</code>. </p>
    pub status: Option<String>,
}

struct OptionGroupMembershipDeserializer;
impl OptionGroupMembershipDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<OptionGroupMembership, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = OptionGroupMembership::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "OptionGroupName" => {
                        obj.option_group_name = Some(try!(StringDeserializer::deserialize(
                            "OptionGroupName",
                            stack
                        )));
                    }
                    "Status" => {
                        obj.status = Some(try!(StringDeserializer::deserialize("Status", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct OptionGroupMembershipListDeserializer;
impl OptionGroupMembershipListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<OptionGroupMembership>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "OptionGroupMembership" {
                        obj.push(try!(OptionGroupMembershipDeserializer::deserialize(
                            "OptionGroupMembership",
                            stack
                        )));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}
/// <p>Contains a list of available options for a DB instance.</p> <p> This data type is used as a response element in the <a>DescribeOrderableDBInstanceOptions</a> action. </p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct OrderableDBInstanceOption {
    /// <p>A list of Availability Zones for a DB instance.</p>
    pub availability_zones: Option<Vec<AvailabilityZone>>,
    /// <p>The DB instance class for a DB instance.</p>
    pub db_instance_class: Option<String>,
    /// <p>The engine type of a DB instance.</p>
    pub engine: Option<String>,
    /// <p>The engine version of a DB instance.</p>
    pub engine_version: Option<String>,
    /// <p>The license model for a DB instance.</p>
    pub license_model: Option<String>,
    /// <p>Maximum total provisioned IOPS for a DB instance.</p>
    pub max_iops_per_db_instance: Option<i64>,
    /// <p>Maximum provisioned IOPS per GiB for a DB instance.</p>
    pub max_iops_per_gib: Option<f64>,
    /// <p>Maximum storage size for a DB instance.</p>
    pub max_storage_size: Option<i64>,
    /// <p>Minimum total provisioned IOPS for a DB instance.</p>
    pub min_iops_per_db_instance: Option<i64>,
    /// <p>Minimum provisioned IOPS per GiB for a DB instance.</p>
    pub min_iops_per_gib: Option<f64>,
    /// <p>Minimum storage size for a DB instance.</p>
    pub min_storage_size: Option<i64>,
    /// <p>Indicates whether a DB instance is Multi-AZ capable.</p>
    pub multi_az_capable: Option<bool>,
    /// <p>Indicates whether a DB instance can have a Read Replica.</p>
    pub read_replica_capable: Option<bool>,
    /// <p>Indicates the storage type for a DB instance.</p>
    pub storage_type: Option<String>,
    /// <p>Indicates whether a DB instance supports Enhanced Monitoring at intervals from 1 to 60 seconds.</p>
    pub supports_enhanced_monitoring: Option<bool>,
    /// <p>Indicates whether a DB instance supports IAM database authentication.</p>
    pub supports_iam_database_authentication: Option<bool>,
    /// <p>Indicates whether a DB instance supports provisioned IOPS.</p>
    pub supports_iops: Option<bool>,
    /// <p>True if a DB instance supports Performance Insights, otherwise false.</p>
    pub supports_performance_insights: Option<bool>,
    /// <p>Indicates whether a DB instance supports encrypted storage.</p>
    pub supports_storage_encryption: Option<bool>,
    /// <p>Indicates whether a DB instance is in a VPC.</p>
    pub vpc: Option<bool>,
}

struct OrderableDBInstanceOptionDeserializer;
impl OrderableDBInstanceOptionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<OrderableDBInstanceOption, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = OrderableDBInstanceOption::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "AvailabilityZones" => {
                        obj.availability_zones =
                            Some(try!(AvailabilityZoneListDeserializer::deserialize(
                                "AvailabilityZones",
                                stack
                            )));
                    }
                    "DBInstanceClass" => {
                        obj.db_instance_class = Some(try!(StringDeserializer::deserialize(
                            "DBInstanceClass",
                            stack
                        )));
                    }
                    "Engine" => {
                        obj.engine = Some(try!(StringDeserializer::deserialize("Engine", stack)));
                    }
                    "EngineVersion" => {
                        obj.engine_version = Some(try!(StringDeserializer::deserialize(
                            "EngineVersion",
                            stack
                        )));
                    }
                    "LicenseModel" => {
                        obj.license_model =
                            Some(try!(StringDeserializer::deserialize("LicenseModel", stack)));
                    }
                    "MaxIopsPerDbInstance" => {
                        obj.max_iops_per_db_instance = Some(try!(
                            IntegerOptionalDeserializer::deserialize("MaxIopsPerDbInstance", stack)
                        ));
                    }
                    "MaxIopsPerGib" => {
                        obj.max_iops_per_gib = Some(try!(DoubleOptionalDeserializer::deserialize(
                            "MaxIopsPerGib",
                            stack
                        )));
                    }
                    "MaxStorageSize" => {
                        obj.max_storage_size = Some(try!(
                            IntegerOptionalDeserializer::deserialize("MaxStorageSize", stack)
                        ));
                    }
                    "MinIopsPerDbInstance" => {
                        obj.min_iops_per_db_instance = Some(try!(
                            IntegerOptionalDeserializer::deserialize("MinIopsPerDbInstance", stack)
                        ));
                    }
                    "MinIopsPerGib" => {
                        obj.min_iops_per_gib = Some(try!(DoubleOptionalDeserializer::deserialize(
                            "MinIopsPerGib",
                            stack
                        )));
                    }
                    "MinStorageSize" => {
                        obj.min_storage_size = Some(try!(
                            IntegerOptionalDeserializer::deserialize("MinStorageSize", stack)
                        ));
                    }
                    "MultiAZCapable" => {
                        obj.multi_az_capable = Some(try!(BooleanDeserializer::deserialize(
                            "MultiAZCapable",
                            stack
                        )));
                    }
                    "ReadReplicaCapable" => {
                        obj.read_replica_capable = Some(try!(BooleanDeserializer::deserialize(
                            "ReadReplicaCapable",
                            stack
                        )));
                    }
                    "StorageType" => {
                        obj.storage_type =
                            Some(try!(StringDeserializer::deserialize("StorageType", stack)));
                    }
                    "SupportsEnhancedMonitoring" => {
                        obj.supports_enhanced_monitoring = Some(try!(
                            BooleanDeserializer::deserialize("SupportsEnhancedMonitoring", stack)
                        ));
                    }
                    "SupportsIAMDatabaseAuthentication" => {
                        obj.supports_iam_database_authentication =
                            Some(try!(BooleanDeserializer::deserialize(
                                "SupportsIAMDatabaseAuthentication",
                                stack
                            )));
                    }
                    "SupportsIops" => {
                        obj.supports_iops = Some(try!(BooleanDeserializer::deserialize(
                            "SupportsIops",
                            stack
                        )));
                    }
                    "SupportsPerformanceInsights" => {
                        obj.supports_performance_insights = Some(try!(
                            BooleanDeserializer::deserialize("SupportsPerformanceInsights", stack)
                        ));
                    }
                    "SupportsStorageEncryption" => {
                        obj.supports_storage_encryption = Some(try!(
                            BooleanDeserializer::deserialize("SupportsStorageEncryption", stack)
                        ));
                    }
                    "Vpc" => {
                        obj.vpc = Some(try!(BooleanDeserializer::deserialize("Vpc", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct OrderableDBInstanceOptionsListDeserializer;
impl OrderableDBInstanceOptionsListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<OrderableDBInstanceOption>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "OrderableDBInstanceOption" {
                        obj.push(try!(OrderableDBInstanceOptionDeserializer::deserialize(
                            "OrderableDBInstanceOption",
                            stack
                        )));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}
/// <p> Contains the result of a successful invocation of the <a>DescribeOrderableDBInstanceOptions</a> action. </p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct OrderableDBInstanceOptionsMessage {
    /// <p> An optional pagination token provided by a previous OrderableDBInstanceOptions request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code> . </p>
    pub marker: Option<String>,
    /// <p>An <a>OrderableDBInstanceOption</a> structure containing information about orderable options for the DB instance.</p>
    pub orderable_db_instance_options: Option<Vec<OrderableDBInstanceOption>>,
}

struct OrderableDBInstanceOptionsMessageDeserializer;
impl OrderableDBInstanceOptionsMessageDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<OrderableDBInstanceOptionsMessage, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = OrderableDBInstanceOptionsMessage::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Marker" => {
                        obj.marker = Some(try!(StringDeserializer::deserialize("Marker", stack)));
                    }
                    "OrderableDBInstanceOptions" => {
                        obj.orderable_db_instance_options = Some(try!(
                            OrderableDBInstanceOptionsListDeserializer::deserialize(
                                "OrderableDBInstanceOptions",
                                stack
                            )
                        ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p> This data type is used as a request parameter in the <a>ModifyDBParameterGroup</a> and <a>ResetDBParameterGroup</a> actions. </p> <p>This data type is used as a response element in the <a>DescribeEngineDefaultParameters</a> and <a>DescribeDBParameters</a> actions.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Parameter {
    /// <p>Specifies the valid range of values for the parameter.</p>
    pub allowed_values: Option<String>,
    /// <p>Indicates when to apply parameter updates.</p>
    pub apply_method: Option<String>,
    /// <p>Specifies the engine specific parameters type.</p>
    pub apply_type: Option<String>,
    /// <p>Specifies the valid data type for the parameter.</p>
    pub data_type: Option<String>,
    /// <p>Provides a description of the parameter.</p>
    pub description: Option<String>,
    /// <p> Indicates whether (<code>true</code>) or not (<code>false</code>) the parameter can be modified. Some parameters have security or operational implications that prevent them from being changed. </p>
    pub is_modifiable: Option<bool>,
    /// <p>The earliest engine version to which the parameter can apply.</p>
    pub minimum_engine_version: Option<String>,
    /// <p>Specifies the name of the parameter.</p>
    pub parameter_name: Option<String>,
    /// <p>Specifies the value of the parameter.</p>
    pub parameter_value: Option<String>,
    /// <p>Indicates the source of the parameter value.</p>
    pub source: Option<String>,
}

struct ParameterDeserializer;
impl ParameterDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Parameter, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = Parameter::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "AllowedValues" => {
                        obj.allowed_values = Some(try!(StringDeserializer::deserialize(
                            "AllowedValues",
                            stack
                        )));
                    }
                    "ApplyMethod" => {
                        obj.apply_method = Some(try!(ApplyMethodDeserializer::deserialize(
                            "ApplyMethod",
                            stack
                        )));
                    }
                    "ApplyType" => {
                        obj.apply_type =
                            Some(try!(StringDeserializer::deserialize("ApplyType", stack)));
                    }
                    "DataType" => {
                        obj.data_type =
                            Some(try!(StringDeserializer::deserialize("DataType", stack)));
                    }
                    "Description" => {
                        obj.description =
                            Some(try!(StringDeserializer::deserialize("Description", stack)));
                    }
                    "IsModifiable" => {
                        obj.is_modifiable = Some(try!(BooleanDeserializer::deserialize(
                            "IsModifiable",
                            stack
                        )));
                    }
                    "MinimumEngineVersion" => {
                        obj.minimum_engine_version = Some(try!(StringDeserializer::deserialize(
                            "MinimumEngineVersion",
                            stack
                        )));
                    }
                    "ParameterName" => {
                        obj.parameter_name = Some(try!(StringDeserializer::deserialize(
                            "ParameterName",
                            stack
                        )));
                    }
                    "ParameterValue" => {
                        obj.parameter_value = Some(try!(StringDeserializer::deserialize(
                            "ParameterValue",
                            stack
                        )));
                    }
                    "Source" => {
                        obj.source = Some(try!(StringDeserializer::deserialize("Source", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

/// Serialize `Parameter` contents to a `SignedRequest`.
struct ParameterSerializer;
impl ParameterSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Parameter) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.allowed_values {
            params.put(&format!("{}{}", prefix, "AllowedValues"), &field_value);
        }
        if let Some(ref field_value) = obj.apply_method {
            params.put(&format!("{}{}", prefix, "ApplyMethod"), &field_value);
        }
        if let Some(ref field_value) = obj.apply_type {
            params.put(&format!("{}{}", prefix, "ApplyType"), &field_value);
        }
        if let Some(ref field_value) = obj.data_type {
            params.put(&format!("{}{}", prefix, "DataType"), &field_value);
        }
        if let Some(ref field_value) = obj.description {
            params.put(&format!("{}{}", prefix, "Description"), &field_value);
        }
        if let Some(ref field_value) = obj.is_modifiable {
            params.put(
                &format!("{}{}", prefix, "IsModifiable"),
                &field_value.to_string(),
            );
        }
        if let Some(ref field_value) = obj.minimum_engine_version {
            params.put(
                &format!("{}{}", prefix, "MinimumEngineVersion"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.parameter_name {
            params.put(&format!("{}{}", prefix, "ParameterName"), &field_value);
        }
        if let Some(ref field_value) = obj.parameter_value {
            params.put(&format!("{}{}", prefix, "ParameterValue"), &field_value);
        }
        if let Some(ref field_value) = obj.source {
            params.put(&format!("{}{}", prefix, "Source"), &field_value);
        }
    }
}

struct ParametersListDeserializer;
impl ParametersListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<Parameter>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "Parameter" {
                        obj.push(try!(ParameterDeserializer::deserialize("Parameter", stack)));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}

/// Serialize `ParametersList` contents to a `SignedRequest`.
struct ParametersListSerializer;
impl ParametersListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<Parameter>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            ParameterSerializer::serialize(params, &key, obj);
        }
    }
}

/// <p>A list of the log types whose configuration is still pending. In other words, these log types are in the process of being activated or deactivated.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct PendingCloudwatchLogsExports {
    /// <p>Log types that are in the process of being enabled. After they are enabled, these log types are exported to CloudWatch Logs.</p>
    pub log_types_to_disable: Option<Vec<String>>,
    /// <p>Log types that are in the process of being deactivated. After they are deactivated, these log types aren't exported to CloudWatch Logs.</p>
    pub log_types_to_enable: Option<Vec<String>>,
}

struct PendingCloudwatchLogsExportsDeserializer;
impl PendingCloudwatchLogsExportsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PendingCloudwatchLogsExports, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = PendingCloudwatchLogsExports::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "LogTypesToDisable" => {
                        obj.log_types_to_disable = Some(try!(
                            LogTypeListDeserializer::deserialize("LogTypesToDisable", stack)
                        ));
                    }
                    "LogTypesToEnable" => {
                        obj.log_types_to_enable = Some(try!(LogTypeListDeserializer::deserialize(
                            "LogTypesToEnable",
                            stack
                        )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Provides information about a pending maintenance action for a resource.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct PendingMaintenanceAction {
    /// <p>The type of pending maintenance action that is available for the resource.</p>
    pub action: Option<String>,
    /// <p>The date of the maintenance window when the action is applied. The maintenance action is applied to the resource during its first maintenance window after this date. If this date is specified, any <code>next-maintenance</code> opt-in requests are ignored.</p>
    pub auto_applied_after_date: Option<String>,
    /// <p>The effective date when the pending maintenance action is applied to the resource. This date takes into account opt-in requests received from the <a>ApplyPendingMaintenanceAction</a> API, the <code>AutoAppliedAfterDate</code>, and the <code>ForcedApplyDate</code>. This value is blank if an opt-in request has not been received and nothing has been specified as <code>AutoAppliedAfterDate</code> or <code>ForcedApplyDate</code>.</p>
    pub current_apply_date: Option<String>,
    /// <p>A description providing more detail about the maintenance action.</p>
    pub description: Option<String>,
    /// <p>The date when the maintenance action is automatically applied. The maintenance action is applied to the resource on this date regardless of the maintenance window for the resource. If this date is specified, any <code>immediate</code> opt-in requests are ignored.</p>
    pub forced_apply_date: Option<String>,
    /// <p>Indicates the type of opt-in request that has been received for the resource.</p>
    pub opt_in_status: Option<String>,
}

struct PendingMaintenanceActionDeserializer;
impl PendingMaintenanceActionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PendingMaintenanceAction, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = PendingMaintenanceAction::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Action" => {
                        obj.action = Some(try!(StringDeserializer::deserialize("Action", stack)));
                    }
                    "AutoAppliedAfterDate" => {
                        obj.auto_applied_after_date = Some(try!(TStampDeserializer::deserialize(
                            "AutoAppliedAfterDate",
                            stack
                        )));
                    }
                    "CurrentApplyDate" => {
                        obj.current_apply_date = Some(try!(TStampDeserializer::deserialize(
                            "CurrentApplyDate",
                            stack
                        )));
                    }
                    "Description" => {
                        obj.description =
                            Some(try!(StringDeserializer::deserialize("Description", stack)));
                    }
                    "ForcedApplyDate" => {
                        obj.forced_apply_date = Some(try!(TStampDeserializer::deserialize(
                            "ForcedApplyDate",
                            stack
                        )));
                    }
                    "OptInStatus" => {
                        obj.opt_in_status =
                            Some(try!(StringDeserializer::deserialize("OptInStatus", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct PendingMaintenanceActionDetailsDeserializer;
impl PendingMaintenanceActionDetailsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<PendingMaintenanceAction>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "PendingMaintenanceAction" {
                        obj.push(try!(PendingMaintenanceActionDeserializer::deserialize(
                            "PendingMaintenanceAction",
                            stack
                        )));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}
struct PendingMaintenanceActionsDeserializer;
impl PendingMaintenanceActionsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<ResourcePendingMaintenanceActions>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "ResourcePendingMaintenanceActions" {
                        obj.push(try!(
                            ResourcePendingMaintenanceActionsDeserializer::deserialize(
                                "ResourcePendingMaintenanceActions",
                                stack
                            )
                        ));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}
/// <p>Data returned from the <b>DescribePendingMaintenanceActions</b> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct PendingMaintenanceActionsMessage {
    /// <p> An optional pagination token provided by a previous <code>DescribePendingMaintenanceActions</code> request. If this parameter is specified, the response includes only records beyond the marker, up to a number of records specified by <code>MaxRecords</code>. </p>
    pub marker: Option<String>,
    /// <p>A list of the pending maintenance actions for the resource.</p>
    pub pending_maintenance_actions: Option<Vec<ResourcePendingMaintenanceActions>>,
}

struct PendingMaintenanceActionsMessageDeserializer;
impl PendingMaintenanceActionsMessageDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PendingMaintenanceActionsMessage, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = PendingMaintenanceActionsMessage::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Marker" => {
                        obj.marker = Some(try!(StringDeserializer::deserialize("Marker", stack)));
                    }
                    "PendingMaintenanceActions" => {
                        obj.pending_maintenance_actions =
                            Some(try!(PendingMaintenanceActionsDeserializer::deserialize(
                                "PendingMaintenanceActions",
                                stack
                            )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p> This data type is used as a response element in the <a>ModifyDBInstance</a> action. </p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct PendingModifiedValues {
    /// <p> Contains the new <code>AllocatedStorage</code> size for the DB instance that will be applied or is currently being applied. </p>
    pub allocated_storage: Option<i64>,
    /// <p>Specifies the pending number of days for which automated backups are retained.</p>
    pub backup_retention_period: Option<i64>,
    /// <p>Specifies the identifier of the CA certificate for the DB instance.</p>
    pub ca_certificate_identifier: Option<String>,
    /// <p> Contains the new <code>DBInstanceClass</code> for the DB instance that will be applied or is currently being applied. </p>
    pub db_instance_class: Option<String>,
    /// <p> Contains the new <code>DBInstanceIdentifier</code> for the DB instance that will be applied or is currently being applied. </p>
    pub db_instance_identifier: Option<String>,
    /// <p>The new DB subnet group for the DB instance. </p>
    pub db_subnet_group_name: Option<String>,
    /// <p>Indicates the database engine version.</p>
    pub engine_version: Option<String>,
    /// <p>Specifies the new Provisioned IOPS value for the DB instance that will be applied or is currently being applied.</p>
    pub iops: Option<i64>,
    /// <p>The license model for the DB instance.</p> <p>Valid values: <code>license-included</code> | <code>bring-your-own-license</code> | <code>general-public-license</code> </p>
    pub license_model: Option<String>,
    /// <p>Contains the pending or currently-in-progress change of the master credentials for the DB instance.</p>
    pub master_user_password: Option<String>,
    /// <p>Indicates that the Single-AZ DB instance is to change to a Multi-AZ deployment.</p>
    pub multi_az: Option<bool>,
    pub pending_cloudwatch_logs_exports: Option<PendingCloudwatchLogsExports>,
    /// <p>Specifies the pending port for the DB instance.</p>
    pub port: Option<i64>,
    /// <p>Specifies the storage type to be associated with the DB instance.</p>
    pub storage_type: Option<String>,
}

struct PendingModifiedValuesDeserializer;
impl PendingModifiedValuesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PendingModifiedValues, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = PendingModifiedValues::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "AllocatedStorage" => {
                        obj.allocated_storage = Some(try!(
                            IntegerOptionalDeserializer::deserialize("AllocatedStorage", stack)
                        ));
                    }
                    "BackupRetentionPeriod" => {
                        obj.backup_retention_period =
                            Some(try!(IntegerOptionalDeserializer::deserialize(
                                "BackupRetentionPeriod",
                                stack
                            )));
                    }
                    "CACertificateIdentifier" => {
                        obj.ca_certificate_identifier = Some(try!(
                            StringDeserializer::deserialize("CACertificateIdentifier", stack)
                        ));
                    }
                    "DBInstanceClass" => {
                        obj.db_instance_class = Some(try!(StringDeserializer::deserialize(
                            "DBInstanceClass",
                            stack
                        )));
                    }
                    "DBInstanceIdentifier" => {
                        obj.db_instance_identifier = Some(try!(StringDeserializer::deserialize(
                            "DBInstanceIdentifier",
                            stack
                        )));
                    }
                    "DBSubnetGroupName" => {
                        obj.db_subnet_group_name = Some(try!(StringDeserializer::deserialize(
                            "DBSubnetGroupName",
                            stack
                        )));
                    }
                    "EngineVersion" => {
                        obj.engine_version = Some(try!(StringDeserializer::deserialize(
                            "EngineVersion",
                            stack
                        )));
                    }
                    "Iops" => {
                        obj.iops = Some(try!(IntegerOptionalDeserializer::deserialize(
                            "Iops", stack
                        )));
                    }
                    "LicenseModel" => {
                        obj.license_model =
                            Some(try!(StringDeserializer::deserialize("LicenseModel", stack)));
                    }
                    "MasterUserPassword" => {
                        obj.master_user_password = Some(try!(StringDeserializer::deserialize(
                            "MasterUserPassword",
                            stack
                        )));
                    }
                    "MultiAZ" => {
                        obj.multi_az = Some(try!(BooleanOptionalDeserializer::deserialize(
                            "MultiAZ", stack
                        )));
                    }
                    "PendingCloudwatchLogsExports" => {
                        obj.pending_cloudwatch_logs_exports =
                            Some(try!(PendingCloudwatchLogsExportsDeserializer::deserialize(
                                "PendingCloudwatchLogsExports",
                                stack
                            )));
                    }
                    "Port" => {
                        obj.port = Some(try!(IntegerOptionalDeserializer::deserialize(
                            "Port", stack
                        )));
                    }
                    "StorageType" => {
                        obj.storage_type =
                            Some(try!(StringDeserializer::deserialize("StorageType", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct PromoteReadReplicaDBClusterMessage {
    /// <p>The identifier of the DB cluster Read Replica to promote. This parameter is not case-sensitive. </p> <p>Constraints:</p> <ul> <li> <p>Must match the identifier of an existing DBCluster Read Replica.</p> </li> </ul> <p>Example: <code>my-cluster-replica1</code> </p>
    pub db_cluster_identifier: String,
}

/// Serialize `PromoteReadReplicaDBClusterMessage` contents to a `SignedRequest`.
struct PromoteReadReplicaDBClusterMessageSerializer;
impl PromoteReadReplicaDBClusterMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &PromoteReadReplicaDBClusterMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "DBClusterIdentifier"),
            &obj.db_cluster_identifier,
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct PromoteReadReplicaDBClusterResult {
    pub db_cluster: Option<DBCluster>,
}

struct PromoteReadReplicaDBClusterResultDeserializer;
impl PromoteReadReplicaDBClusterResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PromoteReadReplicaDBClusterResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = PromoteReadReplicaDBClusterResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "DBCluster" => {
                        obj.db_cluster =
                            Some(try!(DBClusterDeserializer::deserialize("DBCluster", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>A range of integer values.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Range {
    /// <p>The minimum value in the range.</p>
    pub from: Option<i64>,
    /// <p>The step value for the range. For example, if you have a range of 5,000 to 10,000, with a step value of 1,000, the valid values start at 5,000 and step up by 1,000. Even though 7,500 is within the range, it isn't a valid value for the range. The valid values are 5,000, 6,000, 7,000, 8,000... </p>
    pub step: Option<i64>,
    /// <p>The maximum value in the range.</p>
    pub to: Option<i64>,
}

struct RangeDeserializer;
impl RangeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Range, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = Range::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "From" => {
                        obj.from = Some(try!(IntegerDeserializer::deserialize("From", stack)));
                    }
                    "Step" => {
                        obj.step = Some(try!(IntegerOptionalDeserializer::deserialize(
                            "Step", stack
                        )));
                    }
                    "To" => {
                        obj.to = Some(try!(IntegerDeserializer::deserialize("To", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct RangeListDeserializer;
impl RangeListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<Range>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "Range" {
                        obj.push(try!(RangeDeserializer::deserialize("Range", stack)));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}
struct ReadReplicaDBClusterIdentifierListDeserializer;
impl ReadReplicaDBClusterIdentifierListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "ReadReplicaDBClusterIdentifier" {
                        obj.push(try!(StringDeserializer::deserialize(
                            "ReadReplicaDBClusterIdentifier",
                            stack
                        )));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}
struct ReadReplicaDBInstanceIdentifierListDeserializer;
impl ReadReplicaDBInstanceIdentifierListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "ReadReplicaDBInstanceIdentifier" {
                        obj.push(try!(StringDeserializer::deserialize(
                            "ReadReplicaDBInstanceIdentifier",
                            stack
                        )));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}
struct ReadReplicaIdentifierListDeserializer;
impl ReadReplicaIdentifierListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "ReadReplicaIdentifier" {
                        obj.push(try!(StringDeserializer::deserialize(
                            "ReadReplicaIdentifier",
                            stack
                        )));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}
/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct RebootDBInstanceMessage {
    /// <p><p>The DB instance identifier. This parameter is stored as a lowercase string.</p> <p>Constraints:</p> <ul> <li> <p>Must match the identifier of an existing DBInstance.</p> </li> </ul></p>
    pub db_instance_identifier: String,
    /// <p> When <code>true</code>, the reboot is conducted through a MultiAZ failover. </p> <p>Constraint: You can't specify <code>true</code> if the instance is not configured for MultiAZ.</p>
    pub force_failover: Option<bool>,
}

/// Serialize `RebootDBInstanceMessage` contents to a `SignedRequest`.
struct RebootDBInstanceMessageSerializer;
impl RebootDBInstanceMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &RebootDBInstanceMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "DBInstanceIdentifier"),
            &obj.db_instance_identifier,
        );
        if let Some(ref field_value) = obj.force_failover {
            params.put(
                &format!("{}{}", prefix, "ForceFailover"),
                &field_value.to_string(),
            );
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct RebootDBInstanceResult {
    pub db_instance: Option<DBInstance>,
}

struct RebootDBInstanceResultDeserializer;
impl RebootDBInstanceResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<RebootDBInstanceResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = RebootDBInstanceResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "DBInstance" => {
                        obj.db_instance = Some(try!(DBInstanceDeserializer::deserialize(
                            "DBInstance",
                            stack
                        )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct RemoveRoleFromDBClusterMessage {
    /// <p>The name of the DB cluster to disassociate the IAM role from.</p>
    pub db_cluster_identifier: String,
    /// <p>The Amazon Resource Name (ARN) of the IAM role to disassociate from the DB cluster, for example <code>arn:aws:iam::123456789012:role/NeptuneAccessRole</code>.</p>
    pub role_arn: String,
}

/// Serialize `RemoveRoleFromDBClusterMessage` contents to a `SignedRequest`.
struct RemoveRoleFromDBClusterMessageSerializer;
impl RemoveRoleFromDBClusterMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &RemoveRoleFromDBClusterMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "DBClusterIdentifier"),
            &obj.db_cluster_identifier,
        );
        params.put(&format!("{}{}", prefix, "RoleArn"), &obj.role_arn);
    }
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct RemoveSourceIdentifierFromSubscriptionMessage {
    /// <p> The source identifier to be removed from the subscription, such as the <b>DB instance identifier</b> for a DB instance or the name of a security group. </p>
    pub source_identifier: String,
    /// <p>The name of the event notification subscription you want to remove a source identifier from.</p>
    pub subscription_name: String,
}

/// Serialize `RemoveSourceIdentifierFromSubscriptionMessage` contents to a `SignedRequest`.
struct RemoveSourceIdentifierFromSubscriptionMessageSerializer;
impl RemoveSourceIdentifierFromSubscriptionMessageSerializer {
    fn serialize(
        params: &mut Params,
        name: &str,
        obj: &RemoveSourceIdentifierFromSubscriptionMessage,
    ) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "SourceIdentifier"),
            &obj.source_identifier,
        );
        params.put(
            &format!("{}{}", prefix, "SubscriptionName"),
            &obj.subscription_name,
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct RemoveSourceIdentifierFromSubscriptionResult {
    pub event_subscription: Option<EventSubscription>,
}

struct RemoveSourceIdentifierFromSubscriptionResultDeserializer;
impl RemoveSourceIdentifierFromSubscriptionResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<RemoveSourceIdentifierFromSubscriptionResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = RemoveSourceIdentifierFromSubscriptionResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "EventSubscription" => {
                        obj.event_subscription = Some(try!(
                            EventSubscriptionDeserializer::deserialize("EventSubscription", stack)
                        ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct RemoveTagsFromResourceMessage {
    /// <p>The Amazon Neptune resource that the tags are removed from. This value is an Amazon Resource Name (ARN). For information about creating an ARN, see <a href="http://docs.aws.amazon.com/neptune/latest/UserGuide/tagging.ARN.html#tagging.ARN.Constructing"> Constructing an Amazon Resource Name (ARN)</a>.</p>
    pub resource_name: String,
    /// <p>The tag key (name) of the tag to be removed.</p>
    pub tag_keys: Vec<String>,
}

/// Serialize `RemoveTagsFromResourceMessage` contents to a `SignedRequest`.
struct RemoveTagsFromResourceMessageSerializer;
impl RemoveTagsFromResourceMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &RemoveTagsFromResourceMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(&format!("{}{}", prefix, "ResourceName"), &obj.resource_name);
        KeyListSerializer::serialize(params, &format!("{}{}", prefix, "TagKeys"), &obj.tag_keys);
    }
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ResetDBClusterParameterGroupMessage {
    /// <p>The name of the DB cluster parameter group to reset.</p>
    pub db_cluster_parameter_group_name: String,
    /// <p>A list of parameter names in the DB cluster parameter group to reset to the default values. You can't use this parameter if the <code>ResetAllParameters</code> parameter is set to <code>true</code>.</p>
    pub parameters: Option<Vec<Parameter>>,
    /// <p>A value that is set to <code>true</code> to reset all parameters in the DB cluster parameter group to their default values, and <code>false</code> otherwise. You can't use this parameter if there is a list of parameter names specified for the <code>Parameters</code> parameter.</p>
    pub reset_all_parameters: Option<bool>,
}

/// Serialize `ResetDBClusterParameterGroupMessage` contents to a `SignedRequest`.
struct ResetDBClusterParameterGroupMessageSerializer;
impl ResetDBClusterParameterGroupMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ResetDBClusterParameterGroupMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "DBClusterParameterGroupName"),
            &obj.db_cluster_parameter_group_name,
        );
        if let Some(ref field_value) = obj.parameters {
            ParametersListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Parameter"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.reset_all_parameters {
            params.put(
                &format!("{}{}", prefix, "ResetAllParameters"),
                &field_value.to_string(),
            );
        }
    }
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ResetDBParameterGroupMessage {
    /// <p><p>The name of the DB parameter group.</p> <p>Constraints:</p> <ul> <li> <p>Must match the name of an existing DBParameterGroup.</p> </li> </ul></p>
    pub db_parameter_group_name: String,
    /// <p>To reset the entire DB parameter group, specify the <code>DBParameterGroup</code> name and <code>ResetAllParameters</code> parameters. To reset specific parameters, provide a list of the following: <code>ParameterName</code> and <code>ApplyMethod</code>. A maximum of 20 parameters can be modified in a single request.</p> <p>Valid Values (for Apply method): <code>pending-reboot</code> </p>
    pub parameters: Option<Vec<Parameter>>,
    /// <p> Specifies whether (<code>true</code>) or not (<code>false</code>) to reset all parameters in the DB parameter group to default values. </p> <p>Default: <code>true</code> </p>
    pub reset_all_parameters: Option<bool>,
}

/// Serialize `ResetDBParameterGroupMessage` contents to a `SignedRequest`.
struct ResetDBParameterGroupMessageSerializer;
impl ResetDBParameterGroupMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ResetDBParameterGroupMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "DBParameterGroupName"),
            &obj.db_parameter_group_name,
        );
        if let Some(ref field_value) = obj.parameters {
            ParametersListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Parameter"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.reset_all_parameters {
            params.put(
                &format!("{}{}", prefix, "ResetAllParameters"),
                &field_value.to_string(),
            );
        }
    }
}

/// <p>Describes the pending maintenance actions for a resource.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ResourcePendingMaintenanceActions {
    /// <p>A list that provides details about the pending maintenance actions for the resource.</p>
    pub pending_maintenance_action_details: Option<Vec<PendingMaintenanceAction>>,
    /// <p>The ARN of the resource that has pending maintenance actions.</p>
    pub resource_identifier: Option<String>,
}

struct ResourcePendingMaintenanceActionsDeserializer;
impl ResourcePendingMaintenanceActionsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ResourcePendingMaintenanceActions, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ResourcePendingMaintenanceActions::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "PendingMaintenanceActionDetails" => {
                        obj.pending_maintenance_action_details = Some(try!(
                            PendingMaintenanceActionDetailsDeserializer::deserialize(
                                "PendingMaintenanceActionDetails",
                                stack
                            )
                        ));
                    }
                    "ResourceIdentifier" => {
                        obj.resource_identifier = Some(try!(StringDeserializer::deserialize(
                            "ResourceIdentifier",
                            stack
                        )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct RestoreDBClusterFromSnapshotMessage {
    /// <p>Provides the list of EC2 Availability Zones that instances in the restored DB cluster can be created in.</p>
    pub availability_zones: Option<Vec<String>>,
    /// <p>The name of the DB cluster to create from the DB snapshot or DB cluster snapshot. This parameter isn't case-sensitive.</p> <p>Constraints:</p> <ul> <li> <p>Must contain from 1 to 63 letters, numbers, or hyphens</p> </li> <li> <p>First character must be a letter</p> </li> <li> <p>Cannot end with a hyphen or contain two consecutive hyphens</p> </li> </ul> <p>Example: <code>my-snapshot-id</code> </p>
    pub db_cluster_identifier: String,
    /// <p>The name of the DB subnet group to use for the new DB cluster.</p> <p>Constraints: If supplied, must match the name of an existing DBSubnetGroup.</p> <p>Example: <code>mySubnetgroup</code> </p>
    pub db_subnet_group_name: Option<String>,
    /// <p>The database name for the restored DB cluster.</p>
    pub database_name: Option<String>,
    /// <p>True to enable mapping of AWS Identity and Access Management (IAM) accounts to database accounts, and otherwise false.</p> <p>Default: <code>false</code> </p>
    pub enable_iam_database_authentication: Option<bool>,
    /// <p>The database engine to use for the new DB cluster.</p> <p>Default: The same as source</p> <p>Constraint: Must be compatible with the engine of the source</p>
    pub engine: String,
    /// <p>The version of the database engine to use for the new DB cluster.</p>
    pub engine_version: Option<String>,
    /// <p><p>The AWS KMS key identifier to use when restoring an encrypted DB cluster from a DB snapshot or DB cluster snapshot.</p> <p>The KMS key identifier is the Amazon Resource Name (ARN) for the KMS encryption key. If you are restoring a DB cluster with the same AWS account that owns the KMS encryption key used to encrypt the new DB cluster, then you can use the KMS key alias instead of the ARN for the KMS encryption key.</p> <p>If you do not specify a value for the <code>KmsKeyId</code> parameter, then the following will occur:</p> <ul> <li> <p>If the DB snapshot or DB cluster snapshot in <code>SnapshotIdentifier</code> is encrypted, then the restored DB cluster is encrypted using the KMS key that was used to encrypt the DB snapshot or DB cluster snapshot.</p> </li> <li> <p>If the DB snapshot or DB cluster snapshot in <code>SnapshotIdentifier</code> is not encrypted, then the restored DB cluster is not encrypted.</p> </li> </ul></p>
    pub kms_key_id: Option<String>,
    /// <p>The name of the option group to use for the restored DB cluster.</p>
    pub option_group_name: Option<String>,
    /// <p>The port number on which the new DB cluster accepts connections.</p> <p>Constraints: Value must be <code>1150-65535</code> </p> <p>Default: The same port as the original DB cluster.</p>
    pub port: Option<i64>,
    /// <p><p>The identifier for the DB snapshot or DB cluster snapshot to restore from.</p> <p>You can use either the name or the Amazon Resource Name (ARN) to specify a DB cluster snapshot. However, you can use only the ARN to specify a DB snapshot.</p> <p>Constraints:</p> <ul> <li> <p>Must match the identifier of an existing Snapshot.</p> </li> </ul></p>
    pub snapshot_identifier: String,
    /// <p>The tags to be assigned to the restored DB cluster.</p>
    pub tags: Option<Vec<Tag>>,
    /// <p>A list of VPC security groups that the new DB cluster will belong to.</p>
    pub vpc_security_group_ids: Option<Vec<String>>,
}

/// Serialize `RestoreDBClusterFromSnapshotMessage` contents to a `SignedRequest`.
struct RestoreDBClusterFromSnapshotMessageSerializer;
impl RestoreDBClusterFromSnapshotMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &RestoreDBClusterFromSnapshotMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.availability_zones {
            AvailabilityZonesSerializer::serialize(
                params,
                &format!("{}{}", prefix, "AvailabilityZone"),
                field_value,
            );
        }
        params.put(
            &format!("{}{}", prefix, "DBClusterIdentifier"),
            &obj.db_cluster_identifier,
        );
        if let Some(ref field_value) = obj.db_subnet_group_name {
            params.put(&format!("{}{}", prefix, "DBSubnetGroupName"), &field_value);
        }
        if let Some(ref field_value) = obj.database_name {
            params.put(&format!("{}{}", prefix, "DatabaseName"), &field_value);
        }
        if let Some(ref field_value) = obj.enable_iam_database_authentication {
            params.put(
                &format!("{}{}", prefix, "EnableIAMDatabaseAuthentication"),
                &field_value.to_string(),
            );
        }
        params.put(&format!("{}{}", prefix, "Engine"), &obj.engine);
        if let Some(ref field_value) = obj.engine_version {
            params.put(&format!("{}{}", prefix, "EngineVersion"), &field_value);
        }
        if let Some(ref field_value) = obj.kms_key_id {
            params.put(&format!("{}{}", prefix, "KmsKeyId"), &field_value);
        }
        if let Some(ref field_value) = obj.option_group_name {
            params.put(&format!("{}{}", prefix, "OptionGroupName"), &field_value);
        }
        if let Some(ref field_value) = obj.port {
            params.put(&format!("{}{}", prefix, "Port"), &field_value.to_string());
        }
        params.put(
            &format!("{}{}", prefix, "SnapshotIdentifier"),
            &obj.snapshot_identifier,
        );
        if let Some(ref field_value) = obj.tags {
            TagListSerializer::serialize(params, &format!("{}{}", prefix, "Tag"), field_value);
        }
        if let Some(ref field_value) = obj.vpc_security_group_ids {
            VpcSecurityGroupIdListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "VpcSecurityGroupId"),
                field_value,
            );
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct RestoreDBClusterFromSnapshotResult {
    pub db_cluster: Option<DBCluster>,
}

struct RestoreDBClusterFromSnapshotResultDeserializer;
impl RestoreDBClusterFromSnapshotResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<RestoreDBClusterFromSnapshotResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = RestoreDBClusterFromSnapshotResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "DBCluster" => {
                        obj.db_cluster =
                            Some(try!(DBClusterDeserializer::deserialize("DBCluster", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct RestoreDBClusterToPointInTimeMessage {
    /// <p><p>The name of the new DB cluster to be created.</p> <p>Constraints:</p> <ul> <li> <p>Must contain from 1 to 63 letters, numbers, or hyphens</p> </li> <li> <p>First character must be a letter</p> </li> <li> <p>Cannot end with a hyphen or contain two consecutive hyphens</p> </li> </ul></p>
    pub db_cluster_identifier: String,
    /// <p>The DB subnet group name to use for the new DB cluster.</p> <p>Constraints: If supplied, must match the name of an existing DBSubnetGroup.</p> <p>Example: <code>mySubnetgroup</code> </p>
    pub db_subnet_group_name: Option<String>,
    /// <p>True to enable mapping of AWS Identity and Access Management (IAM) accounts to database accounts, and otherwise false.</p> <p>Default: <code>false</code> </p>
    pub enable_iam_database_authentication: Option<bool>,
    /// <p>The AWS KMS key identifier to use when restoring an encrypted DB cluster from an encrypted DB cluster.</p> <p>The KMS key identifier is the Amazon Resource Name (ARN) for the KMS encryption key. If you are restoring a DB cluster with the same AWS account that owns the KMS encryption key used to encrypt the new DB cluster, then you can use the KMS key alias instead of the ARN for the KMS encryption key.</p> <p>You can restore to a new DB cluster and encrypt the new DB cluster with a KMS key that is different than the KMS key used to encrypt the source DB cluster. The new DB cluster is encrypted with the KMS key identified by the <code>KmsKeyId</code> parameter.</p> <p>If you do not specify a value for the <code>KmsKeyId</code> parameter, then the following will occur:</p> <ul> <li> <p>If the DB cluster is encrypted, then the restored DB cluster is encrypted using the KMS key that was used to encrypt the source DB cluster.</p> </li> <li> <p>If the DB cluster is not encrypted, then the restored DB cluster is not encrypted.</p> </li> </ul> <p>If <code>DBClusterIdentifier</code> refers to a DB cluster that is not encrypted, then the restore request is rejected.</p>
    pub kms_key_id: Option<String>,
    /// <p>The name of the option group for the new DB cluster.</p>
    pub option_group_name: Option<String>,
    /// <p>The port number on which the new DB cluster accepts connections.</p> <p>Constraints: Value must be <code>1150-65535</code> </p> <p>Default: The same port as the original DB cluster.</p>
    pub port: Option<i64>,
    /// <p>The date and time to restore the DB cluster to.</p> <p>Valid Values: Value must be a time in Universal Coordinated Time (UTC) format</p> <p>Constraints:</p> <ul> <li> <p>Must be before the latest restorable time for the DB instance</p> </li> <li> <p>Must be specified if <code>UseLatestRestorableTime</code> parameter is not provided</p> </li> <li> <p>Cannot be specified if <code>UseLatestRestorableTime</code> parameter is true</p> </li> <li> <p>Cannot be specified if <code>RestoreType</code> parameter is <code>copy-on-write</code> </p> </li> </ul> <p>Example: <code>2015-03-07T23:45:00Z</code> </p>
    pub restore_to_time: Option<String>,
    /// <p>The type of restore to be performed. You can specify one of the following values:</p> <ul> <li> <p> <code>full-copy</code> - The new DB cluster is restored as a full copy of the source DB cluster.</p> </li> <li> <p> <code>copy-on-write</code> - The new DB cluster is restored as a clone of the source DB cluster.</p> </li> </ul> <p>Constraints: You can't specify <code>copy-on-write</code> if the engine version of the source DB cluster is earlier than 1.11.</p> <p>If you don't specify a <code>RestoreType</code> value, then the new DB cluster is restored as a full copy of the source DB cluster.</p>
    pub restore_type: Option<String>,
    /// <p><p>The identifier of the source DB cluster from which to restore.</p> <p>Constraints:</p> <ul> <li> <p>Must match the identifier of an existing DBCluster.</p> </li> </ul></p>
    pub source_db_cluster_identifier: String,
    pub tags: Option<Vec<Tag>>,
    /// <p>A value that is set to <code>true</code> to restore the DB cluster to the latest restorable backup time, and <code>false</code> otherwise. </p> <p>Default: <code>false</code> </p> <p>Constraints: Cannot be specified if <code>RestoreToTime</code> parameter is provided.</p>
    pub use_latest_restorable_time: Option<bool>,
    /// <p>A list of VPC security groups that the new DB cluster belongs to.</p>
    pub vpc_security_group_ids: Option<Vec<String>>,
}

/// Serialize `RestoreDBClusterToPointInTimeMessage` contents to a `SignedRequest`.
struct RestoreDBClusterToPointInTimeMessageSerializer;
impl RestoreDBClusterToPointInTimeMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &RestoreDBClusterToPointInTimeMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "DBClusterIdentifier"),
            &obj.db_cluster_identifier,
        );
        if let Some(ref field_value) = obj.db_subnet_group_name {
            params.put(&format!("{}{}", prefix, "DBSubnetGroupName"), &field_value);
        }
        if let Some(ref field_value) = obj.enable_iam_database_authentication {
            params.put(
                &format!("{}{}", prefix, "EnableIAMDatabaseAuthentication"),
                &field_value.to_string(),
            );
        }
        if let Some(ref field_value) = obj.kms_key_id {
            params.put(&format!("{}{}", prefix, "KmsKeyId"), &field_value);
        }
        if let Some(ref field_value) = obj.option_group_name {
            params.put(&format!("{}{}", prefix, "OptionGroupName"), &field_value);
        }
        if let Some(ref field_value) = obj.port {
            params.put(&format!("{}{}", prefix, "Port"), &field_value.to_string());
        }
        if let Some(ref field_value) = obj.restore_to_time {
            params.put(&format!("{}{}", prefix, "RestoreToTime"), &field_value);
        }
        if let Some(ref field_value) = obj.restore_type {
            params.put(&format!("{}{}", prefix, "RestoreType"), &field_value);
        }
        params.put(
            &format!("{}{}", prefix, "SourceDBClusterIdentifier"),
            &obj.source_db_cluster_identifier,
        );
        if let Some(ref field_value) = obj.tags {
            TagListSerializer::serialize(params, &format!("{}{}", prefix, "Tag"), field_value);
        }
        if let Some(ref field_value) = obj.use_latest_restorable_time {
            params.put(
                &format!("{}{}", prefix, "UseLatestRestorableTime"),
                &field_value.to_string(),
            );
        }
        if let Some(ref field_value) = obj.vpc_security_group_ids {
            VpcSecurityGroupIdListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "VpcSecurityGroupId"),
                field_value,
            );
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct RestoreDBClusterToPointInTimeResult {
    pub db_cluster: Option<DBCluster>,
}

struct RestoreDBClusterToPointInTimeResultDeserializer;
impl RestoreDBClusterToPointInTimeResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<RestoreDBClusterToPointInTimeResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = RestoreDBClusterToPointInTimeResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "DBCluster" => {
                        obj.db_cluster =
                            Some(try!(DBClusterDeserializer::deserialize("DBCluster", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct SourceIdsListDeserializer;
impl SourceIdsListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "SourceId" {
                        obj.push(try!(StringDeserializer::deserialize("SourceId", stack)));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}

/// Serialize `SourceIdsList` contents to a `SignedRequest`.
struct SourceIdsListSerializer;
impl SourceIdsListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

struct SourceTypeDeserializer;
impl SourceTypeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct StringDeserializer;
impl StringDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p> This data type is used as a response element in the <a>DescribeDBSubnetGroups</a> action. </p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Subnet {
    pub subnet_availability_zone: Option<AvailabilityZone>,
    /// <p>Specifies the identifier of the subnet.</p>
    pub subnet_identifier: Option<String>,
    /// <p>Specifies the status of the subnet.</p>
    pub subnet_status: Option<String>,
}

struct SubnetDeserializer;
impl SubnetDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Subnet, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = Subnet::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "SubnetAvailabilityZone" => {
                        obj.subnet_availability_zone =
                            Some(try!(AvailabilityZoneDeserializer::deserialize(
                                "SubnetAvailabilityZone",
                                stack
                            )));
                    }
                    "SubnetIdentifier" => {
                        obj.subnet_identifier = Some(try!(StringDeserializer::deserialize(
                            "SubnetIdentifier",
                            stack
                        )));
                    }
                    "SubnetStatus" => {
                        obj.subnet_status =
                            Some(try!(StringDeserializer::deserialize("SubnetStatus", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

/// Serialize `SubnetIdentifierList` contents to a `SignedRequest`.
struct SubnetIdentifierListSerializer;
impl SubnetIdentifierListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

struct SubnetListDeserializer;
impl SubnetListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<Subnet>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "Subnet" {
                        obj.push(try!(SubnetDeserializer::deserialize("Subnet", stack)));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}
struct SupportedCharacterSetsListDeserializer;
impl SupportedCharacterSetsListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<CharacterSet>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "CharacterSet" {
                        obj.push(try!(CharacterSetDeserializer::deserialize(
                            "CharacterSet",
                            stack
                        )));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}
struct SupportedTimezonesListDeserializer;
impl SupportedTimezonesListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<Timezone>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "Timezone" {
                        obj.push(try!(TimezoneDeserializer::deserialize("Timezone", stack)));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}
struct TStampDeserializer;
impl TStampDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Metadata assigned to an Amazon Neptune resource consisting of a key-value pair.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Tag {
    /// <p>A key is the required name of the tag. The string value can be from 1 to 128 Unicode characters in length and can't be prefixed with "aws:" or "rds:". The string can only contain only the set of Unicode letters, digits, white-space, '_', '.', '/', '=', '+', '-' (Java regex: "^([\\p{L}\\p{Z}\\p{N}_.:/=+\\-]*)$").</p>
    pub key: Option<String>,
    /// <p>A value is the optional value of the tag. The string value can be from 1 to 256 Unicode characters in length and can't be prefixed with "aws:" or "rds:". The string can only contain only the set of Unicode letters, digits, white-space, '_', '.', '/', '=', '+', '-' (Java regex: "^([\\p{L}\\p{Z}\\p{N}_.:/=+\\-]*)$").</p>
    pub value: Option<String>,
}

struct TagDeserializer;
impl TagDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Tag, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = Tag::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Key" => {
                        obj.key = Some(try!(StringDeserializer::deserialize("Key", stack)));
                    }
                    "Value" => {
                        obj.value = Some(try!(StringDeserializer::deserialize("Value", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

/// Serialize `Tag` contents to a `SignedRequest`.
struct TagSerializer;
impl TagSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Tag) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.key {
            params.put(&format!("{}{}", prefix, "Key"), &field_value);
        }
        if let Some(ref field_value) = obj.value {
            params.put(&format!("{}{}", prefix, "Value"), &field_value);
        }
    }
}

struct TagListDeserializer;
impl TagListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<Tag>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "Tag" {
                        obj.push(try!(TagDeserializer::deserialize("Tag", stack)));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}

/// Serialize `TagList` contents to a `SignedRequest`.
struct TagListSerializer;
impl TagListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<Tag>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            TagSerializer::serialize(params, &key, obj);
        }
    }
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct TagListMessage {
    /// <p>List of tags returned by the ListTagsForResource operation.</p>
    pub tag_list: Option<Vec<Tag>>,
}

struct TagListMessageDeserializer;
impl TagListMessageDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<TagListMessage, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = TagListMessage::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "TagList" => {
                        obj.tag_list =
                            Some(try!(TagListDeserializer::deserialize("TagList", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>A time zone associated with a <a>DBInstance</a>. This data type is an element in the response to the <a>DescribeDBInstances</a>, and the <a>DescribeDBEngineVersions</a> actions. </p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Timezone {
    /// <p>The name of the time zone.</p>
    pub timezone_name: Option<String>,
}

struct TimezoneDeserializer;
impl TimezoneDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Timezone, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = Timezone::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "TimezoneName" => {
                        obj.timezone_name =
                            Some(try!(StringDeserializer::deserialize("TimezoneName", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>The version of the database engine that a DB instance can be upgraded to.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct UpgradeTarget {
    /// <p>A value that indicates whether the target version is applied to any source DB instances that have AutoMinorVersionUpgrade set to true.</p>
    pub auto_upgrade: Option<bool>,
    /// <p>The version of the database engine that a DB instance can be upgraded to.</p>
    pub description: Option<String>,
    /// <p>The name of the upgrade target database engine.</p>
    pub engine: Option<String>,
    /// <p>The version number of the upgrade target database engine.</p>
    pub engine_version: Option<String>,
    /// <p>A value that indicates whether a database engine is upgraded to a major version.</p>
    pub is_major_version_upgrade: Option<bool>,
}

struct UpgradeTargetDeserializer;
impl UpgradeTargetDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<UpgradeTarget, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = UpgradeTarget::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "AutoUpgrade" => {
                        obj.auto_upgrade =
                            Some(try!(BooleanDeserializer::deserialize("AutoUpgrade", stack)));
                    }
                    "Description" => {
                        obj.description =
                            Some(try!(StringDeserializer::deserialize("Description", stack)));
                    }
                    "Engine" => {
                        obj.engine = Some(try!(StringDeserializer::deserialize("Engine", stack)));
                    }
                    "EngineVersion" => {
                        obj.engine_version = Some(try!(StringDeserializer::deserialize(
                            "EngineVersion",
                            stack
                        )));
                    }
                    "IsMajorVersionUpgrade" => {
                        obj.is_major_version_upgrade = Some(try!(
                            BooleanDeserializer::deserialize("IsMajorVersionUpgrade", stack)
                        ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Information about valid modifications that you can make to your DB instance. Contains the result of a successful call to the <a>DescribeValidDBInstanceModifications</a> action. You can use this information when you call <a>ModifyDBInstance</a>. </p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ValidDBInstanceModificationsMessage {
    /// <p>Valid storage options for your DB instance. </p>
    pub storage: Option<Vec<ValidStorageOptions>>,
}

struct ValidDBInstanceModificationsMessageDeserializer;
impl ValidDBInstanceModificationsMessageDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ValidDBInstanceModificationsMessage, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ValidDBInstanceModificationsMessage::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Storage" => {
                        obj.storage = Some(try!(ValidStorageOptionsListDeserializer::deserialize(
                            "Storage", stack
                        )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Information about valid modifications that you can make to your DB instance. Contains the result of a successful call to the <a>DescribeValidDBInstanceModifications</a> action. </p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ValidStorageOptions {
    /// <p>The valid range of Provisioned IOPS to gibibytes of storage multiplier. For example, 3-10, which means that provisioned IOPS can be between 3 and 10 times storage. </p>
    pub iops_to_storage_ratio: Option<Vec<DoubleRange>>,
    /// <p>The valid range of provisioned IOPS. For example, 1000-20000. </p>
    pub provisioned_iops: Option<Vec<Range>>,
    /// <p>The valid range of storage in gibibytes. For example, 100 to 16384. </p>
    pub storage_size: Option<Vec<Range>>,
    /// <p>The valid storage types for your DB instance. For example, gp2, io1. </p>
    pub storage_type: Option<String>,
}

struct ValidStorageOptionsDeserializer;
impl ValidStorageOptionsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ValidStorageOptions, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ValidStorageOptions::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "IopsToStorageRatio" => {
                        obj.iops_to_storage_ratio = Some(try!(
                            DoubleRangeListDeserializer::deserialize("IopsToStorageRatio", stack)
                        ));
                    }
                    "ProvisionedIops" => {
                        obj.provisioned_iops = Some(try!(RangeListDeserializer::deserialize(
                            "ProvisionedIops",
                            stack
                        )));
                    }
                    "StorageSize" => {
                        obj.storage_size = Some(try!(RangeListDeserializer::deserialize(
                            "StorageSize",
                            stack
                        )));
                    }
                    "StorageType" => {
                        obj.storage_type =
                            Some(try!(StringDeserializer::deserialize("StorageType", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct ValidStorageOptionsListDeserializer;
impl ValidStorageOptionsListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<ValidStorageOptions>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "ValidStorageOptions" {
                        obj.push(try!(ValidStorageOptionsDeserializer::deserialize(
                            "ValidStorageOptions",
                            stack
                        )));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}
struct ValidUpgradeTargetListDeserializer;
impl ValidUpgradeTargetListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<UpgradeTarget>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "UpgradeTarget" {
                        obj.push(try!(UpgradeTargetDeserializer::deserialize(
                            "UpgradeTarget",
                            stack
                        )));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}

/// Serialize `VpcSecurityGroupIdList` contents to a `SignedRequest`.
struct VpcSecurityGroupIdListSerializer;
impl VpcSecurityGroupIdListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

/// <p>This data type is used as a response element for queries on VPC security group membership.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct VpcSecurityGroupMembership {
    /// <p>The status of the VPC security group.</p>
    pub status: Option<String>,
    /// <p>The name of the VPC security group.</p>
    pub vpc_security_group_id: Option<String>,
}

struct VpcSecurityGroupMembershipDeserializer;
impl VpcSecurityGroupMembershipDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<VpcSecurityGroupMembership, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = VpcSecurityGroupMembership::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Status" => {
                        obj.status = Some(try!(StringDeserializer::deserialize("Status", stack)));
                    }
                    "VpcSecurityGroupId" => {
                        obj.vpc_security_group_id = Some(try!(StringDeserializer::deserialize(
                            "VpcSecurityGroupId",
                            stack
                        )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct VpcSecurityGroupMembershipListDeserializer;
impl VpcSecurityGroupMembershipListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<VpcSecurityGroupMembership>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "VpcSecurityGroupMembership" {
                        obj.push(try!(VpcSecurityGroupMembershipDeserializer::deserialize(
                            "VpcSecurityGroupMembership",
                            stack
                        )));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}
/// Errors returned by AddRoleToDBCluster
#[derive(Debug, PartialEq)]
pub enum AddRoleToDBClusterError {
    /// <p> <i>DBClusterIdentifier</i> does not refer to an existing DB cluster. </p>
    DBClusterNotFoundFault(String),
    /// <p>The specified IAM role Amazon Resource Name (ARN) is already associated with the specified DB cluster.</p>
    DBClusterRoleAlreadyExistsFault(String),
    /// <p>You have exceeded the maximum number of IAM roles that can be associated with the specified DB cluster.</p>
    DBClusterRoleQuotaExceededFault(String),
    /// <p>The DB cluster is not in a valid state.</p>
    InvalidDBClusterStateFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl AddRoleToDBClusterError {
    pub fn from_response(res: BufferedHttpResponse) -> AddRoleToDBClusterError {
        {
            let reader = EventReader::new(res.body.as_slice());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "DBClusterNotFoundFault" => {
                        return AddRoleToDBClusterError::DBClusterNotFoundFault(String::from(
                            parsed_error.message,
                        ))
                    }
                    "DBClusterRoleAlreadyExists" => {
                        return AddRoleToDBClusterError::DBClusterRoleAlreadyExistsFault(
                            String::from(parsed_error.message),
                        )
                    }
                    "DBClusterRoleQuotaExceeded" => {
                        return AddRoleToDBClusterError::DBClusterRoleQuotaExceededFault(
                            String::from(parsed_error.message),
                        )
                    }
                    "InvalidDBClusterStateFault" => {
                        return AddRoleToDBClusterError::InvalidDBClusterStateFault(String::from(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        AddRoleToDBClusterError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for AddRoleToDBClusterError {
    fn from(err: XmlParseError) -> AddRoleToDBClusterError {
        let XmlParseError(message) = err;
        AddRoleToDBClusterError::ParseError(message.to_string())
    }
}
impl From<CredentialsError> for AddRoleToDBClusterError {
    fn from(err: CredentialsError) -> AddRoleToDBClusterError {
        AddRoleToDBClusterError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AddRoleToDBClusterError {
    fn from(err: HttpDispatchError) -> AddRoleToDBClusterError {
        AddRoleToDBClusterError::HttpDispatch(err)
    }
}
impl From<io::Error> for AddRoleToDBClusterError {
    fn from(err: io::Error) -> AddRoleToDBClusterError {
        AddRoleToDBClusterError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for AddRoleToDBClusterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AddRoleToDBClusterError {
    fn description(&self) -> &str {
        match *self {
            AddRoleToDBClusterError::DBClusterNotFoundFault(ref cause) => cause,
            AddRoleToDBClusterError::DBClusterRoleAlreadyExistsFault(ref cause) => cause,
            AddRoleToDBClusterError::DBClusterRoleQuotaExceededFault(ref cause) => cause,
            AddRoleToDBClusterError::InvalidDBClusterStateFault(ref cause) => cause,
            AddRoleToDBClusterError::Validation(ref cause) => cause,
            AddRoleToDBClusterError::Credentials(ref err) => err.description(),
            AddRoleToDBClusterError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            AddRoleToDBClusterError::ParseError(ref cause) => cause,
            AddRoleToDBClusterError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by AddSourceIdentifierToSubscription
#[derive(Debug, PartialEq)]
pub enum AddSourceIdentifierToSubscriptionError {
    SourceNotFoundFault(String),

    SubscriptionNotFoundFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl AddSourceIdentifierToSubscriptionError {
    pub fn from_response(res: BufferedHttpResponse) -> AddSourceIdentifierToSubscriptionError {
        {
            let reader = EventReader::new(res.body.as_slice());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "SourceNotFound" => {
                        return AddSourceIdentifierToSubscriptionError::SourceNotFoundFault(
                            String::from(parsed_error.message),
                        )
                    }
                    "SubscriptionNotFound" => {
                        return AddSourceIdentifierToSubscriptionError::SubscriptionNotFoundFault(
                            String::from(parsed_error.message),
                        )
                    }
                    _ => {}
                }
            }
        }
        AddSourceIdentifierToSubscriptionError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for AddSourceIdentifierToSubscriptionError {
    fn from(err: XmlParseError) -> AddSourceIdentifierToSubscriptionError {
        let XmlParseError(message) = err;
        AddSourceIdentifierToSubscriptionError::ParseError(message.to_string())
    }
}
impl From<CredentialsError> for AddSourceIdentifierToSubscriptionError {
    fn from(err: CredentialsError) -> AddSourceIdentifierToSubscriptionError {
        AddSourceIdentifierToSubscriptionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AddSourceIdentifierToSubscriptionError {
    fn from(err: HttpDispatchError) -> AddSourceIdentifierToSubscriptionError {
        AddSourceIdentifierToSubscriptionError::HttpDispatch(err)
    }
}
impl From<io::Error> for AddSourceIdentifierToSubscriptionError {
    fn from(err: io::Error) -> AddSourceIdentifierToSubscriptionError {
        AddSourceIdentifierToSubscriptionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for AddSourceIdentifierToSubscriptionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AddSourceIdentifierToSubscriptionError {
    fn description(&self) -> &str {
        match *self {
            AddSourceIdentifierToSubscriptionError::SourceNotFoundFault(ref cause) => cause,
            AddSourceIdentifierToSubscriptionError::SubscriptionNotFoundFault(ref cause) => cause,
            AddSourceIdentifierToSubscriptionError::Validation(ref cause) => cause,
            AddSourceIdentifierToSubscriptionError::Credentials(ref err) => err.description(),
            AddSourceIdentifierToSubscriptionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            AddSourceIdentifierToSubscriptionError::ParseError(ref cause) => cause,
            AddSourceIdentifierToSubscriptionError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by AddTagsToResource
#[derive(Debug, PartialEq)]
pub enum AddTagsToResourceError {
    /// <p> <i>DBClusterIdentifier</i> does not refer to an existing DB cluster. </p>
    DBClusterNotFoundFault(String),
    /// <p> <i>DBInstanceIdentifier</i> does not refer to an existing DB instance. </p>
    DBInstanceNotFoundFault(String),
    /// <p> <i>DBSnapshotIdentifier</i> does not refer to an existing DB snapshot. </p>
    DBSnapshotNotFoundFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl AddTagsToResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> AddTagsToResourceError {
        {
            let reader = EventReader::new(res.body.as_slice());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "DBClusterNotFoundFault" => {
                        return AddTagsToResourceError::DBClusterNotFoundFault(String::from(
                            parsed_error.message,
                        ))
                    }
                    "DBInstanceNotFound" => {
                        return AddTagsToResourceError::DBInstanceNotFoundFault(String::from(
                            parsed_error.message,
                        ))
                    }
                    "DBSnapshotNotFound" => {
                        return AddTagsToResourceError::DBSnapshotNotFoundFault(String::from(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        AddTagsToResourceError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for AddTagsToResourceError {
    fn from(err: XmlParseError) -> AddTagsToResourceError {
        let XmlParseError(message) = err;
        AddTagsToResourceError::ParseError(message.to_string())
    }
}
impl From<CredentialsError> for AddTagsToResourceError {
    fn from(err: CredentialsError) -> AddTagsToResourceError {
        AddTagsToResourceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AddTagsToResourceError {
    fn from(err: HttpDispatchError) -> AddTagsToResourceError {
        AddTagsToResourceError::HttpDispatch(err)
    }
}
impl From<io::Error> for AddTagsToResourceError {
    fn from(err: io::Error) -> AddTagsToResourceError {
        AddTagsToResourceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for AddTagsToResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AddTagsToResourceError {
    fn description(&self) -> &str {
        match *self {
            AddTagsToResourceError::DBClusterNotFoundFault(ref cause) => cause,
            AddTagsToResourceError::DBInstanceNotFoundFault(ref cause) => cause,
            AddTagsToResourceError::DBSnapshotNotFoundFault(ref cause) => cause,
            AddTagsToResourceError::Validation(ref cause) => cause,
            AddTagsToResourceError::Credentials(ref err) => err.description(),
            AddTagsToResourceError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            AddTagsToResourceError::ParseError(ref cause) => cause,
            AddTagsToResourceError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ApplyPendingMaintenanceAction
#[derive(Debug, PartialEq)]
pub enum ApplyPendingMaintenanceActionError {
    /// <p>The specified resource ID was not found.</p>
    ResourceNotFoundFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl ApplyPendingMaintenanceActionError {
    pub fn from_response(res: BufferedHttpResponse) -> ApplyPendingMaintenanceActionError {
        {
            let reader = EventReader::new(res.body.as_slice());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "ResourceNotFoundFault" => {
                        return ApplyPendingMaintenanceActionError::ResourceNotFoundFault(
                            String::from(parsed_error.message),
                        )
                    }
                    _ => {}
                }
            }
        }
        ApplyPendingMaintenanceActionError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for ApplyPendingMaintenanceActionError {
    fn from(err: XmlParseError) -> ApplyPendingMaintenanceActionError {
        let XmlParseError(message) = err;
        ApplyPendingMaintenanceActionError::ParseError(message.to_string())
    }
}
impl From<CredentialsError> for ApplyPendingMaintenanceActionError {
    fn from(err: CredentialsError) -> ApplyPendingMaintenanceActionError {
        ApplyPendingMaintenanceActionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ApplyPendingMaintenanceActionError {
    fn from(err: HttpDispatchError) -> ApplyPendingMaintenanceActionError {
        ApplyPendingMaintenanceActionError::HttpDispatch(err)
    }
}
impl From<io::Error> for ApplyPendingMaintenanceActionError {
    fn from(err: io::Error) -> ApplyPendingMaintenanceActionError {
        ApplyPendingMaintenanceActionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ApplyPendingMaintenanceActionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ApplyPendingMaintenanceActionError {
    fn description(&self) -> &str {
        match *self {
            ApplyPendingMaintenanceActionError::ResourceNotFoundFault(ref cause) => cause,
            ApplyPendingMaintenanceActionError::Validation(ref cause) => cause,
            ApplyPendingMaintenanceActionError::Credentials(ref err) => err.description(),
            ApplyPendingMaintenanceActionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ApplyPendingMaintenanceActionError::ParseError(ref cause) => cause,
            ApplyPendingMaintenanceActionError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by CopyDBClusterParameterGroup
#[derive(Debug, PartialEq)]
pub enum CopyDBClusterParameterGroupError {
    /// <p>A DB parameter group with the same name exists.</p>
    DBParameterGroupAlreadyExistsFault(String),
    /// <p> <i>DBParameterGroupName</i> does not refer to an existing DB parameter group. </p>
    DBParameterGroupNotFoundFault(String),
    /// <p>Request would result in user exceeding the allowed number of DB parameter groups.</p>
    DBParameterGroupQuotaExceededFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl CopyDBClusterParameterGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> CopyDBClusterParameterGroupError {
        {
            let reader = EventReader::new(res.body.as_slice());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "DBParameterGroupAlreadyExists" => {
                        return CopyDBClusterParameterGroupError::DBParameterGroupAlreadyExistsFault(
                            String::from(parsed_error.message),
                        )
                    }
                    "DBParameterGroupNotFound" => {
                        return CopyDBClusterParameterGroupError::DBParameterGroupNotFoundFault(
                            String::from(parsed_error.message),
                        )
                    }
                    "DBParameterGroupQuotaExceeded" => {
                        return CopyDBClusterParameterGroupError::DBParameterGroupQuotaExceededFault(
                            String::from(parsed_error.message),
                        )
                    }
                    _ => {}
                }
            }
        }
        CopyDBClusterParameterGroupError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for CopyDBClusterParameterGroupError {
    fn from(err: XmlParseError) -> CopyDBClusterParameterGroupError {
        let XmlParseError(message) = err;
        CopyDBClusterParameterGroupError::ParseError(message.to_string())
    }
}
impl From<CredentialsError> for CopyDBClusterParameterGroupError {
    fn from(err: CredentialsError) -> CopyDBClusterParameterGroupError {
        CopyDBClusterParameterGroupError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CopyDBClusterParameterGroupError {
    fn from(err: HttpDispatchError) -> CopyDBClusterParameterGroupError {
        CopyDBClusterParameterGroupError::HttpDispatch(err)
    }
}
impl From<io::Error> for CopyDBClusterParameterGroupError {
    fn from(err: io::Error) -> CopyDBClusterParameterGroupError {
        CopyDBClusterParameterGroupError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CopyDBClusterParameterGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CopyDBClusterParameterGroupError {
    fn description(&self) -> &str {
        match *self {
            CopyDBClusterParameterGroupError::DBParameterGroupAlreadyExistsFault(ref cause) => {
                cause
            }
            CopyDBClusterParameterGroupError::DBParameterGroupNotFoundFault(ref cause) => cause,
            CopyDBClusterParameterGroupError::DBParameterGroupQuotaExceededFault(ref cause) => {
                cause
            }
            CopyDBClusterParameterGroupError::Validation(ref cause) => cause,
            CopyDBClusterParameterGroupError::Credentials(ref err) => err.description(),
            CopyDBClusterParameterGroupError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CopyDBClusterParameterGroupError::ParseError(ref cause) => cause,
            CopyDBClusterParameterGroupError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by CopyDBClusterSnapshot
#[derive(Debug, PartialEq)]
pub enum CopyDBClusterSnapshotError {
    /// <p>User already has a DB cluster snapshot with the given identifier.</p>
    DBClusterSnapshotAlreadyExistsFault(String),
    /// <p> <i>DBClusterSnapshotIdentifier</i> does not refer to an existing DB cluster snapshot. </p>
    DBClusterSnapshotNotFoundFault(String),
    /// <p>The supplied value is not a valid DB cluster snapshot state.</p>
    InvalidDBClusterSnapshotStateFault(String),
    /// <p>The DB cluster is not in a valid state.</p>
    InvalidDBClusterStateFault(String),
    /// <p>Error accessing KMS key.</p>
    KMSKeyNotAccessibleFault(String),
    /// <p>Request would result in user exceeding the allowed number of DB snapshots.</p>
    SnapshotQuotaExceededFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl CopyDBClusterSnapshotError {
    pub fn from_response(res: BufferedHttpResponse) -> CopyDBClusterSnapshotError {
        {
            let reader = EventReader::new(res.body.as_slice());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "DBClusterSnapshotAlreadyExistsFault" => {
                        return CopyDBClusterSnapshotError::DBClusterSnapshotAlreadyExistsFault(
                            String::from(parsed_error.message),
                        )
                    }
                    "DBClusterSnapshotNotFoundFault" => {
                        return CopyDBClusterSnapshotError::DBClusterSnapshotNotFoundFault(
                            String::from(parsed_error.message),
                        )
                    }
                    "InvalidDBClusterSnapshotStateFault" => {
                        return CopyDBClusterSnapshotError::InvalidDBClusterSnapshotStateFault(
                            String::from(parsed_error.message),
                        )
                    }
                    "InvalidDBClusterStateFault" => {
                        return CopyDBClusterSnapshotError::InvalidDBClusterStateFault(String::from(
                            parsed_error.message,
                        ))
                    }
                    "KMSKeyNotAccessibleFault" => {
                        return CopyDBClusterSnapshotError::KMSKeyNotAccessibleFault(String::from(
                            parsed_error.message,
                        ))
                    }
                    "SnapshotQuotaExceeded" => {
                        return CopyDBClusterSnapshotError::SnapshotQuotaExceededFault(String::from(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        CopyDBClusterSnapshotError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for CopyDBClusterSnapshotError {
    fn from(err: XmlParseError) -> CopyDBClusterSnapshotError {
        let XmlParseError(message) = err;
        CopyDBClusterSnapshotError::ParseError(message.to_string())
    }
}
impl From<CredentialsError> for CopyDBClusterSnapshotError {
    fn from(err: CredentialsError) -> CopyDBClusterSnapshotError {
        CopyDBClusterSnapshotError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CopyDBClusterSnapshotError {
    fn from(err: HttpDispatchError) -> CopyDBClusterSnapshotError {
        CopyDBClusterSnapshotError::HttpDispatch(err)
    }
}
impl From<io::Error> for CopyDBClusterSnapshotError {
    fn from(err: io::Error) -> CopyDBClusterSnapshotError {
        CopyDBClusterSnapshotError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CopyDBClusterSnapshotError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CopyDBClusterSnapshotError {
    fn description(&self) -> &str {
        match *self {
            CopyDBClusterSnapshotError::DBClusterSnapshotAlreadyExistsFault(ref cause) => cause,
            CopyDBClusterSnapshotError::DBClusterSnapshotNotFoundFault(ref cause) => cause,
            CopyDBClusterSnapshotError::InvalidDBClusterSnapshotStateFault(ref cause) => cause,
            CopyDBClusterSnapshotError::InvalidDBClusterStateFault(ref cause) => cause,
            CopyDBClusterSnapshotError::KMSKeyNotAccessibleFault(ref cause) => cause,
            CopyDBClusterSnapshotError::SnapshotQuotaExceededFault(ref cause) => cause,
            CopyDBClusterSnapshotError::Validation(ref cause) => cause,
            CopyDBClusterSnapshotError::Credentials(ref err) => err.description(),
            CopyDBClusterSnapshotError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CopyDBClusterSnapshotError::ParseError(ref cause) => cause,
            CopyDBClusterSnapshotError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by CopyDBParameterGroup
#[derive(Debug, PartialEq)]
pub enum CopyDBParameterGroupError {
    /// <p>A DB parameter group with the same name exists.</p>
    DBParameterGroupAlreadyExistsFault(String),
    /// <p> <i>DBParameterGroupName</i> does not refer to an existing DB parameter group. </p>
    DBParameterGroupNotFoundFault(String),
    /// <p>Request would result in user exceeding the allowed number of DB parameter groups.</p>
    DBParameterGroupQuotaExceededFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl CopyDBParameterGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> CopyDBParameterGroupError {
        {
            let reader = EventReader::new(res.body.as_slice());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "DBParameterGroupAlreadyExists" => {
                        return CopyDBParameterGroupError::DBParameterGroupAlreadyExistsFault(
                            String::from(parsed_error.message),
                        )
                    }
                    "DBParameterGroupNotFound" => {
                        return CopyDBParameterGroupError::DBParameterGroupNotFoundFault(
                            String::from(parsed_error.message),
                        )
                    }
                    "DBParameterGroupQuotaExceeded" => {
                        return CopyDBParameterGroupError::DBParameterGroupQuotaExceededFault(
                            String::from(parsed_error.message),
                        )
                    }
                    _ => {}
                }
            }
        }
        CopyDBParameterGroupError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for CopyDBParameterGroupError {
    fn from(err: XmlParseError) -> CopyDBParameterGroupError {
        let XmlParseError(message) = err;
        CopyDBParameterGroupError::ParseError(message.to_string())
    }
}
impl From<CredentialsError> for CopyDBParameterGroupError {
    fn from(err: CredentialsError) -> CopyDBParameterGroupError {
        CopyDBParameterGroupError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CopyDBParameterGroupError {
    fn from(err: HttpDispatchError) -> CopyDBParameterGroupError {
        CopyDBParameterGroupError::HttpDispatch(err)
    }
}
impl From<io::Error> for CopyDBParameterGroupError {
    fn from(err: io::Error) -> CopyDBParameterGroupError {
        CopyDBParameterGroupError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CopyDBParameterGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CopyDBParameterGroupError {
    fn description(&self) -> &str {
        match *self {
            CopyDBParameterGroupError::DBParameterGroupAlreadyExistsFault(ref cause) => cause,
            CopyDBParameterGroupError::DBParameterGroupNotFoundFault(ref cause) => cause,
            CopyDBParameterGroupError::DBParameterGroupQuotaExceededFault(ref cause) => cause,
            CopyDBParameterGroupError::Validation(ref cause) => cause,
            CopyDBParameterGroupError::Credentials(ref err) => err.description(),
            CopyDBParameterGroupError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CopyDBParameterGroupError::ParseError(ref cause) => cause,
            CopyDBParameterGroupError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by CreateDBCluster
#[derive(Debug, PartialEq)]
pub enum CreateDBClusterError {
    /// <p>User already has a DB cluster with the given identifier.</p>
    DBClusterAlreadyExistsFault(String),
    /// <p> <i>DBClusterIdentifier</i> does not refer to an existing DB cluster. </p>
    DBClusterNotFoundFault(String),
    /// <p> <i>DBClusterParameterGroupName</i> does not refer to an existing DB Cluster parameter group. </p>
    DBClusterParameterGroupNotFoundFault(String),
    /// <p>User attempted to create a new DB cluster and the user has already reached the maximum allowed DB cluster quota.</p>
    DBClusterQuotaExceededFault(String),
    /// <p> <i>DBInstanceIdentifier</i> does not refer to an existing DB instance. </p>
    DBInstanceNotFoundFault(String),
    /// <p>Subnets in the DB subnet group should cover at least two Availability Zones unless there is only one Availability Zone.</p>
    DBSubnetGroupDoesNotCoverEnoughAZs(String),
    /// <p> <i>DBSubnetGroupName</i> does not refer to an existing DB subnet group. </p>
    DBSubnetGroupNotFoundFault(String),
    /// <p>There is insufficient storage available for the current action. You may be able to resolve this error by updating your subnet group to use different Availability Zones that have more storage available.</p>
    InsufficientStorageClusterCapacityFault(String),
    /// <p>The DB cluster is not in a valid state.</p>
    InvalidDBClusterStateFault(String),
    /// <p> The specified DB instance is not in the <i>available</i> state. </p>
    InvalidDBInstanceStateFault(String),
    /// <p>The DB subnet group cannot be deleted because it is in use.</p>
    InvalidDBSubnetGroupStateFault(String),
    /// <p>The requested subnet is invalid, or multiple subnets were requested that are not all in a common VPC.</p>
    InvalidSubnet(String),
    /// <p>DB subnet group does not cover all Availability Zones after it is created because users' change.</p>
    InvalidVPCNetworkStateFault(String),
    /// <p>Error accessing KMS key.</p>
    KMSKeyNotAccessibleFault(String),
    /// <p>Request would result in user exceeding the allowed amount of storage available across all DB instances.</p>
    StorageQuotaExceededFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl CreateDBClusterError {
    pub fn from_response(res: BufferedHttpResponse) -> CreateDBClusterError {
        {
            let reader = EventReader::new(res.body.as_slice());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "DBClusterAlreadyExistsFault" => {
                        return CreateDBClusterError::DBClusterAlreadyExistsFault(String::from(
                            parsed_error.message,
                        ))
                    }
                    "DBClusterNotFoundFault" => {
                        return CreateDBClusterError::DBClusterNotFoundFault(String::from(
                            parsed_error.message,
                        ))
                    }
                    "DBClusterParameterGroupNotFound" => {
                        return CreateDBClusterError::DBClusterParameterGroupNotFoundFault(
                            String::from(parsed_error.message),
                        )
                    }
                    "DBClusterQuotaExceededFault" => {
                        return CreateDBClusterError::DBClusterQuotaExceededFault(String::from(
                            parsed_error.message,
                        ))
                    }
                    "DBInstanceNotFound" => {
                        return CreateDBClusterError::DBInstanceNotFoundFault(String::from(
                            parsed_error.message,
                        ))
                    }
                    "DBSubnetGroupDoesNotCoverEnoughAZs" => {
                        return CreateDBClusterError::DBSubnetGroupDoesNotCoverEnoughAZs(
                            String::from(parsed_error.message),
                        )
                    }
                    "DBSubnetGroupNotFoundFault" => {
                        return CreateDBClusterError::DBSubnetGroupNotFoundFault(String::from(
                            parsed_error.message,
                        ))
                    }
                    "InsufficientStorageClusterCapacity" => {
                        return CreateDBClusterError::InsufficientStorageClusterCapacityFault(
                            String::from(parsed_error.message),
                        )
                    }
                    "InvalidDBClusterStateFault" => {
                        return CreateDBClusterError::InvalidDBClusterStateFault(String::from(
                            parsed_error.message,
                        ))
                    }
                    "InvalidDBInstanceState" => {
                        return CreateDBClusterError::InvalidDBInstanceStateFault(String::from(
                            parsed_error.message,
                        ))
                    }
                    "InvalidDBSubnetGroupStateFault" => {
                        return CreateDBClusterError::InvalidDBSubnetGroupStateFault(String::from(
                            parsed_error.message,
                        ))
                    }
                    "InvalidSubnet" => {
                        return CreateDBClusterError::InvalidSubnet(String::from(
                            parsed_error.message,
                        ))
                    }
                    "InvalidVPCNetworkStateFault" => {
                        return CreateDBClusterError::InvalidVPCNetworkStateFault(String::from(
                            parsed_error.message,
                        ))
                    }
                    "KMSKeyNotAccessibleFault" => {
                        return CreateDBClusterError::KMSKeyNotAccessibleFault(String::from(
                            parsed_error.message,
                        ))
                    }
                    "StorageQuotaExceeded" => {
                        return CreateDBClusterError::StorageQuotaExceededFault(String::from(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        CreateDBClusterError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for CreateDBClusterError {
    fn from(err: XmlParseError) -> CreateDBClusterError {
        let XmlParseError(message) = err;
        CreateDBClusterError::ParseError(message.to_string())
    }
}
impl From<CredentialsError> for CreateDBClusterError {
    fn from(err: CredentialsError) -> CreateDBClusterError {
        CreateDBClusterError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateDBClusterError {
    fn from(err: HttpDispatchError) -> CreateDBClusterError {
        CreateDBClusterError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateDBClusterError {
    fn from(err: io::Error) -> CreateDBClusterError {
        CreateDBClusterError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateDBClusterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateDBClusterError {
    fn description(&self) -> &str {
        match *self {
            CreateDBClusterError::DBClusterAlreadyExistsFault(ref cause) => cause,
            CreateDBClusterError::DBClusterNotFoundFault(ref cause) => cause,
            CreateDBClusterError::DBClusterParameterGroupNotFoundFault(ref cause) => cause,
            CreateDBClusterError::DBClusterQuotaExceededFault(ref cause) => cause,
            CreateDBClusterError::DBInstanceNotFoundFault(ref cause) => cause,
            CreateDBClusterError::DBSubnetGroupDoesNotCoverEnoughAZs(ref cause) => cause,
            CreateDBClusterError::DBSubnetGroupNotFoundFault(ref cause) => cause,
            CreateDBClusterError::InsufficientStorageClusterCapacityFault(ref cause) => cause,
            CreateDBClusterError::InvalidDBClusterStateFault(ref cause) => cause,
            CreateDBClusterError::InvalidDBInstanceStateFault(ref cause) => cause,
            CreateDBClusterError::InvalidDBSubnetGroupStateFault(ref cause) => cause,
            CreateDBClusterError::InvalidSubnet(ref cause) => cause,
            CreateDBClusterError::InvalidVPCNetworkStateFault(ref cause) => cause,
            CreateDBClusterError::KMSKeyNotAccessibleFault(ref cause) => cause,
            CreateDBClusterError::StorageQuotaExceededFault(ref cause) => cause,
            CreateDBClusterError::Validation(ref cause) => cause,
            CreateDBClusterError::Credentials(ref err) => err.description(),
            CreateDBClusterError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateDBClusterError::ParseError(ref cause) => cause,
            CreateDBClusterError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by CreateDBClusterParameterGroup
#[derive(Debug, PartialEq)]
pub enum CreateDBClusterParameterGroupError {
    /// <p>A DB parameter group with the same name exists.</p>
    DBParameterGroupAlreadyExistsFault(String),
    /// <p>Request would result in user exceeding the allowed number of DB parameter groups.</p>
    DBParameterGroupQuotaExceededFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl CreateDBClusterParameterGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> CreateDBClusterParameterGroupError {
        {
            let reader = EventReader::new(res.body.as_slice());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                                    "DBParameterGroupAlreadyExists" => return CreateDBClusterParameterGroupError::DBParameterGroupAlreadyExistsFault(String::from(parsed_error.message)),"DBParameterGroupQuotaExceeded" => return CreateDBClusterParameterGroupError::DBParameterGroupQuotaExceededFault(String::from(parsed_error.message)),_ => {}
                                }
            }
        }
        CreateDBClusterParameterGroupError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for CreateDBClusterParameterGroupError {
    fn from(err: XmlParseError) -> CreateDBClusterParameterGroupError {
        let XmlParseError(message) = err;
        CreateDBClusterParameterGroupError::ParseError(message.to_string())
    }
}
impl From<CredentialsError> for CreateDBClusterParameterGroupError {
    fn from(err: CredentialsError) -> CreateDBClusterParameterGroupError {
        CreateDBClusterParameterGroupError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateDBClusterParameterGroupError {
    fn from(err: HttpDispatchError) -> CreateDBClusterParameterGroupError {
        CreateDBClusterParameterGroupError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateDBClusterParameterGroupError {
    fn from(err: io::Error) -> CreateDBClusterParameterGroupError {
        CreateDBClusterParameterGroupError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateDBClusterParameterGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateDBClusterParameterGroupError {
    fn description(&self) -> &str {
        match *self {
            CreateDBClusterParameterGroupError::DBParameterGroupAlreadyExistsFault(ref cause) => {
                cause
            }
            CreateDBClusterParameterGroupError::DBParameterGroupQuotaExceededFault(ref cause) => {
                cause
            }
            CreateDBClusterParameterGroupError::Validation(ref cause) => cause,
            CreateDBClusterParameterGroupError::Credentials(ref err) => err.description(),
            CreateDBClusterParameterGroupError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateDBClusterParameterGroupError::ParseError(ref cause) => cause,
            CreateDBClusterParameterGroupError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by CreateDBClusterSnapshot
#[derive(Debug, PartialEq)]
pub enum CreateDBClusterSnapshotError {
    /// <p> <i>DBClusterIdentifier</i> does not refer to an existing DB cluster. </p>
    DBClusterNotFoundFault(String),
    /// <p>User already has a DB cluster snapshot with the given identifier.</p>
    DBClusterSnapshotAlreadyExistsFault(String),
    /// <p>The supplied value is not a valid DB cluster snapshot state.</p>
    InvalidDBClusterSnapshotStateFault(String),
    /// <p>The DB cluster is not in a valid state.</p>
    InvalidDBClusterStateFault(String),
    /// <p>Request would result in user exceeding the allowed number of DB snapshots.</p>
    SnapshotQuotaExceededFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl CreateDBClusterSnapshotError {
    pub fn from_response(res: BufferedHttpResponse) -> CreateDBClusterSnapshotError {
        {
            let reader = EventReader::new(res.body.as_slice());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "DBClusterNotFoundFault" => {
                        return CreateDBClusterSnapshotError::DBClusterNotFoundFault(String::from(
                            parsed_error.message,
                        ))
                    }
                    "DBClusterSnapshotAlreadyExistsFault" => {
                        return CreateDBClusterSnapshotError::DBClusterSnapshotAlreadyExistsFault(
                            String::from(parsed_error.message),
                        )
                    }
                    "InvalidDBClusterSnapshotStateFault" => {
                        return CreateDBClusterSnapshotError::InvalidDBClusterSnapshotStateFault(
                            String::from(parsed_error.message),
                        )
                    }
                    "InvalidDBClusterStateFault" => {
                        return CreateDBClusterSnapshotError::InvalidDBClusterStateFault(
                            String::from(parsed_error.message),
                        )
                    }
                    "SnapshotQuotaExceeded" => {
                        return CreateDBClusterSnapshotError::SnapshotQuotaExceededFault(
                            String::from(parsed_error.message),
                        )
                    }
                    _ => {}
                }
            }
        }
        CreateDBClusterSnapshotError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for CreateDBClusterSnapshotError {
    fn from(err: XmlParseError) -> CreateDBClusterSnapshotError {
        let XmlParseError(message) = err;
        CreateDBClusterSnapshotError::ParseError(message.to_string())
    }
}
impl From<CredentialsError> for CreateDBClusterSnapshotError {
    fn from(err: CredentialsError) -> CreateDBClusterSnapshotError {
        CreateDBClusterSnapshotError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateDBClusterSnapshotError {
    fn from(err: HttpDispatchError) -> CreateDBClusterSnapshotError {
        CreateDBClusterSnapshotError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateDBClusterSnapshotError {
    fn from(err: io::Error) -> CreateDBClusterSnapshotError {
        CreateDBClusterSnapshotError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateDBClusterSnapshotError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateDBClusterSnapshotError {
    fn description(&self) -> &str {
        match *self {
            CreateDBClusterSnapshotError::DBClusterNotFoundFault(ref cause) => cause,
            CreateDBClusterSnapshotError::DBClusterSnapshotAlreadyExistsFault(ref cause) => cause,
            CreateDBClusterSnapshotError::InvalidDBClusterSnapshotStateFault(ref cause) => cause,
            CreateDBClusterSnapshotError::InvalidDBClusterStateFault(ref cause) => cause,
            CreateDBClusterSnapshotError::SnapshotQuotaExceededFault(ref cause) => cause,
            CreateDBClusterSnapshotError::Validation(ref cause) => cause,
            CreateDBClusterSnapshotError::Credentials(ref err) => err.description(),
            CreateDBClusterSnapshotError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateDBClusterSnapshotError::ParseError(ref cause) => cause,
            CreateDBClusterSnapshotError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by CreateDBInstance
#[derive(Debug, PartialEq)]
pub enum CreateDBInstanceError {
    /// <p>Specified CIDRIP or EC2 security group is not authorized for the specified DB security group.</p> <p>Neptune may not also be authorized via IAM to perform necessary actions on your behalf.</p>
    AuthorizationNotFoundFault(String),
    /// <p> <i>DBClusterIdentifier</i> does not refer to an existing DB cluster. </p>
    DBClusterNotFoundFault(String),
    /// <p>User already has a DB instance with the given identifier.</p>
    DBInstanceAlreadyExistsFault(String),
    /// <p> <i>DBParameterGroupName</i> does not refer to an existing DB parameter group. </p>
    DBParameterGroupNotFoundFault(String),
    /// <p> <i>DBSecurityGroupName</i> does not refer to an existing DB security group. </p>
    DBSecurityGroupNotFoundFault(String),
    /// <p>Subnets in the DB subnet group should cover at least two Availability Zones unless there is only one Availability Zone.</p>
    DBSubnetGroupDoesNotCoverEnoughAZs(String),
    /// <p> <i>DBSubnetGroupName</i> does not refer to an existing DB subnet group. </p>
    DBSubnetGroupNotFoundFault(String),
    /// <p> <i>Domain</i> does not refer to an existing Active Directory Domain. </p>
    DomainNotFoundFault(String),
    /// <p>Request would result in user exceeding the allowed number of DB instances.</p>
    InstanceQuotaExceededFault(String),
    /// <p>Specified DB instance class is not available in the specified Availability Zone.</p>
    InsufficientDBInstanceCapacityFault(String),
    /// <p>The DB cluster is not in a valid state.</p>
    InvalidDBClusterStateFault(String),
    /// <p>The requested subnet is invalid, or multiple subnets were requested that are not all in a common VPC.</p>
    InvalidSubnet(String),
    /// <p>DB subnet group does not cover all Availability Zones after it is created because users' change.</p>
    InvalidVPCNetworkStateFault(String),
    /// <p>Error accessing KMS key.</p>
    KMSKeyNotAccessibleFault(String),

    OptionGroupNotFoundFault(String),
    /// <p>Provisioned IOPS not available in the specified Availability Zone.</p>
    ProvisionedIopsNotAvailableInAZFault(String),
    /// <p>Request would result in user exceeding the allowed amount of storage available across all DB instances.</p>
    StorageQuotaExceededFault(String),
    /// <p> <i>StorageType</i> specified cannot be associated with the DB Instance. </p>
    StorageTypeNotSupportedFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl CreateDBInstanceError {
    pub fn from_response(res: BufferedHttpResponse) -> CreateDBInstanceError {
        {
            let reader = EventReader::new(res.body.as_slice());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "AuthorizationNotFound" => {
                        return CreateDBInstanceError::AuthorizationNotFoundFault(String::from(
                            parsed_error.message,
                        ))
                    }
                    "DBClusterNotFoundFault" => {
                        return CreateDBInstanceError::DBClusterNotFoundFault(String::from(
                            parsed_error.message,
                        ))
                    }
                    "DBInstanceAlreadyExists" => {
                        return CreateDBInstanceError::DBInstanceAlreadyExistsFault(String::from(
                            parsed_error.message,
                        ))
                    }
                    "DBParameterGroupNotFound" => {
                        return CreateDBInstanceError::DBParameterGroupNotFoundFault(String::from(
                            parsed_error.message,
                        ))
                    }
                    "DBSecurityGroupNotFound" => {
                        return CreateDBInstanceError::DBSecurityGroupNotFoundFault(String::from(
                            parsed_error.message,
                        ))
                    }
                    "DBSubnetGroupDoesNotCoverEnoughAZs" => {
                        return CreateDBInstanceError::DBSubnetGroupDoesNotCoverEnoughAZs(
                            String::from(parsed_error.message),
                        )
                    }
                    "DBSubnetGroupNotFoundFault" => {
                        return CreateDBInstanceError::DBSubnetGroupNotFoundFault(String::from(
                            parsed_error.message,
                        ))
                    }
                    "DomainNotFoundFault" => {
                        return CreateDBInstanceError::DomainNotFoundFault(String::from(
                            parsed_error.message,
                        ))
                    }
                    "InstanceQuotaExceeded" => {
                        return CreateDBInstanceError::InstanceQuotaExceededFault(String::from(
                            parsed_error.message,
                        ))
                    }
                    "InsufficientDBInstanceCapacity" => {
                        return CreateDBInstanceError::InsufficientDBInstanceCapacityFault(
                            String::from(parsed_error.message),
                        )
                    }
                    "InvalidDBClusterStateFault" => {
                        return CreateDBInstanceError::InvalidDBClusterStateFault(String::from(
                            parsed_error.message,
                        ))
                    }
                    "InvalidSubnet" => {
                        return CreateDBInstanceError::InvalidSubnet(String::from(
                            parsed_error.message,
                        ))
                    }
                    "InvalidVPCNetworkStateFault" => {
                        return CreateDBInstanceError::InvalidVPCNetworkStateFault(String::from(
                            parsed_error.message,
                        ))
                    }
                    "KMSKeyNotAccessibleFault" => {
                        return CreateDBInstanceError::KMSKeyNotAccessibleFault(String::from(
                            parsed_error.message,
                        ))
                    }
                    "OptionGroupNotFoundFault" => {
                        return CreateDBInstanceError::OptionGroupNotFoundFault(String::from(
                            parsed_error.message,
                        ))
                    }
                    "ProvisionedIopsNotAvailableInAZFault" => {
                        return CreateDBInstanceError::ProvisionedIopsNotAvailableInAZFault(
                            String::from(parsed_error.message),
                        )
                    }
                    "StorageQuotaExceeded" => {
                        return CreateDBInstanceError::StorageQuotaExceededFault(String::from(
                            parsed_error.message,
                        ))
                    }
                    "StorageTypeNotSupported" => {
                        return CreateDBInstanceError::StorageTypeNotSupportedFault(String::from(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        CreateDBInstanceError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for CreateDBInstanceError {
    fn from(err: XmlParseError) -> CreateDBInstanceError {
        let XmlParseError(message) = err;
        CreateDBInstanceError::ParseError(message.to_string())
    }
}
impl From<CredentialsError> for CreateDBInstanceError {
    fn from(err: CredentialsError) -> CreateDBInstanceError {
        CreateDBInstanceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateDBInstanceError {
    fn from(err: HttpDispatchError) -> CreateDBInstanceError {
        CreateDBInstanceError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateDBInstanceError {
    fn from(err: io::Error) -> CreateDBInstanceError {
        CreateDBInstanceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateDBInstanceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateDBInstanceError {
    fn description(&self) -> &str {
        match *self {
            CreateDBInstanceError::AuthorizationNotFoundFault(ref cause) => cause,
            CreateDBInstanceError::DBClusterNotFoundFault(ref cause) => cause,
            CreateDBInstanceError::DBInstanceAlreadyExistsFault(ref cause) => cause,
            CreateDBInstanceError::DBParameterGroupNotFoundFault(ref cause) => cause,
            CreateDBInstanceError::DBSecurityGroupNotFoundFault(ref cause) => cause,
            CreateDBInstanceError::DBSubnetGroupDoesNotCoverEnoughAZs(ref cause) => cause,
            CreateDBInstanceError::DBSubnetGroupNotFoundFault(ref cause) => cause,
            CreateDBInstanceError::DomainNotFoundFault(ref cause) => cause,
            CreateDBInstanceError::InstanceQuotaExceededFault(ref cause) => cause,
            CreateDBInstanceError::InsufficientDBInstanceCapacityFault(ref cause) => cause,
            CreateDBInstanceError::InvalidDBClusterStateFault(ref cause) => cause,
            CreateDBInstanceError::InvalidSubnet(ref cause) => cause,
            CreateDBInstanceError::InvalidVPCNetworkStateFault(ref cause) => cause,
            CreateDBInstanceError::KMSKeyNotAccessibleFault(ref cause) => cause,
            CreateDBInstanceError::OptionGroupNotFoundFault(ref cause) => cause,
            CreateDBInstanceError::ProvisionedIopsNotAvailableInAZFault(ref cause) => cause,
            CreateDBInstanceError::StorageQuotaExceededFault(ref cause) => cause,
            CreateDBInstanceError::StorageTypeNotSupportedFault(ref cause) => cause,
            CreateDBInstanceError::Validation(ref cause) => cause,
            CreateDBInstanceError::Credentials(ref err) => err.description(),
            CreateDBInstanceError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateDBInstanceError::ParseError(ref cause) => cause,
            CreateDBInstanceError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by CreateDBParameterGroup
#[derive(Debug, PartialEq)]
pub enum CreateDBParameterGroupError {
    /// <p>A DB parameter group with the same name exists.</p>
    DBParameterGroupAlreadyExistsFault(String),
    /// <p>Request would result in user exceeding the allowed number of DB parameter groups.</p>
    DBParameterGroupQuotaExceededFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl CreateDBParameterGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> CreateDBParameterGroupError {
        {
            let reader = EventReader::new(res.body.as_slice());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "DBParameterGroupAlreadyExists" => {
                        return CreateDBParameterGroupError::DBParameterGroupAlreadyExistsFault(
                            String::from(parsed_error.message),
                        )
                    }
                    "DBParameterGroupQuotaExceeded" => {
                        return CreateDBParameterGroupError::DBParameterGroupQuotaExceededFault(
                            String::from(parsed_error.message),
                        )
                    }
                    _ => {}
                }
            }
        }
        CreateDBParameterGroupError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for CreateDBParameterGroupError {
    fn from(err: XmlParseError) -> CreateDBParameterGroupError {
        let XmlParseError(message) = err;
        CreateDBParameterGroupError::ParseError(message.to_string())
    }
}
impl From<CredentialsError> for CreateDBParameterGroupError {
    fn from(err: CredentialsError) -> CreateDBParameterGroupError {
        CreateDBParameterGroupError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateDBParameterGroupError {
    fn from(err: HttpDispatchError) -> CreateDBParameterGroupError {
        CreateDBParameterGroupError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateDBParameterGroupError {
    fn from(err: io::Error) -> CreateDBParameterGroupError {
        CreateDBParameterGroupError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateDBParameterGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateDBParameterGroupError {
    fn description(&self) -> &str {
        match *self {
            CreateDBParameterGroupError::DBParameterGroupAlreadyExistsFault(ref cause) => cause,
            CreateDBParameterGroupError::DBParameterGroupQuotaExceededFault(ref cause) => cause,
            CreateDBParameterGroupError::Validation(ref cause) => cause,
            CreateDBParameterGroupError::Credentials(ref err) => err.description(),
            CreateDBParameterGroupError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateDBParameterGroupError::ParseError(ref cause) => cause,
            CreateDBParameterGroupError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by CreateDBSubnetGroup
#[derive(Debug, PartialEq)]
pub enum CreateDBSubnetGroupError {
    /// <p> <i>DBSubnetGroupName</i> is already used by an existing DB subnet group. </p>
    DBSubnetGroupAlreadyExistsFault(String),
    /// <p>Subnets in the DB subnet group should cover at least two Availability Zones unless there is only one Availability Zone.</p>
    DBSubnetGroupDoesNotCoverEnoughAZs(String),
    /// <p>Request would result in user exceeding the allowed number of DB subnet groups.</p>
    DBSubnetGroupQuotaExceededFault(String),
    /// <p>Request would result in user exceeding the allowed number of subnets in a DB subnet groups.</p>
    DBSubnetQuotaExceededFault(String),
    /// <p>The requested subnet is invalid, or multiple subnets were requested that are not all in a common VPC.</p>
    InvalidSubnet(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl CreateDBSubnetGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> CreateDBSubnetGroupError {
        {
            let reader = EventReader::new(res.body.as_slice());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "DBSubnetGroupAlreadyExists" => {
                        return CreateDBSubnetGroupError::DBSubnetGroupAlreadyExistsFault(
                            String::from(parsed_error.message),
                        )
                    }
                    "DBSubnetGroupDoesNotCoverEnoughAZs" => {
                        return CreateDBSubnetGroupError::DBSubnetGroupDoesNotCoverEnoughAZs(
                            String::from(parsed_error.message),
                        )
                    }
                    "DBSubnetGroupQuotaExceeded" => {
                        return CreateDBSubnetGroupError::DBSubnetGroupQuotaExceededFault(
                            String::from(parsed_error.message),
                        )
                    }
                    "DBSubnetQuotaExceededFault" => {
                        return CreateDBSubnetGroupError::DBSubnetQuotaExceededFault(String::from(
                            parsed_error.message,
                        ))
                    }
                    "InvalidSubnet" => {
                        return CreateDBSubnetGroupError::InvalidSubnet(String::from(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        CreateDBSubnetGroupError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for CreateDBSubnetGroupError {
    fn from(err: XmlParseError) -> CreateDBSubnetGroupError {
        let XmlParseError(message) = err;
        CreateDBSubnetGroupError::ParseError(message.to_string())
    }
}
impl From<CredentialsError> for CreateDBSubnetGroupError {
    fn from(err: CredentialsError) -> CreateDBSubnetGroupError {
        CreateDBSubnetGroupError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateDBSubnetGroupError {
    fn from(err: HttpDispatchError) -> CreateDBSubnetGroupError {
        CreateDBSubnetGroupError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateDBSubnetGroupError {
    fn from(err: io::Error) -> CreateDBSubnetGroupError {
        CreateDBSubnetGroupError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateDBSubnetGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateDBSubnetGroupError {
    fn description(&self) -> &str {
        match *self {
            CreateDBSubnetGroupError::DBSubnetGroupAlreadyExistsFault(ref cause) => cause,
            CreateDBSubnetGroupError::DBSubnetGroupDoesNotCoverEnoughAZs(ref cause) => cause,
            CreateDBSubnetGroupError::DBSubnetGroupQuotaExceededFault(ref cause) => cause,
            CreateDBSubnetGroupError::DBSubnetQuotaExceededFault(ref cause) => cause,
            CreateDBSubnetGroupError::InvalidSubnet(ref cause) => cause,
            CreateDBSubnetGroupError::Validation(ref cause) => cause,
            CreateDBSubnetGroupError::Credentials(ref err) => err.description(),
            CreateDBSubnetGroupError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateDBSubnetGroupError::ParseError(ref cause) => cause,
            CreateDBSubnetGroupError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by CreateEventSubscription
#[derive(Debug, PartialEq)]
pub enum CreateEventSubscriptionError {
    EventSubscriptionQuotaExceededFault(String),

    SNSInvalidTopicFault(String),

    SNSNoAuthorizationFault(String),

    SNSTopicArnNotFoundFault(String),

    SourceNotFoundFault(String),

    SubscriptionAlreadyExistFault(String),

    SubscriptionCategoryNotFoundFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl CreateEventSubscriptionError {
    pub fn from_response(res: BufferedHttpResponse) -> CreateEventSubscriptionError {
        {
            let reader = EventReader::new(res.body.as_slice());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "EventSubscriptionQuotaExceeded" => {
                        return CreateEventSubscriptionError::EventSubscriptionQuotaExceededFault(
                            String::from(parsed_error.message),
                        )
                    }
                    "SNSInvalidTopic" => {
                        return CreateEventSubscriptionError::SNSInvalidTopicFault(String::from(
                            parsed_error.message,
                        ))
                    }
                    "SNSNoAuthorization" => {
                        return CreateEventSubscriptionError::SNSNoAuthorizationFault(String::from(
                            parsed_error.message,
                        ))
                    }
                    "SNSTopicArnNotFound" => {
                        return CreateEventSubscriptionError::SNSTopicArnNotFoundFault(String::from(
                            parsed_error.message,
                        ))
                    }
                    "SourceNotFound" => {
                        return CreateEventSubscriptionError::SourceNotFoundFault(String::from(
                            parsed_error.message,
                        ))
                    }
                    "SubscriptionAlreadyExist" => {
                        return CreateEventSubscriptionError::SubscriptionAlreadyExistFault(
                            String::from(parsed_error.message),
                        )
                    }
                    "SubscriptionCategoryNotFound" => {
                        return CreateEventSubscriptionError::SubscriptionCategoryNotFoundFault(
                            String::from(parsed_error.message),
                        )
                    }
                    _ => {}
                }
            }
        }
        CreateEventSubscriptionError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for CreateEventSubscriptionError {
    fn from(err: XmlParseError) -> CreateEventSubscriptionError {
        let XmlParseError(message) = err;
        CreateEventSubscriptionError::ParseError(message.to_string())
    }
}
impl From<CredentialsError> for CreateEventSubscriptionError {
    fn from(err: CredentialsError) -> CreateEventSubscriptionError {
        CreateEventSubscriptionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateEventSubscriptionError {
    fn from(err: HttpDispatchError) -> CreateEventSubscriptionError {
        CreateEventSubscriptionError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateEventSubscriptionError {
    fn from(err: io::Error) -> CreateEventSubscriptionError {
        CreateEventSubscriptionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateEventSubscriptionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateEventSubscriptionError {
    fn description(&self) -> &str {
        match *self {
            CreateEventSubscriptionError::EventSubscriptionQuotaExceededFault(ref cause) => cause,
            CreateEventSubscriptionError::SNSInvalidTopicFault(ref cause) => cause,
            CreateEventSubscriptionError::SNSNoAuthorizationFault(ref cause) => cause,
            CreateEventSubscriptionError::SNSTopicArnNotFoundFault(ref cause) => cause,
            CreateEventSubscriptionError::SourceNotFoundFault(ref cause) => cause,
            CreateEventSubscriptionError::SubscriptionAlreadyExistFault(ref cause) => cause,
            CreateEventSubscriptionError::SubscriptionCategoryNotFoundFault(ref cause) => cause,
            CreateEventSubscriptionError::Validation(ref cause) => cause,
            CreateEventSubscriptionError::Credentials(ref err) => err.description(),
            CreateEventSubscriptionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateEventSubscriptionError::ParseError(ref cause) => cause,
            CreateEventSubscriptionError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteDBCluster
#[derive(Debug, PartialEq)]
pub enum DeleteDBClusterError {
    /// <p> <i>DBClusterIdentifier</i> does not refer to an existing DB cluster. </p>
    DBClusterNotFoundFault(String),
    /// <p>User already has a DB cluster snapshot with the given identifier.</p>
    DBClusterSnapshotAlreadyExistsFault(String),
    /// <p>The supplied value is not a valid DB cluster snapshot state.</p>
    InvalidDBClusterSnapshotStateFault(String),
    /// <p>The DB cluster is not in a valid state.</p>
    InvalidDBClusterStateFault(String),
    /// <p>Request would result in user exceeding the allowed number of DB snapshots.</p>
    SnapshotQuotaExceededFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DeleteDBClusterError {
    pub fn from_response(res: BufferedHttpResponse) -> DeleteDBClusterError {
        {
            let reader = EventReader::new(res.body.as_slice());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "DBClusterNotFoundFault" => {
                        return DeleteDBClusterError::DBClusterNotFoundFault(String::from(
                            parsed_error.message,
                        ))
                    }
                    "DBClusterSnapshotAlreadyExistsFault" => {
                        return DeleteDBClusterError::DBClusterSnapshotAlreadyExistsFault(
                            String::from(parsed_error.message),
                        )
                    }
                    "InvalidDBClusterSnapshotStateFault" => {
                        return DeleteDBClusterError::InvalidDBClusterSnapshotStateFault(
                            String::from(parsed_error.message),
                        )
                    }
                    "InvalidDBClusterStateFault" => {
                        return DeleteDBClusterError::InvalidDBClusterStateFault(String::from(
                            parsed_error.message,
                        ))
                    }
                    "SnapshotQuotaExceeded" => {
                        return DeleteDBClusterError::SnapshotQuotaExceededFault(String::from(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        DeleteDBClusterError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DeleteDBClusterError {
    fn from(err: XmlParseError) -> DeleteDBClusterError {
        let XmlParseError(message) = err;
        DeleteDBClusterError::ParseError(message.to_string())
    }
}
impl From<CredentialsError> for DeleteDBClusterError {
    fn from(err: CredentialsError) -> DeleteDBClusterError {
        DeleteDBClusterError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteDBClusterError {
    fn from(err: HttpDispatchError) -> DeleteDBClusterError {
        DeleteDBClusterError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteDBClusterError {
    fn from(err: io::Error) -> DeleteDBClusterError {
        DeleteDBClusterError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteDBClusterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteDBClusterError {
    fn description(&self) -> &str {
        match *self {
            DeleteDBClusterError::DBClusterNotFoundFault(ref cause) => cause,
            DeleteDBClusterError::DBClusterSnapshotAlreadyExistsFault(ref cause) => cause,
            DeleteDBClusterError::InvalidDBClusterSnapshotStateFault(ref cause) => cause,
            DeleteDBClusterError::InvalidDBClusterStateFault(ref cause) => cause,
            DeleteDBClusterError::SnapshotQuotaExceededFault(ref cause) => cause,
            DeleteDBClusterError::Validation(ref cause) => cause,
            DeleteDBClusterError::Credentials(ref err) => err.description(),
            DeleteDBClusterError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteDBClusterError::ParseError(ref cause) => cause,
            DeleteDBClusterError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteDBClusterParameterGroup
#[derive(Debug, PartialEq)]
pub enum DeleteDBClusterParameterGroupError {
    /// <p> <i>DBParameterGroupName</i> does not refer to an existing DB parameter group. </p>
    DBParameterGroupNotFoundFault(String),
    /// <p>The DB parameter group is in use or is in an invalid state. If you are attempting to delete the parameter group, you cannot delete it when the parameter group is in this state.</p>
    InvalidDBParameterGroupStateFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DeleteDBClusterParameterGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> DeleteDBClusterParameterGroupError {
        {
            let reader = EventReader::new(res.body.as_slice());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "DBParameterGroupNotFound" => {
                        return DeleteDBClusterParameterGroupError::DBParameterGroupNotFoundFault(
                            String::from(parsed_error.message),
                        )
                    }
                    "InvalidDBParameterGroupState" => {
                        return DeleteDBClusterParameterGroupError::InvalidDBParameterGroupStateFault(
                            String::from(parsed_error.message),
                        )
                    }
                    _ => {}
                }
            }
        }
        DeleteDBClusterParameterGroupError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DeleteDBClusterParameterGroupError {
    fn from(err: XmlParseError) -> DeleteDBClusterParameterGroupError {
        let XmlParseError(message) = err;
        DeleteDBClusterParameterGroupError::ParseError(message.to_string())
    }
}
impl From<CredentialsError> for DeleteDBClusterParameterGroupError {
    fn from(err: CredentialsError) -> DeleteDBClusterParameterGroupError {
        DeleteDBClusterParameterGroupError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteDBClusterParameterGroupError {
    fn from(err: HttpDispatchError) -> DeleteDBClusterParameterGroupError {
        DeleteDBClusterParameterGroupError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteDBClusterParameterGroupError {
    fn from(err: io::Error) -> DeleteDBClusterParameterGroupError {
        DeleteDBClusterParameterGroupError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteDBClusterParameterGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteDBClusterParameterGroupError {
    fn description(&self) -> &str {
        match *self {
            DeleteDBClusterParameterGroupError::DBParameterGroupNotFoundFault(ref cause) => cause,
            DeleteDBClusterParameterGroupError::InvalidDBParameterGroupStateFault(ref cause) => {
                cause
            }
            DeleteDBClusterParameterGroupError::Validation(ref cause) => cause,
            DeleteDBClusterParameterGroupError::Credentials(ref err) => err.description(),
            DeleteDBClusterParameterGroupError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteDBClusterParameterGroupError::ParseError(ref cause) => cause,
            DeleteDBClusterParameterGroupError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteDBClusterSnapshot
#[derive(Debug, PartialEq)]
pub enum DeleteDBClusterSnapshotError {
    /// <p> <i>DBClusterSnapshotIdentifier</i> does not refer to an existing DB cluster snapshot. </p>
    DBClusterSnapshotNotFoundFault(String),
    /// <p>The supplied value is not a valid DB cluster snapshot state.</p>
    InvalidDBClusterSnapshotStateFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DeleteDBClusterSnapshotError {
    pub fn from_response(res: BufferedHttpResponse) -> DeleteDBClusterSnapshotError {
        {
            let reader = EventReader::new(res.body.as_slice());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "DBClusterSnapshotNotFoundFault" => {
                        return DeleteDBClusterSnapshotError::DBClusterSnapshotNotFoundFault(
                            String::from(parsed_error.message),
                        )
                    }
                    "InvalidDBClusterSnapshotStateFault" => {
                        return DeleteDBClusterSnapshotError::InvalidDBClusterSnapshotStateFault(
                            String::from(parsed_error.message),
                        )
                    }
                    _ => {}
                }
            }
        }
        DeleteDBClusterSnapshotError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DeleteDBClusterSnapshotError {
    fn from(err: XmlParseError) -> DeleteDBClusterSnapshotError {
        let XmlParseError(message) = err;
        DeleteDBClusterSnapshotError::ParseError(message.to_string())
    }
}
impl From<CredentialsError> for DeleteDBClusterSnapshotError {
    fn from(err: CredentialsError) -> DeleteDBClusterSnapshotError {
        DeleteDBClusterSnapshotError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteDBClusterSnapshotError {
    fn from(err: HttpDispatchError) -> DeleteDBClusterSnapshotError {
        DeleteDBClusterSnapshotError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteDBClusterSnapshotError {
    fn from(err: io::Error) -> DeleteDBClusterSnapshotError {
        DeleteDBClusterSnapshotError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteDBClusterSnapshotError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteDBClusterSnapshotError {
    fn description(&self) -> &str {
        match *self {
            DeleteDBClusterSnapshotError::DBClusterSnapshotNotFoundFault(ref cause) => cause,
            DeleteDBClusterSnapshotError::InvalidDBClusterSnapshotStateFault(ref cause) => cause,
            DeleteDBClusterSnapshotError::Validation(ref cause) => cause,
            DeleteDBClusterSnapshotError::Credentials(ref err) => err.description(),
            DeleteDBClusterSnapshotError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteDBClusterSnapshotError::ParseError(ref cause) => cause,
            DeleteDBClusterSnapshotError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteDBInstance
#[derive(Debug, PartialEq)]
pub enum DeleteDBInstanceError {
    /// <p> <i>DBInstanceIdentifier</i> does not refer to an existing DB instance. </p>
    DBInstanceNotFoundFault(String),
    /// <p> <i>DBSnapshotIdentifier</i> is already used by an existing snapshot. </p>
    DBSnapshotAlreadyExistsFault(String),
    /// <p>The DB cluster is not in a valid state.</p>
    InvalidDBClusterStateFault(String),
    /// <p> The specified DB instance is not in the <i>available</i> state. </p>
    InvalidDBInstanceStateFault(String),
    /// <p>Request would result in user exceeding the allowed number of DB snapshots.</p>
    SnapshotQuotaExceededFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DeleteDBInstanceError {
    pub fn from_response(res: BufferedHttpResponse) -> DeleteDBInstanceError {
        {
            let reader = EventReader::new(res.body.as_slice());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "DBInstanceNotFound" => {
                        return DeleteDBInstanceError::DBInstanceNotFoundFault(String::from(
                            parsed_error.message,
                        ))
                    }
                    "DBSnapshotAlreadyExists" => {
                        return DeleteDBInstanceError::DBSnapshotAlreadyExistsFault(String::from(
                            parsed_error.message,
                        ))
                    }
                    "InvalidDBClusterStateFault" => {
                        return DeleteDBInstanceError::InvalidDBClusterStateFault(String::from(
                            parsed_error.message,
                        ))
                    }
                    "InvalidDBInstanceState" => {
                        return DeleteDBInstanceError::InvalidDBInstanceStateFault(String::from(
                            parsed_error.message,
                        ))
                    }
                    "SnapshotQuotaExceeded" => {
                        return DeleteDBInstanceError::SnapshotQuotaExceededFault(String::from(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        DeleteDBInstanceError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DeleteDBInstanceError {
    fn from(err: XmlParseError) -> DeleteDBInstanceError {
        let XmlParseError(message) = err;
        DeleteDBInstanceError::ParseError(message.to_string())
    }
}
impl From<CredentialsError> for DeleteDBInstanceError {
    fn from(err: CredentialsError) -> DeleteDBInstanceError {
        DeleteDBInstanceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteDBInstanceError {
    fn from(err: HttpDispatchError) -> DeleteDBInstanceError {
        DeleteDBInstanceError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteDBInstanceError {
    fn from(err: io::Error) -> DeleteDBInstanceError {
        DeleteDBInstanceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteDBInstanceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteDBInstanceError {
    fn description(&self) -> &str {
        match *self {
            DeleteDBInstanceError::DBInstanceNotFoundFault(ref cause) => cause,
            DeleteDBInstanceError::DBSnapshotAlreadyExistsFault(ref cause) => cause,
            DeleteDBInstanceError::InvalidDBClusterStateFault(ref cause) => cause,
            DeleteDBInstanceError::InvalidDBInstanceStateFault(ref cause) => cause,
            DeleteDBInstanceError::SnapshotQuotaExceededFault(ref cause) => cause,
            DeleteDBInstanceError::Validation(ref cause) => cause,
            DeleteDBInstanceError::Credentials(ref err) => err.description(),
            DeleteDBInstanceError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteDBInstanceError::ParseError(ref cause) => cause,
            DeleteDBInstanceError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteDBParameterGroup
#[derive(Debug, PartialEq)]
pub enum DeleteDBParameterGroupError {
    /// <p> <i>DBParameterGroupName</i> does not refer to an existing DB parameter group. </p>
    DBParameterGroupNotFoundFault(String),
    /// <p>The DB parameter group is in use or is in an invalid state. If you are attempting to delete the parameter group, you cannot delete it when the parameter group is in this state.</p>
    InvalidDBParameterGroupStateFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DeleteDBParameterGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> DeleteDBParameterGroupError {
        {
            let reader = EventReader::new(res.body.as_slice());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "DBParameterGroupNotFound" => {
                        return DeleteDBParameterGroupError::DBParameterGroupNotFoundFault(
                            String::from(parsed_error.message),
                        )
                    }
                    "InvalidDBParameterGroupState" => {
                        return DeleteDBParameterGroupError::InvalidDBParameterGroupStateFault(
                            String::from(parsed_error.message),
                        )
                    }
                    _ => {}
                }
            }
        }
        DeleteDBParameterGroupError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DeleteDBParameterGroupError {
    fn from(err: XmlParseError) -> DeleteDBParameterGroupError {
        let XmlParseError(message) = err;
        DeleteDBParameterGroupError::ParseError(message.to_string())
    }
}
impl From<CredentialsError> for DeleteDBParameterGroupError {
    fn from(err: CredentialsError) -> DeleteDBParameterGroupError {
        DeleteDBParameterGroupError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteDBParameterGroupError {
    fn from(err: HttpDispatchError) -> DeleteDBParameterGroupError {
        DeleteDBParameterGroupError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteDBParameterGroupError {
    fn from(err: io::Error) -> DeleteDBParameterGroupError {
        DeleteDBParameterGroupError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteDBParameterGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteDBParameterGroupError {
    fn description(&self) -> &str {
        match *self {
            DeleteDBParameterGroupError::DBParameterGroupNotFoundFault(ref cause) => cause,
            DeleteDBParameterGroupError::InvalidDBParameterGroupStateFault(ref cause) => cause,
            DeleteDBParameterGroupError::Validation(ref cause) => cause,
            DeleteDBParameterGroupError::Credentials(ref err) => err.description(),
            DeleteDBParameterGroupError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteDBParameterGroupError::ParseError(ref cause) => cause,
            DeleteDBParameterGroupError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteDBSubnetGroup
#[derive(Debug, PartialEq)]
pub enum DeleteDBSubnetGroupError {
    /// <p> <i>DBSubnetGroupName</i> does not refer to an existing DB subnet group. </p>
    DBSubnetGroupNotFoundFault(String),
    /// <p>The DB subnet group cannot be deleted because it is in use.</p>
    InvalidDBSubnetGroupStateFault(String),
    /// <p> The DB subnet is not in the <i>available</i> state. </p>
    InvalidDBSubnetStateFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DeleteDBSubnetGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> DeleteDBSubnetGroupError {
        {
            let reader = EventReader::new(res.body.as_slice());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "DBSubnetGroupNotFoundFault" => {
                        return DeleteDBSubnetGroupError::DBSubnetGroupNotFoundFault(String::from(
                            parsed_error.message,
                        ))
                    }
                    "InvalidDBSubnetGroupStateFault" => {
                        return DeleteDBSubnetGroupError::InvalidDBSubnetGroupStateFault(
                            String::from(parsed_error.message),
                        )
                    }
                    "InvalidDBSubnetStateFault" => {
                        return DeleteDBSubnetGroupError::InvalidDBSubnetStateFault(String::from(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        DeleteDBSubnetGroupError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DeleteDBSubnetGroupError {
    fn from(err: XmlParseError) -> DeleteDBSubnetGroupError {
        let XmlParseError(message) = err;
        DeleteDBSubnetGroupError::ParseError(message.to_string())
    }
}
impl From<CredentialsError> for DeleteDBSubnetGroupError {
    fn from(err: CredentialsError) -> DeleteDBSubnetGroupError {
        DeleteDBSubnetGroupError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteDBSubnetGroupError {
    fn from(err: HttpDispatchError) -> DeleteDBSubnetGroupError {
        DeleteDBSubnetGroupError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteDBSubnetGroupError {
    fn from(err: io::Error) -> DeleteDBSubnetGroupError {
        DeleteDBSubnetGroupError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteDBSubnetGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteDBSubnetGroupError {
    fn description(&self) -> &str {
        match *self {
            DeleteDBSubnetGroupError::DBSubnetGroupNotFoundFault(ref cause) => cause,
            DeleteDBSubnetGroupError::InvalidDBSubnetGroupStateFault(ref cause) => cause,
            DeleteDBSubnetGroupError::InvalidDBSubnetStateFault(ref cause) => cause,
            DeleteDBSubnetGroupError::Validation(ref cause) => cause,
            DeleteDBSubnetGroupError::Credentials(ref err) => err.description(),
            DeleteDBSubnetGroupError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteDBSubnetGroupError::ParseError(ref cause) => cause,
            DeleteDBSubnetGroupError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteEventSubscription
#[derive(Debug, PartialEq)]
pub enum DeleteEventSubscriptionError {
    InvalidEventSubscriptionStateFault(String),

    SubscriptionNotFoundFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DeleteEventSubscriptionError {
    pub fn from_response(res: BufferedHttpResponse) -> DeleteEventSubscriptionError {
        {
            let reader = EventReader::new(res.body.as_slice());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "InvalidEventSubscriptionState" => {
                        return DeleteEventSubscriptionError::InvalidEventSubscriptionStateFault(
                            String::from(parsed_error.message),
                        )
                    }
                    "SubscriptionNotFound" => {
                        return DeleteEventSubscriptionError::SubscriptionNotFoundFault(
                            String::from(parsed_error.message),
                        )
                    }
                    _ => {}
                }
            }
        }
        DeleteEventSubscriptionError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DeleteEventSubscriptionError {
    fn from(err: XmlParseError) -> DeleteEventSubscriptionError {
        let XmlParseError(message) = err;
        DeleteEventSubscriptionError::ParseError(message.to_string())
    }
}
impl From<CredentialsError> for DeleteEventSubscriptionError {
    fn from(err: CredentialsError) -> DeleteEventSubscriptionError {
        DeleteEventSubscriptionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteEventSubscriptionError {
    fn from(err: HttpDispatchError) -> DeleteEventSubscriptionError {
        DeleteEventSubscriptionError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteEventSubscriptionError {
    fn from(err: io::Error) -> DeleteEventSubscriptionError {
        DeleteEventSubscriptionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteEventSubscriptionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteEventSubscriptionError {
    fn description(&self) -> &str {
        match *self {
            DeleteEventSubscriptionError::InvalidEventSubscriptionStateFault(ref cause) => cause,
            DeleteEventSubscriptionError::SubscriptionNotFoundFault(ref cause) => cause,
            DeleteEventSubscriptionError::Validation(ref cause) => cause,
            DeleteEventSubscriptionError::Credentials(ref err) => err.description(),
            DeleteEventSubscriptionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteEventSubscriptionError::ParseError(ref cause) => cause,
            DeleteEventSubscriptionError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DescribeDBClusterParameterGroups
#[derive(Debug, PartialEq)]
pub enum DescribeDBClusterParameterGroupsError {
    /// <p> <i>DBParameterGroupName</i> does not refer to an existing DB parameter group. </p>
    DBParameterGroupNotFoundFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DescribeDBClusterParameterGroupsError {
    pub fn from_response(res: BufferedHttpResponse) -> DescribeDBClusterParameterGroupsError {
        {
            let reader = EventReader::new(res.body.as_slice());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "DBParameterGroupNotFound" => {
                        return DescribeDBClusterParameterGroupsError::DBParameterGroupNotFoundFault(
                            String::from(parsed_error.message),
                        )
                    }
                    _ => {}
                }
            }
        }
        DescribeDBClusterParameterGroupsError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DescribeDBClusterParameterGroupsError {
    fn from(err: XmlParseError) -> DescribeDBClusterParameterGroupsError {
        let XmlParseError(message) = err;
        DescribeDBClusterParameterGroupsError::ParseError(message.to_string())
    }
}
impl From<CredentialsError> for DescribeDBClusterParameterGroupsError {
    fn from(err: CredentialsError) -> DescribeDBClusterParameterGroupsError {
        DescribeDBClusterParameterGroupsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeDBClusterParameterGroupsError {
    fn from(err: HttpDispatchError) -> DescribeDBClusterParameterGroupsError {
        DescribeDBClusterParameterGroupsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeDBClusterParameterGroupsError {
    fn from(err: io::Error) -> DescribeDBClusterParameterGroupsError {
        DescribeDBClusterParameterGroupsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeDBClusterParameterGroupsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeDBClusterParameterGroupsError {
    fn description(&self) -> &str {
        match *self {
            DescribeDBClusterParameterGroupsError::DBParameterGroupNotFoundFault(ref cause) => {
                cause
            }
            DescribeDBClusterParameterGroupsError::Validation(ref cause) => cause,
            DescribeDBClusterParameterGroupsError::Credentials(ref err) => err.description(),
            DescribeDBClusterParameterGroupsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeDBClusterParameterGroupsError::ParseError(ref cause) => cause,
            DescribeDBClusterParameterGroupsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DescribeDBClusterParameters
#[derive(Debug, PartialEq)]
pub enum DescribeDBClusterParametersError {
    /// <p> <i>DBParameterGroupName</i> does not refer to an existing DB parameter group. </p>
    DBParameterGroupNotFoundFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DescribeDBClusterParametersError {
    pub fn from_response(res: BufferedHttpResponse) -> DescribeDBClusterParametersError {
        {
            let reader = EventReader::new(res.body.as_slice());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "DBParameterGroupNotFound" => {
                        return DescribeDBClusterParametersError::DBParameterGroupNotFoundFault(
                            String::from(parsed_error.message),
                        )
                    }
                    _ => {}
                }
            }
        }
        DescribeDBClusterParametersError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DescribeDBClusterParametersError {
    fn from(err: XmlParseError) -> DescribeDBClusterParametersError {
        let XmlParseError(message) = err;
        DescribeDBClusterParametersError::ParseError(message.to_string())
    }
}
impl From<CredentialsError> for DescribeDBClusterParametersError {
    fn from(err: CredentialsError) -> DescribeDBClusterParametersError {
        DescribeDBClusterParametersError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeDBClusterParametersError {
    fn from(err: HttpDispatchError) -> DescribeDBClusterParametersError {
        DescribeDBClusterParametersError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeDBClusterParametersError {
    fn from(err: io::Error) -> DescribeDBClusterParametersError {
        DescribeDBClusterParametersError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeDBClusterParametersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeDBClusterParametersError {
    fn description(&self) -> &str {
        match *self {
            DescribeDBClusterParametersError::DBParameterGroupNotFoundFault(ref cause) => cause,
            DescribeDBClusterParametersError::Validation(ref cause) => cause,
            DescribeDBClusterParametersError::Credentials(ref err) => err.description(),
            DescribeDBClusterParametersError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeDBClusterParametersError::ParseError(ref cause) => cause,
            DescribeDBClusterParametersError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DescribeDBClusterSnapshotAttributes
#[derive(Debug, PartialEq)]
pub enum DescribeDBClusterSnapshotAttributesError {
    /// <p> <i>DBClusterSnapshotIdentifier</i> does not refer to an existing DB cluster snapshot. </p>
    DBClusterSnapshotNotFoundFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DescribeDBClusterSnapshotAttributesError {
    pub fn from_response(res: BufferedHttpResponse) -> DescribeDBClusterSnapshotAttributesError {
        {
            let reader = EventReader::new(res.body.as_slice());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                                    "DBClusterSnapshotNotFoundFault" => return DescribeDBClusterSnapshotAttributesError::DBClusterSnapshotNotFoundFault(String::from(parsed_error.message)),_ => {}
                                }
            }
        }
        DescribeDBClusterSnapshotAttributesError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DescribeDBClusterSnapshotAttributesError {
    fn from(err: XmlParseError) -> DescribeDBClusterSnapshotAttributesError {
        let XmlParseError(message) = err;
        DescribeDBClusterSnapshotAttributesError::ParseError(message.to_string())
    }
}
impl From<CredentialsError> for DescribeDBClusterSnapshotAttributesError {
    fn from(err: CredentialsError) -> DescribeDBClusterSnapshotAttributesError {
        DescribeDBClusterSnapshotAttributesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeDBClusterSnapshotAttributesError {
    fn from(err: HttpDispatchError) -> DescribeDBClusterSnapshotAttributesError {
        DescribeDBClusterSnapshotAttributesError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeDBClusterSnapshotAttributesError {
    fn from(err: io::Error) -> DescribeDBClusterSnapshotAttributesError {
        DescribeDBClusterSnapshotAttributesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeDBClusterSnapshotAttributesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeDBClusterSnapshotAttributesError {
    fn description(&self) -> &str {
        match *self {
            DescribeDBClusterSnapshotAttributesError::DBClusterSnapshotNotFoundFault(ref cause) => {
                cause
            }
            DescribeDBClusterSnapshotAttributesError::Validation(ref cause) => cause,
            DescribeDBClusterSnapshotAttributesError::Credentials(ref err) => err.description(),
            DescribeDBClusterSnapshotAttributesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeDBClusterSnapshotAttributesError::ParseError(ref cause) => cause,
            DescribeDBClusterSnapshotAttributesError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DescribeDBClusterSnapshots
#[derive(Debug, PartialEq)]
pub enum DescribeDBClusterSnapshotsError {
    /// <p> <i>DBClusterSnapshotIdentifier</i> does not refer to an existing DB cluster snapshot. </p>
    DBClusterSnapshotNotFoundFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DescribeDBClusterSnapshotsError {
    pub fn from_response(res: BufferedHttpResponse) -> DescribeDBClusterSnapshotsError {
        {
            let reader = EventReader::new(res.body.as_slice());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "DBClusterSnapshotNotFoundFault" => {
                        return DescribeDBClusterSnapshotsError::DBClusterSnapshotNotFoundFault(
                            String::from(parsed_error.message),
                        )
                    }
                    _ => {}
                }
            }
        }
        DescribeDBClusterSnapshotsError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DescribeDBClusterSnapshotsError {
    fn from(err: XmlParseError) -> DescribeDBClusterSnapshotsError {
        let XmlParseError(message) = err;
        DescribeDBClusterSnapshotsError::ParseError(message.to_string())
    }
}
impl From<CredentialsError> for DescribeDBClusterSnapshotsError {
    fn from(err: CredentialsError) -> DescribeDBClusterSnapshotsError {
        DescribeDBClusterSnapshotsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeDBClusterSnapshotsError {
    fn from(err: HttpDispatchError) -> DescribeDBClusterSnapshotsError {
        DescribeDBClusterSnapshotsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeDBClusterSnapshotsError {
    fn from(err: io::Error) -> DescribeDBClusterSnapshotsError {
        DescribeDBClusterSnapshotsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeDBClusterSnapshotsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeDBClusterSnapshotsError {
    fn description(&self) -> &str {
        match *self {
            DescribeDBClusterSnapshotsError::DBClusterSnapshotNotFoundFault(ref cause) => cause,
            DescribeDBClusterSnapshotsError::Validation(ref cause) => cause,
            DescribeDBClusterSnapshotsError::Credentials(ref err) => err.description(),
            DescribeDBClusterSnapshotsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeDBClusterSnapshotsError::ParseError(ref cause) => cause,
            DescribeDBClusterSnapshotsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DescribeDBClusters
#[derive(Debug, PartialEq)]
pub enum DescribeDBClustersError {
    /// <p> <i>DBClusterIdentifier</i> does not refer to an existing DB cluster. </p>
    DBClusterNotFoundFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DescribeDBClustersError {
    pub fn from_response(res: BufferedHttpResponse) -> DescribeDBClustersError {
        {
            let reader = EventReader::new(res.body.as_slice());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "DBClusterNotFoundFault" => {
                        return DescribeDBClustersError::DBClusterNotFoundFault(String::from(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        DescribeDBClustersError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DescribeDBClustersError {
    fn from(err: XmlParseError) -> DescribeDBClustersError {
        let XmlParseError(message) = err;
        DescribeDBClustersError::ParseError(message.to_string())
    }
}
impl From<CredentialsError> for DescribeDBClustersError {
    fn from(err: CredentialsError) -> DescribeDBClustersError {
        DescribeDBClustersError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeDBClustersError {
    fn from(err: HttpDispatchError) -> DescribeDBClustersError {
        DescribeDBClustersError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeDBClustersError {
    fn from(err: io::Error) -> DescribeDBClustersError {
        DescribeDBClustersError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeDBClustersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeDBClustersError {
    fn description(&self) -> &str {
        match *self {
            DescribeDBClustersError::DBClusterNotFoundFault(ref cause) => cause,
            DescribeDBClustersError::Validation(ref cause) => cause,
            DescribeDBClustersError::Credentials(ref err) => err.description(),
            DescribeDBClustersError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeDBClustersError::ParseError(ref cause) => cause,
            DescribeDBClustersError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DescribeDBEngineVersions
#[derive(Debug, PartialEq)]
pub enum DescribeDBEngineVersionsError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DescribeDBEngineVersionsError {
    pub fn from_response(res: BufferedHttpResponse) -> DescribeDBEngineVersionsError {
        {
            let reader = EventReader::new(res.body.as_slice());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        DescribeDBEngineVersionsError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DescribeDBEngineVersionsError {
    fn from(err: XmlParseError) -> DescribeDBEngineVersionsError {
        let XmlParseError(message) = err;
        DescribeDBEngineVersionsError::ParseError(message.to_string())
    }
}
impl From<CredentialsError> for DescribeDBEngineVersionsError {
    fn from(err: CredentialsError) -> DescribeDBEngineVersionsError {
        DescribeDBEngineVersionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeDBEngineVersionsError {
    fn from(err: HttpDispatchError) -> DescribeDBEngineVersionsError {
        DescribeDBEngineVersionsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeDBEngineVersionsError {
    fn from(err: io::Error) -> DescribeDBEngineVersionsError {
        DescribeDBEngineVersionsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeDBEngineVersionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeDBEngineVersionsError {
    fn description(&self) -> &str {
        match *self {
            DescribeDBEngineVersionsError::Validation(ref cause) => cause,
            DescribeDBEngineVersionsError::Credentials(ref err) => err.description(),
            DescribeDBEngineVersionsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeDBEngineVersionsError::ParseError(ref cause) => cause,
            DescribeDBEngineVersionsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DescribeDBInstances
#[derive(Debug, PartialEq)]
pub enum DescribeDBInstancesError {
    /// <p> <i>DBInstanceIdentifier</i> does not refer to an existing DB instance. </p>
    DBInstanceNotFoundFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DescribeDBInstancesError {
    pub fn from_response(res: BufferedHttpResponse) -> DescribeDBInstancesError {
        {
            let reader = EventReader::new(res.body.as_slice());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "DBInstanceNotFound" => {
                        return DescribeDBInstancesError::DBInstanceNotFoundFault(String::from(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        DescribeDBInstancesError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DescribeDBInstancesError {
    fn from(err: XmlParseError) -> DescribeDBInstancesError {
        let XmlParseError(message) = err;
        DescribeDBInstancesError::ParseError(message.to_string())
    }
}
impl From<CredentialsError> for DescribeDBInstancesError {
    fn from(err: CredentialsError) -> DescribeDBInstancesError {
        DescribeDBInstancesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeDBInstancesError {
    fn from(err: HttpDispatchError) -> DescribeDBInstancesError {
        DescribeDBInstancesError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeDBInstancesError {
    fn from(err: io::Error) -> DescribeDBInstancesError {
        DescribeDBInstancesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeDBInstancesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeDBInstancesError {
    fn description(&self) -> &str {
        match *self {
            DescribeDBInstancesError::DBInstanceNotFoundFault(ref cause) => cause,
            DescribeDBInstancesError::Validation(ref cause) => cause,
            DescribeDBInstancesError::Credentials(ref err) => err.description(),
            DescribeDBInstancesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeDBInstancesError::ParseError(ref cause) => cause,
            DescribeDBInstancesError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DescribeDBParameterGroups
#[derive(Debug, PartialEq)]
pub enum DescribeDBParameterGroupsError {
    /// <p> <i>DBParameterGroupName</i> does not refer to an existing DB parameter group. </p>
    DBParameterGroupNotFoundFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DescribeDBParameterGroupsError {
    pub fn from_response(res: BufferedHttpResponse) -> DescribeDBParameterGroupsError {
        {
            let reader = EventReader::new(res.body.as_slice());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "DBParameterGroupNotFound" => {
                        return DescribeDBParameterGroupsError::DBParameterGroupNotFoundFault(
                            String::from(parsed_error.message),
                        )
                    }
                    _ => {}
                }
            }
        }
        DescribeDBParameterGroupsError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DescribeDBParameterGroupsError {
    fn from(err: XmlParseError) -> DescribeDBParameterGroupsError {
        let XmlParseError(message) = err;
        DescribeDBParameterGroupsError::ParseError(message.to_string())
    }
}
impl From<CredentialsError> for DescribeDBParameterGroupsError {
    fn from(err: CredentialsError) -> DescribeDBParameterGroupsError {
        DescribeDBParameterGroupsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeDBParameterGroupsError {
    fn from(err: HttpDispatchError) -> DescribeDBParameterGroupsError {
        DescribeDBParameterGroupsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeDBParameterGroupsError {
    fn from(err: io::Error) -> DescribeDBParameterGroupsError {
        DescribeDBParameterGroupsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeDBParameterGroupsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeDBParameterGroupsError {
    fn description(&self) -> &str {
        match *self {
            DescribeDBParameterGroupsError::DBParameterGroupNotFoundFault(ref cause) => cause,
            DescribeDBParameterGroupsError::Validation(ref cause) => cause,
            DescribeDBParameterGroupsError::Credentials(ref err) => err.description(),
            DescribeDBParameterGroupsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeDBParameterGroupsError::ParseError(ref cause) => cause,
            DescribeDBParameterGroupsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DescribeDBParameters
#[derive(Debug, PartialEq)]
pub enum DescribeDBParametersError {
    /// <p> <i>DBParameterGroupName</i> does not refer to an existing DB parameter group. </p>
    DBParameterGroupNotFoundFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DescribeDBParametersError {
    pub fn from_response(res: BufferedHttpResponse) -> DescribeDBParametersError {
        {
            let reader = EventReader::new(res.body.as_slice());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "DBParameterGroupNotFound" => {
                        return DescribeDBParametersError::DBParameterGroupNotFoundFault(
                            String::from(parsed_error.message),
                        )
                    }
                    _ => {}
                }
            }
        }
        DescribeDBParametersError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DescribeDBParametersError {
    fn from(err: XmlParseError) -> DescribeDBParametersError {
        let XmlParseError(message) = err;
        DescribeDBParametersError::ParseError(message.to_string())
    }
}
impl From<CredentialsError> for DescribeDBParametersError {
    fn from(err: CredentialsError) -> DescribeDBParametersError {
        DescribeDBParametersError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeDBParametersError {
    fn from(err: HttpDispatchError) -> DescribeDBParametersError {
        DescribeDBParametersError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeDBParametersError {
    fn from(err: io::Error) -> DescribeDBParametersError {
        DescribeDBParametersError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeDBParametersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeDBParametersError {
    fn description(&self) -> &str {
        match *self {
            DescribeDBParametersError::DBParameterGroupNotFoundFault(ref cause) => cause,
            DescribeDBParametersError::Validation(ref cause) => cause,
            DescribeDBParametersError::Credentials(ref err) => err.description(),
            DescribeDBParametersError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeDBParametersError::ParseError(ref cause) => cause,
            DescribeDBParametersError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DescribeDBSubnetGroups
#[derive(Debug, PartialEq)]
pub enum DescribeDBSubnetGroupsError {
    /// <p> <i>DBSubnetGroupName</i> does not refer to an existing DB subnet group. </p>
    DBSubnetGroupNotFoundFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DescribeDBSubnetGroupsError {
    pub fn from_response(res: BufferedHttpResponse) -> DescribeDBSubnetGroupsError {
        {
            let reader = EventReader::new(res.body.as_slice());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "DBSubnetGroupNotFoundFault" => {
                        return DescribeDBSubnetGroupsError::DBSubnetGroupNotFoundFault(
                            String::from(parsed_error.message),
                        )
                    }
                    _ => {}
                }
            }
        }
        DescribeDBSubnetGroupsError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DescribeDBSubnetGroupsError {
    fn from(err: XmlParseError) -> DescribeDBSubnetGroupsError {
        let XmlParseError(message) = err;
        DescribeDBSubnetGroupsError::ParseError(message.to_string())
    }
}
impl From<CredentialsError> for DescribeDBSubnetGroupsError {
    fn from(err: CredentialsError) -> DescribeDBSubnetGroupsError {
        DescribeDBSubnetGroupsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeDBSubnetGroupsError {
    fn from(err: HttpDispatchError) -> DescribeDBSubnetGroupsError {
        DescribeDBSubnetGroupsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeDBSubnetGroupsError {
    fn from(err: io::Error) -> DescribeDBSubnetGroupsError {
        DescribeDBSubnetGroupsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeDBSubnetGroupsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeDBSubnetGroupsError {
    fn description(&self) -> &str {
        match *self {
            DescribeDBSubnetGroupsError::DBSubnetGroupNotFoundFault(ref cause) => cause,
            DescribeDBSubnetGroupsError::Validation(ref cause) => cause,
            DescribeDBSubnetGroupsError::Credentials(ref err) => err.description(),
            DescribeDBSubnetGroupsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeDBSubnetGroupsError::ParseError(ref cause) => cause,
            DescribeDBSubnetGroupsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DescribeEngineDefaultClusterParameters
#[derive(Debug, PartialEq)]
pub enum DescribeEngineDefaultClusterParametersError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DescribeEngineDefaultClusterParametersError {
    pub fn from_response(res: BufferedHttpResponse) -> DescribeEngineDefaultClusterParametersError {
        {
            let reader = EventReader::new(res.body.as_slice());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        DescribeEngineDefaultClusterParametersError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DescribeEngineDefaultClusterParametersError {
    fn from(err: XmlParseError) -> DescribeEngineDefaultClusterParametersError {
        let XmlParseError(message) = err;
        DescribeEngineDefaultClusterParametersError::ParseError(message.to_string())
    }
}
impl From<CredentialsError> for DescribeEngineDefaultClusterParametersError {
    fn from(err: CredentialsError) -> DescribeEngineDefaultClusterParametersError {
        DescribeEngineDefaultClusterParametersError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeEngineDefaultClusterParametersError {
    fn from(err: HttpDispatchError) -> DescribeEngineDefaultClusterParametersError {
        DescribeEngineDefaultClusterParametersError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeEngineDefaultClusterParametersError {
    fn from(err: io::Error) -> DescribeEngineDefaultClusterParametersError {
        DescribeEngineDefaultClusterParametersError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeEngineDefaultClusterParametersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeEngineDefaultClusterParametersError {
    fn description(&self) -> &str {
        match *self {
            DescribeEngineDefaultClusterParametersError::Validation(ref cause) => cause,
            DescribeEngineDefaultClusterParametersError::Credentials(ref err) => err.description(),
            DescribeEngineDefaultClusterParametersError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeEngineDefaultClusterParametersError::ParseError(ref cause) => cause,
            DescribeEngineDefaultClusterParametersError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DescribeEngineDefaultParameters
#[derive(Debug, PartialEq)]
pub enum DescribeEngineDefaultParametersError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DescribeEngineDefaultParametersError {
    pub fn from_response(res: BufferedHttpResponse) -> DescribeEngineDefaultParametersError {
        {
            let reader = EventReader::new(res.body.as_slice());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        DescribeEngineDefaultParametersError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DescribeEngineDefaultParametersError {
    fn from(err: XmlParseError) -> DescribeEngineDefaultParametersError {
        let XmlParseError(message) = err;
        DescribeEngineDefaultParametersError::ParseError(message.to_string())
    }
}
impl From<CredentialsError> for DescribeEngineDefaultParametersError {
    fn from(err: CredentialsError) -> DescribeEngineDefaultParametersError {
        DescribeEngineDefaultParametersError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeEngineDefaultParametersError {
    fn from(err: HttpDispatchError) -> DescribeEngineDefaultParametersError {
        DescribeEngineDefaultParametersError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeEngineDefaultParametersError {
    fn from(err: io::Error) -> DescribeEngineDefaultParametersError {
        DescribeEngineDefaultParametersError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeEngineDefaultParametersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeEngineDefaultParametersError {
    fn description(&self) -> &str {
        match *self {
            DescribeEngineDefaultParametersError::Validation(ref cause) => cause,
            DescribeEngineDefaultParametersError::Credentials(ref err) => err.description(),
            DescribeEngineDefaultParametersError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeEngineDefaultParametersError::ParseError(ref cause) => cause,
            DescribeEngineDefaultParametersError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DescribeEventCategories
#[derive(Debug, PartialEq)]
pub enum DescribeEventCategoriesError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DescribeEventCategoriesError {
    pub fn from_response(res: BufferedHttpResponse) -> DescribeEventCategoriesError {
        {
            let reader = EventReader::new(res.body.as_slice());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        DescribeEventCategoriesError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DescribeEventCategoriesError {
    fn from(err: XmlParseError) -> DescribeEventCategoriesError {
        let XmlParseError(message) = err;
        DescribeEventCategoriesError::ParseError(message.to_string())
    }
}
impl From<CredentialsError> for DescribeEventCategoriesError {
    fn from(err: CredentialsError) -> DescribeEventCategoriesError {
        DescribeEventCategoriesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeEventCategoriesError {
    fn from(err: HttpDispatchError) -> DescribeEventCategoriesError {
        DescribeEventCategoriesError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeEventCategoriesError {
    fn from(err: io::Error) -> DescribeEventCategoriesError {
        DescribeEventCategoriesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeEventCategoriesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeEventCategoriesError {
    fn description(&self) -> &str {
        match *self {
            DescribeEventCategoriesError::Validation(ref cause) => cause,
            DescribeEventCategoriesError::Credentials(ref err) => err.description(),
            DescribeEventCategoriesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeEventCategoriesError::ParseError(ref cause) => cause,
            DescribeEventCategoriesError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DescribeEventSubscriptions
#[derive(Debug, PartialEq)]
pub enum DescribeEventSubscriptionsError {
    SubscriptionNotFoundFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DescribeEventSubscriptionsError {
    pub fn from_response(res: BufferedHttpResponse) -> DescribeEventSubscriptionsError {
        {
            let reader = EventReader::new(res.body.as_slice());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "SubscriptionNotFound" => {
                        return DescribeEventSubscriptionsError::SubscriptionNotFoundFault(
                            String::from(parsed_error.message),
                        )
                    }
                    _ => {}
                }
            }
        }
        DescribeEventSubscriptionsError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DescribeEventSubscriptionsError {
    fn from(err: XmlParseError) -> DescribeEventSubscriptionsError {
        let XmlParseError(message) = err;
        DescribeEventSubscriptionsError::ParseError(message.to_string())
    }
}
impl From<CredentialsError> for DescribeEventSubscriptionsError {
    fn from(err: CredentialsError) -> DescribeEventSubscriptionsError {
        DescribeEventSubscriptionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeEventSubscriptionsError {
    fn from(err: HttpDispatchError) -> DescribeEventSubscriptionsError {
        DescribeEventSubscriptionsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeEventSubscriptionsError {
    fn from(err: io::Error) -> DescribeEventSubscriptionsError {
        DescribeEventSubscriptionsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeEventSubscriptionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeEventSubscriptionsError {
    fn description(&self) -> &str {
        match *self {
            DescribeEventSubscriptionsError::SubscriptionNotFoundFault(ref cause) => cause,
            DescribeEventSubscriptionsError::Validation(ref cause) => cause,
            DescribeEventSubscriptionsError::Credentials(ref err) => err.description(),
            DescribeEventSubscriptionsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeEventSubscriptionsError::ParseError(ref cause) => cause,
            DescribeEventSubscriptionsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DescribeEvents
#[derive(Debug, PartialEq)]
pub enum DescribeEventsError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DescribeEventsError {
    pub fn from_response(res: BufferedHttpResponse) -> DescribeEventsError {
        {
            let reader = EventReader::new(res.body.as_slice());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        DescribeEventsError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DescribeEventsError {
    fn from(err: XmlParseError) -> DescribeEventsError {
        let XmlParseError(message) = err;
        DescribeEventsError::ParseError(message.to_string())
    }
}
impl From<CredentialsError> for DescribeEventsError {
    fn from(err: CredentialsError) -> DescribeEventsError {
        DescribeEventsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeEventsError {
    fn from(err: HttpDispatchError) -> DescribeEventsError {
        DescribeEventsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeEventsError {
    fn from(err: io::Error) -> DescribeEventsError {
        DescribeEventsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeEventsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeEventsError {
    fn description(&self) -> &str {
        match *self {
            DescribeEventsError::Validation(ref cause) => cause,
            DescribeEventsError::Credentials(ref err) => err.description(),
            DescribeEventsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeEventsError::ParseError(ref cause) => cause,
            DescribeEventsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DescribeOrderableDBInstanceOptions
#[derive(Debug, PartialEq)]
pub enum DescribeOrderableDBInstanceOptionsError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DescribeOrderableDBInstanceOptionsError {
    pub fn from_response(res: BufferedHttpResponse) -> DescribeOrderableDBInstanceOptionsError {
        {
            let reader = EventReader::new(res.body.as_slice());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        DescribeOrderableDBInstanceOptionsError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DescribeOrderableDBInstanceOptionsError {
    fn from(err: XmlParseError) -> DescribeOrderableDBInstanceOptionsError {
        let XmlParseError(message) = err;
        DescribeOrderableDBInstanceOptionsError::ParseError(message.to_string())
    }
}
impl From<CredentialsError> for DescribeOrderableDBInstanceOptionsError {
    fn from(err: CredentialsError) -> DescribeOrderableDBInstanceOptionsError {
        DescribeOrderableDBInstanceOptionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeOrderableDBInstanceOptionsError {
    fn from(err: HttpDispatchError) -> DescribeOrderableDBInstanceOptionsError {
        DescribeOrderableDBInstanceOptionsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeOrderableDBInstanceOptionsError {
    fn from(err: io::Error) -> DescribeOrderableDBInstanceOptionsError {
        DescribeOrderableDBInstanceOptionsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeOrderableDBInstanceOptionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeOrderableDBInstanceOptionsError {
    fn description(&self) -> &str {
        match *self {
            DescribeOrderableDBInstanceOptionsError::Validation(ref cause) => cause,
            DescribeOrderableDBInstanceOptionsError::Credentials(ref err) => err.description(),
            DescribeOrderableDBInstanceOptionsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeOrderableDBInstanceOptionsError::ParseError(ref cause) => cause,
            DescribeOrderableDBInstanceOptionsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DescribePendingMaintenanceActions
#[derive(Debug, PartialEq)]
pub enum DescribePendingMaintenanceActionsError {
    /// <p>The specified resource ID was not found.</p>
    ResourceNotFoundFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DescribePendingMaintenanceActionsError {
    pub fn from_response(res: BufferedHttpResponse) -> DescribePendingMaintenanceActionsError {
        {
            let reader = EventReader::new(res.body.as_slice());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "ResourceNotFoundFault" => {
                        return DescribePendingMaintenanceActionsError::ResourceNotFoundFault(
                            String::from(parsed_error.message),
                        )
                    }
                    _ => {}
                }
            }
        }
        DescribePendingMaintenanceActionsError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DescribePendingMaintenanceActionsError {
    fn from(err: XmlParseError) -> DescribePendingMaintenanceActionsError {
        let XmlParseError(message) = err;
        DescribePendingMaintenanceActionsError::ParseError(message.to_string())
    }
}
impl From<CredentialsError> for DescribePendingMaintenanceActionsError {
    fn from(err: CredentialsError) -> DescribePendingMaintenanceActionsError {
        DescribePendingMaintenanceActionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribePendingMaintenanceActionsError {
    fn from(err: HttpDispatchError) -> DescribePendingMaintenanceActionsError {
        DescribePendingMaintenanceActionsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribePendingMaintenanceActionsError {
    fn from(err: io::Error) -> DescribePendingMaintenanceActionsError {
        DescribePendingMaintenanceActionsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribePendingMaintenanceActionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribePendingMaintenanceActionsError {
    fn description(&self) -> &str {
        match *self {
            DescribePendingMaintenanceActionsError::ResourceNotFoundFault(ref cause) => cause,
            DescribePendingMaintenanceActionsError::Validation(ref cause) => cause,
            DescribePendingMaintenanceActionsError::Credentials(ref err) => err.description(),
            DescribePendingMaintenanceActionsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribePendingMaintenanceActionsError::ParseError(ref cause) => cause,
            DescribePendingMaintenanceActionsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DescribeValidDBInstanceModifications
#[derive(Debug, PartialEq)]
pub enum DescribeValidDBInstanceModificationsError {
    /// <p> <i>DBInstanceIdentifier</i> does not refer to an existing DB instance. </p>
    DBInstanceNotFoundFault(String),
    /// <p> The specified DB instance is not in the <i>available</i> state. </p>
    InvalidDBInstanceStateFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DescribeValidDBInstanceModificationsError {
    pub fn from_response(res: BufferedHttpResponse) -> DescribeValidDBInstanceModificationsError {
        {
            let reader = EventReader::new(res.body.as_slice());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                                    "DBInstanceNotFound" => return DescribeValidDBInstanceModificationsError::DBInstanceNotFoundFault(String::from(parsed_error.message)),"InvalidDBInstanceState" => return DescribeValidDBInstanceModificationsError::InvalidDBInstanceStateFault(String::from(parsed_error.message)),_ => {}
                                }
            }
        }
        DescribeValidDBInstanceModificationsError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DescribeValidDBInstanceModificationsError {
    fn from(err: XmlParseError) -> DescribeValidDBInstanceModificationsError {
        let XmlParseError(message) = err;
        DescribeValidDBInstanceModificationsError::ParseError(message.to_string())
    }
}
impl From<CredentialsError> for DescribeValidDBInstanceModificationsError {
    fn from(err: CredentialsError) -> DescribeValidDBInstanceModificationsError {
        DescribeValidDBInstanceModificationsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeValidDBInstanceModificationsError {
    fn from(err: HttpDispatchError) -> DescribeValidDBInstanceModificationsError {
        DescribeValidDBInstanceModificationsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeValidDBInstanceModificationsError {
    fn from(err: io::Error) -> DescribeValidDBInstanceModificationsError {
        DescribeValidDBInstanceModificationsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeValidDBInstanceModificationsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeValidDBInstanceModificationsError {
    fn description(&self) -> &str {
        match *self {
            DescribeValidDBInstanceModificationsError::DBInstanceNotFoundFault(ref cause) => cause,
            DescribeValidDBInstanceModificationsError::InvalidDBInstanceStateFault(ref cause) => {
                cause
            }
            DescribeValidDBInstanceModificationsError::Validation(ref cause) => cause,
            DescribeValidDBInstanceModificationsError::Credentials(ref err) => err.description(),
            DescribeValidDBInstanceModificationsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeValidDBInstanceModificationsError::ParseError(ref cause) => cause,
            DescribeValidDBInstanceModificationsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by FailoverDBCluster
#[derive(Debug, PartialEq)]
pub enum FailoverDBClusterError {
    /// <p> <i>DBClusterIdentifier</i> does not refer to an existing DB cluster. </p>
    DBClusterNotFoundFault(String),
    /// <p>The DB cluster is not in a valid state.</p>
    InvalidDBClusterStateFault(String),
    /// <p> The specified DB instance is not in the <i>available</i> state. </p>
    InvalidDBInstanceStateFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl FailoverDBClusterError {
    pub fn from_response(res: BufferedHttpResponse) -> FailoverDBClusterError {
        {
            let reader = EventReader::new(res.body.as_slice());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "DBClusterNotFoundFault" => {
                        return FailoverDBClusterError::DBClusterNotFoundFault(String::from(
                            parsed_error.message,
                        ))
                    }
                    "InvalidDBClusterStateFault" => {
                        return FailoverDBClusterError::InvalidDBClusterStateFault(String::from(
                            parsed_error.message,
                        ))
                    }
                    "InvalidDBInstanceState" => {
                        return FailoverDBClusterError::InvalidDBInstanceStateFault(String::from(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        FailoverDBClusterError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for FailoverDBClusterError {
    fn from(err: XmlParseError) -> FailoverDBClusterError {
        let XmlParseError(message) = err;
        FailoverDBClusterError::ParseError(message.to_string())
    }
}
impl From<CredentialsError> for FailoverDBClusterError {
    fn from(err: CredentialsError) -> FailoverDBClusterError {
        FailoverDBClusterError::Credentials(err)
    }
}
impl From<HttpDispatchError> for FailoverDBClusterError {
    fn from(err: HttpDispatchError) -> FailoverDBClusterError {
        FailoverDBClusterError::HttpDispatch(err)
    }
}
impl From<io::Error> for FailoverDBClusterError {
    fn from(err: io::Error) -> FailoverDBClusterError {
        FailoverDBClusterError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for FailoverDBClusterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for FailoverDBClusterError {
    fn description(&self) -> &str {
        match *self {
            FailoverDBClusterError::DBClusterNotFoundFault(ref cause) => cause,
            FailoverDBClusterError::InvalidDBClusterStateFault(ref cause) => cause,
            FailoverDBClusterError::InvalidDBInstanceStateFault(ref cause) => cause,
            FailoverDBClusterError::Validation(ref cause) => cause,
            FailoverDBClusterError::Credentials(ref err) => err.description(),
            FailoverDBClusterError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            FailoverDBClusterError::ParseError(ref cause) => cause,
            FailoverDBClusterError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    /// <p> <i>DBClusterIdentifier</i> does not refer to an existing DB cluster. </p>
    DBClusterNotFoundFault(String),
    /// <p> <i>DBInstanceIdentifier</i> does not refer to an existing DB instance. </p>
    DBInstanceNotFoundFault(String),
    /// <p> <i>DBSnapshotIdentifier</i> does not refer to an existing DB snapshot. </p>
    DBSnapshotNotFoundFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl ListTagsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> ListTagsForResourceError {
        {
            let reader = EventReader::new(res.body.as_slice());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "DBClusterNotFoundFault" => {
                        return ListTagsForResourceError::DBClusterNotFoundFault(String::from(
                            parsed_error.message,
                        ))
                    }
                    "DBInstanceNotFound" => {
                        return ListTagsForResourceError::DBInstanceNotFoundFault(String::from(
                            parsed_error.message,
                        ))
                    }
                    "DBSnapshotNotFound" => {
                        return ListTagsForResourceError::DBSnapshotNotFoundFault(String::from(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        ListTagsForResourceError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for ListTagsForResourceError {
    fn from(err: XmlParseError) -> ListTagsForResourceError {
        let XmlParseError(message) = err;
        ListTagsForResourceError::ParseError(message.to_string())
    }
}
impl From<CredentialsError> for ListTagsForResourceError {
    fn from(err: CredentialsError) -> ListTagsForResourceError {
        ListTagsForResourceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListTagsForResourceError {
    fn from(err: HttpDispatchError) -> ListTagsForResourceError {
        ListTagsForResourceError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListTagsForResourceError {
    fn from(err: io::Error) -> ListTagsForResourceError {
        ListTagsForResourceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListTagsForResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListTagsForResourceError {
    fn description(&self) -> &str {
        match *self {
            ListTagsForResourceError::DBClusterNotFoundFault(ref cause) => cause,
            ListTagsForResourceError::DBInstanceNotFoundFault(ref cause) => cause,
            ListTagsForResourceError::DBSnapshotNotFoundFault(ref cause) => cause,
            ListTagsForResourceError::Validation(ref cause) => cause,
            ListTagsForResourceError::Credentials(ref err) => err.description(),
            ListTagsForResourceError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListTagsForResourceError::ParseError(ref cause) => cause,
            ListTagsForResourceError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ModifyDBCluster
#[derive(Debug, PartialEq)]
pub enum ModifyDBClusterError {
    /// <p>User already has a DB cluster with the given identifier.</p>
    DBClusterAlreadyExistsFault(String),
    /// <p> <i>DBClusterIdentifier</i> does not refer to an existing DB cluster. </p>
    DBClusterNotFoundFault(String),
    /// <p> <i>DBClusterParameterGroupName</i> does not refer to an existing DB Cluster parameter group. </p>
    DBClusterParameterGroupNotFoundFault(String),
    /// <p> <i>DBSubnetGroupName</i> does not refer to an existing DB subnet group. </p>
    DBSubnetGroupNotFoundFault(String),
    /// <p>The DB cluster is not in a valid state.</p>
    InvalidDBClusterStateFault(String),
    /// <p> The specified DB instance is not in the <i>available</i> state. </p>
    InvalidDBInstanceStateFault(String),
    /// <p>The state of the DB security group does not allow deletion.</p>
    InvalidDBSecurityGroupStateFault(String),
    /// <p>The DB subnet group cannot be deleted because it is in use.</p>
    InvalidDBSubnetGroupStateFault(String),
    /// <p>The requested subnet is invalid, or multiple subnets were requested that are not all in a common VPC.</p>
    InvalidSubnet(String),
    /// <p>DB subnet group does not cover all Availability Zones after it is created because users' change.</p>
    InvalidVPCNetworkStateFault(String),
    /// <p>Request would result in user exceeding the allowed amount of storage available across all DB instances.</p>
    StorageQuotaExceededFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl ModifyDBClusterError {
    pub fn from_response(res: BufferedHttpResponse) -> ModifyDBClusterError {
        {
            let reader = EventReader::new(res.body.as_slice());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "DBClusterAlreadyExistsFault" => {
                        return ModifyDBClusterError::DBClusterAlreadyExistsFault(String::from(
                            parsed_error.message,
                        ))
                    }
                    "DBClusterNotFoundFault" => {
                        return ModifyDBClusterError::DBClusterNotFoundFault(String::from(
                            parsed_error.message,
                        ))
                    }
                    "DBClusterParameterGroupNotFound" => {
                        return ModifyDBClusterError::DBClusterParameterGroupNotFoundFault(
                            String::from(parsed_error.message),
                        )
                    }
                    "DBSubnetGroupNotFoundFault" => {
                        return ModifyDBClusterError::DBSubnetGroupNotFoundFault(String::from(
                            parsed_error.message,
                        ))
                    }
                    "InvalidDBClusterStateFault" => {
                        return ModifyDBClusterError::InvalidDBClusterStateFault(String::from(
                            parsed_error.message,
                        ))
                    }
                    "InvalidDBInstanceState" => {
                        return ModifyDBClusterError::InvalidDBInstanceStateFault(String::from(
                            parsed_error.message,
                        ))
                    }
                    "InvalidDBSecurityGroupState" => {
                        return ModifyDBClusterError::InvalidDBSecurityGroupStateFault(String::from(
                            parsed_error.message,
                        ))
                    }
                    "InvalidDBSubnetGroupStateFault" => {
                        return ModifyDBClusterError::InvalidDBSubnetGroupStateFault(String::from(
                            parsed_error.message,
                        ))
                    }
                    "InvalidSubnet" => {
                        return ModifyDBClusterError::InvalidSubnet(String::from(
                            parsed_error.message,
                        ))
                    }
                    "InvalidVPCNetworkStateFault" => {
                        return ModifyDBClusterError::InvalidVPCNetworkStateFault(String::from(
                            parsed_error.message,
                        ))
                    }
                    "StorageQuotaExceeded" => {
                        return ModifyDBClusterError::StorageQuotaExceededFault(String::from(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        ModifyDBClusterError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for ModifyDBClusterError {
    fn from(err: XmlParseError) -> ModifyDBClusterError {
        let XmlParseError(message) = err;
        ModifyDBClusterError::ParseError(message.to_string())
    }
}
impl From<CredentialsError> for ModifyDBClusterError {
    fn from(err: CredentialsError) -> ModifyDBClusterError {
        ModifyDBClusterError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ModifyDBClusterError {
    fn from(err: HttpDispatchError) -> ModifyDBClusterError {
        ModifyDBClusterError::HttpDispatch(err)
    }
}
impl From<io::Error> for ModifyDBClusterError {
    fn from(err: io::Error) -> ModifyDBClusterError {
        ModifyDBClusterError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ModifyDBClusterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ModifyDBClusterError {
    fn description(&self) -> &str {
        match *self {
            ModifyDBClusterError::DBClusterAlreadyExistsFault(ref cause) => cause,
            ModifyDBClusterError::DBClusterNotFoundFault(ref cause) => cause,
            ModifyDBClusterError::DBClusterParameterGroupNotFoundFault(ref cause) => cause,
            ModifyDBClusterError::DBSubnetGroupNotFoundFault(ref cause) => cause,
            ModifyDBClusterError::InvalidDBClusterStateFault(ref cause) => cause,
            ModifyDBClusterError::InvalidDBInstanceStateFault(ref cause) => cause,
            ModifyDBClusterError::InvalidDBSecurityGroupStateFault(ref cause) => cause,
            ModifyDBClusterError::InvalidDBSubnetGroupStateFault(ref cause) => cause,
            ModifyDBClusterError::InvalidSubnet(ref cause) => cause,
            ModifyDBClusterError::InvalidVPCNetworkStateFault(ref cause) => cause,
            ModifyDBClusterError::StorageQuotaExceededFault(ref cause) => cause,
            ModifyDBClusterError::Validation(ref cause) => cause,
            ModifyDBClusterError::Credentials(ref err) => err.description(),
            ModifyDBClusterError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ModifyDBClusterError::ParseError(ref cause) => cause,
            ModifyDBClusterError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ModifyDBClusterParameterGroup
#[derive(Debug, PartialEq)]
pub enum ModifyDBClusterParameterGroupError {
    /// <p> <i>DBParameterGroupName</i> does not refer to an existing DB parameter group. </p>
    DBParameterGroupNotFoundFault(String),
    /// <p>The DB parameter group is in use or is in an invalid state. If you are attempting to delete the parameter group, you cannot delete it when the parameter group is in this state.</p>
    InvalidDBParameterGroupStateFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl ModifyDBClusterParameterGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> ModifyDBClusterParameterGroupError {
        {
            let reader = EventReader::new(res.body.as_slice());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "DBParameterGroupNotFound" => {
                        return ModifyDBClusterParameterGroupError::DBParameterGroupNotFoundFault(
                            String::from(parsed_error.message),
                        )
                    }
                    "InvalidDBParameterGroupState" => {
                        return ModifyDBClusterParameterGroupError::InvalidDBParameterGroupStateFault(
                            String::from(parsed_error.message),
                        )
                    }
                    _ => {}
                }
            }
        }
        ModifyDBClusterParameterGroupError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for ModifyDBClusterParameterGroupError {
    fn from(err: XmlParseError) -> ModifyDBClusterParameterGroupError {
        let XmlParseError(message) = err;
        ModifyDBClusterParameterGroupError::ParseError(message.to_string())
    }
}
impl From<CredentialsError> for ModifyDBClusterParameterGroupError {
    fn from(err: CredentialsError) -> ModifyDBClusterParameterGroupError {
        ModifyDBClusterParameterGroupError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ModifyDBClusterParameterGroupError {
    fn from(err: HttpDispatchError) -> ModifyDBClusterParameterGroupError {
        ModifyDBClusterParameterGroupError::HttpDispatch(err)
    }
}
impl From<io::Error> for ModifyDBClusterParameterGroupError {
    fn from(err: io::Error) -> ModifyDBClusterParameterGroupError {
        ModifyDBClusterParameterGroupError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ModifyDBClusterParameterGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ModifyDBClusterParameterGroupError {
    fn description(&self) -> &str {
        match *self {
            ModifyDBClusterParameterGroupError::DBParameterGroupNotFoundFault(ref cause) => cause,
            ModifyDBClusterParameterGroupError::InvalidDBParameterGroupStateFault(ref cause) => {
                cause
            }
            ModifyDBClusterParameterGroupError::Validation(ref cause) => cause,
            ModifyDBClusterParameterGroupError::Credentials(ref err) => err.description(),
            ModifyDBClusterParameterGroupError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ModifyDBClusterParameterGroupError::ParseError(ref cause) => cause,
            ModifyDBClusterParameterGroupError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ModifyDBClusterSnapshotAttribute
#[derive(Debug, PartialEq)]
pub enum ModifyDBClusterSnapshotAttributeError {
    /// <p> <i>DBClusterSnapshotIdentifier</i> does not refer to an existing DB cluster snapshot. </p>
    DBClusterSnapshotNotFoundFault(String),
    /// <p>The supplied value is not a valid DB cluster snapshot state.</p>
    InvalidDBClusterSnapshotStateFault(String),
    /// <p>You have exceeded the maximum number of accounts that you can share a manual DB snapshot with.</p>
    SharedSnapshotQuotaExceededFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl ModifyDBClusterSnapshotAttributeError {
    pub fn from_response(res: BufferedHttpResponse) -> ModifyDBClusterSnapshotAttributeError {
        {
            let reader = EventReader::new(res.body.as_slice());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                                    "DBClusterSnapshotNotFoundFault" => return ModifyDBClusterSnapshotAttributeError::DBClusterSnapshotNotFoundFault(String::from(parsed_error.message)),"InvalidDBClusterSnapshotStateFault" => return ModifyDBClusterSnapshotAttributeError::InvalidDBClusterSnapshotStateFault(String::from(parsed_error.message)),"SharedSnapshotQuotaExceeded" => return ModifyDBClusterSnapshotAttributeError::SharedSnapshotQuotaExceededFault(String::from(parsed_error.message)),_ => {}
                                }
            }
        }
        ModifyDBClusterSnapshotAttributeError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for ModifyDBClusterSnapshotAttributeError {
    fn from(err: XmlParseError) -> ModifyDBClusterSnapshotAttributeError {
        let XmlParseError(message) = err;
        ModifyDBClusterSnapshotAttributeError::ParseError(message.to_string())
    }
}
impl From<CredentialsError> for ModifyDBClusterSnapshotAttributeError {
    fn from(err: CredentialsError) -> ModifyDBClusterSnapshotAttributeError {
        ModifyDBClusterSnapshotAttributeError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ModifyDBClusterSnapshotAttributeError {
    fn from(err: HttpDispatchError) -> ModifyDBClusterSnapshotAttributeError {
        ModifyDBClusterSnapshotAttributeError::HttpDispatch(err)
    }
}
impl From<io::Error> for ModifyDBClusterSnapshotAttributeError {
    fn from(err: io::Error) -> ModifyDBClusterSnapshotAttributeError {
        ModifyDBClusterSnapshotAttributeError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ModifyDBClusterSnapshotAttributeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ModifyDBClusterSnapshotAttributeError {
    fn description(&self) -> &str {
        match *self {
            ModifyDBClusterSnapshotAttributeError::DBClusterSnapshotNotFoundFault(ref cause) => {
                cause
            }
            ModifyDBClusterSnapshotAttributeError::InvalidDBClusterSnapshotStateFault(
                ref cause,
            ) => cause,
            ModifyDBClusterSnapshotAttributeError::SharedSnapshotQuotaExceededFault(ref cause) => {
                cause
            }
            ModifyDBClusterSnapshotAttributeError::Validation(ref cause) => cause,
            ModifyDBClusterSnapshotAttributeError::Credentials(ref err) => err.description(),
            ModifyDBClusterSnapshotAttributeError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ModifyDBClusterSnapshotAttributeError::ParseError(ref cause) => cause,
            ModifyDBClusterSnapshotAttributeError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ModifyDBInstance
#[derive(Debug, PartialEq)]
pub enum ModifyDBInstanceError {
    /// <p>Specified CIDRIP or EC2 security group is not authorized for the specified DB security group.</p> <p>Neptune may not also be authorized via IAM to perform necessary actions on your behalf.</p>
    AuthorizationNotFoundFault(String),
    /// <p> <i>CertificateIdentifier</i> does not refer to an existing certificate. </p>
    CertificateNotFoundFault(String),
    /// <p>User already has a DB instance with the given identifier.</p>
    DBInstanceAlreadyExistsFault(String),
    /// <p> <i>DBInstanceIdentifier</i> does not refer to an existing DB instance. </p>
    DBInstanceNotFoundFault(String),
    /// <p> <i>DBParameterGroupName</i> does not refer to an existing DB parameter group. </p>
    DBParameterGroupNotFoundFault(String),
    /// <p> <i>DBSecurityGroupName</i> does not refer to an existing DB security group. </p>
    DBSecurityGroupNotFoundFault(String),
    /// <p>The DB upgrade failed because a resource the DB depends on could not be modified.</p>
    DBUpgradeDependencyFailureFault(String),
    /// <p> <i>Domain</i> does not refer to an existing Active Directory Domain. </p>
    DomainNotFoundFault(String),
    /// <p>Specified DB instance class is not available in the specified Availability Zone.</p>
    InsufficientDBInstanceCapacityFault(String),
    /// <p> The specified DB instance is not in the <i>available</i> state. </p>
    InvalidDBInstanceStateFault(String),
    /// <p>The state of the DB security group does not allow deletion.</p>
    InvalidDBSecurityGroupStateFault(String),
    /// <p>DB subnet group does not cover all Availability Zones after it is created because users' change.</p>
    InvalidVPCNetworkStateFault(String),

    OptionGroupNotFoundFault(String),
    /// <p>Provisioned IOPS not available in the specified Availability Zone.</p>
    ProvisionedIopsNotAvailableInAZFault(String),
    /// <p>Request would result in user exceeding the allowed amount of storage available across all DB instances.</p>
    StorageQuotaExceededFault(String),
    /// <p> <i>StorageType</i> specified cannot be associated with the DB Instance. </p>
    StorageTypeNotSupportedFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl ModifyDBInstanceError {
    pub fn from_response(res: BufferedHttpResponse) -> ModifyDBInstanceError {
        {
            let reader = EventReader::new(res.body.as_slice());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "AuthorizationNotFound" => {
                        return ModifyDBInstanceError::AuthorizationNotFoundFault(String::from(
                            parsed_error.message,
                        ))
                    }
                    "CertificateNotFound" => {
                        return ModifyDBInstanceError::CertificateNotFoundFault(String::from(
                            parsed_error.message,
                        ))
                    }
                    "DBInstanceAlreadyExists" => {
                        return ModifyDBInstanceError::DBInstanceAlreadyExistsFault(String::from(
                            parsed_error.message,
                        ))
                    }
                    "DBInstanceNotFound" => {
                        return ModifyDBInstanceError::DBInstanceNotFoundFault(String::from(
                            parsed_error.message,
                        ))
                    }
                    "DBParameterGroupNotFound" => {
                        return ModifyDBInstanceError::DBParameterGroupNotFoundFault(String::from(
                            parsed_error.message,
                        ))
                    }
                    "DBSecurityGroupNotFound" => {
                        return ModifyDBInstanceError::DBSecurityGroupNotFoundFault(String::from(
                            parsed_error.message,
                        ))
                    }
                    "DBUpgradeDependencyFailure" => {
                        return ModifyDBInstanceError::DBUpgradeDependencyFailureFault(String::from(
                            parsed_error.message,
                        ))
                    }
                    "DomainNotFoundFault" => {
                        return ModifyDBInstanceError::DomainNotFoundFault(String::from(
                            parsed_error.message,
                        ))
                    }
                    "InsufficientDBInstanceCapacity" => {
                        return ModifyDBInstanceError::InsufficientDBInstanceCapacityFault(
                            String::from(parsed_error.message),
                        )
                    }
                    "InvalidDBInstanceState" => {
                        return ModifyDBInstanceError::InvalidDBInstanceStateFault(String::from(
                            parsed_error.message,
                        ))
                    }
                    "InvalidDBSecurityGroupState" => {
                        return ModifyDBInstanceError::InvalidDBSecurityGroupStateFault(
                            String::from(parsed_error.message),
                        )
                    }
                    "InvalidVPCNetworkStateFault" => {
                        return ModifyDBInstanceError::InvalidVPCNetworkStateFault(String::from(
                            parsed_error.message,
                        ))
                    }
                    "OptionGroupNotFoundFault" => {
                        return ModifyDBInstanceError::OptionGroupNotFoundFault(String::from(
                            parsed_error.message,
                        ))
                    }
                    "ProvisionedIopsNotAvailableInAZFault" => {
                        return ModifyDBInstanceError::ProvisionedIopsNotAvailableInAZFault(
                            String::from(parsed_error.message),
                        )
                    }
                    "StorageQuotaExceeded" => {
                        return ModifyDBInstanceError::StorageQuotaExceededFault(String::from(
                            parsed_error.message,
                        ))
                    }
                    "StorageTypeNotSupported" => {
                        return ModifyDBInstanceError::StorageTypeNotSupportedFault(String::from(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        ModifyDBInstanceError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for ModifyDBInstanceError {
    fn from(err: XmlParseError) -> ModifyDBInstanceError {
        let XmlParseError(message) = err;
        ModifyDBInstanceError::ParseError(message.to_string())
    }
}
impl From<CredentialsError> for ModifyDBInstanceError {
    fn from(err: CredentialsError) -> ModifyDBInstanceError {
        ModifyDBInstanceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ModifyDBInstanceError {
    fn from(err: HttpDispatchError) -> ModifyDBInstanceError {
        ModifyDBInstanceError::HttpDispatch(err)
    }
}
impl From<io::Error> for ModifyDBInstanceError {
    fn from(err: io::Error) -> ModifyDBInstanceError {
        ModifyDBInstanceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ModifyDBInstanceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ModifyDBInstanceError {
    fn description(&self) -> &str {
        match *self {
            ModifyDBInstanceError::AuthorizationNotFoundFault(ref cause) => cause,
            ModifyDBInstanceError::CertificateNotFoundFault(ref cause) => cause,
            ModifyDBInstanceError::DBInstanceAlreadyExistsFault(ref cause) => cause,
            ModifyDBInstanceError::DBInstanceNotFoundFault(ref cause) => cause,
            ModifyDBInstanceError::DBParameterGroupNotFoundFault(ref cause) => cause,
            ModifyDBInstanceError::DBSecurityGroupNotFoundFault(ref cause) => cause,
            ModifyDBInstanceError::DBUpgradeDependencyFailureFault(ref cause) => cause,
            ModifyDBInstanceError::DomainNotFoundFault(ref cause) => cause,
            ModifyDBInstanceError::InsufficientDBInstanceCapacityFault(ref cause) => cause,
            ModifyDBInstanceError::InvalidDBInstanceStateFault(ref cause) => cause,
            ModifyDBInstanceError::InvalidDBSecurityGroupStateFault(ref cause) => cause,
            ModifyDBInstanceError::InvalidVPCNetworkStateFault(ref cause) => cause,
            ModifyDBInstanceError::OptionGroupNotFoundFault(ref cause) => cause,
            ModifyDBInstanceError::ProvisionedIopsNotAvailableInAZFault(ref cause) => cause,
            ModifyDBInstanceError::StorageQuotaExceededFault(ref cause) => cause,
            ModifyDBInstanceError::StorageTypeNotSupportedFault(ref cause) => cause,
            ModifyDBInstanceError::Validation(ref cause) => cause,
            ModifyDBInstanceError::Credentials(ref err) => err.description(),
            ModifyDBInstanceError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ModifyDBInstanceError::ParseError(ref cause) => cause,
            ModifyDBInstanceError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ModifyDBParameterGroup
#[derive(Debug, PartialEq)]
pub enum ModifyDBParameterGroupError {
    /// <p> <i>DBParameterGroupName</i> does not refer to an existing DB parameter group. </p>
    DBParameterGroupNotFoundFault(String),
    /// <p>The DB parameter group is in use or is in an invalid state. If you are attempting to delete the parameter group, you cannot delete it when the parameter group is in this state.</p>
    InvalidDBParameterGroupStateFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl ModifyDBParameterGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> ModifyDBParameterGroupError {
        {
            let reader = EventReader::new(res.body.as_slice());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "DBParameterGroupNotFound" => {
                        return ModifyDBParameterGroupError::DBParameterGroupNotFoundFault(
                            String::from(parsed_error.message),
                        )
                    }
                    "InvalidDBParameterGroupState" => {
                        return ModifyDBParameterGroupError::InvalidDBParameterGroupStateFault(
                            String::from(parsed_error.message),
                        )
                    }
                    _ => {}
                }
            }
        }
        ModifyDBParameterGroupError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for ModifyDBParameterGroupError {
    fn from(err: XmlParseError) -> ModifyDBParameterGroupError {
        let XmlParseError(message) = err;
        ModifyDBParameterGroupError::ParseError(message.to_string())
    }
}
impl From<CredentialsError> for ModifyDBParameterGroupError {
    fn from(err: CredentialsError) -> ModifyDBParameterGroupError {
        ModifyDBParameterGroupError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ModifyDBParameterGroupError {
    fn from(err: HttpDispatchError) -> ModifyDBParameterGroupError {
        ModifyDBParameterGroupError::HttpDispatch(err)
    }
}
impl From<io::Error> for ModifyDBParameterGroupError {
    fn from(err: io::Error) -> ModifyDBParameterGroupError {
        ModifyDBParameterGroupError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ModifyDBParameterGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ModifyDBParameterGroupError {
    fn description(&self) -> &str {
        match *self {
            ModifyDBParameterGroupError::DBParameterGroupNotFoundFault(ref cause) => cause,
            ModifyDBParameterGroupError::InvalidDBParameterGroupStateFault(ref cause) => cause,
            ModifyDBParameterGroupError::Validation(ref cause) => cause,
            ModifyDBParameterGroupError::Credentials(ref err) => err.description(),
            ModifyDBParameterGroupError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ModifyDBParameterGroupError::ParseError(ref cause) => cause,
            ModifyDBParameterGroupError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ModifyDBSubnetGroup
#[derive(Debug, PartialEq)]
pub enum ModifyDBSubnetGroupError {
    /// <p>Subnets in the DB subnet group should cover at least two Availability Zones unless there is only one Availability Zone.</p>
    DBSubnetGroupDoesNotCoverEnoughAZs(String),
    /// <p> <i>DBSubnetGroupName</i> does not refer to an existing DB subnet group. </p>
    DBSubnetGroupNotFoundFault(String),
    /// <p>Request would result in user exceeding the allowed number of subnets in a DB subnet groups.</p>
    DBSubnetQuotaExceededFault(String),
    /// <p>The requested subnet is invalid, or multiple subnets were requested that are not all in a common VPC.</p>
    InvalidSubnet(String),
    /// <p>The DB subnet is already in use in the Availability Zone.</p>
    SubnetAlreadyInUse(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl ModifyDBSubnetGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> ModifyDBSubnetGroupError {
        {
            let reader = EventReader::new(res.body.as_slice());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "DBSubnetGroupDoesNotCoverEnoughAZs" => {
                        return ModifyDBSubnetGroupError::DBSubnetGroupDoesNotCoverEnoughAZs(
                            String::from(parsed_error.message),
                        )
                    }
                    "DBSubnetGroupNotFoundFault" => {
                        return ModifyDBSubnetGroupError::DBSubnetGroupNotFoundFault(String::from(
                            parsed_error.message,
                        ))
                    }
                    "DBSubnetQuotaExceededFault" => {
                        return ModifyDBSubnetGroupError::DBSubnetQuotaExceededFault(String::from(
                            parsed_error.message,
                        ))
                    }
                    "InvalidSubnet" => {
                        return ModifyDBSubnetGroupError::InvalidSubnet(String::from(
                            parsed_error.message,
                        ))
                    }
                    "SubnetAlreadyInUse" => {
                        return ModifyDBSubnetGroupError::SubnetAlreadyInUse(String::from(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        ModifyDBSubnetGroupError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for ModifyDBSubnetGroupError {
    fn from(err: XmlParseError) -> ModifyDBSubnetGroupError {
        let XmlParseError(message) = err;
        ModifyDBSubnetGroupError::ParseError(message.to_string())
    }
}
impl From<CredentialsError> for ModifyDBSubnetGroupError {
    fn from(err: CredentialsError) -> ModifyDBSubnetGroupError {
        ModifyDBSubnetGroupError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ModifyDBSubnetGroupError {
    fn from(err: HttpDispatchError) -> ModifyDBSubnetGroupError {
        ModifyDBSubnetGroupError::HttpDispatch(err)
    }
}
impl From<io::Error> for ModifyDBSubnetGroupError {
    fn from(err: io::Error) -> ModifyDBSubnetGroupError {
        ModifyDBSubnetGroupError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ModifyDBSubnetGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ModifyDBSubnetGroupError {
    fn description(&self) -> &str {
        match *self {
            ModifyDBSubnetGroupError::DBSubnetGroupDoesNotCoverEnoughAZs(ref cause) => cause,
            ModifyDBSubnetGroupError::DBSubnetGroupNotFoundFault(ref cause) => cause,
            ModifyDBSubnetGroupError::DBSubnetQuotaExceededFault(ref cause) => cause,
            ModifyDBSubnetGroupError::InvalidSubnet(ref cause) => cause,
            ModifyDBSubnetGroupError::SubnetAlreadyInUse(ref cause) => cause,
            ModifyDBSubnetGroupError::Validation(ref cause) => cause,
            ModifyDBSubnetGroupError::Credentials(ref err) => err.description(),
            ModifyDBSubnetGroupError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ModifyDBSubnetGroupError::ParseError(ref cause) => cause,
            ModifyDBSubnetGroupError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ModifyEventSubscription
#[derive(Debug, PartialEq)]
pub enum ModifyEventSubscriptionError {
    EventSubscriptionQuotaExceededFault(String),

    SNSInvalidTopicFault(String),

    SNSNoAuthorizationFault(String),

    SNSTopicArnNotFoundFault(String),

    SubscriptionCategoryNotFoundFault(String),

    SubscriptionNotFoundFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl ModifyEventSubscriptionError {
    pub fn from_response(res: BufferedHttpResponse) -> ModifyEventSubscriptionError {
        {
            let reader = EventReader::new(res.body.as_slice());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "EventSubscriptionQuotaExceeded" => {
                        return ModifyEventSubscriptionError::EventSubscriptionQuotaExceededFault(
                            String::from(parsed_error.message),
                        )
                    }
                    "SNSInvalidTopic" => {
                        return ModifyEventSubscriptionError::SNSInvalidTopicFault(String::from(
                            parsed_error.message,
                        ))
                    }
                    "SNSNoAuthorization" => {
                        return ModifyEventSubscriptionError::SNSNoAuthorizationFault(String::from(
                            parsed_error.message,
                        ))
                    }
                    "SNSTopicArnNotFound" => {
                        return ModifyEventSubscriptionError::SNSTopicArnNotFoundFault(String::from(
                            parsed_error.message,
                        ))
                    }
                    "SubscriptionCategoryNotFound" => {
                        return ModifyEventSubscriptionError::SubscriptionCategoryNotFoundFault(
                            String::from(parsed_error.message),
                        )
                    }
                    "SubscriptionNotFound" => {
                        return ModifyEventSubscriptionError::SubscriptionNotFoundFault(
                            String::from(parsed_error.message),
                        )
                    }
                    _ => {}
                }
            }
        }
        ModifyEventSubscriptionError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for ModifyEventSubscriptionError {
    fn from(err: XmlParseError) -> ModifyEventSubscriptionError {
        let XmlParseError(message) = err;
        ModifyEventSubscriptionError::ParseError(message.to_string())
    }
}
impl From<CredentialsError> for ModifyEventSubscriptionError {
    fn from(err: CredentialsError) -> ModifyEventSubscriptionError {
        ModifyEventSubscriptionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ModifyEventSubscriptionError {
    fn from(err: HttpDispatchError) -> ModifyEventSubscriptionError {
        ModifyEventSubscriptionError::HttpDispatch(err)
    }
}
impl From<io::Error> for ModifyEventSubscriptionError {
    fn from(err: io::Error) -> ModifyEventSubscriptionError {
        ModifyEventSubscriptionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ModifyEventSubscriptionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ModifyEventSubscriptionError {
    fn description(&self) -> &str {
        match *self {
            ModifyEventSubscriptionError::EventSubscriptionQuotaExceededFault(ref cause) => cause,
            ModifyEventSubscriptionError::SNSInvalidTopicFault(ref cause) => cause,
            ModifyEventSubscriptionError::SNSNoAuthorizationFault(ref cause) => cause,
            ModifyEventSubscriptionError::SNSTopicArnNotFoundFault(ref cause) => cause,
            ModifyEventSubscriptionError::SubscriptionCategoryNotFoundFault(ref cause) => cause,
            ModifyEventSubscriptionError::SubscriptionNotFoundFault(ref cause) => cause,
            ModifyEventSubscriptionError::Validation(ref cause) => cause,
            ModifyEventSubscriptionError::Credentials(ref err) => err.description(),
            ModifyEventSubscriptionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ModifyEventSubscriptionError::ParseError(ref cause) => cause,
            ModifyEventSubscriptionError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by PromoteReadReplicaDBCluster
#[derive(Debug, PartialEq)]
pub enum PromoteReadReplicaDBClusterError {
    /// <p> <i>DBClusterIdentifier</i> does not refer to an existing DB cluster. </p>
    DBClusterNotFoundFault(String),
    /// <p>The DB cluster is not in a valid state.</p>
    InvalidDBClusterStateFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl PromoteReadReplicaDBClusterError {
    pub fn from_response(res: BufferedHttpResponse) -> PromoteReadReplicaDBClusterError {
        {
            let reader = EventReader::new(res.body.as_slice());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "DBClusterNotFoundFault" => {
                        return PromoteReadReplicaDBClusterError::DBClusterNotFoundFault(
                            String::from(parsed_error.message),
                        )
                    }
                    "InvalidDBClusterStateFault" => {
                        return PromoteReadReplicaDBClusterError::InvalidDBClusterStateFault(
                            String::from(parsed_error.message),
                        )
                    }
                    _ => {}
                }
            }
        }
        PromoteReadReplicaDBClusterError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for PromoteReadReplicaDBClusterError {
    fn from(err: XmlParseError) -> PromoteReadReplicaDBClusterError {
        let XmlParseError(message) = err;
        PromoteReadReplicaDBClusterError::ParseError(message.to_string())
    }
}
impl From<CredentialsError> for PromoteReadReplicaDBClusterError {
    fn from(err: CredentialsError) -> PromoteReadReplicaDBClusterError {
        PromoteReadReplicaDBClusterError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PromoteReadReplicaDBClusterError {
    fn from(err: HttpDispatchError) -> PromoteReadReplicaDBClusterError {
        PromoteReadReplicaDBClusterError::HttpDispatch(err)
    }
}
impl From<io::Error> for PromoteReadReplicaDBClusterError {
    fn from(err: io::Error) -> PromoteReadReplicaDBClusterError {
        PromoteReadReplicaDBClusterError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PromoteReadReplicaDBClusterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PromoteReadReplicaDBClusterError {
    fn description(&self) -> &str {
        match *self {
            PromoteReadReplicaDBClusterError::DBClusterNotFoundFault(ref cause) => cause,
            PromoteReadReplicaDBClusterError::InvalidDBClusterStateFault(ref cause) => cause,
            PromoteReadReplicaDBClusterError::Validation(ref cause) => cause,
            PromoteReadReplicaDBClusterError::Credentials(ref err) => err.description(),
            PromoteReadReplicaDBClusterError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            PromoteReadReplicaDBClusterError::ParseError(ref cause) => cause,
            PromoteReadReplicaDBClusterError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by RebootDBInstance
#[derive(Debug, PartialEq)]
pub enum RebootDBInstanceError {
    /// <p> <i>DBInstanceIdentifier</i> does not refer to an existing DB instance. </p>
    DBInstanceNotFoundFault(String),
    /// <p> The specified DB instance is not in the <i>available</i> state. </p>
    InvalidDBInstanceStateFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl RebootDBInstanceError {
    pub fn from_response(res: BufferedHttpResponse) -> RebootDBInstanceError {
        {
            let reader = EventReader::new(res.body.as_slice());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "DBInstanceNotFound" => {
                        return RebootDBInstanceError::DBInstanceNotFoundFault(String::from(
                            parsed_error.message,
                        ))
                    }
                    "InvalidDBInstanceState" => {
                        return RebootDBInstanceError::InvalidDBInstanceStateFault(String::from(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        RebootDBInstanceError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for RebootDBInstanceError {
    fn from(err: XmlParseError) -> RebootDBInstanceError {
        let XmlParseError(message) = err;
        RebootDBInstanceError::ParseError(message.to_string())
    }
}
impl From<CredentialsError> for RebootDBInstanceError {
    fn from(err: CredentialsError) -> RebootDBInstanceError {
        RebootDBInstanceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RebootDBInstanceError {
    fn from(err: HttpDispatchError) -> RebootDBInstanceError {
        RebootDBInstanceError::HttpDispatch(err)
    }
}
impl From<io::Error> for RebootDBInstanceError {
    fn from(err: io::Error) -> RebootDBInstanceError {
        RebootDBInstanceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for RebootDBInstanceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RebootDBInstanceError {
    fn description(&self) -> &str {
        match *self {
            RebootDBInstanceError::DBInstanceNotFoundFault(ref cause) => cause,
            RebootDBInstanceError::InvalidDBInstanceStateFault(ref cause) => cause,
            RebootDBInstanceError::Validation(ref cause) => cause,
            RebootDBInstanceError::Credentials(ref err) => err.description(),
            RebootDBInstanceError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            RebootDBInstanceError::ParseError(ref cause) => cause,
            RebootDBInstanceError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by RemoveRoleFromDBCluster
#[derive(Debug, PartialEq)]
pub enum RemoveRoleFromDBClusterError {
    /// <p> <i>DBClusterIdentifier</i> does not refer to an existing DB cluster. </p>
    DBClusterNotFoundFault(String),
    /// <p>The specified IAM role Amazon Resource Name (ARN) is not associated with the specified DB cluster.</p>
    DBClusterRoleNotFoundFault(String),
    /// <p>The DB cluster is not in a valid state.</p>
    InvalidDBClusterStateFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl RemoveRoleFromDBClusterError {
    pub fn from_response(res: BufferedHttpResponse) -> RemoveRoleFromDBClusterError {
        {
            let reader = EventReader::new(res.body.as_slice());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "DBClusterNotFoundFault" => {
                        return RemoveRoleFromDBClusterError::DBClusterNotFoundFault(String::from(
                            parsed_error.message,
                        ))
                    }
                    "DBClusterRoleNotFound" => {
                        return RemoveRoleFromDBClusterError::DBClusterRoleNotFoundFault(
                            String::from(parsed_error.message),
                        )
                    }
                    "InvalidDBClusterStateFault" => {
                        return RemoveRoleFromDBClusterError::InvalidDBClusterStateFault(
                            String::from(parsed_error.message),
                        )
                    }
                    _ => {}
                }
            }
        }
        RemoveRoleFromDBClusterError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for RemoveRoleFromDBClusterError {
    fn from(err: XmlParseError) -> RemoveRoleFromDBClusterError {
        let XmlParseError(message) = err;
        RemoveRoleFromDBClusterError::ParseError(message.to_string())
    }
}
impl From<CredentialsError> for RemoveRoleFromDBClusterError {
    fn from(err: CredentialsError) -> RemoveRoleFromDBClusterError {
        RemoveRoleFromDBClusterError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RemoveRoleFromDBClusterError {
    fn from(err: HttpDispatchError) -> RemoveRoleFromDBClusterError {
        RemoveRoleFromDBClusterError::HttpDispatch(err)
    }
}
impl From<io::Error> for RemoveRoleFromDBClusterError {
    fn from(err: io::Error) -> RemoveRoleFromDBClusterError {
        RemoveRoleFromDBClusterError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for RemoveRoleFromDBClusterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RemoveRoleFromDBClusterError {
    fn description(&self) -> &str {
        match *self {
            RemoveRoleFromDBClusterError::DBClusterNotFoundFault(ref cause) => cause,
            RemoveRoleFromDBClusterError::DBClusterRoleNotFoundFault(ref cause) => cause,
            RemoveRoleFromDBClusterError::InvalidDBClusterStateFault(ref cause) => cause,
            RemoveRoleFromDBClusterError::Validation(ref cause) => cause,
            RemoveRoleFromDBClusterError::Credentials(ref err) => err.description(),
            RemoveRoleFromDBClusterError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            RemoveRoleFromDBClusterError::ParseError(ref cause) => cause,
            RemoveRoleFromDBClusterError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by RemoveSourceIdentifierFromSubscription
#[derive(Debug, PartialEq)]
pub enum RemoveSourceIdentifierFromSubscriptionError {
    SourceNotFoundFault(String),

    SubscriptionNotFoundFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl RemoveSourceIdentifierFromSubscriptionError {
    pub fn from_response(res: BufferedHttpResponse) -> RemoveSourceIdentifierFromSubscriptionError {
        {
            let reader = EventReader::new(res.body.as_slice());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                                    "SourceNotFound" => return RemoveSourceIdentifierFromSubscriptionError::SourceNotFoundFault(String::from(parsed_error.message)),"SubscriptionNotFound" => return RemoveSourceIdentifierFromSubscriptionError::SubscriptionNotFoundFault(String::from(parsed_error.message)),_ => {}
                                }
            }
        }
        RemoveSourceIdentifierFromSubscriptionError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for RemoveSourceIdentifierFromSubscriptionError {
    fn from(err: XmlParseError) -> RemoveSourceIdentifierFromSubscriptionError {
        let XmlParseError(message) = err;
        RemoveSourceIdentifierFromSubscriptionError::ParseError(message.to_string())
    }
}
impl From<CredentialsError> for RemoveSourceIdentifierFromSubscriptionError {
    fn from(err: CredentialsError) -> RemoveSourceIdentifierFromSubscriptionError {
        RemoveSourceIdentifierFromSubscriptionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RemoveSourceIdentifierFromSubscriptionError {
    fn from(err: HttpDispatchError) -> RemoveSourceIdentifierFromSubscriptionError {
        RemoveSourceIdentifierFromSubscriptionError::HttpDispatch(err)
    }
}
impl From<io::Error> for RemoveSourceIdentifierFromSubscriptionError {
    fn from(err: io::Error) -> RemoveSourceIdentifierFromSubscriptionError {
        RemoveSourceIdentifierFromSubscriptionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for RemoveSourceIdentifierFromSubscriptionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RemoveSourceIdentifierFromSubscriptionError {
    fn description(&self) -> &str {
        match *self {
            RemoveSourceIdentifierFromSubscriptionError::SourceNotFoundFault(ref cause) => cause,
            RemoveSourceIdentifierFromSubscriptionError::SubscriptionNotFoundFault(ref cause) => {
                cause
            }
            RemoveSourceIdentifierFromSubscriptionError::Validation(ref cause) => cause,
            RemoveSourceIdentifierFromSubscriptionError::Credentials(ref err) => err.description(),
            RemoveSourceIdentifierFromSubscriptionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            RemoveSourceIdentifierFromSubscriptionError::ParseError(ref cause) => cause,
            RemoveSourceIdentifierFromSubscriptionError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by RemoveTagsFromResource
#[derive(Debug, PartialEq)]
pub enum RemoveTagsFromResourceError {
    /// <p> <i>DBClusterIdentifier</i> does not refer to an existing DB cluster. </p>
    DBClusterNotFoundFault(String),
    /// <p> <i>DBInstanceIdentifier</i> does not refer to an existing DB instance. </p>
    DBInstanceNotFoundFault(String),
    /// <p> <i>DBSnapshotIdentifier</i> does not refer to an existing DB snapshot. </p>
    DBSnapshotNotFoundFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl RemoveTagsFromResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RemoveTagsFromResourceError {
        {
            let reader = EventReader::new(res.body.as_slice());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "DBClusterNotFoundFault" => {
                        return RemoveTagsFromResourceError::DBClusterNotFoundFault(String::from(
                            parsed_error.message,
                        ))
                    }
                    "DBInstanceNotFound" => {
                        return RemoveTagsFromResourceError::DBInstanceNotFoundFault(String::from(
                            parsed_error.message,
                        ))
                    }
                    "DBSnapshotNotFound" => {
                        return RemoveTagsFromResourceError::DBSnapshotNotFoundFault(String::from(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        RemoveTagsFromResourceError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for RemoveTagsFromResourceError {
    fn from(err: XmlParseError) -> RemoveTagsFromResourceError {
        let XmlParseError(message) = err;
        RemoveTagsFromResourceError::ParseError(message.to_string())
    }
}
impl From<CredentialsError> for RemoveTagsFromResourceError {
    fn from(err: CredentialsError) -> RemoveTagsFromResourceError {
        RemoveTagsFromResourceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RemoveTagsFromResourceError {
    fn from(err: HttpDispatchError) -> RemoveTagsFromResourceError {
        RemoveTagsFromResourceError::HttpDispatch(err)
    }
}
impl From<io::Error> for RemoveTagsFromResourceError {
    fn from(err: io::Error) -> RemoveTagsFromResourceError {
        RemoveTagsFromResourceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for RemoveTagsFromResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RemoveTagsFromResourceError {
    fn description(&self) -> &str {
        match *self {
            RemoveTagsFromResourceError::DBClusterNotFoundFault(ref cause) => cause,
            RemoveTagsFromResourceError::DBInstanceNotFoundFault(ref cause) => cause,
            RemoveTagsFromResourceError::DBSnapshotNotFoundFault(ref cause) => cause,
            RemoveTagsFromResourceError::Validation(ref cause) => cause,
            RemoveTagsFromResourceError::Credentials(ref err) => err.description(),
            RemoveTagsFromResourceError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            RemoveTagsFromResourceError::ParseError(ref cause) => cause,
            RemoveTagsFromResourceError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ResetDBClusterParameterGroup
#[derive(Debug, PartialEq)]
pub enum ResetDBClusterParameterGroupError {
    /// <p> <i>DBParameterGroupName</i> does not refer to an existing DB parameter group. </p>
    DBParameterGroupNotFoundFault(String),
    /// <p>The DB parameter group is in use or is in an invalid state. If you are attempting to delete the parameter group, you cannot delete it when the parameter group is in this state.</p>
    InvalidDBParameterGroupStateFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl ResetDBClusterParameterGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> ResetDBClusterParameterGroupError {
        {
            let reader = EventReader::new(res.body.as_slice());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "DBParameterGroupNotFound" => {
                        return ResetDBClusterParameterGroupError::DBParameterGroupNotFoundFault(
                            String::from(parsed_error.message),
                        )
                    }
                    "InvalidDBParameterGroupState" => {
                        return ResetDBClusterParameterGroupError::InvalidDBParameterGroupStateFault(
                            String::from(parsed_error.message),
                        )
                    }
                    _ => {}
                }
            }
        }
        ResetDBClusterParameterGroupError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for ResetDBClusterParameterGroupError {
    fn from(err: XmlParseError) -> ResetDBClusterParameterGroupError {
        let XmlParseError(message) = err;
        ResetDBClusterParameterGroupError::ParseError(message.to_string())
    }
}
impl From<CredentialsError> for ResetDBClusterParameterGroupError {
    fn from(err: CredentialsError) -> ResetDBClusterParameterGroupError {
        ResetDBClusterParameterGroupError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ResetDBClusterParameterGroupError {
    fn from(err: HttpDispatchError) -> ResetDBClusterParameterGroupError {
        ResetDBClusterParameterGroupError::HttpDispatch(err)
    }
}
impl From<io::Error> for ResetDBClusterParameterGroupError {
    fn from(err: io::Error) -> ResetDBClusterParameterGroupError {
        ResetDBClusterParameterGroupError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ResetDBClusterParameterGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ResetDBClusterParameterGroupError {
    fn description(&self) -> &str {
        match *self {
            ResetDBClusterParameterGroupError::DBParameterGroupNotFoundFault(ref cause) => cause,
            ResetDBClusterParameterGroupError::InvalidDBParameterGroupStateFault(ref cause) => {
                cause
            }
            ResetDBClusterParameterGroupError::Validation(ref cause) => cause,
            ResetDBClusterParameterGroupError::Credentials(ref err) => err.description(),
            ResetDBClusterParameterGroupError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ResetDBClusterParameterGroupError::ParseError(ref cause) => cause,
            ResetDBClusterParameterGroupError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ResetDBParameterGroup
#[derive(Debug, PartialEq)]
pub enum ResetDBParameterGroupError {
    /// <p> <i>DBParameterGroupName</i> does not refer to an existing DB parameter group. </p>
    DBParameterGroupNotFoundFault(String),
    /// <p>The DB parameter group is in use or is in an invalid state. If you are attempting to delete the parameter group, you cannot delete it when the parameter group is in this state.</p>
    InvalidDBParameterGroupStateFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl ResetDBParameterGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> ResetDBParameterGroupError {
        {
            let reader = EventReader::new(res.body.as_slice());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "DBParameterGroupNotFound" => {
                        return ResetDBParameterGroupError::DBParameterGroupNotFoundFault(
                            String::from(parsed_error.message),
                        )
                    }
                    "InvalidDBParameterGroupState" => {
                        return ResetDBParameterGroupError::InvalidDBParameterGroupStateFault(
                            String::from(parsed_error.message),
                        )
                    }
                    _ => {}
                }
            }
        }
        ResetDBParameterGroupError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for ResetDBParameterGroupError {
    fn from(err: XmlParseError) -> ResetDBParameterGroupError {
        let XmlParseError(message) = err;
        ResetDBParameterGroupError::ParseError(message.to_string())
    }
}
impl From<CredentialsError> for ResetDBParameterGroupError {
    fn from(err: CredentialsError) -> ResetDBParameterGroupError {
        ResetDBParameterGroupError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ResetDBParameterGroupError {
    fn from(err: HttpDispatchError) -> ResetDBParameterGroupError {
        ResetDBParameterGroupError::HttpDispatch(err)
    }
}
impl From<io::Error> for ResetDBParameterGroupError {
    fn from(err: io::Error) -> ResetDBParameterGroupError {
        ResetDBParameterGroupError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ResetDBParameterGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ResetDBParameterGroupError {
    fn description(&self) -> &str {
        match *self {
            ResetDBParameterGroupError::DBParameterGroupNotFoundFault(ref cause) => cause,
            ResetDBParameterGroupError::InvalidDBParameterGroupStateFault(ref cause) => cause,
            ResetDBParameterGroupError::Validation(ref cause) => cause,
            ResetDBParameterGroupError::Credentials(ref err) => err.description(),
            ResetDBParameterGroupError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ResetDBParameterGroupError::ParseError(ref cause) => cause,
            ResetDBParameterGroupError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by RestoreDBClusterFromSnapshot
#[derive(Debug, PartialEq)]
pub enum RestoreDBClusterFromSnapshotError {
    /// <p>User already has a DB cluster with the given identifier.</p>
    DBClusterAlreadyExistsFault(String),
    /// <p>User attempted to create a new DB cluster and the user has already reached the maximum allowed DB cluster quota.</p>
    DBClusterQuotaExceededFault(String),
    /// <p> <i>DBClusterSnapshotIdentifier</i> does not refer to an existing DB cluster snapshot. </p>
    DBClusterSnapshotNotFoundFault(String),
    /// <p> <i>DBSnapshotIdentifier</i> does not refer to an existing DB snapshot. </p>
    DBSnapshotNotFoundFault(String),
    /// <p> <i>DBSubnetGroupName</i> does not refer to an existing DB subnet group. </p>
    DBSubnetGroupNotFoundFault(String),
    /// <p>The DB cluster does not have enough capacity for the current operation.</p>
    InsufficientDBClusterCapacityFault(String),
    /// <p>There is insufficient storage available for the current action. You may be able to resolve this error by updating your subnet group to use different Availability Zones that have more storage available.</p>
    InsufficientStorageClusterCapacityFault(String),
    /// <p>The supplied value is not a valid DB cluster snapshot state.</p>
    InvalidDBClusterSnapshotStateFault(String),
    /// <p>The state of the DB snapshot does not allow deletion.</p>
    InvalidDBSnapshotStateFault(String),
    /// <p>Cannot restore from vpc backup to non-vpc DB instance.</p>
    InvalidRestoreFault(String),
    /// <p>The requested subnet is invalid, or multiple subnets were requested that are not all in a common VPC.</p>
    InvalidSubnet(String),
    /// <p>DB subnet group does not cover all Availability Zones after it is created because users' change.</p>
    InvalidVPCNetworkStateFault(String),
    /// <p>Error accessing KMS key.</p>
    KMSKeyNotAccessibleFault(String),

    OptionGroupNotFoundFault(String),
    /// <p>Request would result in user exceeding the allowed amount of storage available across all DB instances.</p>
    StorageQuotaExceededFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl RestoreDBClusterFromSnapshotError {
    pub fn from_response(res: BufferedHttpResponse) -> RestoreDBClusterFromSnapshotError {
        {
            let reader = EventReader::new(res.body.as_slice());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                                    "DBClusterAlreadyExistsFault" => return RestoreDBClusterFromSnapshotError::DBClusterAlreadyExistsFault(String::from(parsed_error.message)),"DBClusterQuotaExceededFault" => return RestoreDBClusterFromSnapshotError::DBClusterQuotaExceededFault(String::from(parsed_error.message)),"DBClusterSnapshotNotFoundFault" => return RestoreDBClusterFromSnapshotError::DBClusterSnapshotNotFoundFault(String::from(parsed_error.message)),"DBSnapshotNotFound" => return RestoreDBClusterFromSnapshotError::DBSnapshotNotFoundFault(String::from(parsed_error.message)),"DBSubnetGroupNotFoundFault" => return RestoreDBClusterFromSnapshotError::DBSubnetGroupNotFoundFault(String::from(parsed_error.message)),"InsufficientDBClusterCapacityFault" => return RestoreDBClusterFromSnapshotError::InsufficientDBClusterCapacityFault(String::from(parsed_error.message)),"InsufficientStorageClusterCapacity" => return RestoreDBClusterFromSnapshotError::InsufficientStorageClusterCapacityFault(String::from(parsed_error.message)),"InvalidDBClusterSnapshotStateFault" => return RestoreDBClusterFromSnapshotError::InvalidDBClusterSnapshotStateFault(String::from(parsed_error.message)),"InvalidDBSnapshotState" => return RestoreDBClusterFromSnapshotError::InvalidDBSnapshotStateFault(String::from(parsed_error.message)),"InvalidRestoreFault" => return RestoreDBClusterFromSnapshotError::InvalidRestoreFault(String::from(parsed_error.message)),"InvalidSubnet" => return RestoreDBClusterFromSnapshotError::InvalidSubnet(String::from(parsed_error.message)),"InvalidVPCNetworkStateFault" => return RestoreDBClusterFromSnapshotError::InvalidVPCNetworkStateFault(String::from(parsed_error.message)),"KMSKeyNotAccessibleFault" => return RestoreDBClusterFromSnapshotError::KMSKeyNotAccessibleFault(String::from(parsed_error.message)),"OptionGroupNotFoundFault" => return RestoreDBClusterFromSnapshotError::OptionGroupNotFoundFault(String::from(parsed_error.message)),"StorageQuotaExceeded" => return RestoreDBClusterFromSnapshotError::StorageQuotaExceededFault(String::from(parsed_error.message)),_ => {}
                                }
            }
        }
        RestoreDBClusterFromSnapshotError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for RestoreDBClusterFromSnapshotError {
    fn from(err: XmlParseError) -> RestoreDBClusterFromSnapshotError {
        let XmlParseError(message) = err;
        RestoreDBClusterFromSnapshotError::ParseError(message.to_string())
    }
}
impl From<CredentialsError> for RestoreDBClusterFromSnapshotError {
    fn from(err: CredentialsError) -> RestoreDBClusterFromSnapshotError {
        RestoreDBClusterFromSnapshotError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RestoreDBClusterFromSnapshotError {
    fn from(err: HttpDispatchError) -> RestoreDBClusterFromSnapshotError {
        RestoreDBClusterFromSnapshotError::HttpDispatch(err)
    }
}
impl From<io::Error> for RestoreDBClusterFromSnapshotError {
    fn from(err: io::Error) -> RestoreDBClusterFromSnapshotError {
        RestoreDBClusterFromSnapshotError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for RestoreDBClusterFromSnapshotError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RestoreDBClusterFromSnapshotError {
    fn description(&self) -> &str {
        match *self {
            RestoreDBClusterFromSnapshotError::DBClusterAlreadyExistsFault(ref cause) => cause,
            RestoreDBClusterFromSnapshotError::DBClusterQuotaExceededFault(ref cause) => cause,
            RestoreDBClusterFromSnapshotError::DBClusterSnapshotNotFoundFault(ref cause) => cause,
            RestoreDBClusterFromSnapshotError::DBSnapshotNotFoundFault(ref cause) => cause,
            RestoreDBClusterFromSnapshotError::DBSubnetGroupNotFoundFault(ref cause) => cause,
            RestoreDBClusterFromSnapshotError::InsufficientDBClusterCapacityFault(ref cause) => {
                cause
            }
            RestoreDBClusterFromSnapshotError::InsufficientStorageClusterCapacityFault(
                ref cause,
            ) => cause,
            RestoreDBClusterFromSnapshotError::InvalidDBClusterSnapshotStateFault(ref cause) => {
                cause
            }
            RestoreDBClusterFromSnapshotError::InvalidDBSnapshotStateFault(ref cause) => cause,
            RestoreDBClusterFromSnapshotError::InvalidRestoreFault(ref cause) => cause,
            RestoreDBClusterFromSnapshotError::InvalidSubnet(ref cause) => cause,
            RestoreDBClusterFromSnapshotError::InvalidVPCNetworkStateFault(ref cause) => cause,
            RestoreDBClusterFromSnapshotError::KMSKeyNotAccessibleFault(ref cause) => cause,
            RestoreDBClusterFromSnapshotError::OptionGroupNotFoundFault(ref cause) => cause,
            RestoreDBClusterFromSnapshotError::StorageQuotaExceededFault(ref cause) => cause,
            RestoreDBClusterFromSnapshotError::Validation(ref cause) => cause,
            RestoreDBClusterFromSnapshotError::Credentials(ref err) => err.description(),
            RestoreDBClusterFromSnapshotError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            RestoreDBClusterFromSnapshotError::ParseError(ref cause) => cause,
            RestoreDBClusterFromSnapshotError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by RestoreDBClusterToPointInTime
#[derive(Debug, PartialEq)]
pub enum RestoreDBClusterToPointInTimeError {
    /// <p>User already has a DB cluster with the given identifier.</p>
    DBClusterAlreadyExistsFault(String),
    /// <p> <i>DBClusterIdentifier</i> does not refer to an existing DB cluster. </p>
    DBClusterNotFoundFault(String),
    /// <p>User attempted to create a new DB cluster and the user has already reached the maximum allowed DB cluster quota.</p>
    DBClusterQuotaExceededFault(String),
    /// <p> <i>DBClusterSnapshotIdentifier</i> does not refer to an existing DB cluster snapshot. </p>
    DBClusterSnapshotNotFoundFault(String),
    /// <p> <i>DBSubnetGroupName</i> does not refer to an existing DB subnet group. </p>
    DBSubnetGroupNotFoundFault(String),
    /// <p>The DB cluster does not have enough capacity for the current operation.</p>
    InsufficientDBClusterCapacityFault(String),
    /// <p>There is insufficient storage available for the current action. You may be able to resolve this error by updating your subnet group to use different Availability Zones that have more storage available.</p>
    InsufficientStorageClusterCapacityFault(String),
    /// <p>The supplied value is not a valid DB cluster snapshot state.</p>
    InvalidDBClusterSnapshotStateFault(String),
    /// <p>The DB cluster is not in a valid state.</p>
    InvalidDBClusterStateFault(String),
    /// <p>The state of the DB snapshot does not allow deletion.</p>
    InvalidDBSnapshotStateFault(String),
    /// <p>Cannot restore from vpc backup to non-vpc DB instance.</p>
    InvalidRestoreFault(String),
    /// <p>The requested subnet is invalid, or multiple subnets were requested that are not all in a common VPC.</p>
    InvalidSubnet(String),
    /// <p>DB subnet group does not cover all Availability Zones after it is created because users' change.</p>
    InvalidVPCNetworkStateFault(String),
    /// <p>Error accessing KMS key.</p>
    KMSKeyNotAccessibleFault(String),

    OptionGroupNotFoundFault(String),
    /// <p>Request would result in user exceeding the allowed amount of storage available across all DB instances.</p>
    StorageQuotaExceededFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl RestoreDBClusterToPointInTimeError {
    pub fn from_response(res: BufferedHttpResponse) -> RestoreDBClusterToPointInTimeError {
        {
            let reader = EventReader::new(res.body.as_slice());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                                    "DBClusterAlreadyExistsFault" => return RestoreDBClusterToPointInTimeError::DBClusterAlreadyExistsFault(String::from(parsed_error.message)),"DBClusterNotFoundFault" => return RestoreDBClusterToPointInTimeError::DBClusterNotFoundFault(String::from(parsed_error.message)),"DBClusterQuotaExceededFault" => return RestoreDBClusterToPointInTimeError::DBClusterQuotaExceededFault(String::from(parsed_error.message)),"DBClusterSnapshotNotFoundFault" => return RestoreDBClusterToPointInTimeError::DBClusterSnapshotNotFoundFault(String::from(parsed_error.message)),"DBSubnetGroupNotFoundFault" => return RestoreDBClusterToPointInTimeError::DBSubnetGroupNotFoundFault(String::from(parsed_error.message)),"InsufficientDBClusterCapacityFault" => return RestoreDBClusterToPointInTimeError::InsufficientDBClusterCapacityFault(String::from(parsed_error.message)),"InsufficientStorageClusterCapacity" => return RestoreDBClusterToPointInTimeError::InsufficientStorageClusterCapacityFault(String::from(parsed_error.message)),"InvalidDBClusterSnapshotStateFault" => return RestoreDBClusterToPointInTimeError::InvalidDBClusterSnapshotStateFault(String::from(parsed_error.message)),"InvalidDBClusterStateFault" => return RestoreDBClusterToPointInTimeError::InvalidDBClusterStateFault(String::from(parsed_error.message)),"InvalidDBSnapshotState" => return RestoreDBClusterToPointInTimeError::InvalidDBSnapshotStateFault(String::from(parsed_error.message)),"InvalidRestoreFault" => return RestoreDBClusterToPointInTimeError::InvalidRestoreFault(String::from(parsed_error.message)),"InvalidSubnet" => return RestoreDBClusterToPointInTimeError::InvalidSubnet(String::from(parsed_error.message)),"InvalidVPCNetworkStateFault" => return RestoreDBClusterToPointInTimeError::InvalidVPCNetworkStateFault(String::from(parsed_error.message)),"KMSKeyNotAccessibleFault" => return RestoreDBClusterToPointInTimeError::KMSKeyNotAccessibleFault(String::from(parsed_error.message)),"OptionGroupNotFoundFault" => return RestoreDBClusterToPointInTimeError::OptionGroupNotFoundFault(String::from(parsed_error.message)),"StorageQuotaExceeded" => return RestoreDBClusterToPointInTimeError::StorageQuotaExceededFault(String::from(parsed_error.message)),_ => {}
                                }
            }
        }
        RestoreDBClusterToPointInTimeError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for RestoreDBClusterToPointInTimeError {
    fn from(err: XmlParseError) -> RestoreDBClusterToPointInTimeError {
        let XmlParseError(message) = err;
        RestoreDBClusterToPointInTimeError::ParseError(message.to_string())
    }
}
impl From<CredentialsError> for RestoreDBClusterToPointInTimeError {
    fn from(err: CredentialsError) -> RestoreDBClusterToPointInTimeError {
        RestoreDBClusterToPointInTimeError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RestoreDBClusterToPointInTimeError {
    fn from(err: HttpDispatchError) -> RestoreDBClusterToPointInTimeError {
        RestoreDBClusterToPointInTimeError::HttpDispatch(err)
    }
}
impl From<io::Error> for RestoreDBClusterToPointInTimeError {
    fn from(err: io::Error) -> RestoreDBClusterToPointInTimeError {
        RestoreDBClusterToPointInTimeError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for RestoreDBClusterToPointInTimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RestoreDBClusterToPointInTimeError {
    fn description(&self) -> &str {
        match *self {
            RestoreDBClusterToPointInTimeError::DBClusterAlreadyExistsFault(ref cause) => cause,
            RestoreDBClusterToPointInTimeError::DBClusterNotFoundFault(ref cause) => cause,
            RestoreDBClusterToPointInTimeError::DBClusterQuotaExceededFault(ref cause) => cause,
            RestoreDBClusterToPointInTimeError::DBClusterSnapshotNotFoundFault(ref cause) => cause,
            RestoreDBClusterToPointInTimeError::DBSubnetGroupNotFoundFault(ref cause) => cause,
            RestoreDBClusterToPointInTimeError::InsufficientDBClusterCapacityFault(ref cause) => {
                cause
            }
            RestoreDBClusterToPointInTimeError::InsufficientStorageClusterCapacityFault(
                ref cause,
            ) => cause,
            RestoreDBClusterToPointInTimeError::InvalidDBClusterSnapshotStateFault(ref cause) => {
                cause
            }
            RestoreDBClusterToPointInTimeError::InvalidDBClusterStateFault(ref cause) => cause,
            RestoreDBClusterToPointInTimeError::InvalidDBSnapshotStateFault(ref cause) => cause,
            RestoreDBClusterToPointInTimeError::InvalidRestoreFault(ref cause) => cause,
            RestoreDBClusterToPointInTimeError::InvalidSubnet(ref cause) => cause,
            RestoreDBClusterToPointInTimeError::InvalidVPCNetworkStateFault(ref cause) => cause,
            RestoreDBClusterToPointInTimeError::KMSKeyNotAccessibleFault(ref cause) => cause,
            RestoreDBClusterToPointInTimeError::OptionGroupNotFoundFault(ref cause) => cause,
            RestoreDBClusterToPointInTimeError::StorageQuotaExceededFault(ref cause) => cause,
            RestoreDBClusterToPointInTimeError::Validation(ref cause) => cause,
            RestoreDBClusterToPointInTimeError::Credentials(ref err) => err.description(),
            RestoreDBClusterToPointInTimeError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            RestoreDBClusterToPointInTimeError::ParseError(ref cause) => cause,
            RestoreDBClusterToPointInTimeError::Unknown(_) => "unknown error",
        }
    }
}
/// Trait representing the capabilities of the Amazon Neptune API. Amazon Neptune clients implement this trait.
pub trait Neptune {
    /// <p>Associates an Identity and Access Management (IAM) role from an Neptune DB cluster. </p>
    fn add_role_to_db_cluster(
        &self,
        input: AddRoleToDBClusterMessage,
    ) -> RusotoFuture<(), AddRoleToDBClusterError>;

    /// <p>Adds a source identifier to an existing event notification subscription.</p>
    fn add_source_identifier_to_subscription(
        &self,
        input: AddSourceIdentifierToSubscriptionMessage,
    ) -> RusotoFuture<AddSourceIdentifierToSubscriptionResult, AddSourceIdentifierToSubscriptionError>;

    /// <p>Adds metadata tags to an Amazon Neptune resource. These tags can also be used with cost allocation reporting to track cost associated with Amazon Neptune resources, or used in a Condition statement in an IAM policy for Amazon Neptune.</p>
    fn add_tags_to_resource(
        &self,
        input: AddTagsToResourceMessage,
    ) -> RusotoFuture<(), AddTagsToResourceError>;

    /// <p>Applies a pending maintenance action to a resource (for example, to a DB instance).</p>
    fn apply_pending_maintenance_action(
        &self,
        input: ApplyPendingMaintenanceActionMessage,
    ) -> RusotoFuture<ApplyPendingMaintenanceActionResult, ApplyPendingMaintenanceActionError>;

    /// <p>Copies the specified DB cluster parameter group.</p>
    fn copy_db_cluster_parameter_group(
        &self,
        input: CopyDBClusterParameterGroupMessage,
    ) -> RusotoFuture<CopyDBClusterParameterGroupResult, CopyDBClusterParameterGroupError>;

    /// <p>Copies a snapshot of a DB cluster.</p> <p>To copy a DB cluster snapshot from a shared manual DB cluster snapshot, <code>SourceDBClusterSnapshotIdentifier</code> must be the Amazon Resource Name (ARN) of the shared DB cluster snapshot.</p> <p>You can copy an encrypted DB cluster snapshot from another AWS Region. In that case, the AWS Region where you call the <code>CopyDBClusterSnapshot</code> action is the destination AWS Region for the encrypted DB cluster snapshot to be copied to. To copy an encrypted DB cluster snapshot from another AWS Region, you must provide the following values:</p> <ul> <li> <p> <code>KmsKeyId</code> - The AWS Key Management System (AWS KMS) key identifier for the key to use to encrypt the copy of the DB cluster snapshot in the destination AWS Region.</p> </li> <li> <p> <code>PreSignedUrl</code> - A URL that contains a Signature Version 4 signed request for the <code>CopyDBClusterSnapshot</code> action to be called in the source AWS Region where the DB cluster snapshot is copied from. The pre-signed URL must be a valid request for the <code>CopyDBClusterSnapshot</code> API action that can be executed in the source AWS Region that contains the encrypted DB cluster snapshot to be copied.</p> <p>The pre-signed URL request must contain the following parameter values:</p> <ul> <li> <p> <code>KmsKeyId</code> - The KMS key identifier for the key to use to encrypt the copy of the DB cluster snapshot in the destination AWS Region. This is the same identifier for both the <code>CopyDBClusterSnapshot</code> action that is called in the destination AWS Region, and the action contained in the pre-signed URL.</p> </li> <li> <p> <code>DestinationRegion</code> - The name of the AWS Region that the DB cluster snapshot will be created in.</p> </li> <li> <p> <code>SourceDBClusterSnapshotIdentifier</code> - The DB cluster snapshot identifier for the encrypted DB cluster snapshot to be copied. This identifier must be in the Amazon Resource Name (ARN) format for the source AWS Region. For example, if you are copying an encrypted DB cluster snapshot from the us-west-2 AWS Region, then your <code>SourceDBClusterSnapshotIdentifier</code> looks like the following example: <code>arn:aws:rds:us-west-2:123456789012:cluster-snapshot:neptune-cluster1-snapshot-20161115</code>.</p> </li> </ul> <p>To learn how to generate a Signature Version 4 signed request, see <a href="http://docs.aws.amazon.com/AmazonS3/latest/API/sigv4-query-string-auth.html"> Authenticating Requests: Using Query Parameters (AWS Signature Version 4)</a> and <a href="http://docs.aws.amazon.com/general/latest/gr/signature-version-4.html"> Signature Version 4 Signing Process</a>.</p> </li> <li> <p> <code>TargetDBClusterSnapshotIdentifier</code> - The identifier for the new copy of the DB cluster snapshot in the destination AWS Region.</p> </li> <li> <p> <code>SourceDBClusterSnapshotIdentifier</code> - The DB cluster snapshot identifier for the encrypted DB cluster snapshot to be copied. This identifier must be in the ARN format for the source AWS Region and is the same value as the <code>SourceDBClusterSnapshotIdentifier</code> in the pre-signed URL. </p> </li> </ul> <p>To cancel the copy operation once it is in progress, delete the target DB cluster snapshot identified by <code>TargetDBClusterSnapshotIdentifier</code> while that DB cluster snapshot is in "copying" status.</p>
    fn copy_db_cluster_snapshot(
        &self,
        input: CopyDBClusterSnapshotMessage,
    ) -> RusotoFuture<CopyDBClusterSnapshotResult, CopyDBClusterSnapshotError>;

    /// <p>Copies the specified DB parameter group.</p>
    fn copy_db_parameter_group(
        &self,
        input: CopyDBParameterGroupMessage,
    ) -> RusotoFuture<CopyDBParameterGroupResult, CopyDBParameterGroupError>;

    /// <p>Creates a new Amazon Neptune DB cluster.</p> <p>You can use the <code>ReplicationSourceIdentifier</code> parameter to create the DB cluster as a Read Replica of another DB cluster or Amazon Neptune DB instance. For cross-region replication where the DB cluster identified by <code>ReplicationSourceIdentifier</code> is encrypted, you must also specify the <code>PreSignedUrl</code> parameter.</p>
    fn create_db_cluster(
        &self,
        input: CreateDBClusterMessage,
    ) -> RusotoFuture<CreateDBClusterResult, CreateDBClusterError>;

    /// <p><p>Creates a new DB cluster parameter group.</p> <p>Parameters in a DB cluster parameter group apply to all of the instances in a DB cluster.</p> <p> A DB cluster parameter group is initially created with the default parameters for the database engine used by instances in the DB cluster. To provide custom values for any of the parameters, you must modify the group after creating it using <a>ModifyDBClusterParameterGroup</a>. Once you&#39;ve created a DB cluster parameter group, you need to associate it with your DB cluster using <a>ModifyDBCluster</a>. When you associate a new DB cluster parameter group with a running DB cluster, you need to reboot the DB instances in the DB cluster without failover for the new DB cluster parameter group and associated settings to take effect. </p> <important> <p>After you create a DB cluster parameter group, you should wait at least 5 minutes before creating your first DB cluster that uses that DB cluster parameter group as the default parameter group. This allows Amazon Neptune to fully complete the create action before the DB cluster parameter group is used as the default for a new DB cluster. This is especially important for parameters that are critical when creating the default database for a DB cluster, such as the character set for the default database defined by the <code>character<em>set</em>database</code> parameter. You can use the <i>Parameter Groups</i> option of the <a href="https://console.aws.amazon.com/rds/">Amazon Neptune console</a> or the <a>DescribeDBClusterParameters</a> command to verify that your DB cluster parameter group has been created or modified.</p> </important></p>
    fn create_db_cluster_parameter_group(
        &self,
        input: CreateDBClusterParameterGroupMessage,
    ) -> RusotoFuture<CreateDBClusterParameterGroupResult, CreateDBClusterParameterGroupError>;

    /// <p>Creates a snapshot of a DB cluster. </p>
    fn create_db_cluster_snapshot(
        &self,
        input: CreateDBClusterSnapshotMessage,
    ) -> RusotoFuture<CreateDBClusterSnapshotResult, CreateDBClusterSnapshotError>;

    /// <p>Creates a new DB instance.</p>
    fn create_db_instance(
        &self,
        input: CreateDBInstanceMessage,
    ) -> RusotoFuture<CreateDBInstanceResult, CreateDBInstanceError>;

    /// <p><p>Creates a new DB parameter group.</p> <p> A DB parameter group is initially created with the default parameters for the database engine used by the DB instance. To provide custom values for any of the parameters, you must modify the group after creating it using <i>ModifyDBParameterGroup</i>. Once you&#39;ve created a DB parameter group, you need to associate it with your DB instance using <i>ModifyDBInstance</i>. When you associate a new DB parameter group with a running DB instance, you need to reboot the DB instance without failover for the new DB parameter group and associated settings to take effect. </p> <important> <p>After you create a DB parameter group, you should wait at least 5 minutes before creating your first DB instance that uses that DB parameter group as the default parameter group. This allows Amazon Neptune to fully complete the create action before the parameter group is used as the default for a new DB instance. This is especially important for parameters that are critical when creating the default database for a DB instance, such as the character set for the default database defined by the <code>character<em>set</em>database</code> parameter. You can use the <i>Parameter Groups</i> option of the Amazon Neptune console or the <i>DescribeDBParameters</i> command to verify that your DB parameter group has been created or modified.</p> </important></p>
    fn create_db_parameter_group(
        &self,
        input: CreateDBParameterGroupMessage,
    ) -> RusotoFuture<CreateDBParameterGroupResult, CreateDBParameterGroupError>;

    /// <p>Creates a new DB subnet group. DB subnet groups must contain at least one subnet in at least two AZs in the AWS Region.</p>
    fn create_db_subnet_group(
        &self,
        input: CreateDBSubnetGroupMessage,
    ) -> RusotoFuture<CreateDBSubnetGroupResult, CreateDBSubnetGroupError>;

    /// <p>Creates an event notification subscription. This action requires a topic ARN (Amazon Resource Name) created by either the Neptune console, the SNS console, or the SNS API. To obtain an ARN with SNS, you must create a topic in Amazon SNS and subscribe to the topic. The ARN is displayed in the SNS console.</p> <p>You can specify the type of source (SourceType) you want to be notified of, provide a list of Neptune sources (SourceIds) that triggers the events, and provide a list of event categories (EventCategories) for events you want to be notified of. For example, you can specify SourceType = db-instance, SourceIds = mydbinstance1, mydbinstance2 and EventCategories = Availability, Backup.</p> <p>If you specify both the SourceType and SourceIds, such as SourceType = db-instance and SourceIdentifier = myDBInstance1, you are notified of all the db-instance events for the specified source. If you specify a SourceType but do not specify a SourceIdentifier, you receive notice of the events for that source type for all your Neptune sources. If you do not specify either the SourceType nor the SourceIdentifier, you are notified of events generated from all Neptune sources belonging to your customer account.</p>
    fn create_event_subscription(
        &self,
        input: CreateEventSubscriptionMessage,
    ) -> RusotoFuture<CreateEventSubscriptionResult, CreateEventSubscriptionError>;

    /// <p><p>The DeleteDBCluster action deletes a previously provisioned DB cluster. When you delete a DB cluster, all automated backups for that DB cluster are deleted and can&#39;t be recovered. Manual DB cluster snapshots of the specified DB cluster are not deleted.</p> <p/></p>
    fn delete_db_cluster(
        &self,
        input: DeleteDBClusterMessage,
    ) -> RusotoFuture<DeleteDBClusterResult, DeleteDBClusterError>;

    /// <p>Deletes a specified DB cluster parameter group. The DB cluster parameter group to be deleted can't be associated with any DB clusters.</p>
    fn delete_db_cluster_parameter_group(
        &self,
        input: DeleteDBClusterParameterGroupMessage,
    ) -> RusotoFuture<(), DeleteDBClusterParameterGroupError>;

    /// <p><p>Deletes a DB cluster snapshot. If the snapshot is being copied, the copy operation is terminated.</p> <note> <p>The DB cluster snapshot must be in the <code>available</code> state to be deleted.</p> </note></p>
    fn delete_db_cluster_snapshot(
        &self,
        input: DeleteDBClusterSnapshotMessage,
    ) -> RusotoFuture<DeleteDBClusterSnapshotResult, DeleteDBClusterSnapshotError>;

    /// <p>The DeleteDBInstance action deletes a previously provisioned DB instance. When you delete a DB instance, all automated backups for that instance are deleted and can't be recovered. Manual DB snapshots of the DB instance to be deleted by <code>DeleteDBInstance</code> are not deleted.</p> <p> If you request a final DB snapshot the status of the Amazon Neptune DB instance is <code>deleting</code> until the DB snapshot is created. The API action <code>DescribeDBInstance</code> is used to monitor the status of this operation. The action can't be canceled or reverted once submitted. </p> <p>Note that when a DB instance is in a failure state and has a status of <code>failed</code>, <code>incompatible-restore</code>, or <code>incompatible-network</code>, you can only delete it when the <code>SkipFinalSnapshot</code> parameter is set to <code>true</code>.</p> <p>If the specified DB instance is part of a DB cluster, you can't delete the DB instance if both of the following conditions are true:</p> <ul> <li> <p>The DB cluster is a Read Replica of another DB cluster.</p> </li> <li> <p>The DB instance is the only instance in the DB cluster.</p> </li> </ul> <p>To delete a DB instance in this case, first call the <a>PromoteReadReplicaDBCluster</a> API action to promote the DB cluster so it's no longer a Read Replica. After the promotion completes, then call the <code>DeleteDBInstance</code> API action to delete the final instance in the DB cluster.</p>
    fn delete_db_instance(
        &self,
        input: DeleteDBInstanceMessage,
    ) -> RusotoFuture<DeleteDBInstanceResult, DeleteDBInstanceError>;

    /// <p>Deletes a specified DBParameterGroup. The DBParameterGroup to be deleted can't be associated with any DB instances.</p>
    fn delete_db_parameter_group(
        &self,
        input: DeleteDBParameterGroupMessage,
    ) -> RusotoFuture<(), DeleteDBParameterGroupError>;

    /// <p><p>Deletes a DB subnet group.</p> <note> <p>The specified database subnet group must not be associated with any DB instances.</p> </note></p>
    fn delete_db_subnet_group(
        &self,
        input: DeleteDBSubnetGroupMessage,
    ) -> RusotoFuture<(), DeleteDBSubnetGroupError>;

    /// <p>Deletes an event notification subscription.</p>
    fn delete_event_subscription(
        &self,
        input: DeleteEventSubscriptionMessage,
    ) -> RusotoFuture<DeleteEventSubscriptionResult, DeleteEventSubscriptionError>;

    /// <p> Returns a list of <code>DBClusterParameterGroup</code> descriptions. If a <code>DBClusterParameterGroupName</code> parameter is specified, the list will contain only the description of the specified DB cluster parameter group. </p>
    fn describe_db_cluster_parameter_groups(
        &self,
        input: DescribeDBClusterParameterGroupsMessage,
    ) -> RusotoFuture<DBClusterParameterGroupsMessage, DescribeDBClusterParameterGroupsError>;

    /// <p>Returns the detailed parameter list for a particular DB cluster parameter group.</p>
    fn describe_db_cluster_parameters(
        &self,
        input: DescribeDBClusterParametersMessage,
    ) -> RusotoFuture<DBClusterParameterGroupDetails, DescribeDBClusterParametersError>;

    /// <p>Returns a list of DB cluster snapshot attribute names and values for a manual DB cluster snapshot.</p> <p>When sharing snapshots with other AWS accounts, <code>DescribeDBClusterSnapshotAttributes</code> returns the <code>restore</code> attribute and a list of IDs for the AWS accounts that are authorized to copy or restore the manual DB cluster snapshot. If <code>all</code> is included in the list of values for the <code>restore</code> attribute, then the manual DB cluster snapshot is public and can be copied or restored by all AWS accounts.</p> <p>To add or remove access for an AWS account to copy or restore a manual DB cluster snapshot, or to make the manual DB cluster snapshot public or private, use the <a>ModifyDBClusterSnapshotAttribute</a> API action.</p>
    fn describe_db_cluster_snapshot_attributes(
        &self,
        input: DescribeDBClusterSnapshotAttributesMessage,
    ) -> RusotoFuture<
        DescribeDBClusterSnapshotAttributesResult,
        DescribeDBClusterSnapshotAttributesError,
    >;

    /// <p>Returns information about DB cluster snapshots. This API action supports pagination.</p>
    fn describe_db_cluster_snapshots(
        &self,
        input: DescribeDBClusterSnapshotsMessage,
    ) -> RusotoFuture<DBClusterSnapshotMessage, DescribeDBClusterSnapshotsError>;

    /// <p>Returns information about provisioned DB clusters. This API supports pagination.</p>
    fn describe_db_clusters(
        &self,
        input: DescribeDBClustersMessage,
    ) -> RusotoFuture<DBClusterMessage, DescribeDBClustersError>;

    /// <p>Returns a list of the available DB engines.</p>
    fn describe_db_engine_versions(
        &self,
        input: DescribeDBEngineVersionsMessage,
    ) -> RusotoFuture<DBEngineVersionMessage, DescribeDBEngineVersionsError>;

    /// <p>Returns information about provisioned instances. This API supports pagination.</p>
    fn describe_db_instances(
        &self,
        input: DescribeDBInstancesMessage,
    ) -> RusotoFuture<DBInstanceMessage, DescribeDBInstancesError>;

    /// <p> Returns a list of <code>DBParameterGroup</code> descriptions. If a <code>DBParameterGroupName</code> is specified, the list will contain only the description of the specified DB parameter group. </p>
    fn describe_db_parameter_groups(
        &self,
        input: DescribeDBParameterGroupsMessage,
    ) -> RusotoFuture<DBParameterGroupsMessage, DescribeDBParameterGroupsError>;

    /// <p>Returns the detailed parameter list for a particular DB parameter group.</p>
    fn describe_db_parameters(
        &self,
        input: DescribeDBParametersMessage,
    ) -> RusotoFuture<DBParameterGroupDetails, DescribeDBParametersError>;

    /// <p>Returns a list of DBSubnetGroup descriptions. If a DBSubnetGroupName is specified, the list will contain only the descriptions of the specified DBSubnetGroup.</p> <p>For an overview of CIDR ranges, go to the <a href="http://en.wikipedia.org/wiki/Classless_Inter-Domain_Routing">Wikipedia Tutorial</a>. </p>
    fn describe_db_subnet_groups(
        &self,
        input: DescribeDBSubnetGroupsMessage,
    ) -> RusotoFuture<DBSubnetGroupMessage, DescribeDBSubnetGroupsError>;

    /// <p>Returns the default engine and system parameter information for the cluster database engine.</p>
    fn describe_engine_default_cluster_parameters(
        &self,
        input: DescribeEngineDefaultClusterParametersMessage,
    ) -> RusotoFuture<
        DescribeEngineDefaultClusterParametersResult,
        DescribeEngineDefaultClusterParametersError,
    >;

    /// <p>Returns the default engine and system parameter information for the specified database engine.</p>
    fn describe_engine_default_parameters(
        &self,
        input: DescribeEngineDefaultParametersMessage,
    ) -> RusotoFuture<DescribeEngineDefaultParametersResult, DescribeEngineDefaultParametersError>;

    /// <p>Displays a list of categories for all event source types, or, if specified, for a specified source type. </p>
    fn describe_event_categories(
        &self,
        input: DescribeEventCategoriesMessage,
    ) -> RusotoFuture<EventCategoriesMessage, DescribeEventCategoriesError>;

    /// <p>Lists all the subscription descriptions for a customer account. The description for a subscription includes SubscriptionName, SNSTopicARN, CustomerID, SourceType, SourceID, CreationTime, and Status.</p> <p>If you specify a SubscriptionName, lists the description for that subscription.</p>
    fn describe_event_subscriptions(
        &self,
        input: DescribeEventSubscriptionsMessage,
    ) -> RusotoFuture<EventSubscriptionsMessage, DescribeEventSubscriptionsError>;

    /// <p>Returns events related to DB instances, DB security groups, DB snapshots, and DB parameter groups for the past 14 days. Events specific to a particular DB instance, DB security group, database snapshot, or DB parameter group can be obtained by providing the name as a parameter. By default, the past hour of events are returned.</p>
    fn describe_events(
        &self,
        input: DescribeEventsMessage,
    ) -> RusotoFuture<EventsMessage, DescribeEventsError>;

    /// <p>Returns a list of orderable DB instance options for the specified engine.</p>
    fn describe_orderable_db_instance_options(
        &self,
        input: DescribeOrderableDBInstanceOptionsMessage,
    ) -> RusotoFuture<OrderableDBInstanceOptionsMessage, DescribeOrderableDBInstanceOptionsError>;

    /// <p>Returns a list of resources (for example, DB instances) that have at least one pending maintenance action.</p>
    fn describe_pending_maintenance_actions(
        &self,
        input: DescribePendingMaintenanceActionsMessage,
    ) -> RusotoFuture<PendingMaintenanceActionsMessage, DescribePendingMaintenanceActionsError>;

    /// <p>You can call <a>DescribeValidDBInstanceModifications</a> to learn what modifications you can make to your DB instance. You can use this information when you call <a>ModifyDBInstance</a>. </p>
    fn describe_valid_db_instance_modifications(
        &self,
        input: DescribeValidDBInstanceModificationsMessage,
    ) -> RusotoFuture<
        DescribeValidDBInstanceModificationsResult,
        DescribeValidDBInstanceModificationsError,
    >;

    /// <p>Forces a failover for a DB cluster.</p> <p>A failover for a DB cluster promotes one of the Read Replicas (read-only instances) in the DB cluster to be the primary instance (the cluster writer).</p> <p>Amazon Neptune will automatically fail over to a Read Replica, if one exists, when the primary instance fails. You can force a failover when you want to simulate a failure of a primary instance for testing. Because each instance in a DB cluster has its own endpoint address, you will need to clean up and re-establish any existing connections that use those endpoint addresses when the failover is complete.</p>
    fn failover_db_cluster(
        &self,
        input: FailoverDBClusterMessage,
    ) -> RusotoFuture<FailoverDBClusterResult, FailoverDBClusterError>;

    /// <p>Lists all tags on an Amazon Neptune resource.</p>
    fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceMessage,
    ) -> RusotoFuture<TagListMessage, ListTagsForResourceError>;

    /// <p>Modify a setting for a DB cluster. You can change one or more database configuration parameters by specifying these parameters and the new values in the request. </p>
    fn modify_db_cluster(
        &self,
        input: ModifyDBClusterMessage,
    ) -> RusotoFuture<ModifyDBClusterResult, ModifyDBClusterError>;

    /// <p><p> Modifies the parameters of a DB cluster parameter group. To modify more than one parameter, submit a list of the following: <code>ParameterName</code>, <code>ParameterValue</code>, and <code>ApplyMethod</code>. A maximum of 20 parameters can be modified in a single request. </p> <note> <p>Changes to dynamic parameters are applied immediately. Changes to static parameters require a reboot without failover to the DB cluster associated with the parameter group before the change can take effect.</p> </note> <important> <p>After you create a DB cluster parameter group, you should wait at least 5 minutes before creating your first DB cluster that uses that DB cluster parameter group as the default parameter group. This allows Amazon Neptune to fully complete the create action before the parameter group is used as the default for a new DB cluster. This is especially important for parameters that are critical when creating the default database for a DB cluster, such as the character set for the default database defined by the <code>character<em>set</em>database</code> parameter. You can use the <i>Parameter Groups</i> option of the Amazon Neptune console or the <a>DescribeDBClusterParameters</a> command to verify that your DB cluster parameter group has been created or modified.</p> </important></p>
    fn modify_db_cluster_parameter_group(
        &self,
        input: ModifyDBClusterParameterGroupMessage,
    ) -> RusotoFuture<DBClusterParameterGroupNameMessage, ModifyDBClusterParameterGroupError>;

    /// <p>Adds an attribute and values to, or removes an attribute and values from, a manual DB cluster snapshot.</p> <p>To share a manual DB cluster snapshot with other AWS accounts, specify <code>restore</code> as the <code>AttributeName</code> and use the <code>ValuesToAdd</code> parameter to add a list of IDs of the AWS accounts that are authorized to restore the manual DB cluster snapshot. Use the value <code>all</code> to make the manual DB cluster snapshot public, which means that it can be copied or restored by all AWS accounts. Do not add the <code>all</code> value for any manual DB cluster snapshots that contain private information that you don't want available to all AWS accounts. If a manual DB cluster snapshot is encrypted, it can be shared, but only by specifying a list of authorized AWS account IDs for the <code>ValuesToAdd</code> parameter. You can't use <code>all</code> as a value for that parameter in this case.</p> <p>To view which AWS accounts have access to copy or restore a manual DB cluster snapshot, or whether a manual DB cluster snapshot public or private, use the <a>DescribeDBClusterSnapshotAttributes</a> API action.</p>
    fn modify_db_cluster_snapshot_attribute(
        &self,
        input: ModifyDBClusterSnapshotAttributeMessage,
    ) -> RusotoFuture<ModifyDBClusterSnapshotAttributeResult, ModifyDBClusterSnapshotAttributeError>;

    /// <p>Modifies settings for a DB instance. You can change one or more database configuration parameters by specifying these parameters and the new values in the request. To learn what modifications you can make to your DB instance, call <a>DescribeValidDBInstanceModifications</a> before you call <a>ModifyDBInstance</a>. </p>
    fn modify_db_instance(
        &self,
        input: ModifyDBInstanceMessage,
    ) -> RusotoFuture<ModifyDBInstanceResult, ModifyDBInstanceError>;

    /// <p><p> Modifies the parameters of a DB parameter group. To modify more than one parameter, submit a list of the following: <code>ParameterName</code>, <code>ParameterValue</code>, and <code>ApplyMethod</code>. A maximum of 20 parameters can be modified in a single request. </p> <note> <p>Changes to dynamic parameters are applied immediately. Changes to static parameters require a reboot without failover to the DB instance associated with the parameter group before the change can take effect.</p> </note> <important> <p>After you modify a DB parameter group, you should wait at least 5 minutes before creating your first DB instance that uses that DB parameter group as the default parameter group. This allows Amazon Neptune to fully complete the modify action before the parameter group is used as the default for a new DB instance. This is especially important for parameters that are critical when creating the default database for a DB instance, such as the character set for the default database defined by the <code>character<em>set</em>database</code> parameter. You can use the <i>Parameter Groups</i> option of the Amazon Neptune console or the <i>DescribeDBParameters</i> command to verify that your DB parameter group has been created or modified.</p> </important></p>
    fn modify_db_parameter_group(
        &self,
        input: ModifyDBParameterGroupMessage,
    ) -> RusotoFuture<DBParameterGroupNameMessage, ModifyDBParameterGroupError>;

    /// <p>Modifies an existing DB subnet group. DB subnet groups must contain at least one subnet in at least two AZs in the AWS Region.</p>
    fn modify_db_subnet_group(
        &self,
        input: ModifyDBSubnetGroupMessage,
    ) -> RusotoFuture<ModifyDBSubnetGroupResult, ModifyDBSubnetGroupError>;

    /// <p>Modifies an existing event notification subscription. Note that you can't modify the source identifiers using this call; to change source identifiers for a subscription, use the <a>AddSourceIdentifierToSubscription</a> and <a>RemoveSourceIdentifierFromSubscription</a> calls.</p> <p>You can see a list of the event categories for a given SourceType by using the <b>DescribeEventCategories</b> action.</p>
    fn modify_event_subscription(
        &self,
        input: ModifyEventSubscriptionMessage,
    ) -> RusotoFuture<ModifyEventSubscriptionResult, ModifyEventSubscriptionError>;

    /// <p>Promotes a Read Replica DB cluster to a standalone DB cluster.</p>
    fn promote_read_replica_db_cluster(
        &self,
        input: PromoteReadReplicaDBClusterMessage,
    ) -> RusotoFuture<PromoteReadReplicaDBClusterResult, PromoteReadReplicaDBClusterError>;

    /// <p>You might need to reboot your DB instance, usually for maintenance reasons. For example, if you make certain modifications, or if you change the DB parameter group associated with the DB instance, you must reboot the instance for the changes to take effect. </p> <p>Rebooting a DB instance restarts the database engine service. Rebooting a DB instance results in a momentary outage, during which the DB instance status is set to rebooting. </p>
    fn reboot_db_instance(
        &self,
        input: RebootDBInstanceMessage,
    ) -> RusotoFuture<RebootDBInstanceResult, RebootDBInstanceError>;

    /// <p>Disassociates an Identity and Access Management (IAM) role from a DB cluster. </p>
    fn remove_role_from_db_cluster(
        &self,
        input: RemoveRoleFromDBClusterMessage,
    ) -> RusotoFuture<(), RemoveRoleFromDBClusterError>;

    /// <p>Removes a source identifier from an existing event notification subscription.</p>
    fn remove_source_identifier_from_subscription(
        &self,
        input: RemoveSourceIdentifierFromSubscriptionMessage,
    ) -> RusotoFuture<
        RemoveSourceIdentifierFromSubscriptionResult,
        RemoveSourceIdentifierFromSubscriptionError,
    >;

    /// <p>Removes metadata tags from an Amazon Neptune resource.</p>
    fn remove_tags_from_resource(
        &self,
        input: RemoveTagsFromResourceMessage,
    ) -> RusotoFuture<(), RemoveTagsFromResourceError>;

    /// <p> Modifies the parameters of a DB cluster parameter group to the default value. To reset specific parameters submit a list of the following: <code>ParameterName</code> and <code>ApplyMethod</code>. To reset the entire DB cluster parameter group, specify the <code>DBClusterParameterGroupName</code> and <code>ResetAllParameters</code> parameters. </p> <p> When resetting the entire group, dynamic parameters are updated immediately and static parameters are set to <code>pending-reboot</code> to take effect on the next DB instance restart or <a>RebootDBInstance</a> request. You must call <a>RebootDBInstance</a> for every DB instance in your DB cluster that you want the updated static parameter to apply to.</p>
    fn reset_db_cluster_parameter_group(
        &self,
        input: ResetDBClusterParameterGroupMessage,
    ) -> RusotoFuture<DBClusterParameterGroupNameMessage, ResetDBClusterParameterGroupError>;

    /// <p>Modifies the parameters of a DB parameter group to the engine/system default value. To reset specific parameters, provide a list of the following: <code>ParameterName</code> and <code>ApplyMethod</code>. To reset the entire DB parameter group, specify the <code>DBParameterGroup</code> name and <code>ResetAllParameters</code> parameters. When resetting the entire group, dynamic parameters are updated immediately and static parameters are set to <code>pending-reboot</code> to take effect on the next DB instance restart or <code>RebootDBInstance</code> request. </p>
    fn reset_db_parameter_group(
        &self,
        input: ResetDBParameterGroupMessage,
    ) -> RusotoFuture<DBParameterGroupNameMessage, ResetDBParameterGroupError>;

    /// <p>Creates a new DB cluster from a DB snapshot or DB cluster snapshot.</p> <p>If a DB snapshot is specified, the target DB cluster is created from the source DB snapshot with a default configuration and default security group.</p> <p>If a DB cluster snapshot is specified, the target DB cluster is created from the source DB cluster restore point with the same configuration as the original source DB cluster, except that the new DB cluster is created with the default security group.</p>
    fn restore_db_cluster_from_snapshot(
        &self,
        input: RestoreDBClusterFromSnapshotMessage,
    ) -> RusotoFuture<RestoreDBClusterFromSnapshotResult, RestoreDBClusterFromSnapshotError>;

    /// <p><p>Restores a DB cluster to an arbitrary point in time. Users can restore to any point in time before <code>LatestRestorableTime</code> for up to <code>BackupRetentionPeriod</code> days. The target DB cluster is created from the source DB cluster with the same configuration as the original DB cluster, except that the new DB cluster is created with the default DB security group. </p> <note> <p>This action only restores the DB cluster, not the DB instances for that DB cluster. You must invoke the <a>CreateDBInstance</a> action to create DB instances for the restored DB cluster, specifying the identifier of the restored DB cluster in <code>DBClusterIdentifier</code>. You can create DB instances only after the <code>RestoreDBClusterToPointInTime</code> action has completed and the DB cluster is available.</p> </note></p>
    fn restore_db_cluster_to_point_in_time(
        &self,
        input: RestoreDBClusterToPointInTimeMessage,
    ) -> RusotoFuture<RestoreDBClusterToPointInTimeResult, RestoreDBClusterToPointInTimeError>;
}
/// A client for the Amazon Neptune API.
pub struct NeptuneClient {
    client: Client,
    region: region::Region,
}

impl NeptuneClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> NeptuneClient {
        NeptuneClient {
            client: Client::shared(),
            region: region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> NeptuneClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        P::Future: Send,
        D: DispatchSignedRequest + Send + Sync + 'static,
        D::Future: Send,
    {
        NeptuneClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl Neptune for NeptuneClient {
    /// <p>Associates an Identity and Access Management (IAM) role from an Neptune DB cluster. </p>
    fn add_role_to_db_cluster(
        &self,
        input: AddRoleToDBClusterMessage,
    ) -> RusotoFuture<(), AddRoleToDBClusterError> {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "AddRoleToDBCluster");
        params.put("Version", "2014-10-31");
        AddRoleToDBClusterMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(
            serde_urlencoded::to_string(&params).unwrap().into_bytes(),
        ));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(AddRoleToDBClusterError::from_response(response))),
                );
            }

            Box::new(future::ok(::std::mem::drop(response)))
        })
    }

    /// <p>Adds a source identifier to an existing event notification subscription.</p>
    fn add_source_identifier_to_subscription(
        &self,
        input: AddSourceIdentifierToSubscriptionMessage,
    ) -> RusotoFuture<AddSourceIdentifierToSubscriptionResult, AddSourceIdentifierToSubscriptionError>
    {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "AddSourceIdentifierToSubscription");
        params.put("Version", "2014-10-31");
        AddSourceIdentifierToSubscriptionMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(
            serde_urlencoded::to_string(&params).unwrap().into_bytes(),
        ));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(AddSourceIdentifierToSubscriptionError::from_response(
                        response,
                    ))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = AddSourceIdentifierToSubscriptionResult::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(
                        AddSourceIdentifierToSubscriptionResultDeserializer::deserialize(
                            "AddSourceIdentifierToSubscriptionResult",
                            &mut stack
                        )
                    );
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        })
    }

    /// <p>Adds metadata tags to an Amazon Neptune resource. These tags can also be used with cost allocation reporting to track cost associated with Amazon Neptune resources, or used in a Condition statement in an IAM policy for Amazon Neptune.</p>
    fn add_tags_to_resource(
        &self,
        input: AddTagsToResourceMessage,
    ) -> RusotoFuture<(), AddTagsToResourceError> {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "AddTagsToResource");
        params.put("Version", "2014-10-31");
        AddTagsToResourceMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(
            serde_urlencoded::to_string(&params).unwrap().into_bytes(),
        ));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(AddTagsToResourceError::from_response(response))),
                );
            }

            Box::new(future::ok(::std::mem::drop(response)))
        })
    }

    /// <p>Applies a pending maintenance action to a resource (for example, to a DB instance).</p>
    fn apply_pending_maintenance_action(
        &self,
        input: ApplyPendingMaintenanceActionMessage,
    ) -> RusotoFuture<ApplyPendingMaintenanceActionResult, ApplyPendingMaintenanceActionError> {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ApplyPendingMaintenanceAction");
        params.put("Version", "2014-10-31");
        ApplyPendingMaintenanceActionMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(
            serde_urlencoded::to_string(&params).unwrap().into_bytes(),
        ));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ApplyPendingMaintenanceActionError::from_response(response))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = ApplyPendingMaintenanceActionResult::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(
                        ApplyPendingMaintenanceActionResultDeserializer::deserialize(
                            "ApplyPendingMaintenanceActionResult",
                            &mut stack
                        )
                    );
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        })
    }

    /// <p>Copies the specified DB cluster parameter group.</p>
    fn copy_db_cluster_parameter_group(
        &self,
        input: CopyDBClusterParameterGroupMessage,
    ) -> RusotoFuture<CopyDBClusterParameterGroupResult, CopyDBClusterParameterGroupError> {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "CopyDBClusterParameterGroup");
        params.put("Version", "2014-10-31");
        CopyDBClusterParameterGroupMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(
            serde_urlencoded::to_string(&params).unwrap().into_bytes(),
        ));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CopyDBClusterParameterGroupError::from_response(response))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = CopyDBClusterParameterGroupResult::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(CopyDBClusterParameterGroupResultDeserializer::deserialize(
                        "CopyDBClusterParameterGroupResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        })
    }

    /// <p>Copies a snapshot of a DB cluster.</p> <p>To copy a DB cluster snapshot from a shared manual DB cluster snapshot, <code>SourceDBClusterSnapshotIdentifier</code> must be the Amazon Resource Name (ARN) of the shared DB cluster snapshot.</p> <p>You can copy an encrypted DB cluster snapshot from another AWS Region. In that case, the AWS Region where you call the <code>CopyDBClusterSnapshot</code> action is the destination AWS Region for the encrypted DB cluster snapshot to be copied to. To copy an encrypted DB cluster snapshot from another AWS Region, you must provide the following values:</p> <ul> <li> <p> <code>KmsKeyId</code> - The AWS Key Management System (AWS KMS) key identifier for the key to use to encrypt the copy of the DB cluster snapshot in the destination AWS Region.</p> </li> <li> <p> <code>PreSignedUrl</code> - A URL that contains a Signature Version 4 signed request for the <code>CopyDBClusterSnapshot</code> action to be called in the source AWS Region where the DB cluster snapshot is copied from. The pre-signed URL must be a valid request for the <code>CopyDBClusterSnapshot</code> API action that can be executed in the source AWS Region that contains the encrypted DB cluster snapshot to be copied.</p> <p>The pre-signed URL request must contain the following parameter values:</p> <ul> <li> <p> <code>KmsKeyId</code> - The KMS key identifier for the key to use to encrypt the copy of the DB cluster snapshot in the destination AWS Region. This is the same identifier for both the <code>CopyDBClusterSnapshot</code> action that is called in the destination AWS Region, and the action contained in the pre-signed URL.</p> </li> <li> <p> <code>DestinationRegion</code> - The name of the AWS Region that the DB cluster snapshot will be created in.</p> </li> <li> <p> <code>SourceDBClusterSnapshotIdentifier</code> - The DB cluster snapshot identifier for the encrypted DB cluster snapshot to be copied. This identifier must be in the Amazon Resource Name (ARN) format for the source AWS Region. For example, if you are copying an encrypted DB cluster snapshot from the us-west-2 AWS Region, then your <code>SourceDBClusterSnapshotIdentifier</code> looks like the following example: <code>arn:aws:rds:us-west-2:123456789012:cluster-snapshot:neptune-cluster1-snapshot-20161115</code>.</p> </li> </ul> <p>To learn how to generate a Signature Version 4 signed request, see <a href="http://docs.aws.amazon.com/AmazonS3/latest/API/sigv4-query-string-auth.html"> Authenticating Requests: Using Query Parameters (AWS Signature Version 4)</a> and <a href="http://docs.aws.amazon.com/general/latest/gr/signature-version-4.html"> Signature Version 4 Signing Process</a>.</p> </li> <li> <p> <code>TargetDBClusterSnapshotIdentifier</code> - The identifier for the new copy of the DB cluster snapshot in the destination AWS Region.</p> </li> <li> <p> <code>SourceDBClusterSnapshotIdentifier</code> - The DB cluster snapshot identifier for the encrypted DB cluster snapshot to be copied. This identifier must be in the ARN format for the source AWS Region and is the same value as the <code>SourceDBClusterSnapshotIdentifier</code> in the pre-signed URL. </p> </li> </ul> <p>To cancel the copy operation once it is in progress, delete the target DB cluster snapshot identified by <code>TargetDBClusterSnapshotIdentifier</code> while that DB cluster snapshot is in "copying" status.</p>
    fn copy_db_cluster_snapshot(
        &self,
        input: CopyDBClusterSnapshotMessage,
    ) -> RusotoFuture<CopyDBClusterSnapshotResult, CopyDBClusterSnapshotError> {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "CopyDBClusterSnapshot");
        params.put("Version", "2014-10-31");
        CopyDBClusterSnapshotMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(
            serde_urlencoded::to_string(&params).unwrap().into_bytes(),
        ));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(CopyDBClusterSnapshotError::from_response(response))
                    }),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = CopyDBClusterSnapshotResult::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(CopyDBClusterSnapshotResultDeserializer::deserialize(
                        "CopyDBClusterSnapshotResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        })
    }

    /// <p>Copies the specified DB parameter group.</p>
    fn copy_db_parameter_group(
        &self,
        input: CopyDBParameterGroupMessage,
    ) -> RusotoFuture<CopyDBParameterGroupResult, CopyDBParameterGroupError> {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "CopyDBParameterGroup");
        params.put("Version", "2014-10-31");
        CopyDBParameterGroupMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(
            serde_urlencoded::to_string(&params).unwrap().into_bytes(),
        ));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(CopyDBParameterGroupError::from_response(response))
                    }),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = CopyDBParameterGroupResult::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(CopyDBParameterGroupResultDeserializer::deserialize(
                        "CopyDBParameterGroupResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        })
    }

    /// <p>Creates a new Amazon Neptune DB cluster.</p> <p>You can use the <code>ReplicationSourceIdentifier</code> parameter to create the DB cluster as a Read Replica of another DB cluster or Amazon Neptune DB instance. For cross-region replication where the DB cluster identified by <code>ReplicationSourceIdentifier</code> is encrypted, you must also specify the <code>PreSignedUrl</code> parameter.</p>
    fn create_db_cluster(
        &self,
        input: CreateDBClusterMessage,
    ) -> RusotoFuture<CreateDBClusterResult, CreateDBClusterError> {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "CreateDBCluster");
        params.put("Version", "2014-10-31");
        CreateDBClusterMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(
            serde_urlencoded::to_string(&params).unwrap().into_bytes(),
        ));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateDBClusterError::from_response(response))),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = CreateDBClusterResult::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(CreateDBClusterResultDeserializer::deserialize(
                        "CreateDBClusterResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        })
    }

    /// <p><p>Creates a new DB cluster parameter group.</p> <p>Parameters in a DB cluster parameter group apply to all of the instances in a DB cluster.</p> <p> A DB cluster parameter group is initially created with the default parameters for the database engine used by instances in the DB cluster. To provide custom values for any of the parameters, you must modify the group after creating it using <a>ModifyDBClusterParameterGroup</a>. Once you&#39;ve created a DB cluster parameter group, you need to associate it with your DB cluster using <a>ModifyDBCluster</a>. When you associate a new DB cluster parameter group with a running DB cluster, you need to reboot the DB instances in the DB cluster without failover for the new DB cluster parameter group and associated settings to take effect. </p> <important> <p>After you create a DB cluster parameter group, you should wait at least 5 minutes before creating your first DB cluster that uses that DB cluster parameter group as the default parameter group. This allows Amazon Neptune to fully complete the create action before the DB cluster parameter group is used as the default for a new DB cluster. This is especially important for parameters that are critical when creating the default database for a DB cluster, such as the character set for the default database defined by the <code>character<em>set</em>database</code> parameter. You can use the <i>Parameter Groups</i> option of the <a href="https://console.aws.amazon.com/rds/">Amazon Neptune console</a> or the <a>DescribeDBClusterParameters</a> command to verify that your DB cluster parameter group has been created or modified.</p> </important></p>
    fn create_db_cluster_parameter_group(
        &self,
        input: CreateDBClusterParameterGroupMessage,
    ) -> RusotoFuture<CreateDBClusterParameterGroupResult, CreateDBClusterParameterGroupError> {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "CreateDBClusterParameterGroup");
        params.put("Version", "2014-10-31");
        CreateDBClusterParameterGroupMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(
            serde_urlencoded::to_string(&params).unwrap().into_bytes(),
        ));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateDBClusterParameterGroupError::from_response(response))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = CreateDBClusterParameterGroupResult::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(
                        CreateDBClusterParameterGroupResultDeserializer::deserialize(
                            "CreateDBClusterParameterGroupResult",
                            &mut stack
                        )
                    );
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        })
    }

    /// <p>Creates a snapshot of a DB cluster. </p>
    fn create_db_cluster_snapshot(
        &self,
        input: CreateDBClusterSnapshotMessage,
    ) -> RusotoFuture<CreateDBClusterSnapshotResult, CreateDBClusterSnapshotError> {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "CreateDBClusterSnapshot");
        params.put("Version", "2014-10-31");
        CreateDBClusterSnapshotMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(
            serde_urlencoded::to_string(&params).unwrap().into_bytes(),
        ));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateDBClusterSnapshotError::from_response(response))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = CreateDBClusterSnapshotResult::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(CreateDBClusterSnapshotResultDeserializer::deserialize(
                        "CreateDBClusterSnapshotResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        })
    }

    /// <p>Creates a new DB instance.</p>
    fn create_db_instance(
        &self,
        input: CreateDBInstanceMessage,
    ) -> RusotoFuture<CreateDBInstanceResult, CreateDBInstanceError> {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "CreateDBInstance");
        params.put("Version", "2014-10-31");
        CreateDBInstanceMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(
            serde_urlencoded::to_string(&params).unwrap().into_bytes(),
        ));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateDBInstanceError::from_response(response))),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = CreateDBInstanceResult::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(CreateDBInstanceResultDeserializer::deserialize(
                        "CreateDBInstanceResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        })
    }

    /// <p><p>Creates a new DB parameter group.</p> <p> A DB parameter group is initially created with the default parameters for the database engine used by the DB instance. To provide custom values for any of the parameters, you must modify the group after creating it using <i>ModifyDBParameterGroup</i>. Once you&#39;ve created a DB parameter group, you need to associate it with your DB instance using <i>ModifyDBInstance</i>. When you associate a new DB parameter group with a running DB instance, you need to reboot the DB instance without failover for the new DB parameter group and associated settings to take effect. </p> <important> <p>After you create a DB parameter group, you should wait at least 5 minutes before creating your first DB instance that uses that DB parameter group as the default parameter group. This allows Amazon Neptune to fully complete the create action before the parameter group is used as the default for a new DB instance. This is especially important for parameters that are critical when creating the default database for a DB instance, such as the character set for the default database defined by the <code>character<em>set</em>database</code> parameter. You can use the <i>Parameter Groups</i> option of the Amazon Neptune console or the <i>DescribeDBParameters</i> command to verify that your DB parameter group has been created or modified.</p> </important></p>
    fn create_db_parameter_group(
        &self,
        input: CreateDBParameterGroupMessage,
    ) -> RusotoFuture<CreateDBParameterGroupResult, CreateDBParameterGroupError> {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "CreateDBParameterGroup");
        params.put("Version", "2014-10-31");
        CreateDBParameterGroupMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(
            serde_urlencoded::to_string(&params).unwrap().into_bytes(),
        ));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateDBParameterGroupError::from_response(response))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = CreateDBParameterGroupResult::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(CreateDBParameterGroupResultDeserializer::deserialize(
                        "CreateDBParameterGroupResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        })
    }

    /// <p>Creates a new DB subnet group. DB subnet groups must contain at least one subnet in at least two AZs in the AWS Region.</p>
    fn create_db_subnet_group(
        &self,
        input: CreateDBSubnetGroupMessage,
    ) -> RusotoFuture<CreateDBSubnetGroupResult, CreateDBSubnetGroupError> {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "CreateDBSubnetGroup");
        params.put("Version", "2014-10-31");
        CreateDBSubnetGroupMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(
            serde_urlencoded::to_string(&params).unwrap().into_bytes(),
        ));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(CreateDBSubnetGroupError::from_response(response))
                    }),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = CreateDBSubnetGroupResult::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(CreateDBSubnetGroupResultDeserializer::deserialize(
                        "CreateDBSubnetGroupResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        })
    }

    /// <p>Creates an event notification subscription. This action requires a topic ARN (Amazon Resource Name) created by either the Neptune console, the SNS console, or the SNS API. To obtain an ARN with SNS, you must create a topic in Amazon SNS and subscribe to the topic. The ARN is displayed in the SNS console.</p> <p>You can specify the type of source (SourceType) you want to be notified of, provide a list of Neptune sources (SourceIds) that triggers the events, and provide a list of event categories (EventCategories) for events you want to be notified of. For example, you can specify SourceType = db-instance, SourceIds = mydbinstance1, mydbinstance2 and EventCategories = Availability, Backup.</p> <p>If you specify both the SourceType and SourceIds, such as SourceType = db-instance and SourceIdentifier = myDBInstance1, you are notified of all the db-instance events for the specified source. If you specify a SourceType but do not specify a SourceIdentifier, you receive notice of the events for that source type for all your Neptune sources. If you do not specify either the SourceType nor the SourceIdentifier, you are notified of events generated from all Neptune sources belonging to your customer account.</p>
    fn create_event_subscription(
        &self,
        input: CreateEventSubscriptionMessage,
    ) -> RusotoFuture<CreateEventSubscriptionResult, CreateEventSubscriptionError> {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "CreateEventSubscription");
        params.put("Version", "2014-10-31");
        CreateEventSubscriptionMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(
            serde_urlencoded::to_string(&params).unwrap().into_bytes(),
        ));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateEventSubscriptionError::from_response(response))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = CreateEventSubscriptionResult::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(CreateEventSubscriptionResultDeserializer::deserialize(
                        "CreateEventSubscriptionResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        })
    }

    /// <p><p>The DeleteDBCluster action deletes a previously provisioned DB cluster. When you delete a DB cluster, all automated backups for that DB cluster are deleted and can&#39;t be recovered. Manual DB cluster snapshots of the specified DB cluster are not deleted.</p> <p/></p>
    fn delete_db_cluster(
        &self,
        input: DeleteDBClusterMessage,
    ) -> RusotoFuture<DeleteDBClusterResult, DeleteDBClusterError> {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeleteDBCluster");
        params.put("Version", "2014-10-31");
        DeleteDBClusterMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(
            serde_urlencoded::to_string(&params).unwrap().into_bytes(),
        ));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteDBClusterError::from_response(response))),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DeleteDBClusterResult::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(DeleteDBClusterResultDeserializer::deserialize(
                        "DeleteDBClusterResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        })
    }

    /// <p>Deletes a specified DB cluster parameter group. The DB cluster parameter group to be deleted can't be associated with any DB clusters.</p>
    fn delete_db_cluster_parameter_group(
        &self,
        input: DeleteDBClusterParameterGroupMessage,
    ) -> RusotoFuture<(), DeleteDBClusterParameterGroupError> {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeleteDBClusterParameterGroup");
        params.put("Version", "2014-10-31");
        DeleteDBClusterParameterGroupMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(
            serde_urlencoded::to_string(&params).unwrap().into_bytes(),
        ));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteDBClusterParameterGroupError::from_response(response))
                }));
            }

            Box::new(future::ok(::std::mem::drop(response)))
        })
    }

    /// <p><p>Deletes a DB cluster snapshot. If the snapshot is being copied, the copy operation is terminated.</p> <note> <p>The DB cluster snapshot must be in the <code>available</code> state to be deleted.</p> </note></p>
    fn delete_db_cluster_snapshot(
        &self,
        input: DeleteDBClusterSnapshotMessage,
    ) -> RusotoFuture<DeleteDBClusterSnapshotResult, DeleteDBClusterSnapshotError> {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeleteDBClusterSnapshot");
        params.put("Version", "2014-10-31");
        DeleteDBClusterSnapshotMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(
            serde_urlencoded::to_string(&params).unwrap().into_bytes(),
        ));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteDBClusterSnapshotError::from_response(response))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DeleteDBClusterSnapshotResult::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(DeleteDBClusterSnapshotResultDeserializer::deserialize(
                        "DeleteDBClusterSnapshotResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        })
    }

    /// <p>The DeleteDBInstance action deletes a previously provisioned DB instance. When you delete a DB instance, all automated backups for that instance are deleted and can't be recovered. Manual DB snapshots of the DB instance to be deleted by <code>DeleteDBInstance</code> are not deleted.</p> <p> If you request a final DB snapshot the status of the Amazon Neptune DB instance is <code>deleting</code> until the DB snapshot is created. The API action <code>DescribeDBInstance</code> is used to monitor the status of this operation. The action can't be canceled or reverted once submitted. </p> <p>Note that when a DB instance is in a failure state and has a status of <code>failed</code>, <code>incompatible-restore</code>, or <code>incompatible-network</code>, you can only delete it when the <code>SkipFinalSnapshot</code> parameter is set to <code>true</code>.</p> <p>If the specified DB instance is part of a DB cluster, you can't delete the DB instance if both of the following conditions are true:</p> <ul> <li> <p>The DB cluster is a Read Replica of another DB cluster.</p> </li> <li> <p>The DB instance is the only instance in the DB cluster.</p> </li> </ul> <p>To delete a DB instance in this case, first call the <a>PromoteReadReplicaDBCluster</a> API action to promote the DB cluster so it's no longer a Read Replica. After the promotion completes, then call the <code>DeleteDBInstance</code> API action to delete the final instance in the DB cluster.</p>
    fn delete_db_instance(
        &self,
        input: DeleteDBInstanceMessage,
    ) -> RusotoFuture<DeleteDBInstanceResult, DeleteDBInstanceError> {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeleteDBInstance");
        params.put("Version", "2014-10-31");
        DeleteDBInstanceMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(
            serde_urlencoded::to_string(&params).unwrap().into_bytes(),
        ));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteDBInstanceError::from_response(response))),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DeleteDBInstanceResult::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(DeleteDBInstanceResultDeserializer::deserialize(
                        "DeleteDBInstanceResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        })
    }

    /// <p>Deletes a specified DBParameterGroup. The DBParameterGroup to be deleted can't be associated with any DB instances.</p>
    fn delete_db_parameter_group(
        &self,
        input: DeleteDBParameterGroupMessage,
    ) -> RusotoFuture<(), DeleteDBParameterGroupError> {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeleteDBParameterGroup");
        params.put("Version", "2014-10-31");
        DeleteDBParameterGroupMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(
            serde_urlencoded::to_string(&params).unwrap().into_bytes(),
        ));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteDBParameterGroupError::from_response(response))
                }));
            }

            Box::new(future::ok(::std::mem::drop(response)))
        })
    }

    /// <p><p>Deletes a DB subnet group.</p> <note> <p>The specified database subnet group must not be associated with any DB instances.</p> </note></p>
    fn delete_db_subnet_group(
        &self,
        input: DeleteDBSubnetGroupMessage,
    ) -> RusotoFuture<(), DeleteDBSubnetGroupError> {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeleteDBSubnetGroup");
        params.put("Version", "2014-10-31");
        DeleteDBSubnetGroupMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(
            serde_urlencoded::to_string(&params).unwrap().into_bytes(),
        ));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DeleteDBSubnetGroupError::from_response(response))
                    }),
                );
            }

            Box::new(future::ok(::std::mem::drop(response)))
        })
    }

    /// <p>Deletes an event notification subscription.</p>
    fn delete_event_subscription(
        &self,
        input: DeleteEventSubscriptionMessage,
    ) -> RusotoFuture<DeleteEventSubscriptionResult, DeleteEventSubscriptionError> {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeleteEventSubscription");
        params.put("Version", "2014-10-31");
        DeleteEventSubscriptionMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(
            serde_urlencoded::to_string(&params).unwrap().into_bytes(),
        ));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteEventSubscriptionError::from_response(response))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DeleteEventSubscriptionResult::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(DeleteEventSubscriptionResultDeserializer::deserialize(
                        "DeleteEventSubscriptionResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        })
    }

    /// <p> Returns a list of <code>DBClusterParameterGroup</code> descriptions. If a <code>DBClusterParameterGroupName</code> parameter is specified, the list will contain only the description of the specified DB cluster parameter group. </p>
    fn describe_db_cluster_parameter_groups(
        &self,
        input: DescribeDBClusterParameterGroupsMessage,
    ) -> RusotoFuture<DBClusterParameterGroupsMessage, DescribeDBClusterParameterGroupsError> {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeDBClusterParameterGroups");
        params.put("Version", "2014-10-31");
        DescribeDBClusterParameterGroupsMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(
            serde_urlencoded::to_string(&params).unwrap().into_bytes(),
        ));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeDBClusterParameterGroupsError::from_response(
                        response,
                    ))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DBClusterParameterGroupsMessage::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(DBClusterParameterGroupsMessageDeserializer::deserialize(
                        "DescribeDBClusterParameterGroupsResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        })
    }

    /// <p>Returns the detailed parameter list for a particular DB cluster parameter group.</p>
    fn describe_db_cluster_parameters(
        &self,
        input: DescribeDBClusterParametersMessage,
    ) -> RusotoFuture<DBClusterParameterGroupDetails, DescribeDBClusterParametersError> {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeDBClusterParameters");
        params.put("Version", "2014-10-31");
        DescribeDBClusterParametersMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(
            serde_urlencoded::to_string(&params).unwrap().into_bytes(),
        ));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeDBClusterParametersError::from_response(response))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DBClusterParameterGroupDetails::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(DBClusterParameterGroupDetailsDeserializer::deserialize(
                        "DescribeDBClusterParametersResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        })
    }

    /// <p>Returns a list of DB cluster snapshot attribute names and values for a manual DB cluster snapshot.</p> <p>When sharing snapshots with other AWS accounts, <code>DescribeDBClusterSnapshotAttributes</code> returns the <code>restore</code> attribute and a list of IDs for the AWS accounts that are authorized to copy or restore the manual DB cluster snapshot. If <code>all</code> is included in the list of values for the <code>restore</code> attribute, then the manual DB cluster snapshot is public and can be copied or restored by all AWS accounts.</p> <p>To add or remove access for an AWS account to copy or restore a manual DB cluster snapshot, or to make the manual DB cluster snapshot public or private, use the <a>ModifyDBClusterSnapshotAttribute</a> API action.</p>
    fn describe_db_cluster_snapshot_attributes(
        &self,
        input: DescribeDBClusterSnapshotAttributesMessage,
    ) -> RusotoFuture<
        DescribeDBClusterSnapshotAttributesResult,
        DescribeDBClusterSnapshotAttributesError,
    > {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeDBClusterSnapshotAttributes");
        params.put("Version", "2014-10-31");
        DescribeDBClusterSnapshotAttributesMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(
            serde_urlencoded::to_string(&params).unwrap().into_bytes(),
        ));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeDBClusterSnapshotAttributesError::from_response(
                        response,
                    ))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DescribeDBClusterSnapshotAttributesResult::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(
                        DescribeDBClusterSnapshotAttributesResultDeserializer::deserialize(
                            "DescribeDBClusterSnapshotAttributesResult",
                            &mut stack
                        )
                    );
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        })
    }

    /// <p>Returns information about DB cluster snapshots. This API action supports pagination.</p>
    fn describe_db_cluster_snapshots(
        &self,
        input: DescribeDBClusterSnapshotsMessage,
    ) -> RusotoFuture<DBClusterSnapshotMessage, DescribeDBClusterSnapshotsError> {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeDBClusterSnapshots");
        params.put("Version", "2014-10-31");
        DescribeDBClusterSnapshotsMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(
            serde_urlencoded::to_string(&params).unwrap().into_bytes(),
        ));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeDBClusterSnapshotsError::from_response(response))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DBClusterSnapshotMessage::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(DBClusterSnapshotMessageDeserializer::deserialize(
                        "DescribeDBClusterSnapshotsResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        })
    }

    /// <p>Returns information about provisioned DB clusters. This API supports pagination.</p>
    fn describe_db_clusters(
        &self,
        input: DescribeDBClustersMessage,
    ) -> RusotoFuture<DBClusterMessage, DescribeDBClustersError> {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeDBClusters");
        params.put("Version", "2014-10-31");
        DescribeDBClustersMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(
            serde_urlencoded::to_string(&params).unwrap().into_bytes(),
        ));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DescribeDBClustersError::from_response(response))),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DBClusterMessage::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(DBClusterMessageDeserializer::deserialize(
                        "DescribeDBClustersResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        })
    }

    /// <p>Returns a list of the available DB engines.</p>
    fn describe_db_engine_versions(
        &self,
        input: DescribeDBEngineVersionsMessage,
    ) -> RusotoFuture<DBEngineVersionMessage, DescribeDBEngineVersionsError> {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeDBEngineVersions");
        params.put("Version", "2014-10-31");
        DescribeDBEngineVersionsMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(
            serde_urlencoded::to_string(&params).unwrap().into_bytes(),
        ));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeDBEngineVersionsError::from_response(response))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DBEngineVersionMessage::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(DBEngineVersionMessageDeserializer::deserialize(
                        "DescribeDBEngineVersionsResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        })
    }

    /// <p>Returns information about provisioned instances. This API supports pagination.</p>
    fn describe_db_instances(
        &self,
        input: DescribeDBInstancesMessage,
    ) -> RusotoFuture<DBInstanceMessage, DescribeDBInstancesError> {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeDBInstances");
        params.put("Version", "2014-10-31");
        DescribeDBInstancesMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(
            serde_urlencoded::to_string(&params).unwrap().into_bytes(),
        ));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DescribeDBInstancesError::from_response(response))
                    }),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DBInstanceMessage::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(DBInstanceMessageDeserializer::deserialize(
                        "DescribeDBInstancesResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        })
    }

    /// <p> Returns a list of <code>DBParameterGroup</code> descriptions. If a <code>DBParameterGroupName</code> is specified, the list will contain only the description of the specified DB parameter group. </p>
    fn describe_db_parameter_groups(
        &self,
        input: DescribeDBParameterGroupsMessage,
    ) -> RusotoFuture<DBParameterGroupsMessage, DescribeDBParameterGroupsError> {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeDBParameterGroups");
        params.put("Version", "2014-10-31");
        DescribeDBParameterGroupsMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(
            serde_urlencoded::to_string(&params).unwrap().into_bytes(),
        ));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeDBParameterGroupsError::from_response(response))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DBParameterGroupsMessage::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(DBParameterGroupsMessageDeserializer::deserialize(
                        "DescribeDBParameterGroupsResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        })
    }

    /// <p>Returns the detailed parameter list for a particular DB parameter group.</p>
    fn describe_db_parameters(
        &self,
        input: DescribeDBParametersMessage,
    ) -> RusotoFuture<DBParameterGroupDetails, DescribeDBParametersError> {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeDBParameters");
        params.put("Version", "2014-10-31");
        DescribeDBParametersMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(
            serde_urlencoded::to_string(&params).unwrap().into_bytes(),
        ));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DescribeDBParametersError::from_response(response))
                    }),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DBParameterGroupDetails::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(DBParameterGroupDetailsDeserializer::deserialize(
                        "DescribeDBParametersResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        })
    }

    /// <p>Returns a list of DBSubnetGroup descriptions. If a DBSubnetGroupName is specified, the list will contain only the descriptions of the specified DBSubnetGroup.</p> <p>For an overview of CIDR ranges, go to the <a href="http://en.wikipedia.org/wiki/Classless_Inter-Domain_Routing">Wikipedia Tutorial</a>. </p>
    fn describe_db_subnet_groups(
        &self,
        input: DescribeDBSubnetGroupsMessage,
    ) -> RusotoFuture<DBSubnetGroupMessage, DescribeDBSubnetGroupsError> {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeDBSubnetGroups");
        params.put("Version", "2014-10-31");
        DescribeDBSubnetGroupsMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(
            serde_urlencoded::to_string(&params).unwrap().into_bytes(),
        ));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeDBSubnetGroupsError::from_response(response))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DBSubnetGroupMessage::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(DBSubnetGroupMessageDeserializer::deserialize(
                        "DescribeDBSubnetGroupsResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        })
    }

    /// <p>Returns the default engine and system parameter information for the cluster database engine.</p>
    fn describe_engine_default_cluster_parameters(
        &self,
        input: DescribeEngineDefaultClusterParametersMessage,
    ) -> RusotoFuture<
        DescribeEngineDefaultClusterParametersResult,
        DescribeEngineDefaultClusterParametersError,
    > {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeEngineDefaultClusterParameters");
        params.put("Version", "2014-10-31");
        DescribeEngineDefaultClusterParametersMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(
            serde_urlencoded::to_string(&params).unwrap().into_bytes(),
        ));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeEngineDefaultClusterParametersError::from_response(
                        response,
                    ))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DescribeEngineDefaultClusterParametersResult::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(
                        DescribeEngineDefaultClusterParametersResultDeserializer::deserialize(
                            "DescribeEngineDefaultClusterParametersResult",
                            &mut stack
                        )
                    );
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        })
    }

    /// <p>Returns the default engine and system parameter information for the specified database engine.</p>
    fn describe_engine_default_parameters(
        &self,
        input: DescribeEngineDefaultParametersMessage,
    ) -> RusotoFuture<DescribeEngineDefaultParametersResult, DescribeEngineDefaultParametersError>
    {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeEngineDefaultParameters");
        params.put("Version", "2014-10-31");
        DescribeEngineDefaultParametersMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(
            serde_urlencoded::to_string(&params).unwrap().into_bytes(),
        ));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeEngineDefaultParametersError::from_response(
                        response,
                    ))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DescribeEngineDefaultParametersResult::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(
                        DescribeEngineDefaultParametersResultDeserializer::deserialize(
                            "DescribeEngineDefaultParametersResult",
                            &mut stack
                        )
                    );
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        })
    }

    /// <p>Displays a list of categories for all event source types, or, if specified, for a specified source type. </p>
    fn describe_event_categories(
        &self,
        input: DescribeEventCategoriesMessage,
    ) -> RusotoFuture<EventCategoriesMessage, DescribeEventCategoriesError> {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeEventCategories");
        params.put("Version", "2014-10-31");
        DescribeEventCategoriesMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(
            serde_urlencoded::to_string(&params).unwrap().into_bytes(),
        ));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeEventCategoriesError::from_response(response))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = EventCategoriesMessage::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(EventCategoriesMessageDeserializer::deserialize(
                        "DescribeEventCategoriesResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        })
    }

    /// <p>Lists all the subscription descriptions for a customer account. The description for a subscription includes SubscriptionName, SNSTopicARN, CustomerID, SourceType, SourceID, CreationTime, and Status.</p> <p>If you specify a SubscriptionName, lists the description for that subscription.</p>
    fn describe_event_subscriptions(
        &self,
        input: DescribeEventSubscriptionsMessage,
    ) -> RusotoFuture<EventSubscriptionsMessage, DescribeEventSubscriptionsError> {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeEventSubscriptions");
        params.put("Version", "2014-10-31");
        DescribeEventSubscriptionsMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(
            serde_urlencoded::to_string(&params).unwrap().into_bytes(),
        ));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeEventSubscriptionsError::from_response(response))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = EventSubscriptionsMessage::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(EventSubscriptionsMessageDeserializer::deserialize(
                        "DescribeEventSubscriptionsResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        })
    }

    /// <p>Returns events related to DB instances, DB security groups, DB snapshots, and DB parameter groups for the past 14 days. Events specific to a particular DB instance, DB security group, database snapshot, or DB parameter group can be obtained by providing the name as a parameter. By default, the past hour of events are returned.</p>
    fn describe_events(
        &self,
        input: DescribeEventsMessage,
    ) -> RusotoFuture<EventsMessage, DescribeEventsError> {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeEvents");
        params.put("Version", "2014-10-31");
        DescribeEventsMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(
            serde_urlencoded::to_string(&params).unwrap().into_bytes(),
        ));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DescribeEventsError::from_response(response))),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = EventsMessage::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(EventsMessageDeserializer::deserialize(
                        "DescribeEventsResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        })
    }

    /// <p>Returns a list of orderable DB instance options for the specified engine.</p>
    fn describe_orderable_db_instance_options(
        &self,
        input: DescribeOrderableDBInstanceOptionsMessage,
    ) -> RusotoFuture<OrderableDBInstanceOptionsMessage, DescribeOrderableDBInstanceOptionsError>
    {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeOrderableDBInstanceOptions");
        params.put("Version", "2014-10-31");
        DescribeOrderableDBInstanceOptionsMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(
            serde_urlencoded::to_string(&params).unwrap().into_bytes(),
        ));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeOrderableDBInstanceOptionsError::from_response(
                        response,
                    ))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = OrderableDBInstanceOptionsMessage::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(OrderableDBInstanceOptionsMessageDeserializer::deserialize(
                        "DescribeOrderableDBInstanceOptionsResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        })
    }

    /// <p>Returns a list of resources (for example, DB instances) that have at least one pending maintenance action.</p>
    fn describe_pending_maintenance_actions(
        &self,
        input: DescribePendingMaintenanceActionsMessage,
    ) -> RusotoFuture<PendingMaintenanceActionsMessage, DescribePendingMaintenanceActionsError>
    {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribePendingMaintenanceActions");
        params.put("Version", "2014-10-31");
        DescribePendingMaintenanceActionsMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(
            serde_urlencoded::to_string(&params).unwrap().into_bytes(),
        ));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribePendingMaintenanceActionsError::from_response(
                        response,
                    ))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = PendingMaintenanceActionsMessage::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(PendingMaintenanceActionsMessageDeserializer::deserialize(
                        "DescribePendingMaintenanceActionsResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        })
    }

    /// <p>You can call <a>DescribeValidDBInstanceModifications</a> to learn what modifications you can make to your DB instance. You can use this information when you call <a>ModifyDBInstance</a>. </p>
    fn describe_valid_db_instance_modifications(
        &self,
        input: DescribeValidDBInstanceModificationsMessage,
    ) -> RusotoFuture<
        DescribeValidDBInstanceModificationsResult,
        DescribeValidDBInstanceModificationsError,
    > {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeValidDBInstanceModifications");
        params.put("Version", "2014-10-31");
        DescribeValidDBInstanceModificationsMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(
            serde_urlencoded::to_string(&params).unwrap().into_bytes(),
        ));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeValidDBInstanceModificationsError::from_response(
                        response,
                    ))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DescribeValidDBInstanceModificationsResult::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(
                        DescribeValidDBInstanceModificationsResultDeserializer::deserialize(
                            "DescribeValidDBInstanceModificationsResult",
                            &mut stack
                        )
                    );
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        })
    }

    /// <p>Forces a failover for a DB cluster.</p> <p>A failover for a DB cluster promotes one of the Read Replicas (read-only instances) in the DB cluster to be the primary instance (the cluster writer).</p> <p>Amazon Neptune will automatically fail over to a Read Replica, if one exists, when the primary instance fails. You can force a failover when you want to simulate a failure of a primary instance for testing. Because each instance in a DB cluster has its own endpoint address, you will need to clean up and re-establish any existing connections that use those endpoint addresses when the failover is complete.</p>
    fn failover_db_cluster(
        &self,
        input: FailoverDBClusterMessage,
    ) -> RusotoFuture<FailoverDBClusterResult, FailoverDBClusterError> {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "FailoverDBCluster");
        params.put("Version", "2014-10-31");
        FailoverDBClusterMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(
            serde_urlencoded::to_string(&params).unwrap().into_bytes(),
        ));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(FailoverDBClusterError::from_response(response))),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = FailoverDBClusterResult::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(FailoverDBClusterResultDeserializer::deserialize(
                        "FailoverDBClusterResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        })
    }

    /// <p>Lists all tags on an Amazon Neptune resource.</p>
    fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceMessage,
    ) -> RusotoFuture<TagListMessage, ListTagsForResourceError> {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ListTagsForResource");
        params.put("Version", "2014-10-31");
        ListTagsForResourceMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(
            serde_urlencoded::to_string(&params).unwrap().into_bytes(),
        ));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(ListTagsForResourceError::from_response(response))
                    }),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = TagListMessage::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(TagListMessageDeserializer::deserialize(
                        "ListTagsForResourceResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        })
    }

    /// <p>Modify a setting for a DB cluster. You can change one or more database configuration parameters by specifying these parameters and the new values in the request. </p>
    fn modify_db_cluster(
        &self,
        input: ModifyDBClusterMessage,
    ) -> RusotoFuture<ModifyDBClusterResult, ModifyDBClusterError> {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ModifyDBCluster");
        params.put("Version", "2014-10-31");
        ModifyDBClusterMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(
            serde_urlencoded::to_string(&params).unwrap().into_bytes(),
        ));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ModifyDBClusterError::from_response(response))),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = ModifyDBClusterResult::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(ModifyDBClusterResultDeserializer::deserialize(
                        "ModifyDBClusterResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        })
    }

    /// <p><p> Modifies the parameters of a DB cluster parameter group. To modify more than one parameter, submit a list of the following: <code>ParameterName</code>, <code>ParameterValue</code>, and <code>ApplyMethod</code>. A maximum of 20 parameters can be modified in a single request. </p> <note> <p>Changes to dynamic parameters are applied immediately. Changes to static parameters require a reboot without failover to the DB cluster associated with the parameter group before the change can take effect.</p> </note> <important> <p>After you create a DB cluster parameter group, you should wait at least 5 minutes before creating your first DB cluster that uses that DB cluster parameter group as the default parameter group. This allows Amazon Neptune to fully complete the create action before the parameter group is used as the default for a new DB cluster. This is especially important for parameters that are critical when creating the default database for a DB cluster, such as the character set for the default database defined by the <code>character<em>set</em>database</code> parameter. You can use the <i>Parameter Groups</i> option of the Amazon Neptune console or the <a>DescribeDBClusterParameters</a> command to verify that your DB cluster parameter group has been created or modified.</p> </important></p>
    fn modify_db_cluster_parameter_group(
        &self,
        input: ModifyDBClusterParameterGroupMessage,
    ) -> RusotoFuture<DBClusterParameterGroupNameMessage, ModifyDBClusterParameterGroupError> {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ModifyDBClusterParameterGroup");
        params.put("Version", "2014-10-31");
        ModifyDBClusterParameterGroupMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(
            serde_urlencoded::to_string(&params).unwrap().into_bytes(),
        ));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ModifyDBClusterParameterGroupError::from_response(response))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DBClusterParameterGroupNameMessage::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(DBClusterParameterGroupNameMessageDeserializer::deserialize(
                        "ModifyDBClusterParameterGroupResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        })
    }

    /// <p>Adds an attribute and values to, or removes an attribute and values from, a manual DB cluster snapshot.</p> <p>To share a manual DB cluster snapshot with other AWS accounts, specify <code>restore</code> as the <code>AttributeName</code> and use the <code>ValuesToAdd</code> parameter to add a list of IDs of the AWS accounts that are authorized to restore the manual DB cluster snapshot. Use the value <code>all</code> to make the manual DB cluster snapshot public, which means that it can be copied or restored by all AWS accounts. Do not add the <code>all</code> value for any manual DB cluster snapshots that contain private information that you don't want available to all AWS accounts. If a manual DB cluster snapshot is encrypted, it can be shared, but only by specifying a list of authorized AWS account IDs for the <code>ValuesToAdd</code> parameter. You can't use <code>all</code> as a value for that parameter in this case.</p> <p>To view which AWS accounts have access to copy or restore a manual DB cluster snapshot, or whether a manual DB cluster snapshot public or private, use the <a>DescribeDBClusterSnapshotAttributes</a> API action.</p>
    fn modify_db_cluster_snapshot_attribute(
        &self,
        input: ModifyDBClusterSnapshotAttributeMessage,
    ) -> RusotoFuture<ModifyDBClusterSnapshotAttributeResult, ModifyDBClusterSnapshotAttributeError>
    {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ModifyDBClusterSnapshotAttribute");
        params.put("Version", "2014-10-31");
        ModifyDBClusterSnapshotAttributeMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(
            serde_urlencoded::to_string(&params).unwrap().into_bytes(),
        ));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ModifyDBClusterSnapshotAttributeError::from_response(
                        response,
                    ))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = ModifyDBClusterSnapshotAttributeResult::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(
                        ModifyDBClusterSnapshotAttributeResultDeserializer::deserialize(
                            "ModifyDBClusterSnapshotAttributeResult",
                            &mut stack
                        )
                    );
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        })
    }

    /// <p>Modifies settings for a DB instance. You can change one or more database configuration parameters by specifying these parameters and the new values in the request. To learn what modifications you can make to your DB instance, call <a>DescribeValidDBInstanceModifications</a> before you call <a>ModifyDBInstance</a>. </p>
    fn modify_db_instance(
        &self,
        input: ModifyDBInstanceMessage,
    ) -> RusotoFuture<ModifyDBInstanceResult, ModifyDBInstanceError> {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ModifyDBInstance");
        params.put("Version", "2014-10-31");
        ModifyDBInstanceMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(
            serde_urlencoded::to_string(&params).unwrap().into_bytes(),
        ));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ModifyDBInstanceError::from_response(response))),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = ModifyDBInstanceResult::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(ModifyDBInstanceResultDeserializer::deserialize(
                        "ModifyDBInstanceResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        })
    }

    /// <p><p> Modifies the parameters of a DB parameter group. To modify more than one parameter, submit a list of the following: <code>ParameterName</code>, <code>ParameterValue</code>, and <code>ApplyMethod</code>. A maximum of 20 parameters can be modified in a single request. </p> <note> <p>Changes to dynamic parameters are applied immediately. Changes to static parameters require a reboot without failover to the DB instance associated with the parameter group before the change can take effect.</p> </note> <important> <p>After you modify a DB parameter group, you should wait at least 5 minutes before creating your first DB instance that uses that DB parameter group as the default parameter group. This allows Amazon Neptune to fully complete the modify action before the parameter group is used as the default for a new DB instance. This is especially important for parameters that are critical when creating the default database for a DB instance, such as the character set for the default database defined by the <code>character<em>set</em>database</code> parameter. You can use the <i>Parameter Groups</i> option of the Amazon Neptune console or the <i>DescribeDBParameters</i> command to verify that your DB parameter group has been created or modified.</p> </important></p>
    fn modify_db_parameter_group(
        &self,
        input: ModifyDBParameterGroupMessage,
    ) -> RusotoFuture<DBParameterGroupNameMessage, ModifyDBParameterGroupError> {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ModifyDBParameterGroup");
        params.put("Version", "2014-10-31");
        ModifyDBParameterGroupMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(
            serde_urlencoded::to_string(&params).unwrap().into_bytes(),
        ));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ModifyDBParameterGroupError::from_response(response))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DBParameterGroupNameMessage::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(DBParameterGroupNameMessageDeserializer::deserialize(
                        "ModifyDBParameterGroupResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        })
    }

    /// <p>Modifies an existing DB subnet group. DB subnet groups must contain at least one subnet in at least two AZs in the AWS Region.</p>
    fn modify_db_subnet_group(
        &self,
        input: ModifyDBSubnetGroupMessage,
    ) -> RusotoFuture<ModifyDBSubnetGroupResult, ModifyDBSubnetGroupError> {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ModifyDBSubnetGroup");
        params.put("Version", "2014-10-31");
        ModifyDBSubnetGroupMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(
            serde_urlencoded::to_string(&params).unwrap().into_bytes(),
        ));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(ModifyDBSubnetGroupError::from_response(response))
                    }),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = ModifyDBSubnetGroupResult::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(ModifyDBSubnetGroupResultDeserializer::deserialize(
                        "ModifyDBSubnetGroupResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        })
    }

    /// <p>Modifies an existing event notification subscription. Note that you can't modify the source identifiers using this call; to change source identifiers for a subscription, use the <a>AddSourceIdentifierToSubscription</a> and <a>RemoveSourceIdentifierFromSubscription</a> calls.</p> <p>You can see a list of the event categories for a given SourceType by using the <b>DescribeEventCategories</b> action.</p>
    fn modify_event_subscription(
        &self,
        input: ModifyEventSubscriptionMessage,
    ) -> RusotoFuture<ModifyEventSubscriptionResult, ModifyEventSubscriptionError> {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ModifyEventSubscription");
        params.put("Version", "2014-10-31");
        ModifyEventSubscriptionMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(
            serde_urlencoded::to_string(&params).unwrap().into_bytes(),
        ));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ModifyEventSubscriptionError::from_response(response))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = ModifyEventSubscriptionResult::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(ModifyEventSubscriptionResultDeserializer::deserialize(
                        "ModifyEventSubscriptionResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        })
    }

    /// <p>Promotes a Read Replica DB cluster to a standalone DB cluster.</p>
    fn promote_read_replica_db_cluster(
        &self,
        input: PromoteReadReplicaDBClusterMessage,
    ) -> RusotoFuture<PromoteReadReplicaDBClusterResult, PromoteReadReplicaDBClusterError> {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "PromoteReadReplicaDBCluster");
        params.put("Version", "2014-10-31");
        PromoteReadReplicaDBClusterMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(
            serde_urlencoded::to_string(&params).unwrap().into_bytes(),
        ));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(PromoteReadReplicaDBClusterError::from_response(response))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = PromoteReadReplicaDBClusterResult::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(PromoteReadReplicaDBClusterResultDeserializer::deserialize(
                        "PromoteReadReplicaDBClusterResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        })
    }

    /// <p>You might need to reboot your DB instance, usually for maintenance reasons. For example, if you make certain modifications, or if you change the DB parameter group associated with the DB instance, you must reboot the instance for the changes to take effect. </p> <p>Rebooting a DB instance restarts the database engine service. Rebooting a DB instance results in a momentary outage, during which the DB instance status is set to rebooting. </p>
    fn reboot_db_instance(
        &self,
        input: RebootDBInstanceMessage,
    ) -> RusotoFuture<RebootDBInstanceResult, RebootDBInstanceError> {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "RebootDBInstance");
        params.put("Version", "2014-10-31");
        RebootDBInstanceMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(
            serde_urlencoded::to_string(&params).unwrap().into_bytes(),
        ));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(RebootDBInstanceError::from_response(response))),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = RebootDBInstanceResult::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(RebootDBInstanceResultDeserializer::deserialize(
                        "RebootDBInstanceResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        })
    }

    /// <p>Disassociates an Identity and Access Management (IAM) role from a DB cluster. </p>
    fn remove_role_from_db_cluster(
        &self,
        input: RemoveRoleFromDBClusterMessage,
    ) -> RusotoFuture<(), RemoveRoleFromDBClusterError> {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "RemoveRoleFromDBCluster");
        params.put("Version", "2014-10-31");
        RemoveRoleFromDBClusterMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(
            serde_urlencoded::to_string(&params).unwrap().into_bytes(),
        ));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(RemoveRoleFromDBClusterError::from_response(response))
                }));
            }

            Box::new(future::ok(::std::mem::drop(response)))
        })
    }

    /// <p>Removes a source identifier from an existing event notification subscription.</p>
    fn remove_source_identifier_from_subscription(
        &self,
        input: RemoveSourceIdentifierFromSubscriptionMessage,
    ) -> RusotoFuture<
        RemoveSourceIdentifierFromSubscriptionResult,
        RemoveSourceIdentifierFromSubscriptionError,
    > {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "RemoveSourceIdentifierFromSubscription");
        params.put("Version", "2014-10-31");
        RemoveSourceIdentifierFromSubscriptionMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(
            serde_urlencoded::to_string(&params).unwrap().into_bytes(),
        ));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(RemoveSourceIdentifierFromSubscriptionError::from_response(
                        response,
                    ))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = RemoveSourceIdentifierFromSubscriptionResult::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(
                        RemoveSourceIdentifierFromSubscriptionResultDeserializer::deserialize(
                            "RemoveSourceIdentifierFromSubscriptionResult",
                            &mut stack
                        )
                    );
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        })
    }

    /// <p>Removes metadata tags from an Amazon Neptune resource.</p>
    fn remove_tags_from_resource(
        &self,
        input: RemoveTagsFromResourceMessage,
    ) -> RusotoFuture<(), RemoveTagsFromResourceError> {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "RemoveTagsFromResource");
        params.put("Version", "2014-10-31");
        RemoveTagsFromResourceMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(
            serde_urlencoded::to_string(&params).unwrap().into_bytes(),
        ));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(RemoveTagsFromResourceError::from_response(response))
                }));
            }

            Box::new(future::ok(::std::mem::drop(response)))
        })
    }

    /// <p> Modifies the parameters of a DB cluster parameter group to the default value. To reset specific parameters submit a list of the following: <code>ParameterName</code> and <code>ApplyMethod</code>. To reset the entire DB cluster parameter group, specify the <code>DBClusterParameterGroupName</code> and <code>ResetAllParameters</code> parameters. </p> <p> When resetting the entire group, dynamic parameters are updated immediately and static parameters are set to <code>pending-reboot</code> to take effect on the next DB instance restart or <a>RebootDBInstance</a> request. You must call <a>RebootDBInstance</a> for every DB instance in your DB cluster that you want the updated static parameter to apply to.</p>
    fn reset_db_cluster_parameter_group(
        &self,
        input: ResetDBClusterParameterGroupMessage,
    ) -> RusotoFuture<DBClusterParameterGroupNameMessage, ResetDBClusterParameterGroupError> {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ResetDBClusterParameterGroup");
        params.put("Version", "2014-10-31");
        ResetDBClusterParameterGroupMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(
            serde_urlencoded::to_string(&params).unwrap().into_bytes(),
        ));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ResetDBClusterParameterGroupError::from_response(response))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DBClusterParameterGroupNameMessage::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(DBClusterParameterGroupNameMessageDeserializer::deserialize(
                        "ResetDBClusterParameterGroupResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        })
    }

    /// <p>Modifies the parameters of a DB parameter group to the engine/system default value. To reset specific parameters, provide a list of the following: <code>ParameterName</code> and <code>ApplyMethod</code>. To reset the entire DB parameter group, specify the <code>DBParameterGroup</code> name and <code>ResetAllParameters</code> parameters. When resetting the entire group, dynamic parameters are updated immediately and static parameters are set to <code>pending-reboot</code> to take effect on the next DB instance restart or <code>RebootDBInstance</code> request. </p>
    fn reset_db_parameter_group(
        &self,
        input: ResetDBParameterGroupMessage,
    ) -> RusotoFuture<DBParameterGroupNameMessage, ResetDBParameterGroupError> {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ResetDBParameterGroup");
        params.put("Version", "2014-10-31");
        ResetDBParameterGroupMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(
            serde_urlencoded::to_string(&params).unwrap().into_bytes(),
        ));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(ResetDBParameterGroupError::from_response(response))
                    }),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DBParameterGroupNameMessage::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(DBParameterGroupNameMessageDeserializer::deserialize(
                        "ResetDBParameterGroupResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        })
    }

    /// <p>Creates a new DB cluster from a DB snapshot or DB cluster snapshot.</p> <p>If a DB snapshot is specified, the target DB cluster is created from the source DB snapshot with a default configuration and default security group.</p> <p>If a DB cluster snapshot is specified, the target DB cluster is created from the source DB cluster restore point with the same configuration as the original source DB cluster, except that the new DB cluster is created with the default security group.</p>
    fn restore_db_cluster_from_snapshot(
        &self,
        input: RestoreDBClusterFromSnapshotMessage,
    ) -> RusotoFuture<RestoreDBClusterFromSnapshotResult, RestoreDBClusterFromSnapshotError> {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "RestoreDBClusterFromSnapshot");
        params.put("Version", "2014-10-31");
        RestoreDBClusterFromSnapshotMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(
            serde_urlencoded::to_string(&params).unwrap().into_bytes(),
        ));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(RestoreDBClusterFromSnapshotError::from_response(response))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = RestoreDBClusterFromSnapshotResult::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(RestoreDBClusterFromSnapshotResultDeserializer::deserialize(
                        "RestoreDBClusterFromSnapshotResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        })
    }

    /// <p><p>Restores a DB cluster to an arbitrary point in time. Users can restore to any point in time before <code>LatestRestorableTime</code> for up to <code>BackupRetentionPeriod</code> days. The target DB cluster is created from the source DB cluster with the same configuration as the original DB cluster, except that the new DB cluster is created with the default DB security group. </p> <note> <p>This action only restores the DB cluster, not the DB instances for that DB cluster. You must invoke the <a>CreateDBInstance</a> action to create DB instances for the restored DB cluster, specifying the identifier of the restored DB cluster in <code>DBClusterIdentifier</code>. You can create DB instances only after the <code>RestoreDBClusterToPointInTime</code> action has completed and the DB cluster is available.</p> </note></p>
    fn restore_db_cluster_to_point_in_time(
        &self,
        input: RestoreDBClusterToPointInTimeMessage,
    ) -> RusotoFuture<RestoreDBClusterToPointInTimeResult, RestoreDBClusterToPointInTimeError> {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "RestoreDBClusterToPointInTime");
        params.put("Version", "2014-10-31");
        RestoreDBClusterToPointInTimeMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(
            serde_urlencoded::to_string(&params).unwrap().into_bytes(),
        ));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(RestoreDBClusterToPointInTimeError::from_response(response))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = RestoreDBClusterToPointInTimeResult::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(
                        RestoreDBClusterToPointInTimeResultDeserializer::deserialize(
                            "RestoreDBClusterToPointInTimeResult",
                            &mut stack
                        )
                    );
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        })
    }
}

#[cfg(test)]
mod protocol_tests {}
