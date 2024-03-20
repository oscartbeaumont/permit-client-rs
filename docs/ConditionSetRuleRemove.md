# ConditionSetRuleRemove

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**user_set** | **String** | The userset that will be unassigned these permission, i.e: all the users matching this rule will lose the specified permission | 
**permission** | **String** | The permission that will be removed from the userset *on* the resourceset. The permission can be either a resource action id, or `{resource_key}:{action_key}`, i.e: the \"permission name\". | 
**resource_set** | **String** | The resourceset that represents the resources that are no longer granted for access, i.e: all the resources matching this rule can no longer be accessed by the userset, and will be revoked the specified *permission* | 
**is_role** | Option<**bool**> | if True, will set the condition set rule to the role's autogen user-set. | [optional][default to false]
**is_resource** | Option<**bool**> | if True, will set the condition set rule to the resource's autogen resource-set. | [optional][default to false]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


