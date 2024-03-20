# ConditionSetRuleRead

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique id of the condition set rule | 
**key** | **String** | A unique id by which Permit will identify this condition set rule. | 
**user_set** | **String** | the userset that is currently granted permissions, i.e: all the users matching this rule are granted the permission on the resourceset | 
**permission** | **String** | a permission that is currently granted to the userset *on* the resourceset. | 
**resource_set** | **String** | the resourceset that represents the resources that are currently granted for access, i.e: all the resources matching this rule can be accessed by the userset to perform the granted *permission* | 
**organization_id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique id of the organization that the condition set rule belongs to. | 
**project_id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique id of the project that the condition set rule belongs to. | 
**environment_id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique id of the environment that the condition set rule belongs to. | 
**created_at** | **String** | Date and time when the condition set rule was created (ISO_8601 format). | 
**updated_at** | **String** | Date and time when the condition set rule was last updated/modified (ISO_8601 format). | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


