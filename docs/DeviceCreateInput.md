# DeviceCreateInput

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**facility** | **String** |  | 
**plan** | **String** |  | 
**hostname** | Option<**String**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**billing_cycle** | Option<**String**> |  | [optional]
**operating_system** | **String** |  | 
**always_pxe** | Option<**bool**> |  | [optional]
**ipxe_script_url** | Option<**String**> |  | [optional]
**userdata** | Option<**String**> |  | [optional]
**locked** | Option<**bool**> |  | [optional]
**customdata** | Option<**String**> |  | [optional]
**hardware_reservation_id** | Option<**String**> |  | [optional]
**spot_instance** | Option<**bool**> |  | [optional]
**spot_price_max** | Option<**f32**> |  | [optional]
**termination_time** | Option<**String**> |  | [optional]
**tags** | Option<**Vec<String>**> |  | [optional]
**project_ssh_keys** | Option<**Vec<String>**> |  | [optional]
**user_ssh_keys** | Option<**Vec<String>**> |  | [optional]
**features** | Option<**Vec<String>**> |  | [optional]
**public_ipv4_subnet_size** | Option<**f32**> |  | [optional]
**private_ipv4_subnet_size** | Option<**f32**> |  | [optional]
**ip_addresses** | Option<[**Vec<crate::models::InstancesBatchCreateInputIpAddresses>**](InstancesBatchCreateInput_ip_addresses.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


