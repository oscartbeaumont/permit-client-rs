# ResourceInstanceRead

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**key** | **String** | A unique identifier by which Permit will identify the resource instance for permission checks. You will later pass this identifier to the `permit.check()` API. A key can be anything: for example the resource db id, a url slug, a UUID or anything else as long as it's unique on your end. The resource instance key must be url-friendly. | 
**tenant** | **String** | the *key* of the tenant that this resource belongs to, used to enforce tenant boundaries in multi-tenant apps. | 
**resource** | **String** | the *key* of the resource (type) of this resource instance. For example: if this resource instance is the annual budget document, the key of the resource might be `document`. | 
**id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique id of the resource instance | 
**organization_id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique id of the organization that the resource instance belongs to. | 
**project_id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique id of the project that the resource instance belongs to. | 
**environment_id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique id of the environment that the resource instance belongs to. | 
**created_at** | **String** | Date and time when the resource instance was created (ISO_8601 format). | 
**updated_at** | **String** | Date and time when the resource instance was last updated/modified (ISO_8601 format). | 
**resource_id** | [**uuid::Uuid**](uuid::Uuid.md) | the id of the resource (type) of this resource instance. | 
**tenant_id** | [**uuid::Uuid**](uuid::Uuid.md) | the id of the tenant of this resource instance. | 
**attributes** | Option<[**serde_json::Value**](.md)> | Arbitrary resource attributes that will be used to enforce attribute-based access control policies. | [optional][default to {}]
**relationships** | Option<[**Vec<models::RelationshipTupleBlockRead>**](RelationshipTupleBlockRead.md)> | The relationships of the resource instance. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


