# AuditLogModel

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**timestamp** | **String** |  | 
**query** | Option<**String**> |  | [optional]
**user_key** | Option<**String**> |  | [optional]
**user_email** | Option<**String**> |  | [optional]
**user_name** | Option<**String**> |  | [optional]
**resource_type** | Option<**String**> |  | [optional]
**tenant** | Option<**String**> |  | [optional]
**action** | Option<**String**> |  | [optional]
**decision** | Option<**bool**> |  | [optional]
**reason** | Option<**String**> |  | [optional]
**org_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**project_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**env_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**pdp_config_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**input** | Option<[**serde_json::Value**](.md)> |  | [optional]
**result** | Option<[**serde_json::Value**](.md)> |  | [optional]
**context** | Option<[**serde_json::Value**](.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


