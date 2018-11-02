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

use rusoto_core::signature::SignedRequest;
use serde_json;
use serde_json::from_slice;
use serde_json::Value as SerdeJsonValue;
/// <p>An object representing an AWS Batch array job.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ArrayProperties {
    /// <p>The size of the array job.</p>
    #[serde(rename = "size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
}

/// <p>An object representing the array properties of a job.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ArrayPropertiesDetail {
    /// <p>The job index within the array that is associated with this job. This parameter is returned for array job children.</p>
    #[serde(rename = "index")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<i64>,
    /// <p>The size of the array job. This parameter is returned for parent array jobs.</p>
    #[serde(rename = "size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    /// <p>A summary of the number of array job children in each available job status. This parameter is returned for parent array jobs.</p>
    #[serde(rename = "statusSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_summary: Option<::std::collections::HashMap<String, i64>>,
}

/// <p>An object representing the array properties of a job.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ArrayPropertiesSummary {
    /// <p>The job index within the array that is associated with this job. This parameter is returned for children of array jobs.</p>
    #[serde(rename = "index")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<i64>,
    /// <p>The size of the array job. This parameter is returned for parent array jobs.</p>
    #[serde(rename = "size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
}

/// <p>An object representing the details of a container that is part of a job attempt.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct AttemptContainerDetail {
    /// <p>The Amazon Resource Name (ARN) of the Amazon ECS container instance that hosts the job attempt.</p>
    #[serde(rename = "containerInstanceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_instance_arn: Option<String>,
    /// <p>The exit code for the job attempt. A non-zero exit code is considered a failure.</p>
    #[serde(rename = "exitCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exit_code: Option<i64>,
    /// <p>The name of the CloudWatch Logs log stream associated with the container. The log group for AWS Batch jobs is <code>/aws/batch/job</code>. Each container attempt receives a log stream name when they reach the <code>RUNNING</code> status.</p>
    #[serde(rename = "logStreamName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_stream_name: Option<String>,
    /// <p>A short (255 max characters) human-readable string to provide additional details about a running or stopped container.</p>
    #[serde(rename = "reason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the Amazon ECS task that is associated with the job attempt. Each container attempt receives a task ARN when they reach the <code>STARTING</code> status.</p>
    #[serde(rename = "taskArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_arn: Option<String>,
}

/// <p>An object representing a job attempt.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct AttemptDetail {
    /// <p>Details about the container in this job attempt.</p>
    #[serde(rename = "container")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container: Option<AttemptContainerDetail>,
    /// <p>The Unix time stamp (in seconds and milliseconds) for when the attempt was started (when the attempt transitioned from the <code>STARTING</code> state to the <code>RUNNING</code> state).</p>
    #[serde(rename = "startedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<i64>,
    /// <p>A short, human-readable string to provide additional details about the current status of the job attempt.</p>
    #[serde(rename = "statusReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
    /// <p>The Unix time stamp (in seconds and milliseconds) for when the attempt was stopped (when the attempt transitioned from the <code>RUNNING</code> state to a terminal state, such as <code>SUCCEEDED</code> or <code>FAILED</code>).</p>
    #[serde(rename = "stoppedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stopped_at: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CancelJobRequest {
    /// <p>The AWS Batch job ID of the job to cancel.</p>
    #[serde(rename = "jobId")]
    pub job_id: String,
    /// <p>A message to attach to the job that explains the reason for canceling it. This message is returned by future <a>DescribeJobs</a> operations on the job. This message is also recorded in the AWS Batch activity logs. </p>
    #[serde(rename = "reason")]
    pub reason: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CancelJobResponse {}

/// <p>An object representing an AWS Batch compute environment.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ComputeEnvironmentDetail {
    /// <p>The Amazon Resource Name (ARN) of the compute environment. </p>
    #[serde(rename = "computeEnvironmentArn")]
    pub compute_environment_arn: String,
    /// <p>The name of the compute environment. </p>
    #[serde(rename = "computeEnvironmentName")]
    pub compute_environment_name: String,
    /// <p>The compute resources defined for the compute environment. </p>
    #[serde(rename = "computeResources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_resources: Option<ComputeResource>,
    /// <p>The Amazon Resource Name (ARN) of the underlying Amazon ECS cluster used by the compute environment. </p>
    #[serde(rename = "ecsClusterArn")]
    pub ecs_cluster_arn: String,
    /// <p>The service role associated with the compute environment that allows AWS Batch to make calls to AWS API operations on your behalf.</p>
    #[serde(rename = "serviceRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_role: Option<String>,
    /// <p>The state of the compute environment. The valid values are <code>ENABLED</code> or <code>DISABLED</code>. An <code>ENABLED</code> state indicates that you can register instances with the compute environment and that the associated instances can accept jobs. </p>
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>The current status of the compute environment (for example, <code>CREATING</code> or <code>VALID</code>).</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>A short, human-readable string to provide additional details about the current status of the compute environment.</p>
    #[serde(rename = "statusReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
    /// <p>The type of the compute environment.</p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>The order in which compute environments are tried for job placement within a queue. Compute environments are tried in ascending order. For example, if two compute environments are associated with a job queue, the compute environment with a lower order integer value is tried for job placement first.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComputeEnvironmentOrder {
    /// <p>The Amazon Resource Name (ARN) of the compute environment.</p>
    #[serde(rename = "computeEnvironment")]
    pub compute_environment: String,
    /// <p>The order of the compute environment.</p>
    #[serde(rename = "order")]
    pub order: i64,
}

/// <p>An object representing an AWS Batch compute resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComputeResource {
    /// <p>The minimum percentage that a Spot Instance price must be when compared with the On-Demand price for that instance type before instances are launched. For example, if your bid percentage is 20%, then the Spot price must be below 20% of the current On-Demand price for that EC2 instance.</p>
    #[serde(rename = "bidPercentage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bid_percentage: Option<i64>,
    /// <p>The desired number of EC2 vCPUS in the compute environment. </p>
    #[serde(rename = "desiredvCpus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desiredv_cpus: Option<i64>,
    /// <p>The EC2 key pair that is used for instances launched in the compute environment.</p>
    #[serde(rename = "ec2KeyPair")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec_2_key_pair: Option<String>,
    /// <p>The Amazon Machine Image (AMI) ID used for instances launched in the compute environment.</p>
    #[serde(rename = "imageId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,
    /// <p>The Amazon ECS instance profile applied to Amazon EC2 instances in a compute environment. You can specify the short name or full Amazon Resource Name (ARN) of an instance profile. For example, <code>ecsInstanceRole</code> or <code>arn:aws:iam::&lt;aws_account_id&gt;:instance-profile/ecsInstanceRole</code>. For more information, see <a href="http://docs.aws.amazon.com/batch/latest/userguide/instance_IAM_role.html">Amazon ECS Instance Role</a> in the <i>AWS Batch User Guide</i>.</p>
    #[serde(rename = "instanceRole")]
    pub instance_role: String,
    /// <p>The instances types that may be launched. You can specify instance families to launch any instance type within those families (for example, <code>c4</code> or <code>p3</code>), or you can specify specific sizes within a family (such as <code>c4.8xlarge</code>). You can also choose <code>optimal</code> to pick instance types (from the latest C, M, and R instance families) on the fly that match the demand of your job queues.</p>
    #[serde(rename = "instanceTypes")]
    pub instance_types: Vec<String>,
    /// <p>The maximum number of EC2 vCPUs that an environment can reach. </p>
    #[serde(rename = "maxvCpus")]
    pub maxv_cpus: i64,
    /// <p>The minimum number of EC2 vCPUs that an environment should maintain. </p>
    #[serde(rename = "minvCpus")]
    pub minv_cpus: i64,
    /// <p>The EC2 security group that is associated with instances launched in the compute environment. </p>
    #[serde(rename = "securityGroupIds")]
    pub security_group_ids: Vec<String>,
    /// <p>The Amazon Resource Name (ARN) of the Amazon EC2 Spot Fleet IAM role applied to a <code>SPOT</code> compute environment.</p>
    #[serde(rename = "spotIamFleetRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spot_iam_fleet_role: Option<String>,
    /// <p>The VPC subnets into which the compute resources are launched. </p>
    #[serde(rename = "subnets")]
    pub subnets: Vec<String>,
    /// <p>Key-value pair tags to be applied to resources that are launched in the compute environment. </p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>The type of compute environment.</p>
    #[serde(rename = "type")]
    pub type_: String,
}

/// <p>An object representing the attributes of a compute environment that can be updated.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ComputeResourceUpdate {
    /// <p>The desired number of EC2 vCPUS in the compute environment.</p>
    #[serde(rename = "desiredvCpus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desiredv_cpus: Option<i64>,
    /// <p>The maximum number of EC2 vCPUs that an environment can reach.</p>
    #[serde(rename = "maxvCpus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxv_cpus: Option<i64>,
    /// <p>The minimum number of EC2 vCPUs that an environment should maintain.</p>
    #[serde(rename = "minvCpus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minv_cpus: Option<i64>,
}

/// <p>An object representing the details of a container that is part of a job.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ContainerDetail {
    /// <p>The command that is passed to the container. </p>
    #[serde(rename = "command")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<Vec<String>>,
    /// <p>The Amazon Resource Name (ARN) of the container instance on which the container is running.</p>
    #[serde(rename = "containerInstanceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_instance_arn: Option<String>,
    /// <p><p>The environment variables to pass to a container.</p> <note> <p>Environment variables must not start with <code>AWS_BATCH</code>; this naming convention is reserved for variables that are set by the AWS Batch service.</p> </note></p>
    #[serde(rename = "environment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<Vec<KeyValuePair>>,
    /// <p>The exit code to return upon completion.</p>
    #[serde(rename = "exitCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exit_code: Option<i64>,
    /// <p>The image used to start the container.</p>
    #[serde(rename = "image")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    /// <p>The Amazon Resource Name (ARN) associated with the job upon execution. </p>
    #[serde(rename = "jobRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_role_arn: Option<String>,
    /// <p>The name of the CloudWatch Logs log stream associated with the container. The log group for AWS Batch jobs is <code>/aws/batch/job</code>. Each container attempt receives a log stream name when they reach the <code>RUNNING</code> status.</p>
    #[serde(rename = "logStreamName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_stream_name: Option<String>,
    /// <p>The number of MiB of memory reserved for the job.</p>
    #[serde(rename = "memory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<i64>,
    /// <p>The mount points for data volumes in your container.</p>
    #[serde(rename = "mountPoints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_points: Option<Vec<MountPoint>>,
    /// <p>When this parameter is true, the container is given elevated privileges on the host container instance (similar to the <code>root</code> user).</p>
    #[serde(rename = "privileged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privileged: Option<bool>,
    /// <p>When this parameter is true, the container is given read-only access to its root file system.</p>
    #[serde(rename = "readonlyRootFilesystem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readonly_root_filesystem: Option<bool>,
    /// <p>A short (255 max characters) human-readable string to provide additional details about a running or stopped container.</p>
    #[serde(rename = "reason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the Amazon ECS task that is associated with the container job. Each container attempt receives a task ARN when they reach the <code>STARTING</code> status.</p>
    #[serde(rename = "taskArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_arn: Option<String>,
    /// <p>A list of <code>ulimit</code> values to set in the container.</p>
    #[serde(rename = "ulimits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ulimits: Option<Vec<Ulimit>>,
    /// <p>The user name to use inside the container.</p>
    #[serde(rename = "user")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    /// <p>The number of VCPUs allocated for the job. </p>
    #[serde(rename = "vcpus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcpus: Option<i64>,
    /// <p>A list of volumes associated with the job.</p>
    #[serde(rename = "volumes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumes: Option<Vec<Volume>>,
}

/// <p>The overrides that should be sent to a container.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ContainerOverrides {
    /// <p>The command to send to the container that overrides the default command from the Docker image or the job definition.</p>
    #[serde(rename = "command")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<Vec<String>>,
    /// <p><p>The environment variables to send to the container. You can add new environment variables, which are added to the container at launch, or you can override the existing environment variables from the Docker image or the job definition.</p> <note> <p>Environment variables must not start with <code>AWS_BATCH</code>; this naming convention is reserved for variables that are set by the AWS Batch service.</p> </note></p>
    #[serde(rename = "environment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<Vec<KeyValuePair>>,
    /// <p>The number of MiB of memory reserved for the job. This value overrides the value set in the job definition.</p>
    #[serde(rename = "memory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<i64>,
    /// <p>The number of vCPUs to reserve for the container. This value overrides the value set in the job definition.</p>
    #[serde(rename = "vcpus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcpus: Option<i64>,
}

/// <p>Container properties are used in job definitions to describe the container that is launched as part of a job.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContainerProperties {
    /// <p>The command that is passed to the container. This parameter maps to <code>Cmd</code> in the <a href="https://docs.docker.com/engine/reference/api/docker_remote_api_v1.23/#create-a-container">Create a container</a> section of the <a href="https://docs.docker.com/engine/reference/api/docker_remote_api_v1.23/">Docker Remote API</a> and the <code>COMMAND</code> parameter to <a href="https://docs.docker.com/engine/reference/run/">docker run</a>. For more information, see <a href="https://docs.docker.com/engine/reference/builder/#cmd">https://docs.docker.com/engine/reference/builder/#cmd</a>.</p>
    #[serde(rename = "command")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<Vec<String>>,
    /// <p><p>The environment variables to pass to a container. This parameter maps to <code>Env</code> in the <a href="https://docs.docker.com/engine/reference/api/docker_remote_api_v1.23/#create-a-container">Create a container</a> section of the <a href="https://docs.docker.com/engine/reference/api/docker_remote_api_v1.23/">Docker Remote API</a> and the <code>--env</code> option to <a href="https://docs.docker.com/engine/reference/run/">docker run</a>.</p> <important> <p>We do not recommend using plaintext environment variables for sensitive information, such as credential data.</p> </important> <note> <p>Environment variables must not start with <code>AWS_BATCH</code>; this naming convention is reserved for variables that are set by the AWS Batch service.</p> </note></p>
    #[serde(rename = "environment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<Vec<KeyValuePair>>,
    /// <p><p>The image used to start a container. This string is passed directly to the Docker daemon. Images in the Docker Hub registry are available by default. Other repositories are specified with <code> <i>repository-url</i>/<i>image</i>:<i>tag</i> </code>. Up to 255 letters (uppercase and lowercase), numbers, hyphens, underscores, colons, periods, forward slashes, and number signs are allowed. This parameter maps to <code>Image</code> in the <a href="https://docs.docker.com/engine/reference/api/docker_remote_api_v1.23/#create-a-container">Create a container</a> section of the <a href="https://docs.docker.com/engine/reference/api/docker_remote_api_v1.23/">Docker Remote API</a> and the <code>IMAGE</code> parameter of <a href="https://docs.docker.com/engine/reference/run/">docker run</a>.</p> <ul> <li> <p>Images in Amazon ECR repositories use the full registry and repository URI (for example, <code>012345678910.dkr.ecr.&lt;region-name&gt;.amazonaws.com/&lt;repository-name&gt;</code>). </p> </li> <li> <p>Images in official repositories on Docker Hub use a single name (for example, <code>ubuntu</code> or <code>mongo</code>).</p> </li> <li> <p>Images in other repositories on Docker Hub are qualified with an organization name (for example, <code>amazon/amazon-ecs-agent</code>).</p> </li> <li> <p>Images in other online repositories are qualified further by a domain name (for example, <code>quay.io/assemblyline/ubuntu</code>).</p> </li> </ul></p>
    #[serde(rename = "image")]
    pub image: String,
    /// <p>The Amazon Resource Name (ARN) of the IAM role that the container can assume for AWS permissions.</p>
    #[serde(rename = "jobRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_role_arn: Option<String>,
    /// <p><p>The hard limit (in MiB) of memory to present to the container. If your container attempts to exceed the memory specified here, the container is killed. This parameter maps to <code>Memory</code> in the <a href="https://docs.docker.com/engine/reference/api/docker_remote_api_v1.23/#create-a-container">Create a container</a> section of the <a href="https://docs.docker.com/engine/reference/api/docker_remote_api_v1.23/">Docker Remote API</a> and the <code>--memory</code> option to <a href="https://docs.docker.com/engine/reference/run/">docker run</a>. You must specify at least 4 MiB of memory for a job.</p> <note> <p>If you are trying to maximize your resource utilization by providing your jobs as much memory as possible for a particular instance type, see <a href="http://docs.aws.amazon.com/batch/latest/userguide/memory-management.html">Memory Management</a> in the <i>AWS Batch User Guide</i>.</p> </note></p>
    #[serde(rename = "memory")]
    pub memory: i64,
    /// <p>The mount points for data volumes in your container. This parameter maps to <code>Volumes</code> in the <a href="https://docs.docker.com/engine/reference/api/docker_remote_api_v1.23/#create-a-container">Create a container</a> section of the <a href="https://docs.docker.com/engine/reference/api/docker_remote_api_v1.23/">Docker Remote API</a> and the <code>--volume</code> option to <a href="https://docs.docker.com/engine/reference/run/">docker run</a>.</p>
    #[serde(rename = "mountPoints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_points: Option<Vec<MountPoint>>,
    /// <p>When this parameter is true, the container is given elevated privileges on the host container instance (similar to the <code>root</code> user). This parameter maps to <code>Privileged</code> in the <a href="https://docs.docker.com/engine/reference/api/docker_remote_api_v1.23/#create-a-container">Create a container</a> section of the <a href="https://docs.docker.com/engine/reference/api/docker_remote_api_v1.23/">Docker Remote API</a> and the <code>--privileged</code> option to <a href="https://docs.docker.com/engine/reference/run/">docker run</a>.</p>
    #[serde(rename = "privileged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privileged: Option<bool>,
    /// <p>When this parameter is true, the container is given read-only access to its root file system. This parameter maps to <code>ReadonlyRootfs</code> in the <a href="https://docs.docker.com/engine/reference/api/docker_remote_api_v1.23/#create-a-container">Create a container</a> section of the <a href="https://docs.docker.com/engine/reference/api/docker_remote_api_v1.23/">Docker Remote API</a> and the <code>--read-only</code> option to <code>docker run</code>.</p>
    #[serde(rename = "readonlyRootFilesystem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readonly_root_filesystem: Option<bool>,
    /// <p>A list of <code>ulimits</code> to set in the container. This parameter maps to <code>Ulimits</code> in the <a href="https://docs.docker.com/engine/reference/api/docker_remote_api_v1.23/#create-a-container">Create a container</a> section of the <a href="https://docs.docker.com/engine/reference/api/docker_remote_api_v1.23/">Docker Remote API</a> and the <code>--ulimit</code> option to <a href="https://docs.docker.com/engine/reference/run/">docker run</a>.</p>
    #[serde(rename = "ulimits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ulimits: Option<Vec<Ulimit>>,
    /// <p>The user name to use inside the container. This parameter maps to <code>User</code> in the <a href="https://docs.docker.com/engine/reference/api/docker_remote_api_v1.23/#create-a-container">Create a container</a> section of the <a href="https://docs.docker.com/engine/reference/api/docker_remote_api_v1.23/">Docker Remote API</a> and the <code>--user</code> option to <a href="https://docs.docker.com/engine/reference/run/">docker run</a>.</p>
    #[serde(rename = "user")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    /// <p>The number of vCPUs reserved for the container. This parameter maps to <code>CpuShares</code> in the <a href="https://docs.docker.com/engine/reference/api/docker_remote_api_v1.23/#create-a-container">Create a container</a> section of the <a href="https://docs.docker.com/engine/reference/api/docker_remote_api_v1.23/">Docker Remote API</a> and the <code>--cpu-shares</code> option to <a href="https://docs.docker.com/engine/reference/run/">docker run</a>. Each vCPU is equivalent to 1,024 CPU shares. You must specify at least one vCPU.</p>
    #[serde(rename = "vcpus")]
    pub vcpus: i64,
    /// <p>A list of data volumes used in a job.</p>
    #[serde(rename = "volumes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumes: Option<Vec<Volume>>,
}

/// <p>An object representing summary details of a container within a job.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ContainerSummary {
    /// <p>The exit code to return upon completion.</p>
    #[serde(rename = "exitCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exit_code: Option<i64>,
    /// <p>A short (255 max characters) human-readable string to provide additional details about a running or stopped container.</p>
    #[serde(rename = "reason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateComputeEnvironmentRequest {
    /// <p>The name for your compute environment. Up to 128 letters (uppercase and lowercase), numbers, hyphens, and underscores are allowed.</p>
    #[serde(rename = "computeEnvironmentName")]
    pub compute_environment_name: String,
    /// <p>Details of the compute resources managed by the compute environment. This parameter is required for managed compute environments.</p>
    #[serde(rename = "computeResources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_resources: Option<ComputeResource>,
    /// <p><p>The full Amazon Resource Name (ARN) of the IAM role that allows AWS Batch to make calls to other AWS services on your behalf.</p> <p>If your specified role has a path other than <code>/</code>, then you must either specify the full role ARN (this is recommended) or prefix the role name with the path.</p> <note> <p>Depending on how you created your AWS Batch service role, its ARN may contain the <code>service-role</code> path prefix. When you only specify the name of the service role, AWS Batch assumes that your ARN does not use the <code>service-role</code> path prefix. Because of this, we recommend that you specify the full ARN of your service role when you create compute environments.</p> </note></p>
    #[serde(rename = "serviceRole")]
    pub service_role: String,
    /// <p>The state of the compute environment. If the state is <code>ENABLED</code>, then the compute environment accepts jobs from a queue and can scale out automatically based on queues.</p>
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>The type of the compute environment. </p>
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateComputeEnvironmentResponse {
    /// <p>The Amazon Resource Name (ARN) of the compute environment. </p>
    #[serde(rename = "computeEnvironmentArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_environment_arn: Option<String>,
    /// <p>The name of the compute environment.</p>
    #[serde(rename = "computeEnvironmentName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_environment_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateJobQueueRequest {
    /// <p>The set of compute environments mapped to a job queue and their order relative to each other. The job scheduler uses this parameter to determine which compute environment should execute a given job. Compute environments must be in the <code>VALID</code> state before you can associate them with a job queue. You can associate up to three compute environments with a job queue.</p>
    #[serde(rename = "computeEnvironmentOrder")]
    pub compute_environment_order: Vec<ComputeEnvironmentOrder>,
    /// <p>The name of the job queue.</p>
    #[serde(rename = "jobQueueName")]
    pub job_queue_name: String,
    /// <p>The priority of the job queue. Job queues with a higher priority (or a higher integer value for the <code>priority</code> parameter) are evaluated first when associated with same compute environment. Priority is determined in descending order, for example, a job queue with a priority value of <code>10</code> is given scheduling preference over a job queue with a priority value of <code>1</code>.</p>
    #[serde(rename = "priority")]
    pub priority: i64,
    /// <p>The state of the job queue. If the job queue state is <code>ENABLED</code>, it is able to accept jobs.</p>
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateJobQueueResponse {
    /// <p>The Amazon Resource Name (ARN) of the job queue.</p>
    #[serde(rename = "jobQueueArn")]
    pub job_queue_arn: String,
    /// <p>The name of the job queue.</p>
    #[serde(rename = "jobQueueName")]
    pub job_queue_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteComputeEnvironmentRequest {
    /// <p>The name or Amazon Resource Name (ARN) of the compute environment to delete. </p>
    #[serde(rename = "computeEnvironment")]
    pub compute_environment: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteComputeEnvironmentResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteJobQueueRequest {
    /// <p>The short name or full Amazon Resource Name (ARN) of the queue to delete. </p>
    #[serde(rename = "jobQueue")]
    pub job_queue: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteJobQueueResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeregisterJobDefinitionRequest {
    /// <p>The name and revision (<code>name:revision</code>) or full Amazon Resource Name (ARN) of the job definition to deregister. </p>
    #[serde(rename = "jobDefinition")]
    pub job_definition: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeregisterJobDefinitionResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeComputeEnvironmentsRequest {
    /// <p>A list of up to 100 compute environment names or full Amazon Resource Name (ARN) entries. </p>
    #[serde(rename = "computeEnvironments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_environments: Option<Vec<String>>,
    /// <p>The maximum number of cluster results returned by <code>DescribeComputeEnvironments</code> in paginated output. When this parameter is used, <code>DescribeComputeEnvironments</code> only returns <code>maxResults</code> results in a single page along with a <code>nextToken</code> response element. The remaining results of the initial request can be seen by sending another <code>DescribeComputeEnvironments</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 100. If this parameter is not used, then <code>DescribeComputeEnvironments</code> returns up to 100 results and a <code>nextToken</code> value if applicable.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p><p>The <code>nextToken</code> value returned from a previous paginated <code>DescribeComputeEnvironments</code> request where <code>maxResults</code> was used and the results exceeded the value of that parameter. Pagination continues from the end of the previous results that returned the <code>nextToken</code> value. This value is <code>null</code> when there are no more results to return.</p> <note> <p>This token should be treated as an opaque identifier that is only used to retrieve the next items in a list and not for other programmatic purposes.</p> </note></p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeComputeEnvironmentsResponse {
    /// <p>The list of compute environments.</p>
    #[serde(rename = "computeEnvironments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_environments: Option<Vec<ComputeEnvironmentDetail>>,
    /// <p>The <code>nextToken</code> value to include in a future <code>DescribeComputeEnvironments</code> request. When the results of a <code>DescribeJobDefinitions</code> request exceed <code>maxResults</code>, this value can be used to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeJobDefinitionsRequest {
    /// <p>The name of the job definition to describe.</p>
    #[serde(rename = "jobDefinitionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_definition_name: Option<String>,
    /// <p>A space-separated list of up to 100 job definition names or full Amazon Resource Name (ARN) entries.</p>
    #[serde(rename = "jobDefinitions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_definitions: Option<Vec<String>>,
    /// <p>The maximum number of results returned by <code>DescribeJobDefinitions</code> in paginated output. When this parameter is used, <code>DescribeJobDefinitions</code> only returns <code>maxResults</code> results in a single page along with a <code>nextToken</code> response element. The remaining results of the initial request can be seen by sending another <code>DescribeJobDefinitions</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 100. If this parameter is not used, then <code>DescribeJobDefinitions</code> returns up to 100 results and a <code>nextToken</code> value if applicable.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p><p>The <code>nextToken</code> value returned from a previous paginated <code>DescribeJobDefinitions</code> request where <code>maxResults</code> was used and the results exceeded the value of that parameter. Pagination continues from the end of the previous results that returned the <code>nextToken</code> value. This value is <code>null</code> when there are no more results to return.</p> <note> <p>This token should be treated as an opaque identifier that is only used to retrieve the next items in a list and not for other programmatic purposes.</p> </note></p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The status with which to filter job definitions.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeJobDefinitionsResponse {
    /// <p>The list of job definitions. </p>
    #[serde(rename = "jobDefinitions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_definitions: Option<Vec<JobDefinition>>,
    /// <p>The <code>nextToken</code> value to include in a future <code>DescribeJobDefinitions</code> request. When the results of a <code>DescribeJobDefinitions</code> request exceed <code>maxResults</code>, this value can be used to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeJobQueuesRequest {
    /// <p>A list of up to 100 queue names or full queue Amazon Resource Name (ARN) entries.</p>
    #[serde(rename = "jobQueues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_queues: Option<Vec<String>>,
    /// <p>The maximum number of results returned by <code>DescribeJobQueues</code> in paginated output. When this parameter is used, <code>DescribeJobQueues</code> only returns <code>maxResults</code> results in a single page along with a <code>nextToken</code> response element. The remaining results of the initial request can be seen by sending another <code>DescribeJobQueues</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 100. If this parameter is not used, then <code>DescribeJobQueues</code> returns up to 100 results and a <code>nextToken</code> value if applicable.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p><p>The <code>nextToken</code> value returned from a previous paginated <code>DescribeJobQueues</code> request where <code>maxResults</code> was used and the results exceeded the value of that parameter. Pagination continues from the end of the previous results that returned the <code>nextToken</code> value. This value is <code>null</code> when there are no more results to return.</p> <note> <p>This token should be treated as an opaque identifier that is only used to retrieve the next items in a list and not for other programmatic purposes.</p> </note></p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeJobQueuesResponse {
    /// <p>The list of job queues. </p>
    #[serde(rename = "jobQueues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_queues: Option<Vec<JobQueueDetail>>,
    /// <p>The <code>nextToken</code> value to include in a future <code>DescribeJobQueues</code> request. When the results of a <code>DescribeJobQueues</code> request exceed <code>maxResults</code>, this value can be used to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeJobsRequest {
    /// <p>A space-separated list of up to 100 job IDs.</p>
    #[serde(rename = "jobs")]
    pub jobs: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeJobsResponse {
    /// <p>The list of jobs. </p>
    #[serde(rename = "jobs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jobs: Option<Vec<JobDetail>>,
}

/// <p>The contents of the <code>host</code> parameter determine whether your data volume persists on the host container instance and where it is stored. If the host parameter is empty, then the Docker daemon assigns a host path for your data volume, but the data is not guaranteed to persist after the containers associated with it stop running.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Host {
    /// <p>The path on the host container instance that is presented to the container. If this parameter is empty, then the Docker daemon has assigned a host path for you. If the <code>host</code> parameter contains a <code>sourcePath</code> file location, then the data volume persists at the specified location on the host container instance until you delete it manually. If the <code>sourcePath</code> value does not exist on the host container instance, the Docker daemon creates it. If the location does exist, the contents of the source path folder are exported.</p>
    #[serde(rename = "sourcePath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_path: Option<String>,
}

/// <p>An object representing an AWS Batch job definition.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct JobDefinition {
    /// <p>An object with various properties specific to container-based jobs. </p>
    #[serde(rename = "containerProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_properties: Option<ContainerProperties>,
    /// <p>The Amazon Resource Name (ARN) for the job definition. </p>
    #[serde(rename = "jobDefinitionArn")]
    pub job_definition_arn: String,
    /// <p>The name of the job definition. </p>
    #[serde(rename = "jobDefinitionName")]
    pub job_definition_name: String,
    /// <p>Default parameters or parameter substitution placeholders that are set in the job definition. Parameters are specified as a key-value pair mapping. Parameters in a <code>SubmitJob</code> request override any corresponding parameter defaults from the job definition.</p>
    #[serde(rename = "parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<::std::collections::HashMap<String, String>>,
    /// <p>The retry strategy to use for failed jobs that are submitted with this job definition.</p>
    #[serde(rename = "retryStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_strategy: Option<RetryStrategy>,
    /// <p>The revision of the job definition.</p>
    #[serde(rename = "revision")]
    pub revision: i64,
    /// <p>The status of the job definition.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The timeout configuration for jobs that are submitted with this job definition. You can specify a timeout duration after which AWS Batch terminates your jobs if they have not finished.</p>
    #[serde(rename = "timeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<JobTimeout>,
    /// <p>The type of job definition.</p>
    #[serde(rename = "type")]
    pub type_: String,
}

/// <p>An object representing an AWS Batch job dependency.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JobDependency {
    /// <p>The job ID of the AWS Batch job associated with this dependency.</p>
    #[serde(rename = "jobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    /// <p>The type of the job dependency.</p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>An object representing an AWS Batch job.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct JobDetail {
    /// <p>The array properties of the job, if it is an array job.</p>
    #[serde(rename = "arrayProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub array_properties: Option<ArrayPropertiesDetail>,
    /// <p>A list of job attempts associated with this job.</p>
    #[serde(rename = "attempts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attempts: Option<Vec<AttemptDetail>>,
    /// <p>An object representing the details of the container that is associated with the job.</p>
    #[serde(rename = "container")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container: Option<ContainerDetail>,
    /// <p>The Unix time stamp (in seconds and milliseconds) for when the job was created. For non-array jobs and parent array jobs, this is when the job entered the <code>SUBMITTED</code> state (at the time <a>SubmitJob</a> was called). For array child jobs, this is when the child job was spawned by its parent and entered the <code>PENDING</code> state.</p>
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
    /// <p>A list of job names or IDs on which this job depends.</p>
    #[serde(rename = "dependsOn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depends_on: Option<Vec<JobDependency>>,
    /// <p>The job definition that is used by this job.</p>
    #[serde(rename = "jobDefinition")]
    pub job_definition: String,
    /// <p>The ID for the job.</p>
    #[serde(rename = "jobId")]
    pub job_id: String,
    /// <p>The name of the job.</p>
    #[serde(rename = "jobName")]
    pub job_name: String,
    /// <p>The Amazon Resource Name (ARN) of the job queue with which the job is associated.</p>
    #[serde(rename = "jobQueue")]
    pub job_queue: String,
    /// <p>Additional parameters passed to the job that replace parameter substitution placeholders or override any corresponding parameter defaults from the job definition. </p>
    #[serde(rename = "parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<::std::collections::HashMap<String, String>>,
    /// <p>The retry strategy to use for this job if an attempt fails.</p>
    #[serde(rename = "retryStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_strategy: Option<RetryStrategy>,
    /// <p>The Unix time stamp (in seconds and milliseconds) for when the job was started (when the job transitioned from the <code>STARTING</code> state to the <code>RUNNING</code> state).</p>
    #[serde(rename = "startedAt")]
    pub started_at: i64,
    /// <p>The current status for the job.</p>
    #[serde(rename = "status")]
    pub status: String,
    /// <p>A short, human-readable string to provide additional details about the current status of the job. </p>
    #[serde(rename = "statusReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
    /// <p>The Unix time stamp (in seconds and milliseconds) for when the job was stopped (when the job transitioned from the <code>RUNNING</code> state to a terminal state, such as <code>SUCCEEDED</code> or <code>FAILED</code>).</p>
    #[serde(rename = "stoppedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stopped_at: Option<i64>,
    /// <p>The timeout configuration for the job. </p>
    #[serde(rename = "timeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<JobTimeout>,
}

/// <p>An object representing the details of an AWS Batch job queue.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct JobQueueDetail {
    /// <p>The compute environments that are attached to the job queue and the order in which job placement is preferred. Compute environments are selected for job placement in ascending order.</p>
    #[serde(rename = "computeEnvironmentOrder")]
    pub compute_environment_order: Vec<ComputeEnvironmentOrder>,
    /// <p>The Amazon Resource Name (ARN) of the job queue.</p>
    #[serde(rename = "jobQueueArn")]
    pub job_queue_arn: String,
    /// <p>The name of the job queue.</p>
    #[serde(rename = "jobQueueName")]
    pub job_queue_name: String,
    /// <p>The priority of the job queue. </p>
    #[serde(rename = "priority")]
    pub priority: i64,
    /// <p>Describes the ability of the queue to accept new jobs.</p>
    #[serde(rename = "state")]
    pub state: String,
    /// <p>The status of the job queue (for example, <code>CREATING</code> or <code>VALID</code>).</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>A short, human-readable string to provide additional details about the current status of the job queue.</p>
    #[serde(rename = "statusReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
}

/// <p>An object representing summary details of a job.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct JobSummary {
    /// <p>The array properties of the job, if it is an array job.</p>
    #[serde(rename = "arrayProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub array_properties: Option<ArrayPropertiesSummary>,
    /// <p>An object representing the details of the container that is associated with the job.</p>
    #[serde(rename = "container")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container: Option<ContainerSummary>,
    /// <p>The Unix time stamp for when the job was created. For non-array jobs and parent array jobs, this is when the job entered the <code>SUBMITTED</code> state (at the time <a>SubmitJob</a> was called). For array child jobs, this is when the child job was spawned by its parent and entered the <code>PENDING</code> state.</p>
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
    /// <p>The ID of the job.</p>
    #[serde(rename = "jobId")]
    pub job_id: String,
    /// <p>The name of the job.</p>
    #[serde(rename = "jobName")]
    pub job_name: String,
    /// <p>The Unix time stamp for when the job was started (when the job transitioned from the <code>STARTING</code> state to the <code>RUNNING</code> state).</p>
    #[serde(rename = "startedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<i64>,
    /// <p>The current status for the job.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>A short, human-readable string to provide additional details about the current status of the job.</p>
    #[serde(rename = "statusReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
    /// <p>The Unix time stamp for when the job was stopped (when the job transitioned from the <code>RUNNING</code> state to a terminal state, such as <code>SUCCEEDED</code> or <code>FAILED</code>).</p>
    #[serde(rename = "stoppedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stopped_at: Option<i64>,
}

/// <p>An object representing a job timeout configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JobTimeout {
    /// <p>The time duration in seconds (measured from the job attempt's <code>startedAt</code> timestamp) after which AWS Batch terminates your jobs if they have not finished.</p>
    #[serde(rename = "attemptDurationSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attempt_duration_seconds: Option<i64>,
}

/// <p>A key-value pair object.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KeyValuePair {
    /// <p>The name of the key-value pair. For environment variables, this is the name of the environment variable.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The value of the key-value pair. For environment variables, this is the value of the environment variable.</p>
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListJobsRequest {
    /// <p>The job ID for an array job. Specifying an array job ID with this parameter lists all child jobs from within the specified array.</p>
    #[serde(rename = "arrayJobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub array_job_id: Option<String>,
    /// <p>The name or full Amazon Resource Name (ARN) of the job queue with which to list jobs.</p>
    #[serde(rename = "jobQueue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_queue: Option<String>,
    /// <p>The job status with which to filter jobs in the specified queue. If you do not specify a status, only <code>RUNNING</code> jobs are returned.</p>
    #[serde(rename = "jobStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
    /// <p>The maximum number of results returned by <code>ListJobs</code> in paginated output. When this parameter is used, <code>ListJobs</code> only returns <code>maxResults</code> results in a single page along with a <code>nextToken</code> response element. The remaining results of the initial request can be seen by sending another <code>ListJobs</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 100. If this parameter is not used, then <code>ListJobs</code> returns up to 100 results and a <code>nextToken</code> value if applicable.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p><p>The <code>nextToken</code> value returned from a previous paginated <code>ListJobs</code> request where <code>maxResults</code> was used and the results exceeded the value of that parameter. Pagination continues from the end of the previous results that returned the <code>nextToken</code> value. This value is <code>null</code> when there are no more results to return.</p> <note> <p>This token should be treated as an opaque identifier that is only used to retrieve the next items in a list and not for other programmatic purposes.</p> </note></p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListJobsResponse {
    /// <p>A list of job summaries that match the request.</p>
    #[serde(rename = "jobSummaryList")]
    pub job_summary_list: Vec<JobSummary>,
    /// <p>The <code>nextToken</code> value to include in a future <code>ListJobs</code> request. When the results of a <code>ListJobs</code> request exceed <code>maxResults</code>, this value can be used to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Details on a Docker volume mount point that is used in a job's container properties.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MountPoint {
    /// <p>The path on the container at which to mount the host volume.</p>
    #[serde(rename = "containerPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_path: Option<String>,
    /// <p>If this value is <code>true</code>, the container has read-only access to the volume; otherwise, the container can write to the volume. The default value is <code>false</code>.</p>
    #[serde(rename = "readOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    /// <p>The name of the volume to mount.</p>
    #[serde(rename = "sourceVolume")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_volume: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RegisterJobDefinitionRequest {
    /// <p>An object with various properties specific for container-based jobs. This parameter is required if the <code>type</code> parameter is <code>container</code>.</p>
    #[serde(rename = "containerProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_properties: Option<ContainerProperties>,
    /// <p>The name of the job definition to register. Up to 128 letters (uppercase and lowercase), numbers, hyphens, and underscores are allowed.</p>
    #[serde(rename = "jobDefinitionName")]
    pub job_definition_name: String,
    /// <p>Default parameter substitution placeholders to set in the job definition. Parameters are specified as a key-value pair mapping. Parameters in a <code>SubmitJob</code> request override any corresponding parameter defaults from the job definition.</p>
    #[serde(rename = "parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<::std::collections::HashMap<String, String>>,
    /// <p>The retry strategy to use for failed jobs that are submitted with this job definition. Any retry strategy that is specified during a <a>SubmitJob</a> operation overrides the retry strategy defined here. If a job is terminated due to a timeout, it is not retried. </p>
    #[serde(rename = "retryStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_strategy: Option<RetryStrategy>,
    /// <p>The timeout configuration for jobs that are submitted with this job definition, after which AWS Batch terminates your jobs if they have not finished. If a job is terminated due to a timeout, it is not retried. The minimum value for the timeout is 60 seconds. Any timeout configuration that is specified during a <a>SubmitJob</a> operation overrides the timeout configuration defined here. For more information, see <a href="http://docs.aws.amazon.com/AmazonECS/latest/developerguide/job_timeouts.html">Job Timeouts</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    #[serde(rename = "timeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<JobTimeout>,
    /// <p>The type of job definition.</p>
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct RegisterJobDefinitionResponse {
    /// <p>The Amazon Resource Name (ARN) of the job definition. </p>
    #[serde(rename = "jobDefinitionArn")]
    pub job_definition_arn: String,
    /// <p>The name of the job definition.</p>
    #[serde(rename = "jobDefinitionName")]
    pub job_definition_name: String,
    /// <p>The revision of the job definition.</p>
    #[serde(rename = "revision")]
    pub revision: i64,
}

/// <p>The retry strategy associated with a job.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RetryStrategy {
    /// <p>The number of times to move a job to the <code>RUNNABLE</code> status. You may specify between 1 and 10 attempts. If the value of <code>attempts</code> is greater than one, the job is retried if it fails until it has moved to <code>RUNNABLE</code> that many times.</p>
    #[serde(rename = "attempts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attempts: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct SubmitJobRequest {
    /// <p>The array properties for the submitted job, such as the size of the array. The array size can be between 2 and 10,000. If you specify array properties for a job, it becomes an array job. For more information, see <a href="http://docs.aws.amazon.com/batch/latest/userguide/array_jobs.html">Array Jobs</a> in the <i>AWS Batch User Guide</i>.</p>
    #[serde(rename = "arrayProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub array_properties: Option<ArrayProperties>,
    /// <p>A list of container overrides in JSON format that specify the name of a container in the specified job definition and the overrides it should receive. You can override the default command for a container (that is specified in the job definition or the Docker image) with a <code>command</code> override. You can also override existing environment variables (that are specified in the job definition or Docker image) on a container or add new environment variables to it with an <code>environment</code> override.</p>
    #[serde(rename = "containerOverrides")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_overrides: Option<ContainerOverrides>,
    /// <p>A list of dependencies for the job. A job can depend upon a maximum of 20 jobs. You can specify a <code>SEQUENTIAL</code> type dependency without specifying a job ID for array jobs so that each child array job completes sequentially, starting at index 0. You can also specify an <code>N_TO_N</code> type dependency with a job ID for array jobs so that each index child of this job must wait for the corresponding index child of each dependency to complete before it can begin.</p>
    #[serde(rename = "dependsOn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depends_on: Option<Vec<JobDependency>>,
    /// <p>The job definition used by this job. This value can be either a <code>name:revision</code> or the Amazon Resource Name (ARN) for the job definition.</p>
    #[serde(rename = "jobDefinition")]
    pub job_definition: String,
    /// <p>The name of the job. The first character must be alphanumeric, and up to 128 letters (uppercase and lowercase), numbers, hyphens, and underscores are allowed. </p>
    #[serde(rename = "jobName")]
    pub job_name: String,
    /// <p>The job queue into which the job is submitted. You can specify either the name or the Amazon Resource Name (ARN) of the queue. </p>
    #[serde(rename = "jobQueue")]
    pub job_queue: String,
    /// <p>Additional parameters passed to the job that replace parameter substitution placeholders that are set in the job definition. Parameters are specified as a key and value pair mapping. Parameters in a <code>SubmitJob</code> request override any corresponding parameter defaults from the job definition.</p>
    #[serde(rename = "parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<::std::collections::HashMap<String, String>>,
    /// <p>The retry strategy to use for failed jobs from this <a>SubmitJob</a> operation. When a retry strategy is specified here, it overrides the retry strategy defined in the job definition.</p>
    #[serde(rename = "retryStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_strategy: Option<RetryStrategy>,
    /// <p>The timeout configuration for this <a>SubmitJob</a> operation. You can specify a timeout duration after which AWS Batch terminates your jobs if they have not finished. If a job is terminated due to a timeout, it is not retried. The minimum value for the timeout is 60 seconds. This configuration overrides any timeout configuration specified in the job definition. For array jobs, child jobs have the same timeout configuration as the parent job. For more information, see <a href="http://docs.aws.amazon.com/AmazonECS/latest/developerguide/job_timeouts.html">Job Timeouts</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    #[serde(rename = "timeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<JobTimeout>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct SubmitJobResponse {
    /// <p>The unique identifier for the job.</p>
    #[serde(rename = "jobId")]
    pub job_id: String,
    /// <p>The name of the job. </p>
    #[serde(rename = "jobName")]
    pub job_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct TerminateJobRequest {
    /// <p>The AWS Batch job ID of the job to terminate.</p>
    #[serde(rename = "jobId")]
    pub job_id: String,
    /// <p>A message to attach to the job that explains the reason for canceling it. This message is returned by future <a>DescribeJobs</a> operations on the job. This message is also recorded in the AWS Batch activity logs. </p>
    #[serde(rename = "reason")]
    pub reason: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct TerminateJobResponse {}

/// <p>The <code>ulimit</code> settings to pass to the container.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ulimit {
    /// <p>The hard limit for the <code>ulimit</code> type.</p>
    #[serde(rename = "hardLimit")]
    pub hard_limit: i64,
    /// <p>The <code>type</code> of the <code>ulimit</code>.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The soft limit for the <code>ulimit</code> type.</p>
    #[serde(rename = "softLimit")]
    pub soft_limit: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateComputeEnvironmentRequest {
    /// <p>The name or full Amazon Resource Name (ARN) of the compute environment to update.</p>
    #[serde(rename = "computeEnvironment")]
    pub compute_environment: String,
    /// <p>Details of the compute resources managed by the compute environment. Required for a managed compute environment.</p>
    #[serde(rename = "computeResources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_resources: Option<ComputeResourceUpdate>,
    /// <p><p>The full Amazon Resource Name (ARN) of the IAM role that allows AWS Batch to make calls to other AWS services on your behalf.</p> <p>If your specified role has a path other than <code>/</code>, then you must either specify the full role ARN (this is recommended) or prefix the role name with the path.</p> <note> <p>Depending on how you created your AWS Batch service role, its ARN may contain the <code>service-role</code> path prefix. When you only specify the name of the service role, AWS Batch assumes that your ARN does not use the <code>service-role</code> path prefix. Because of this, we recommend that you specify the full ARN of your service role when you create compute environments.</p> </note></p>
    #[serde(rename = "serviceRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_role: Option<String>,
    /// <p>The state of the compute environment. Compute environments in the <code>ENABLED</code> state can accept jobs from a queue and scale in or out automatically based on the workload demand of its associated queues.</p>
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateComputeEnvironmentResponse {
    /// <p>The Amazon Resource Name (ARN) of the compute environment. </p>
    #[serde(rename = "computeEnvironmentArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_environment_arn: Option<String>,
    /// <p>The name of compute environment.</p>
    #[serde(rename = "computeEnvironmentName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_environment_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateJobQueueRequest {
    /// <p>Details the set of compute environments mapped to a job queue and their order relative to each other. This is one of the parameters used by the job scheduler to determine which compute environment should execute a given job. </p>
    #[serde(rename = "computeEnvironmentOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_environment_order: Option<Vec<ComputeEnvironmentOrder>>,
    /// <p>The name or the Amazon Resource Name (ARN) of the job queue.</p>
    #[serde(rename = "jobQueue")]
    pub job_queue: String,
    /// <p>The priority of the job queue. Job queues with a higher priority (or a higher integer value for the <code>priority</code> parameter) are evaluated first when associated with same compute environment. Priority is determined in descending order, for example, a job queue with a priority value of <code>10</code> is given scheduling preference over a job queue with a priority value of <code>1</code>.</p>
    #[serde(rename = "priority")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,
    /// <p>Describes the queue's ability to accept new jobs.</p>
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateJobQueueResponse {
    /// <p>The Amazon Resource Name (ARN) of the job queue.</p>
    #[serde(rename = "jobQueueArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_queue_arn: Option<String>,
    /// <p>The name of the job queue.</p>
    #[serde(rename = "jobQueueName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_queue_name: Option<String>,
}

/// <p>A data volume used in a job's container properties.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Volume {
    /// <p>The contents of the <code>host</code> parameter determine whether your data volume persists on the host container instance and where it is stored. If the host parameter is empty, then the Docker daemon assigns a host path for your data volume. However, the data is not guaranteed to persist after the containers associated with it stop running.</p>
    #[serde(rename = "host")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<Host>,
    /// <p>The name of the volume. Up to 255 letters (uppercase and lowercase), numbers, hyphens, and underscores are allowed. This name is referenced in the <code>sourceVolume</code> parameter of container definition <code>mountPoints</code>.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Errors returned by CancelJob
#[derive(Debug, PartialEq)]
pub enum CancelJobError {
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an identifier that is not valid. </p>
    Client(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
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

impl CancelJobError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> CancelJobError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "ClientException" => return CancelJobError::Client(String::from(error_message)),
                "ServerException" => return CancelJobError::Server(String::from(error_message)),
                "ValidationException" => {
                    return CancelJobError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return CancelJobError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for CancelJobError {
    fn from(err: serde_json::error::Error) -> CancelJobError {
        CancelJobError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for CancelJobError {
    fn from(err: CredentialsError) -> CancelJobError {
        CancelJobError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CancelJobError {
    fn from(err: HttpDispatchError) -> CancelJobError {
        CancelJobError::HttpDispatch(err)
    }
}
impl From<io::Error> for CancelJobError {
    fn from(err: io::Error) -> CancelJobError {
        CancelJobError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CancelJobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CancelJobError {
    fn description(&self) -> &str {
        match *self {
            CancelJobError::Client(ref cause) => cause,
            CancelJobError::Server(ref cause) => cause,
            CancelJobError::Validation(ref cause) => cause,
            CancelJobError::Credentials(ref err) => err.description(),
            CancelJobError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CancelJobError::ParseError(ref cause) => cause,
            CancelJobError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by CreateComputeEnvironment
#[derive(Debug, PartialEq)]
pub enum CreateComputeEnvironmentError {
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an identifier that is not valid. </p>
    Client(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
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

impl CreateComputeEnvironmentError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> CreateComputeEnvironmentError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "ClientException" => {
                    return CreateComputeEnvironmentError::Client(String::from(error_message))
                }
                "ServerException" => {
                    return CreateComputeEnvironmentError::Server(String::from(error_message))
                }
                "ValidationException" => {
                    return CreateComputeEnvironmentError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return CreateComputeEnvironmentError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for CreateComputeEnvironmentError {
    fn from(err: serde_json::error::Error) -> CreateComputeEnvironmentError {
        CreateComputeEnvironmentError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateComputeEnvironmentError {
    fn from(err: CredentialsError) -> CreateComputeEnvironmentError {
        CreateComputeEnvironmentError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateComputeEnvironmentError {
    fn from(err: HttpDispatchError) -> CreateComputeEnvironmentError {
        CreateComputeEnvironmentError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateComputeEnvironmentError {
    fn from(err: io::Error) -> CreateComputeEnvironmentError {
        CreateComputeEnvironmentError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateComputeEnvironmentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateComputeEnvironmentError {
    fn description(&self) -> &str {
        match *self {
            CreateComputeEnvironmentError::Client(ref cause) => cause,
            CreateComputeEnvironmentError::Server(ref cause) => cause,
            CreateComputeEnvironmentError::Validation(ref cause) => cause,
            CreateComputeEnvironmentError::Credentials(ref err) => err.description(),
            CreateComputeEnvironmentError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateComputeEnvironmentError::ParseError(ref cause) => cause,
            CreateComputeEnvironmentError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by CreateJobQueue
#[derive(Debug, PartialEq)]
pub enum CreateJobQueueError {
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an identifier that is not valid. </p>
    Client(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
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

impl CreateJobQueueError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> CreateJobQueueError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "ClientException" => {
                    return CreateJobQueueError::Client(String::from(error_message))
                }
                "ServerException" => {
                    return CreateJobQueueError::Server(String::from(error_message))
                }
                "ValidationException" => {
                    return CreateJobQueueError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return CreateJobQueueError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for CreateJobQueueError {
    fn from(err: serde_json::error::Error) -> CreateJobQueueError {
        CreateJobQueueError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateJobQueueError {
    fn from(err: CredentialsError) -> CreateJobQueueError {
        CreateJobQueueError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateJobQueueError {
    fn from(err: HttpDispatchError) -> CreateJobQueueError {
        CreateJobQueueError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateJobQueueError {
    fn from(err: io::Error) -> CreateJobQueueError {
        CreateJobQueueError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateJobQueueError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateJobQueueError {
    fn description(&self) -> &str {
        match *self {
            CreateJobQueueError::Client(ref cause) => cause,
            CreateJobQueueError::Server(ref cause) => cause,
            CreateJobQueueError::Validation(ref cause) => cause,
            CreateJobQueueError::Credentials(ref err) => err.description(),
            CreateJobQueueError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateJobQueueError::ParseError(ref cause) => cause,
            CreateJobQueueError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteComputeEnvironment
#[derive(Debug, PartialEq)]
pub enum DeleteComputeEnvironmentError {
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an identifier that is not valid. </p>
    Client(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
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

impl DeleteComputeEnvironmentError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> DeleteComputeEnvironmentError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "ClientException" => {
                    return DeleteComputeEnvironmentError::Client(String::from(error_message))
                }
                "ServerException" => {
                    return DeleteComputeEnvironmentError::Server(String::from(error_message))
                }
                "ValidationException" => {
                    return DeleteComputeEnvironmentError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DeleteComputeEnvironmentError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteComputeEnvironmentError {
    fn from(err: serde_json::error::Error) -> DeleteComputeEnvironmentError {
        DeleteComputeEnvironmentError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteComputeEnvironmentError {
    fn from(err: CredentialsError) -> DeleteComputeEnvironmentError {
        DeleteComputeEnvironmentError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteComputeEnvironmentError {
    fn from(err: HttpDispatchError) -> DeleteComputeEnvironmentError {
        DeleteComputeEnvironmentError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteComputeEnvironmentError {
    fn from(err: io::Error) -> DeleteComputeEnvironmentError {
        DeleteComputeEnvironmentError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteComputeEnvironmentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteComputeEnvironmentError {
    fn description(&self) -> &str {
        match *self {
            DeleteComputeEnvironmentError::Client(ref cause) => cause,
            DeleteComputeEnvironmentError::Server(ref cause) => cause,
            DeleteComputeEnvironmentError::Validation(ref cause) => cause,
            DeleteComputeEnvironmentError::Credentials(ref err) => err.description(),
            DeleteComputeEnvironmentError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteComputeEnvironmentError::ParseError(ref cause) => cause,
            DeleteComputeEnvironmentError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteJobQueue
#[derive(Debug, PartialEq)]
pub enum DeleteJobQueueError {
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an identifier that is not valid. </p>
    Client(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
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

impl DeleteJobQueueError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> DeleteJobQueueError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "ClientException" => {
                    return DeleteJobQueueError::Client(String::from(error_message))
                }
                "ServerException" => {
                    return DeleteJobQueueError::Server(String::from(error_message))
                }
                "ValidationException" => {
                    return DeleteJobQueueError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DeleteJobQueueError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteJobQueueError {
    fn from(err: serde_json::error::Error) -> DeleteJobQueueError {
        DeleteJobQueueError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteJobQueueError {
    fn from(err: CredentialsError) -> DeleteJobQueueError {
        DeleteJobQueueError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteJobQueueError {
    fn from(err: HttpDispatchError) -> DeleteJobQueueError {
        DeleteJobQueueError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteJobQueueError {
    fn from(err: io::Error) -> DeleteJobQueueError {
        DeleteJobQueueError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteJobQueueError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteJobQueueError {
    fn description(&self) -> &str {
        match *self {
            DeleteJobQueueError::Client(ref cause) => cause,
            DeleteJobQueueError::Server(ref cause) => cause,
            DeleteJobQueueError::Validation(ref cause) => cause,
            DeleteJobQueueError::Credentials(ref err) => err.description(),
            DeleteJobQueueError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteJobQueueError::ParseError(ref cause) => cause,
            DeleteJobQueueError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeregisterJobDefinition
#[derive(Debug, PartialEq)]
pub enum DeregisterJobDefinitionError {
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an identifier that is not valid. </p>
    Client(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
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

impl DeregisterJobDefinitionError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> DeregisterJobDefinitionError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "ClientException" => {
                    return DeregisterJobDefinitionError::Client(String::from(error_message))
                }
                "ServerException" => {
                    return DeregisterJobDefinitionError::Server(String::from(error_message))
                }
                "ValidationException" => {
                    return DeregisterJobDefinitionError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DeregisterJobDefinitionError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeregisterJobDefinitionError {
    fn from(err: serde_json::error::Error) -> DeregisterJobDefinitionError {
        DeregisterJobDefinitionError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeregisterJobDefinitionError {
    fn from(err: CredentialsError) -> DeregisterJobDefinitionError {
        DeregisterJobDefinitionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeregisterJobDefinitionError {
    fn from(err: HttpDispatchError) -> DeregisterJobDefinitionError {
        DeregisterJobDefinitionError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeregisterJobDefinitionError {
    fn from(err: io::Error) -> DeregisterJobDefinitionError {
        DeregisterJobDefinitionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeregisterJobDefinitionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeregisterJobDefinitionError {
    fn description(&self) -> &str {
        match *self {
            DeregisterJobDefinitionError::Client(ref cause) => cause,
            DeregisterJobDefinitionError::Server(ref cause) => cause,
            DeregisterJobDefinitionError::Validation(ref cause) => cause,
            DeregisterJobDefinitionError::Credentials(ref err) => err.description(),
            DeregisterJobDefinitionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeregisterJobDefinitionError::ParseError(ref cause) => cause,
            DeregisterJobDefinitionError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DescribeComputeEnvironments
#[derive(Debug, PartialEq)]
pub enum DescribeComputeEnvironmentsError {
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an identifier that is not valid. </p>
    Client(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
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

impl DescribeComputeEnvironmentsError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> DescribeComputeEnvironmentsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "ClientException" => {
                    return DescribeComputeEnvironmentsError::Client(String::from(error_message))
                }
                "ServerException" => {
                    return DescribeComputeEnvironmentsError::Server(String::from(error_message))
                }
                "ValidationException" => {
                    return DescribeComputeEnvironmentsError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DescribeComputeEnvironmentsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DescribeComputeEnvironmentsError {
    fn from(err: serde_json::error::Error) -> DescribeComputeEnvironmentsError {
        DescribeComputeEnvironmentsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeComputeEnvironmentsError {
    fn from(err: CredentialsError) -> DescribeComputeEnvironmentsError {
        DescribeComputeEnvironmentsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeComputeEnvironmentsError {
    fn from(err: HttpDispatchError) -> DescribeComputeEnvironmentsError {
        DescribeComputeEnvironmentsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeComputeEnvironmentsError {
    fn from(err: io::Error) -> DescribeComputeEnvironmentsError {
        DescribeComputeEnvironmentsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeComputeEnvironmentsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeComputeEnvironmentsError {
    fn description(&self) -> &str {
        match *self {
            DescribeComputeEnvironmentsError::Client(ref cause) => cause,
            DescribeComputeEnvironmentsError::Server(ref cause) => cause,
            DescribeComputeEnvironmentsError::Validation(ref cause) => cause,
            DescribeComputeEnvironmentsError::Credentials(ref err) => err.description(),
            DescribeComputeEnvironmentsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeComputeEnvironmentsError::ParseError(ref cause) => cause,
            DescribeComputeEnvironmentsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DescribeJobDefinitions
#[derive(Debug, PartialEq)]
pub enum DescribeJobDefinitionsError {
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an identifier that is not valid. </p>
    Client(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
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

impl DescribeJobDefinitionsError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> DescribeJobDefinitionsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "ClientException" => {
                    return DescribeJobDefinitionsError::Client(String::from(error_message))
                }
                "ServerException" => {
                    return DescribeJobDefinitionsError::Server(String::from(error_message))
                }
                "ValidationException" => {
                    return DescribeJobDefinitionsError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DescribeJobDefinitionsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DescribeJobDefinitionsError {
    fn from(err: serde_json::error::Error) -> DescribeJobDefinitionsError {
        DescribeJobDefinitionsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeJobDefinitionsError {
    fn from(err: CredentialsError) -> DescribeJobDefinitionsError {
        DescribeJobDefinitionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeJobDefinitionsError {
    fn from(err: HttpDispatchError) -> DescribeJobDefinitionsError {
        DescribeJobDefinitionsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeJobDefinitionsError {
    fn from(err: io::Error) -> DescribeJobDefinitionsError {
        DescribeJobDefinitionsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeJobDefinitionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeJobDefinitionsError {
    fn description(&self) -> &str {
        match *self {
            DescribeJobDefinitionsError::Client(ref cause) => cause,
            DescribeJobDefinitionsError::Server(ref cause) => cause,
            DescribeJobDefinitionsError::Validation(ref cause) => cause,
            DescribeJobDefinitionsError::Credentials(ref err) => err.description(),
            DescribeJobDefinitionsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeJobDefinitionsError::ParseError(ref cause) => cause,
            DescribeJobDefinitionsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DescribeJobQueues
#[derive(Debug, PartialEq)]
pub enum DescribeJobQueuesError {
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an identifier that is not valid. </p>
    Client(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
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

impl DescribeJobQueuesError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> DescribeJobQueuesError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "ClientException" => {
                    return DescribeJobQueuesError::Client(String::from(error_message))
                }
                "ServerException" => {
                    return DescribeJobQueuesError::Server(String::from(error_message))
                }
                "ValidationException" => {
                    return DescribeJobQueuesError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DescribeJobQueuesError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DescribeJobQueuesError {
    fn from(err: serde_json::error::Error) -> DescribeJobQueuesError {
        DescribeJobQueuesError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeJobQueuesError {
    fn from(err: CredentialsError) -> DescribeJobQueuesError {
        DescribeJobQueuesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeJobQueuesError {
    fn from(err: HttpDispatchError) -> DescribeJobQueuesError {
        DescribeJobQueuesError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeJobQueuesError {
    fn from(err: io::Error) -> DescribeJobQueuesError {
        DescribeJobQueuesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeJobQueuesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeJobQueuesError {
    fn description(&self) -> &str {
        match *self {
            DescribeJobQueuesError::Client(ref cause) => cause,
            DescribeJobQueuesError::Server(ref cause) => cause,
            DescribeJobQueuesError::Validation(ref cause) => cause,
            DescribeJobQueuesError::Credentials(ref err) => err.description(),
            DescribeJobQueuesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeJobQueuesError::ParseError(ref cause) => cause,
            DescribeJobQueuesError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DescribeJobs
#[derive(Debug, PartialEq)]
pub enum DescribeJobsError {
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an identifier that is not valid. </p>
    Client(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
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

impl DescribeJobsError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> DescribeJobsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "ClientException" => return DescribeJobsError::Client(String::from(error_message)),
                "ServerException" => return DescribeJobsError::Server(String::from(error_message)),
                "ValidationException" => {
                    return DescribeJobsError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DescribeJobsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DescribeJobsError {
    fn from(err: serde_json::error::Error) -> DescribeJobsError {
        DescribeJobsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeJobsError {
    fn from(err: CredentialsError) -> DescribeJobsError {
        DescribeJobsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeJobsError {
    fn from(err: HttpDispatchError) -> DescribeJobsError {
        DescribeJobsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeJobsError {
    fn from(err: io::Error) -> DescribeJobsError {
        DescribeJobsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeJobsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeJobsError {
    fn description(&self) -> &str {
        match *self {
            DescribeJobsError::Client(ref cause) => cause,
            DescribeJobsError::Server(ref cause) => cause,
            DescribeJobsError::Validation(ref cause) => cause,
            DescribeJobsError::Credentials(ref err) => err.description(),
            DescribeJobsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeJobsError::ParseError(ref cause) => cause,
            DescribeJobsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListJobs
#[derive(Debug, PartialEq)]
pub enum ListJobsError {
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an identifier that is not valid. </p>
    Client(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
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

impl ListJobsError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> ListJobsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "ClientException" => return ListJobsError::Client(String::from(error_message)),
                "ServerException" => return ListJobsError::Server(String::from(error_message)),
                "ValidationException" => {
                    return ListJobsError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return ListJobsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListJobsError {
    fn from(err: serde_json::error::Error) -> ListJobsError {
        ListJobsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListJobsError {
    fn from(err: CredentialsError) -> ListJobsError {
        ListJobsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListJobsError {
    fn from(err: HttpDispatchError) -> ListJobsError {
        ListJobsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListJobsError {
    fn from(err: io::Error) -> ListJobsError {
        ListJobsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListJobsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListJobsError {
    fn description(&self) -> &str {
        match *self {
            ListJobsError::Client(ref cause) => cause,
            ListJobsError::Server(ref cause) => cause,
            ListJobsError::Validation(ref cause) => cause,
            ListJobsError::Credentials(ref err) => err.description(),
            ListJobsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListJobsError::ParseError(ref cause) => cause,
            ListJobsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by RegisterJobDefinition
#[derive(Debug, PartialEq)]
pub enum RegisterJobDefinitionError {
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an identifier that is not valid. </p>
    Client(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
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

impl RegisterJobDefinitionError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> RegisterJobDefinitionError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "ClientException" => {
                    return RegisterJobDefinitionError::Client(String::from(error_message))
                }
                "ServerException" => {
                    return RegisterJobDefinitionError::Server(String::from(error_message))
                }
                "ValidationException" => {
                    return RegisterJobDefinitionError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return RegisterJobDefinitionError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for RegisterJobDefinitionError {
    fn from(err: serde_json::error::Error) -> RegisterJobDefinitionError {
        RegisterJobDefinitionError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for RegisterJobDefinitionError {
    fn from(err: CredentialsError) -> RegisterJobDefinitionError {
        RegisterJobDefinitionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RegisterJobDefinitionError {
    fn from(err: HttpDispatchError) -> RegisterJobDefinitionError {
        RegisterJobDefinitionError::HttpDispatch(err)
    }
}
impl From<io::Error> for RegisterJobDefinitionError {
    fn from(err: io::Error) -> RegisterJobDefinitionError {
        RegisterJobDefinitionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for RegisterJobDefinitionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RegisterJobDefinitionError {
    fn description(&self) -> &str {
        match *self {
            RegisterJobDefinitionError::Client(ref cause) => cause,
            RegisterJobDefinitionError::Server(ref cause) => cause,
            RegisterJobDefinitionError::Validation(ref cause) => cause,
            RegisterJobDefinitionError::Credentials(ref err) => err.description(),
            RegisterJobDefinitionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            RegisterJobDefinitionError::ParseError(ref cause) => cause,
            RegisterJobDefinitionError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by SubmitJob
#[derive(Debug, PartialEq)]
pub enum SubmitJobError {
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an identifier that is not valid. </p>
    Client(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
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

impl SubmitJobError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> SubmitJobError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "ClientException" => return SubmitJobError::Client(String::from(error_message)),
                "ServerException" => return SubmitJobError::Server(String::from(error_message)),
                "ValidationException" => {
                    return SubmitJobError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return SubmitJobError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for SubmitJobError {
    fn from(err: serde_json::error::Error) -> SubmitJobError {
        SubmitJobError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for SubmitJobError {
    fn from(err: CredentialsError) -> SubmitJobError {
        SubmitJobError::Credentials(err)
    }
}
impl From<HttpDispatchError> for SubmitJobError {
    fn from(err: HttpDispatchError) -> SubmitJobError {
        SubmitJobError::HttpDispatch(err)
    }
}
impl From<io::Error> for SubmitJobError {
    fn from(err: io::Error) -> SubmitJobError {
        SubmitJobError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for SubmitJobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SubmitJobError {
    fn description(&self) -> &str {
        match *self {
            SubmitJobError::Client(ref cause) => cause,
            SubmitJobError::Server(ref cause) => cause,
            SubmitJobError::Validation(ref cause) => cause,
            SubmitJobError::Credentials(ref err) => err.description(),
            SubmitJobError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            SubmitJobError::ParseError(ref cause) => cause,
            SubmitJobError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by TerminateJob
#[derive(Debug, PartialEq)]
pub enum TerminateJobError {
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an identifier that is not valid. </p>
    Client(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
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

impl TerminateJobError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> TerminateJobError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "ClientException" => return TerminateJobError::Client(String::from(error_message)),
                "ServerException" => return TerminateJobError::Server(String::from(error_message)),
                "ValidationException" => {
                    return TerminateJobError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return TerminateJobError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for TerminateJobError {
    fn from(err: serde_json::error::Error) -> TerminateJobError {
        TerminateJobError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for TerminateJobError {
    fn from(err: CredentialsError) -> TerminateJobError {
        TerminateJobError::Credentials(err)
    }
}
impl From<HttpDispatchError> for TerminateJobError {
    fn from(err: HttpDispatchError) -> TerminateJobError {
        TerminateJobError::HttpDispatch(err)
    }
}
impl From<io::Error> for TerminateJobError {
    fn from(err: io::Error) -> TerminateJobError {
        TerminateJobError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for TerminateJobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for TerminateJobError {
    fn description(&self) -> &str {
        match *self {
            TerminateJobError::Client(ref cause) => cause,
            TerminateJobError::Server(ref cause) => cause,
            TerminateJobError::Validation(ref cause) => cause,
            TerminateJobError::Credentials(ref err) => err.description(),
            TerminateJobError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            TerminateJobError::ParseError(ref cause) => cause,
            TerminateJobError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by UpdateComputeEnvironment
#[derive(Debug, PartialEq)]
pub enum UpdateComputeEnvironmentError {
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an identifier that is not valid. </p>
    Client(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
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

impl UpdateComputeEnvironmentError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> UpdateComputeEnvironmentError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "ClientException" => {
                    return UpdateComputeEnvironmentError::Client(String::from(error_message))
                }
                "ServerException" => {
                    return UpdateComputeEnvironmentError::Server(String::from(error_message))
                }
                "ValidationException" => {
                    return UpdateComputeEnvironmentError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return UpdateComputeEnvironmentError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for UpdateComputeEnvironmentError {
    fn from(err: serde_json::error::Error) -> UpdateComputeEnvironmentError {
        UpdateComputeEnvironmentError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateComputeEnvironmentError {
    fn from(err: CredentialsError) -> UpdateComputeEnvironmentError {
        UpdateComputeEnvironmentError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateComputeEnvironmentError {
    fn from(err: HttpDispatchError) -> UpdateComputeEnvironmentError {
        UpdateComputeEnvironmentError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateComputeEnvironmentError {
    fn from(err: io::Error) -> UpdateComputeEnvironmentError {
        UpdateComputeEnvironmentError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateComputeEnvironmentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateComputeEnvironmentError {
    fn description(&self) -> &str {
        match *self {
            UpdateComputeEnvironmentError::Client(ref cause) => cause,
            UpdateComputeEnvironmentError::Server(ref cause) => cause,
            UpdateComputeEnvironmentError::Validation(ref cause) => cause,
            UpdateComputeEnvironmentError::Credentials(ref err) => err.description(),
            UpdateComputeEnvironmentError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateComputeEnvironmentError::ParseError(ref cause) => cause,
            UpdateComputeEnvironmentError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by UpdateJobQueue
#[derive(Debug, PartialEq)]
pub enum UpdateJobQueueError {
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an identifier that is not valid. </p>
    Client(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
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

impl UpdateJobQueueError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> UpdateJobQueueError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "ClientException" => {
                    return UpdateJobQueueError::Client(String::from(error_message))
                }
                "ServerException" => {
                    return UpdateJobQueueError::Server(String::from(error_message))
                }
                "ValidationException" => {
                    return UpdateJobQueueError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return UpdateJobQueueError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for UpdateJobQueueError {
    fn from(err: serde_json::error::Error) -> UpdateJobQueueError {
        UpdateJobQueueError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateJobQueueError {
    fn from(err: CredentialsError) -> UpdateJobQueueError {
        UpdateJobQueueError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateJobQueueError {
    fn from(err: HttpDispatchError) -> UpdateJobQueueError {
        UpdateJobQueueError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateJobQueueError {
    fn from(err: io::Error) -> UpdateJobQueueError {
        UpdateJobQueueError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateJobQueueError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateJobQueueError {
    fn description(&self) -> &str {
        match *self {
            UpdateJobQueueError::Client(ref cause) => cause,
            UpdateJobQueueError::Server(ref cause) => cause,
            UpdateJobQueueError::Validation(ref cause) => cause,
            UpdateJobQueueError::Credentials(ref err) => err.description(),
            UpdateJobQueueError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateJobQueueError::ParseError(ref cause) => cause,
            UpdateJobQueueError::Unknown(_) => "unknown error",
        }
    }
}
/// Trait representing the capabilities of the AWS Batch API. AWS Batch clients implement this trait.
pub trait Batch {
    /// <p>Cancels a job in an AWS Batch job queue. Jobs that are in the <code>SUBMITTED</code>, <code>PENDING</code>, or <code>RUNNABLE</code> state are cancelled. Jobs that have progressed to <code>STARTING</code> or <code>RUNNING</code> are not cancelled (but the API operation still succeeds, even if no job is cancelled); these jobs must be terminated with the <a>TerminateJob</a> operation.</p>
    fn cancel_job(
        &self,
        input: CancelJobRequest,
    ) -> RusotoFuture<CancelJobResponse, CancelJobError>;

    /// <p>Creates an AWS Batch compute environment. You can create <code>MANAGED</code> or <code>UNMANAGED</code> compute environments.</p> <p>In a managed compute environment, AWS Batch manages the compute resources within the environment, based on the compute resources that you specify. Instances launched into a managed compute environment use a recent, approved version of the Amazon ECS-optimized AMI. You can choose to use Amazon EC2 On-Demand Instances in your managed compute environment, or you can use Amazon EC2 Spot Instances that only launch when the Spot bid price is below a specified percentage of the On-Demand price.</p> <p>In an unmanaged compute environment, you can manage your own compute resources. This provides more compute resource configuration options, such as using a custom AMI, but you must ensure that your AMI meets the Amazon ECS container instance AMI specification. For more information, see <a href="http://docs.aws.amazon.com/AmazonECS/latest/developerguide/container_instance_AMIs.html">Container Instance AMIs</a> in the <i>Amazon Elastic Container Service Developer Guide</i>. After you have created your unmanaged compute environment, you can use the <a>DescribeComputeEnvironments</a> operation to find the Amazon ECS cluster that is associated with it and then manually launch your container instances into that Amazon ECS cluster. For more information, see <a href="http://docs.aws.amazon.com/AmazonECS/latest/developerguide/launch_container_instance.html">Launching an Amazon ECS Container Instance</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    fn create_compute_environment(
        &self,
        input: CreateComputeEnvironmentRequest,
    ) -> RusotoFuture<CreateComputeEnvironmentResponse, CreateComputeEnvironmentError>;

    /// <p>Creates an AWS Batch job queue. When you create a job queue, you associate one or more compute environments to the queue and assign an order of preference for the compute environments.</p> <p>You also set a priority to the job queue that determines the order in which the AWS Batch scheduler places jobs onto its associated compute environments. For example, if a compute environment is associated with more than one job queue, the job queue with a higher priority is given preference for scheduling jobs to that compute environment.</p>
    fn create_job_queue(
        &self,
        input: CreateJobQueueRequest,
    ) -> RusotoFuture<CreateJobQueueResponse, CreateJobQueueError>;

    /// <p>Deletes an AWS Batch compute environment.</p> <p>Before you can delete a compute environment, you must set its state to <code>DISABLED</code> with the <a>UpdateComputeEnvironment</a> API operation and disassociate it from any job queues with the <a>UpdateJobQueue</a> API operation.</p>
    fn delete_compute_environment(
        &self,
        input: DeleteComputeEnvironmentRequest,
    ) -> RusotoFuture<DeleteComputeEnvironmentResponse, DeleteComputeEnvironmentError>;

    /// <p>Deletes the specified job queue. You must first disable submissions for a queue with the <a>UpdateJobQueue</a> operation. All jobs in the queue are terminated when you delete a job queue.</p> <p>It is not necessary to disassociate compute environments from a queue before submitting a <code>DeleteJobQueue</code> request. </p>
    fn delete_job_queue(
        &self,
        input: DeleteJobQueueRequest,
    ) -> RusotoFuture<DeleteJobQueueResponse, DeleteJobQueueError>;

    /// <p>Deregisters an AWS Batch job definition.</p>
    fn deregister_job_definition(
        &self,
        input: DeregisterJobDefinitionRequest,
    ) -> RusotoFuture<DeregisterJobDefinitionResponse, DeregisterJobDefinitionError>;

    /// <p>Describes one or more of your compute environments.</p> <p>If you are using an unmanaged compute environment, you can use the <code>DescribeComputeEnvironment</code> operation to determine the <code>ecsClusterArn</code> that you should launch your Amazon ECS container instances into.</p>
    fn describe_compute_environments(
        &self,
        input: DescribeComputeEnvironmentsRequest,
    ) -> RusotoFuture<DescribeComputeEnvironmentsResponse, DescribeComputeEnvironmentsError>;

    /// <p>Describes a list of job definitions. You can specify a <code>status</code> (such as <code>ACTIVE</code>) to only return job definitions that match that status.</p>
    fn describe_job_definitions(
        &self,
        input: DescribeJobDefinitionsRequest,
    ) -> RusotoFuture<DescribeJobDefinitionsResponse, DescribeJobDefinitionsError>;

    /// <p>Describes one or more of your job queues.</p>
    fn describe_job_queues(
        &self,
        input: DescribeJobQueuesRequest,
    ) -> RusotoFuture<DescribeJobQueuesResponse, DescribeJobQueuesError>;

    /// <p>Describes a list of AWS Batch jobs.</p>
    fn describe_jobs(
        &self,
        input: DescribeJobsRequest,
    ) -> RusotoFuture<DescribeJobsResponse, DescribeJobsError>;

    /// <p>Returns a list of task jobs for a specified job queue. You can filter the results by job status with the <code>jobStatus</code> parameter. If you do not specify a status, only <code>RUNNING</code> jobs are returned.</p>
    fn list_jobs(&self, input: ListJobsRequest) -> RusotoFuture<ListJobsResponse, ListJobsError>;

    /// <p>Registers an AWS Batch job definition. </p>
    fn register_job_definition(
        &self,
        input: RegisterJobDefinitionRequest,
    ) -> RusotoFuture<RegisterJobDefinitionResponse, RegisterJobDefinitionError>;

    /// <p>Submits an AWS Batch job from a job definition. Parameters specified during <a>SubmitJob</a> override parameters defined in the job definition. </p>
    fn submit_job(
        &self,
        input: SubmitJobRequest,
    ) -> RusotoFuture<SubmitJobResponse, SubmitJobError>;

    /// <p>Terminates a job in a job queue. Jobs that are in the <code>STARTING</code> or <code>RUNNING</code> state are terminated, which causes them to transition to <code>FAILED</code>. Jobs that have not progressed to the <code>STARTING</code> state are cancelled.</p>
    fn terminate_job(
        &self,
        input: TerminateJobRequest,
    ) -> RusotoFuture<TerminateJobResponse, TerminateJobError>;

    /// <p>Updates an AWS Batch compute environment.</p>
    fn update_compute_environment(
        &self,
        input: UpdateComputeEnvironmentRequest,
    ) -> RusotoFuture<UpdateComputeEnvironmentResponse, UpdateComputeEnvironmentError>;

    /// <p>Updates a job queue.</p>
    fn update_job_queue(
        &self,
        input: UpdateJobQueueRequest,
    ) -> RusotoFuture<UpdateJobQueueResponse, UpdateJobQueueError>;
}
/// A client for the AWS Batch API.
pub struct BatchClient {
    client: Client,
    region: region::Region,
}

impl BatchClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> BatchClient {
        BatchClient {
            client: Client::shared(),
            region: region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> BatchClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        P::Future: Send,
        D: DispatchSignedRequest + Send + Sync + 'static,
        D::Future: Send,
    {
        BatchClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl Batch for BatchClient {
    /// <p>Cancels a job in an AWS Batch job queue. Jobs that are in the <code>SUBMITTED</code>, <code>PENDING</code>, or <code>RUNNABLE</code> state are cancelled. Jobs that have progressed to <code>STARTING</code> or <code>RUNNING</code> are not cancelled (but the API operation still succeeds, even if no job is cancelled); these jobs must be terminated with the <a>TerminateJob</a> operation.</p>
    fn cancel_job(
        &self,
        input: CancelJobRequest,
    ) -> RusotoFuture<CancelJobResponse, CancelJobError> {
        let request_uri = "/v1/canceljob";

        let mut request = SignedRequest::new("POST", "batch", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<CancelJobResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CancelJobError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates an AWS Batch compute environment. You can create <code>MANAGED</code> or <code>UNMANAGED</code> compute environments.</p> <p>In a managed compute environment, AWS Batch manages the compute resources within the environment, based on the compute resources that you specify. Instances launched into a managed compute environment use a recent, approved version of the Amazon ECS-optimized AMI. You can choose to use Amazon EC2 On-Demand Instances in your managed compute environment, or you can use Amazon EC2 Spot Instances that only launch when the Spot bid price is below a specified percentage of the On-Demand price.</p> <p>In an unmanaged compute environment, you can manage your own compute resources. This provides more compute resource configuration options, such as using a custom AMI, but you must ensure that your AMI meets the Amazon ECS container instance AMI specification. For more information, see <a href="http://docs.aws.amazon.com/AmazonECS/latest/developerguide/container_instance_AMIs.html">Container Instance AMIs</a> in the <i>Amazon Elastic Container Service Developer Guide</i>. After you have created your unmanaged compute environment, you can use the <a>DescribeComputeEnvironments</a> operation to find the Amazon ECS cluster that is associated with it and then manually launch your container instances into that Amazon ECS cluster. For more information, see <a href="http://docs.aws.amazon.com/AmazonECS/latest/developerguide/launch_container_instance.html">Launching an Amazon ECS Container Instance</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    fn create_compute_environment(
        &self,
        input: CreateComputeEnvironmentRequest,
    ) -> RusotoFuture<CreateComputeEnvironmentResponse, CreateComputeEnvironmentError> {
        let request_uri = "/v1/createcomputeenvironment";

        let mut request = SignedRequest::new("POST", "batch", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<CreateComputeEnvironmentResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateComputeEnvironmentError::from_response(response))
                }))
            }
        })
    }

    /// <p>Creates an AWS Batch job queue. When you create a job queue, you associate one or more compute environments to the queue and assign an order of preference for the compute environments.</p> <p>You also set a priority to the job queue that determines the order in which the AWS Batch scheduler places jobs onto its associated compute environments. For example, if a compute environment is associated with more than one job queue, the job queue with a higher priority is given preference for scheduling jobs to that compute environment.</p>
    fn create_job_queue(
        &self,
        input: CreateJobQueueRequest,
    ) -> RusotoFuture<CreateJobQueueResponse, CreateJobQueueError> {
        let request_uri = "/v1/createjobqueue";

        let mut request = SignedRequest::new("POST", "batch", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<CreateJobQueueResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateJobQueueError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes an AWS Batch compute environment.</p> <p>Before you can delete a compute environment, you must set its state to <code>DISABLED</code> with the <a>UpdateComputeEnvironment</a> API operation and disassociate it from any job queues with the <a>UpdateJobQueue</a> API operation.</p>
    fn delete_compute_environment(
        &self,
        input: DeleteComputeEnvironmentRequest,
    ) -> RusotoFuture<DeleteComputeEnvironmentResponse, DeleteComputeEnvironmentError> {
        let request_uri = "/v1/deletecomputeenvironment";

        let mut request = SignedRequest::new("POST", "batch", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<DeleteComputeEnvironmentResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteComputeEnvironmentError::from_response(response))
                }))
            }
        })
    }

    /// <p>Deletes the specified job queue. You must first disable submissions for a queue with the <a>UpdateJobQueue</a> operation. All jobs in the queue are terminated when you delete a job queue.</p> <p>It is not necessary to disassociate compute environments from a queue before submitting a <code>DeleteJobQueue</code> request. </p>
    fn delete_job_queue(
        &self,
        input: DeleteJobQueueRequest,
    ) -> RusotoFuture<DeleteJobQueueResponse, DeleteJobQueueError> {
        let request_uri = "/v1/deletejobqueue";

        let mut request = SignedRequest::new("POST", "batch", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<DeleteJobQueueResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteJobQueueError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deregisters an AWS Batch job definition.</p>
    fn deregister_job_definition(
        &self,
        input: DeregisterJobDefinitionRequest,
    ) -> RusotoFuture<DeregisterJobDefinitionResponse, DeregisterJobDefinitionError> {
        let request_uri = "/v1/deregisterjobdefinition";

        let mut request = SignedRequest::new("POST", "batch", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<DeregisterJobDefinitionResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeregisterJobDefinitionError::from_response(response))
                }))
            }
        })
    }

    /// <p>Describes one or more of your compute environments.</p> <p>If you are using an unmanaged compute environment, you can use the <code>DescribeComputeEnvironment</code> operation to determine the <code>ecsClusterArn</code> that you should launch your Amazon ECS container instances into.</p>
    fn describe_compute_environments(
        &self,
        input: DescribeComputeEnvironmentsRequest,
    ) -> RusotoFuture<DescribeComputeEnvironmentsResponse, DescribeComputeEnvironmentsError> {
        let request_uri = "/v1/describecomputeenvironments";

        let mut request = SignedRequest::new("POST", "batch", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<DescribeComputeEnvironmentsResponse>(&body)
                            .unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeComputeEnvironmentsError::from_response(response))
                }))
            }
        })
    }

    /// <p>Describes a list of job definitions. You can specify a <code>status</code> (such as <code>ACTIVE</code>) to only return job definitions that match that status.</p>
    fn describe_job_definitions(
        &self,
        input: DescribeJobDefinitionsRequest,
    ) -> RusotoFuture<DescribeJobDefinitionsResponse, DescribeJobDefinitionsError> {
        let request_uri = "/v1/describejobdefinitions";

        let mut request = SignedRequest::new("POST", "batch", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<DescribeJobDefinitionsResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DescribeJobDefinitionsError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Describes one or more of your job queues.</p>
    fn describe_job_queues(
        &self,
        input: DescribeJobQueuesRequest,
    ) -> RusotoFuture<DescribeJobQueuesResponse, DescribeJobQueuesError> {
        let request_uri = "/v1/describejobqueues";

        let mut request = SignedRequest::new("POST", "batch", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<DescribeJobQueuesResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DescribeJobQueuesError::from_response(response))),
                )
            }
        })
    }

    /// <p>Describes a list of AWS Batch jobs.</p>
    fn describe_jobs(
        &self,
        input: DescribeJobsRequest,
    ) -> RusotoFuture<DescribeJobsResponse, DescribeJobsError> {
        let request_uri = "/v1/describejobs";

        let mut request = SignedRequest::new("POST", "batch", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<DescribeJobsResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DescribeJobsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns a list of task jobs for a specified job queue. You can filter the results by job status with the <code>jobStatus</code> parameter. If you do not specify a status, only <code>RUNNING</code> jobs are returned.</p>
    fn list_jobs(&self, input: ListJobsRequest) -> RusotoFuture<ListJobsResponse, ListJobsError> {
        let request_uri = "/v1/listjobs";

        let mut request = SignedRequest::new("POST", "batch", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<ListJobsResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListJobsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Registers an AWS Batch job definition. </p>
    fn register_job_definition(
        &self,
        input: RegisterJobDefinitionRequest,
    ) -> RusotoFuture<RegisterJobDefinitionResponse, RegisterJobDefinitionError> {
        let request_uri = "/v1/registerjobdefinition";

        let mut request = SignedRequest::new("POST", "batch", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<RegisterJobDefinitionResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(RegisterJobDefinitionError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Submits an AWS Batch job from a job definition. Parameters specified during <a>SubmitJob</a> override parameters defined in the job definition. </p>
    fn submit_job(
        &self,
        input: SubmitJobRequest,
    ) -> RusotoFuture<SubmitJobResponse, SubmitJobError> {
        let request_uri = "/v1/submitjob";

        let mut request = SignedRequest::new("POST", "batch", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<SubmitJobResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(SubmitJobError::from_response(response))),
                )
            }
        })
    }

    /// <p>Terminates a job in a job queue. Jobs that are in the <code>STARTING</code> or <code>RUNNING</code> state are terminated, which causes them to transition to <code>FAILED</code>. Jobs that have not progressed to the <code>STARTING</code> state are cancelled.</p>
    fn terminate_job(
        &self,
        input: TerminateJobRequest,
    ) -> RusotoFuture<TerminateJobResponse, TerminateJobError> {
        let request_uri = "/v1/terminatejob";

        let mut request = SignedRequest::new("POST", "batch", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<TerminateJobResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(TerminateJobError::from_response(response))),
                )
            }
        })
    }

    /// <p>Updates an AWS Batch compute environment.</p>
    fn update_compute_environment(
        &self,
        input: UpdateComputeEnvironmentRequest,
    ) -> RusotoFuture<UpdateComputeEnvironmentResponse, UpdateComputeEnvironmentError> {
        let request_uri = "/v1/updatecomputeenvironment";

        let mut request = SignedRequest::new("POST", "batch", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<UpdateComputeEnvironmentResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateComputeEnvironmentError::from_response(response))
                }))
            }
        })
    }

    /// <p>Updates a job queue.</p>
    fn update_job_queue(
        &self,
        input: UpdateJobQueueRequest,
    ) -> RusotoFuture<UpdateJobQueueResponse, UpdateJobQueueError> {
        let request_uri = "/v1/updatejobqueue";

        let mut request = SignedRequest::new("POST", "batch", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<UpdateJobQueueResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateJobQueueError::from_response(response))),
                )
            }
        })
    }
}

#[cfg(test)]
mod protocol_tests {}
