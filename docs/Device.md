# Device

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> |  | [optional]
**short_id** | Option<**String**> |  | [optional]
**hostname** | Option<**String**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**state** | Option<**String**> |  | [optional]
**tags** | Option<**Vec<String>**> |  | [optional]
**image_url** | Option<**String**> |  | [optional]
**billing_cycle** | Option<**String**> |  | [optional]
**user** | Option<**String**> |  | [optional]
**iqn** | Option<**String**> |  | [optional]
**locked** | Option<**bool**> |  | [optional]
**bonding_mode** | Option<**i32**> |  | [optional]
**created_at** | Option<**String**> |  | [optional]
**updated_at** | Option<**String**> |  | [optional]
**spot_instance** | Option<**bool**> |  | [optional]
**spot_price_max** | Option<**f32**> |  | [optional]
**termination_time** | Option<**String**> |  | [optional]
**customdata** | Option<**String**> |  | [optional]
**provisioning_percentage** | Option<**f32**> |  | [optional]
**operating_system** | Option<[**crate::models::OperatingSystem**](OperatingSystem.md)> |  | [optional]
**always_pxe** | Option<**bool**> |  | [optional]
**ipxe_script_url** | Option<**String**> |  | [optional]
**location** | Option<[**crate::models::HardwareLocation**](HardwareLocation.md)> |  | [optional]
**facility** | Option<[**crate::models::Facility**](Facility.md)> |  | [optional]
**plan** | Option<[**crate::models::Plan**](Plan.md)> |  | [optional]
**userdata** | Option<**String**> |  | [optional]
**root_password** | Option<**String**> |  | [optional]
**href** | Option<**String**> |  | [optional]
**project** | Option<[**crate::models::Href**](Href.md)> |  | [optional]
**project_lite** | Option<[**crate::models::Href**](Href.md)> |  | [optional]
**volumes** | Option<[**Vec<crate::models::Href>**](Href.md)> |  | [optional]
**hardware_reservation** | Option<[**crate::models::Href**](Href.md)> |  | [optional]
**ssh_keys** | Option<[**Vec<crate::models::Href>**](Href.md)> |  | [optional]
**ip_addresses** | Option<[**Vec<crate::models::IpAssignment>**](IPAssignment.md)> |  | [optional]
**provisioning_events** | Option<[**Vec<crate::models::Event>**](Event.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


