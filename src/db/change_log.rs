// Uses
use std::{borrow::Cow, collections::HashMap, hash::Hash};

use chrono::Utc;
use diesel::{insert_into, RunQueryDsl, SqliteConnection};
use serde_json::to_string as to_json_string;
use serde_with::skip_serializing_none;

use super::{
	models::{
		DeviceAttachmentExisting,
		DeviceAttachmentMetadata,
		DeviceAttachmentNew,
		DeviceAttachmentUpsert,
		DeviceChangeNew,
		DeviceComponent,
		DeviceComponentNew,
		DeviceComponentUpsert,
		DeviceData,
		DeviceDataNew,
		DeviceInfo,
		DeviceKeyInfoNew,
	},
	schema,
};
use crate::error::{Context, Error};

pub trait Diff<B, A>
where
	Self: Sized,
{
	/// Calculates the diff from a `before` and `after` copy of the data.
	///
	/// Returns `None` if there is no difference.
	fn calculate_diff(before: &B, after: &A) -> Option<Self>;

	/// Simply determines if the diff is empty (non-useful).
	fn is_empty(&self) -> bool;
}

fn none_if_empty<D: Diff<B, A>, B, A>(diff: D) -> Option<D> {
	if diff.is_empty() {
		None
	} else {
		Some(diff)
	}
}

// TODO: Revisit the diff calculation for deleted devices
fn calculate_vec_diff<D, I, B, BI, A, AI, NS, SN>(
	before: &Vec<B>,
	get_before_identifier: BI,
	after: &Vec<A>,
	get_after_identifier: AI,
	new_item_handler: NS,
	deleted_item_handler: SN,
) -> Option<Vec<D>>
where
	D: Diff<B, A>,
	I: Eq + Hash,
	BI: Fn(&B) -> I,
	AI: Fn(&A) -> I,
	NS: Fn(&A) -> Option<D>,
	SN: Fn(&B) -> Option<D>,
{
	// Build a map of changes to items
	let mut item_change_map = HashMap::with_capacity(before.len());
	for item_before in before {
		if item_change_map
			.insert(
				get_before_identifier(item_before),
				(Some(item_before), None),
			)
			.is_some()
		{
			panic!("duplicate items exist in the before list");
		}
	}
	for item_after in after {
		item_change_map
			.entry(get_after_identifier(item_after))
			.and_modify(|e| {
				assert!(e.1.is_none(), "duplicate items exist in the after list");
				*e = (e.0, Some(item_after));
			})
			.or_insert((None, Some(item_after)));
	}

	// Build the final list of changes
	let mut modified_items = Vec::with_capacity(item_change_map.len());
	for item_change in item_change_map {
		match item_change.1 {
			(Some(before), Some(after)) => {
				if let Some(diff) = D::calculate_diff(before, after) {
					modified_items.push(diff);
				}
			}
			(None, Some(after)) => {
				if let Some(diff) = new_item_handler(after) {
					modified_items.push(diff);
				}
			}
			(Some(before), None) => {
				if let Some(diff) = deleted_item_handler(before) {
					modified_items.push(diff);
				}
			}
			(None, None) => {
				unreachable!("not possible because something has to exist to create an entry");
			}
		}
	}

	if modified_items.is_empty() {
		None
	} else {
		Some(modified_items)
	}
}

pub fn log_change(
	conn: &mut SqliteConnection,
	device_id: i32,
	user_id_value: i32,
	diff: &DeviceDiff<'_>,
) -> Result<(), Error> {
	// Uses
	use schema::device_changes::dsl::*;

	let serialised_diff =
		to_json_string(diff).with_context("unable to serialise the diff to JSON")?;

	// Log the change in the database
	insert_into(device_changes)
		.values(DeviceChangeNew {
			device_key_info_id: device_id,
			timestamp:          Utc::now().naive_utc(),
			done_automatically: false,
			user_id:            user_id_value,
			change:             Cow::from(serialised_diff),
		})
		.execute(conn)
		.with_context("unable to insert into device_changes")?;

	Ok(())
}

/// Represents a set of changes to a given device. (one click of the `Update`
/// button)
#[skip_serializing_none]
#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceDiff<'a> {
	#[serde(default)]
	pub device_key_info:    Option<DeviceKeyInfoDiff>,
	#[serde(default)]
	pub device_data:        Option<DeviceDataDiff<'a>>,
	#[serde(default)]
	pub device_components:  Option<DeviceComponentsDiff<'a>>,
	#[serde(default)]
	pub device_attachments: Option<DeviceAttachmentsDiff<'a>>,
}

impl<'a>
	Diff<
		(
			DeviceInfo<'a>,
			Vec<DeviceData<'a>>,
			Vec<DeviceComponent<'a>>,
			Vec<DeviceAttachmentMetadata<'a>>,
		),
		(
			DeviceKeyInfoNew<'a>,
			Vec<DeviceDataNew<'a>>,
			Vec<DeviceComponentUpsert<'a>>,
			Vec<DeviceAttachmentUpsert<'a>>,
		),
	> for DeviceDiff<'a>
{
	fn calculate_diff(
		before: &(
			DeviceInfo<'a>,
			Vec<DeviceData<'a>>,
			Vec<DeviceComponent<'a>>,
			Vec<DeviceAttachmentMetadata<'a>>,
		),
		after: &(
			DeviceKeyInfoNew<'a>,
			Vec<DeviceDataNew<'a>>,
			Vec<DeviceComponentUpsert<'a>>,
			Vec<DeviceAttachmentUpsert<'a>>,
		),
	) -> Option<Self> {
		none_if_empty(Self {
			device_key_info:    DeviceKeyInfoDiff::calculate_diff(&before.0, &after.0),
			device_data:        DeviceDataDiff::calculate_diff(&before.1, &after.1),
			device_components:  DeviceComponentsDiff::calculate_diff(&before.2, &after.2),
			device_attachments: DeviceAttachmentsDiff::calculate_diff(&before.3, &after.3),
		})
	}

	fn is_empty(&self) -> bool {
		self.device_key_info.is_none()
			&& self.device_data.is_none()
			&& self.device_components.is_none()
			&& self.device_attachments.is_none()
	}
}

impl<'a>
	From<&(
		DeviceKeyInfoNew<'a>,
		Vec<DeviceDataNew<'a>>,
		Vec<DeviceComponentUpsert<'a>>,
		Vec<DeviceAttachmentUpsert<'a>>,
	)> for DeviceDiff<'a>
{
	fn from(
		after: &(
			DeviceKeyInfoNew<'a>,
			Vec<DeviceDataNew<'a>>,
			Vec<DeviceComponentUpsert<'a>>,
			Vec<DeviceAttachmentUpsert<'a>>,
		),
	) -> Self {
		Self {
			device_key_info:    none_if_empty(DeviceKeyInfoDiff::from(&after.0)),
			device_data:        none_if_empty(DeviceDataDiff::from(&after.1)),
			device_components:  none_if_empty(DeviceComponentsDiff::from(&after.2)),
			device_attachments: none_if_empty(DeviceAttachmentsDiff::from(&after.3)),
		}
	}
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "operation", rename_all = "camelCase")]
pub enum DeviceKeyInfoDiff {
	Add(DeviceKeyInfoDiffData),
	Edit(DeviceKeyInfoDiffData),
	Delete,
	Restore,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceKeyInfoDiffData {
	#[serde(default)]
	pub location_id: Option<i32>,
}

impl Diff<DeviceInfo<'_>, DeviceKeyInfoNew<'_>> for DeviceKeyInfoDiff {
	fn calculate_diff(before: &DeviceInfo<'_>, after: &DeviceKeyInfoNew<'_>) -> Option<Self> {
		none_if_empty(Self::Edit(DeviceKeyInfoDiffData {
			location_id: (before.location_id != after.location_id).then_some(after.location_id),
		}))
	}

	fn is_empty(&self) -> bool {
		match self {
			Self::Add(diff) | Self::Edit(diff) => diff.location_id.is_none(),
			Self::Delete | Self::Restore => false,
		}
	}
}

impl From<&DeviceKeyInfoNew<'_>> for DeviceKeyInfoDiff {
	fn from(after: &DeviceKeyInfoNew<'_>) -> Self {
		Self::Add(DeviceKeyInfoDiffData {
			location_id: Some(after.location_id),
		})
	}
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceDataDiff<'a>(Vec<DeviceDataColumnDiff<'a>>);

impl<'a> Diff<Vec<DeviceData<'a>>, Vec<DeviceDataNew<'a>>> for DeviceDataDiff<'a> {
	fn calculate_diff(
		before: &Vec<DeviceData<'a>>,
		after: &Vec<DeviceDataNew<'a>>,
	) -> Option<Self> {
		calculate_vec_diff(
			before,
			|before| before.column_definition_id,
			after,
			|after| after.column_definition_id,
			|after| Some(DeviceDataColumnDiff::from(after)),
			|_| None,
		)
		.map(Self)
	}

	fn is_empty(&self) -> bool {
		self.0.iter().all(Diff::is_empty)
	}
}

impl<'a> From<&Vec<DeviceDataNew<'a>>> for DeviceDataDiff<'a> {
	fn from(after: &Vec<DeviceDataNew<'a>>) -> Self {
		Self(after.iter().map(DeviceDataColumnDiff::from).collect())
	}
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceDataColumnDiff<'a> {
	pub column_definition_id: i32,
	#[serde(default)]
	pub data_value:           Option<Cow<'a, str>>,
}

impl<'a> Diff<DeviceData<'a>, DeviceDataNew<'a>> for DeviceDataColumnDiff<'a> {
	fn calculate_diff(before: &DeviceData<'a>, after: &DeviceDataNew<'a>) -> Option<Self> {
		assert_eq!(
			before.column_definition_id, after.column_definition_id,
			"column_definition_id values must match"
		);

		none_if_empty(Self {
			column_definition_id: after.column_definition_id,
			data_value:           (before.data_value != after.data_value)
				.then_some(after.data_value.clone()),
		})
	}

	fn is_empty(&self) -> bool {
		self.data_value.is_none()
	}
}

impl<'a> From<&DeviceDataNew<'a>> for DeviceDataColumnDiff<'a> {
	fn from(after: &DeviceDataNew<'a>) -> Self {
		Self {
			column_definition_id: after.column_definition_id,
			data_value:           Some(after.data_value.clone()),
		}
	}
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceComponentsDiff<'a>(Vec<DeviceComponentsComponentDiff<'a>>);

impl<'a> Diff<Vec<DeviceComponent<'a>>, Vec<DeviceComponentUpsert<'a>>>
	for DeviceComponentsDiff<'a>
{
	fn calculate_diff(
		before: &Vec<DeviceComponent<'a>>,
		after: &Vec<DeviceComponentUpsert<'a>>,
	) -> Option<Self> {
		calculate_vec_diff(
			before,
			|before| before.component_id.clone(),
			after,
			|after| match after {
				DeviceComponentUpsert::NewExisting(DeviceComponentNew { component_id, .. })
				| DeviceComponentUpsert::Delete(component_id) => component_id.clone(),
			},
			|after| Some(DeviceComponentsComponentDiff::from(after)),
			|before| Some(DeviceComponentsComponentDiff::from(before)),
		)
		.map(Self)
	}

	fn is_empty(&self) -> bool {
		self.0.iter().all(Diff::is_empty)
	}
}

impl<'a> From<&Vec<DeviceComponentUpsert<'a>>> for DeviceComponentsDiff<'a> {
	fn from(after: &Vec<DeviceComponentUpsert<'a>>) -> Self {
		Self(
			after
				.iter()
				.map(DeviceComponentsComponentDiff::from)
				.collect(),
		)
	}
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "operation", rename_all = "camelCase")]
pub enum DeviceComponentsComponentDiff<'a> {
	Add(DeviceComponentsComponentDiffData<'a>),
	Edit(DeviceComponentsComponentDiffData<'a>),
	#[serde(rename_all = "camelCase")]
	Delete {
		component_id: Cow<'a, str>,
	},
	#[serde(rename_all = "camelCase")]
	Restore {
		component_id: Cow<'a, str>,
	},
}

impl<'a> Diff<DeviceComponent<'a>, DeviceComponentUpsert<'a>>
	for DeviceComponentsComponentDiff<'a>
{
	fn calculate_diff(
		before: &DeviceComponent<'a>,
		after: &DeviceComponentUpsert<'a>,
	) -> Option<Self> {
		match after {
			DeviceComponentUpsert::NewExisting(new_component) => {
				assert_eq!(
					before.component_id, new_component.component_id,
					"component_id values must match"
				);

				none_if_empty(Self::Edit(DeviceComponentsComponentDiffData {
					component_id:   new_component.component_id.clone(),
					component_type: (before.component_type != new_component.component_type)
						.then_some(new_component.component_type.clone()),
				}))
			}
			DeviceComponentUpsert::Delete(component_id) => Some(Self::Delete {
				component_id: component_id.clone(),
			}),
		}
	}

	fn is_empty(&self) -> bool {
		match self {
			Self::Add(diff) | Self::Edit(diff) => diff.component_type.is_none(),
			Self::Delete { .. } | Self::Restore { .. } => false,
		}
	}
}

impl<'a> From<&DeviceComponent<'a>> for DeviceComponentsComponentDiff<'a> {
	fn from(before: &DeviceComponent<'a>) -> Self {
		Self::Delete {
			component_id: before.component_id.clone(),
		}
	}
}

impl<'a> From<&DeviceComponentUpsert<'a>> for DeviceComponentsComponentDiff<'a> {
	fn from(after: &DeviceComponentUpsert<'a>) -> Self {
		Self::Add(match after {
			DeviceComponentUpsert::NewExisting(DeviceComponentNew {
				component_id,
				component_type,
				..
			}) => DeviceComponentsComponentDiffData {
				component_id:   component_id.clone(),
				component_type: Some(component_type.clone()),
			},
			DeviceComponentUpsert::Delete(_) => {
				unreachable!("the component should already exist if it's being deleted")
			}
		})
	}
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceComponentsComponentDiffData<'a> {
	pub component_id:   Cow<'a, str>,
	#[serde(default)]
	pub component_type: Option<Cow<'a, str>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceAttachmentsDiff<'a>(Vec<DeviceAttachmentsAttachmentDiff<'a>>);

impl<'a> Diff<Vec<DeviceAttachmentMetadata<'a>>, Vec<DeviceAttachmentUpsert<'a>>>
	for DeviceAttachmentsDiff<'a>
{
	fn calculate_diff(
		before: &Vec<DeviceAttachmentMetadata<'a>>,
		after: &Vec<DeviceAttachmentUpsert<'a>>,
	) -> Option<Self> {
		calculate_vec_diff(
			before,
			|before| before.attachment_id.clone(),
			after,
			|after| match after {
				DeviceAttachmentUpsert::New(DeviceAttachmentNew { attachment_id, .. })
				| DeviceAttachmentUpsert::Existing(DeviceAttachmentExisting {
					attachment_id,
					..
				})
				| DeviceAttachmentUpsert::Delete(attachment_id) => attachment_id.clone(),
			},
			|after| Some(DeviceAttachmentsAttachmentDiff::from(after)),
			|before| Some(DeviceAttachmentsAttachmentDiff::from(before)),
		)
		.map(Self)
	}

	fn is_empty(&self) -> bool {
		self.0.iter().all(Diff::is_empty)
	}
}

impl<'a> From<&Vec<DeviceAttachmentUpsert<'a>>> for DeviceAttachmentsDiff<'a> {
	fn from(after: &Vec<DeviceAttachmentUpsert<'a>>) -> Self {
		Self(
			after
				.iter()
				.map(DeviceAttachmentsAttachmentDiff::from)
				.collect(),
		)
	}
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "operation", rename_all = "camelCase")]
pub enum DeviceAttachmentsAttachmentDiff<'a> {
	Add(DeviceAttachmentsAttachmentDiffData<'a>),
	Edit(DeviceAttachmentsAttachmentDiffData<'a>),
	#[serde(rename_all = "camelCase")]
	Delete {
		attachment_id: Cow<'a, str>,
	},
	#[serde(rename_all = "camelCase")]
	Restore {
		attachment_id: Cow<'a, str>,
	},
}

impl<'a> Diff<DeviceAttachmentMetadata<'a>, DeviceAttachmentUpsert<'a>>
	for DeviceAttachmentsAttachmentDiff<'a>
{
	fn calculate_diff(
		before: &DeviceAttachmentMetadata<'a>,
		after: &DeviceAttachmentUpsert<'a>,
	) -> Option<Self> {
		let (after_attachment_id, after_description, after_file_name) = match after {
			DeviceAttachmentUpsert::New(DeviceAttachmentNew {
				attachment_id,
				description,
				file_name,
				..
			}) => (attachment_id, description, Some(file_name)),
			DeviceAttachmentUpsert::Existing(DeviceAttachmentExisting {
				attachment_id,
				description,
				..
			}) => (attachment_id, description, None),
			DeviceAttachmentUpsert::Delete(attachment_id) => {
				return Some(Self::Delete {
					attachment_id: attachment_id.clone(),
				})
			}
		};

		assert_eq!(
			before.attachment_id.as_ref(),
			after_attachment_id,
			"attachment_id values must match"
		);

		none_if_empty(Self::Edit(DeviceAttachmentsAttachmentDiffData {
			attachment_id: after_attachment_id.clone(),
			description:   (before.description.as_ref() != after_description)
				.then_some(after_description.clone()),
			file_name:     after_file_name.and_then(|after_file_name_value| {
				(before.file_name.as_ref() != after_file_name_value)
					.then_some(after_file_name_value.clone())
			}),
		}))
	}

	fn is_empty(&self) -> bool {
		match self {
			Self::Add(diff) | Self::Edit(diff) => {
				diff.description.is_none() && diff.file_name.is_none()
			}
			Self::Delete { .. } | Self::Restore { .. } => false,
		}
	}
}

impl<'a> From<&DeviceAttachmentMetadata<'a>> for DeviceAttachmentsAttachmentDiff<'a> {
	fn from(before: &DeviceAttachmentMetadata<'a>) -> Self {
		Self::Delete {
			attachment_id: before.attachment_id.clone(),
		}
	}
}

impl<'a> From<&DeviceAttachmentUpsert<'a>> for DeviceAttachmentsAttachmentDiff<'a> {
	fn from(after: &DeviceAttachmentUpsert<'a>) -> Self {
		Self::Add(match after {
			DeviceAttachmentUpsert::New(DeviceAttachmentNew {
				attachment_id,
				description,
				file_name,
				..
			}) => DeviceAttachmentsAttachmentDiffData {
				attachment_id: attachment_id.clone(),
				description:   Some(description.clone()),
				file_name:     Some(file_name.clone()),
			},
			DeviceAttachmentUpsert::Existing(_) | DeviceAttachmentUpsert::Delete(_) => {
				unreachable!("the attachment should already exist if it's being updated")
			}
		})
	}
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceAttachmentsAttachmentDiffData<'a> {
	pub attachment_id: Cow<'a, str>,
	#[serde(default)]
	pub description:   Option<Cow<'a, str>>,
	#[serde(default)]
	pub file_name:     Option<Cow<'a, str>>,
}
