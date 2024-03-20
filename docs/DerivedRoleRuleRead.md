# DerivedRoleRuleRead

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**role_id** | Option<[**serde_json::Value**](.md)> | the role id that needs to exist on the related resource (from the relation) | 
**resource_id** | Option<[**serde_json::Value**](.md)> | the resource id that needs to exist on the related role (from the relation) | 
**relation_id** | Option<[**serde_json::Value**](.md)> | the relation id that needs to exist between the resource and the related resource | 
**role** | Option<[**serde_json::Value**](.md)> | the role key that needs to exist on the related resource (from the relation) | 
**on_resource** | Option<[**serde_json::Value**](.md)> | the resource key that needs to exist on the related role (from the relation) | 
**linked_by_relation** | Option<[**serde_json::Value**](.md)> | the relation key that needs to exist between the resource and the related resource | 
**when** | Option<[**models::DerivationSettings**](DerivationSettings.md)> | the settings of the derived role rule | [optional][default to {no_direct_roles_on_object=false}]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


