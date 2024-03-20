# ResourceInstanceCreate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**key** | **String** | A unique identifier by which Permit will identify the resource instance for permission checks. You will later pass this identifier to the `permit.check()` API. A key can be anything: for example the resource db id, a url slug, a UUID or anything else as long as it's unique on your end. The resource instance key must be url-friendly. | 
**tenant** | **String** | the *key* of the tenant that this resource belongs to, used to enforce tenant boundaries in multi-tenant apps. | 
**resource** | **String** | the *key* of the resource (type) of this resource instance. For example: if this resource instance is the annual budget document, the key of the resource might be `document`. | 
**attributes** | Option<[**serde_json::Value**](.md)> | Arbitrary resource attributes that will be used to enforce attribute-based access control policies. | [optional][default to {}]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


