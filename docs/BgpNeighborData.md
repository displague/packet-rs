# BgpNeighborData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**address_family** | Option<**f32**> | Address Family for IP Address | [optional]
**customer_as** | Option<**f32**> | The customer's ASN. In a local BGP deployment, this will be an internal ASN used to route within the data center. For a global BGP deployment, this will be the your own ASN, configured when you set up BGP for your project. | [optional]
**customer_ip** | Option<**String**> | The device's IP address. For an IPv4 BGP session, this is typically the private bond0 address for the device. | [optional]
**md5_enabled** | Option<**bool**> | True if an MD5 password is configured for the project. | [optional]
**md5_password** | Option<**String**> | The MD5 password configured for the project, if set. | [optional]
**multihop** | Option<**bool**> | True when the BGP session should be configured as multihop. | [optional]
**peer_as** | Option<**f32**> | The Peer ASN to use when configuring BGP on your device. | [optional]
**peer_ips** | Option<**Vec<String>**> | A list of one or more IP addresses to use for the Peer IP section of your BGP configuration. For non-multihop sessions, this will typically be a single gateway address for the device. For multihop sessions, it will be a list of IPs. | [optional]
**routes_in** | Option<[**Vec<crate::models::BgpNeighborDataRoutesIn>**](BgpNeighborData_routes_in.md)> | A list of project subnets | [optional]
**routes_out** | Option<[**Vec<crate::models::BgpNeighborDataRoutesOut>**](BgpNeighborData_routes_out.md)> | A list of outgoing routes. Only populated if the BGP session has default route enabled. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


