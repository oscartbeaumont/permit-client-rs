# \ConditionSetRulesApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**assign_set_permissions**](ConditionSetRulesApi.md#assign_set_permissions) | **POST** /v2/facts/{proj_id}/{env_id}/set_rules | Assign Set Permissions
[**list_set_permissions**](ConditionSetRulesApi.md#list_set_permissions) | **GET** /v2/facts/{proj_id}/{env_id}/set_rules | List Set Permissions
[**unassign_set_permissions**](ConditionSetRulesApi.md#unassign_set_permissions) | **DELETE** /v2/facts/{proj_id}/{env_id}/set_rules | Unassign Set Permissions



## assign_set_permissions

> Vec<models::ConditionSetRuleRead> assign_set_permissions(proj_id, env_id, condition_set_rule_create)
Assign Set Permissions

Grant permissions to a user set *on* a resource set.  If the permission is already granted, it is skipped.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**condition_set_rule_create** | [**ConditionSetRuleCreate**](ConditionSetRuleCreate.md) |  | [required] |

### Return type

[**Vec<models::ConditionSetRuleRead>**](ConditionSetRuleRead.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_set_permissions

> Vec<models::ConditionSetRuleRead> list_set_permissions(proj_id, env_id, user_set, permission, resource_set, page, per_page)
List Set Permissions

Lists the condition set rules matching the filter. - If the `user_set` filter is present, will only return the permissions set of that user set. - If the `permission` filter is present, will only return the permissions sets that equals to the queried permission. - If the `resource_set` filter is present, will only return the permissions set of that resource set.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**user_set** | Option<**String**> | optional user set filter, will only return rules where the permission is granted to this user set |  |
**permission** | Option<**String**> | optional permission filter, will only return condition set rules granting this permission |  |
**resource_set** | Option<**String**> | optional resource set filter, will only return rules where the permission is granted on this resource set |  |
**page** | Option<**i32**> | Page number of the results to fetch, starting at 1. |  |[default to 1]
**per_page** | Option<**i32**> | The number of results per page (max 100). |  |[default to 30]

### Return type

[**Vec<models::ConditionSetRuleRead>**](ConditionSetRuleRead.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unassign_set_permissions

> unassign_set_permissions(proj_id, env_id, condition_set_rule_remove)
Unassign Set Permissions

Revokes permissions to a user set *on* a resource set.  If the permission is not granted, it is skipped.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**condition_set_rule_remove** | [**ConditionSetRuleRemove**](ConditionSetRuleRemove.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

