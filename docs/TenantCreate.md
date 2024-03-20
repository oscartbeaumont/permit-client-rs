# TenantCreate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**key** | **String** | A unique id by which Permit will identify the tenant. The tenant key must be url-friendly (slugified). | 
**name** | **String** | A descriptive name for the tenant | 
**description** | Option<**String**> | an optional longer description of the tenant | [optional]
**attributes** | Option<[**serde_json::Value**](.md)> | Arbitraty tenant attributes that will be used to enforce attribute-based access control policies. | [optional][default to {}]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


