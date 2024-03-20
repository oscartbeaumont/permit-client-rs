# PdpConfigRead

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**name** | Option<**String**> |  | [optional]
**organization_id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique id of the organization that the pdp_config belongs to. | 
**project_id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique id of the project that the pdp_config belongs to. | 
**environment_id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique id of the environment that the pdp_config belongs to. | 
**client_secret** | **String** |  | 
**num_shards** | Option<**i32**> |  | [optional]
**debug_audit_logs** | Option<**bool**> | Whether debug audit logs are enabled or not | [optional][default to true]
**min_pdp_version** | Option<**String**> | The minimum image version of PDP that can connect to this config | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


