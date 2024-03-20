# \AuditLogsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_detailed_audit_log**](AuditLogsApi.md#get_detailed_audit_log) | **GET** /v2/pdps/{proj_id}/{env_id}/audit_logs/{log_id} | Get detailed audit log
[**list_audit_logs**](AuditLogsApi.md#list_audit_logs) | **GET** /v2/pdps/{proj_id}/{env_id}/audit_logs | List Audit Logs



## get_detailed_audit_log

> models::DetailedAuditLogModel get_detailed_audit_log(proj_id, env_id, log_id)
Get detailed audit log

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**log_id** | **uuid::Uuid** | The unique id of the audit log | [required] |

### Return type

[**models::DetailedAuditLogModel**](DetailedAuditLogModel.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_audit_logs

> models::LimitedPaginatedResultAuditLogModel list_audit_logs(proj_id, env_id, pdp_id, users, decision, resources, tenant, action, timestamp_from, timestamp_to, sort_by, page, per_page)
List Audit Logs

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**pdp_id** | Option<**uuid::Uuid**> | Filter by pdp config id |  |
**users** | Option<[**Vec<String>**](String.md)> | List of user keys or emails to filter by |  |
**decision** | Option<**bool**> | Filter by decision result |  |
**resources** | Option<[**Vec<String>**](String.md)> | Filter by resources |  |
**tenant** | Option<**String**> | Filter by tenant |  |
**action** | Option<**String**> | Filter by action |  |
**timestamp_from** | Option<**i32**> | Filter by timestamp from |  |
**timestamp_to** | Option<**i32**> | Filter by timestamp to |  |
**sort_by** | Option<[**models::AuditLogSortKey**](.md)> | Sort by column |  |[default to timestamp]
**page** | Option<**i32**> | Page number of the results to fetch, starting at 1. |  |[default to 1]
**per_page** | Option<**i32**> | The number of results per page (max 100). |  |[default to 30]

### Return type

[**models::LimitedPaginatedResultAuditLogModel**](LimitedPaginatedResult_AuditLogModel_.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

