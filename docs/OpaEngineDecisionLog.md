# OpaEngineDecisionLog

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**engine** | Option<**String**> |  | [optional][default to Opa]
**decision_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**labels** | [**models::OpaLabels**](OPALabels.md) |  | 
**timestamp** | **String** |  | 
**path** | **String** |  | 
**input** | Option<[**serde_json::Value**](.md)> |  | [optional]
**result** | Option<[**serde_json::Value**](.md)> |  | [optional]
**metrics** | [**models::OpaMetrics**](OPAMetrics.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


