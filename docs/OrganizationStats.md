# OrganizationStats

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**key** | **String** | A URL-friendly name of the organization (i.e: slug). You will be able to query later using this key instead of the id (UUID) of the organization. | 
**id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique id of the organization | 
**is_enterprise** | **bool** | Is this an enterprise account? | 
**usage_limits** | Option<[**models::UsageLimits**](UsageLimits.md)> | Usage limits for this organization | [optional][default to {mau=1000, tenants=20, billing_tier=free}]
**created_at** | **String** | Date and time when the organization was created (ISO_8601 format). | 
**updated_at** | **String** | Date and time when the organization was last updated/modified (ISO_8601 format). | 
**name** | **String** | The name of the organization, usually it's your company's name. | 
**settings** | Option<[**serde_json::Value**](.md)> | the settings for this project | [optional]
**stats** | [**models::OrganizationStatistics**](OrganizationStatistics.md) |  | 
**historical_usage** | [**models::HistoricalUsage**](HistoricalUsage.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


