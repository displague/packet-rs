# Organization

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> |  | [optional]
**name** | Option<**String**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**website** | Option<**String**> |  | [optional]
**twitter** | Option<**String**> |  | [optional]
**logo** | Option<[**std::path::PathBuf**](std::path::PathBuf.md)> |  | [optional]
**created_at** | Option<**String**> |  | [optional]
**updated_at** | Option<**String**> |  | [optional]
**projects** | Option<[**Vec<crate::models::Href>**](Href.md)> |  | [optional]
**members** | Option<[**Vec<crate::models::Href>**](Href.md)> |  | [optional]
**memberships** | Option<[**Vec<crate::models::Href>**](Href.md)> |  | [optional]
**address** | Option<[**crate::models::Address**](Address.md)> |  | [optional]
**billing_address** | Option<[**crate::models::Address**](Address.md)> |  | [optional]
**entitlement** | Option<[**crate::models::Entitlement**](Entitlement.md)> |  | [optional]
**terms** | Option<**i32**> |  | [optional]
**credit_amount** | Option<**f32**> |  | [optional]
**customdata** | Option<**String**> |  | [optional]
**enforce_2fa_at** | Option<**String**> | Force to all members to have enabled the two factor authentication after that date, unless the value is null | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


