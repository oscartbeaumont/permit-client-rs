# \AccessRequestsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**approve_access_request**](AccessRequestsApi.md#approve_access_request) | **PUT** /v2/elements/{proj_id}/{env_id}/config/{elements_config_id}/access_requests/{access_request_id}/approve | Approve Access Request
[**cancel_access_request**](AccessRequestsApi.md#cancel_access_request) | **PUT** /v2/elements/{proj_id}/{env_id}/config/{elements_config_id}/access_requests/{access_request_id}/cancel | Cancel Access Request
[**create_access_request**](AccessRequestsApi.md#create_access_request) | **POST** /v2/elements/{proj_id}/{env_id}/config/{elements_config_id}/access_requests | Create Access Request
[**deny_access_request**](AccessRequestsApi.md#deny_access_request) | **PUT** /v2/elements/{proj_id}/{env_id}/config/{elements_config_id}/access_requests/{access_request_id}/deny | Deny Access Request
[**get_access_request**](AccessRequestsApi.md#get_access_request) | **GET** /v2/elements/{proj_id}/{env_id}/config/{elements_config_id}/access_requests/{access_request_id} | Get Access Request
[**list_access_requests**](AccessRequestsApi.md#list_access_requests) | **GET** /v2/elements/{proj_id}/{env_id}/config/{elements_config_id}/access_requests | List Access Requests
[**update_access_request_reviewer**](AccessRequestsApi.md#update_access_request_reviewer) | **PATCH** /v2/elements/{proj_id}/{env_id}/config/{elements_config_id}/access_requests/{access_request_id}/reviewer | Update Access Request Reviewer



## approve_access_request

> models::AccessRequestApproved approve_access_request(proj_id, access_request_id, env_id, elements_config_id, access_request_review)
Approve Access Request

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**access_request_id** | **uuid::Uuid** | Either the unique id of the access_request, or the URL-friendly key of the access_request (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**elements_config_id** | **String** | Either the unique id of the elements_config, or the URL-friendly key of the elements_config (i.e: the \"slug\"). | [required] |
**access_request_review** | [**AccessRequestReview**](AccessRequestReview.md) |  | [required] |

### Return type

[**models::AccessRequestApproved**](AccessRequestApproved.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cancel_access_request

> models::AccessRequestCanceled cancel_access_request(proj_id, access_request_id, env_id, elements_config_id)
Cancel Access Request

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**access_request_id** | **String** | Either the unique id of the access_request, or the URL-friendly key of the access_request (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**elements_config_id** | **String** | Either the unique id of the elements_config, or the URL-friendly key of the elements_config (i.e: the \"slug\"). | [required] |

### Return type

[**models::AccessRequestCanceled**](AccessRequestCanceled.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_access_request

> models::AccessRequestRead create_access_request(proj_id, elements_config_id, env_id, access_request_user_create)
Create Access Request

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**elements_config_id** | **String** | Either the unique id of the elements_config, or the URL-friendly key of the elements_config (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**access_request_user_create** | [**AccessRequestUserCreate**](AccessRequestUserCreate.md) |  | [required] |

### Return type

[**models::AccessRequestRead**](AccessRequestRead.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## deny_access_request

> models::AccessRequestDenied deny_access_request(proj_id, access_request_id, env_id, elements_config_id, access_request_review_deny)
Deny Access Request

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**access_request_id** | **String** | Either the unique id of the access_request, or the URL-friendly key of the access_request (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**elements_config_id** | **String** | Either the unique id of the elements_config, or the URL-friendly key of the elements_config (i.e: the \"slug\"). | [required] |
**access_request_review_deny** | [**AccessRequestReviewDeny**](AccessRequestReviewDeny.md) |  | [required] |

### Return type

[**models::AccessRequestDenied**](AccessRequestDenied.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_access_request

> models::AccessRequestRead get_access_request(proj_id, elements_config_id, access_request_id, env_id, page, per_page)
Get Access Request

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**elements_config_id** | **String** | Either the unique id of the elements_config, or the URL-friendly key of the elements_config (i.e: the \"slug\"). | [required] |
**access_request_id** | **String** | Either the unique id of the access_request, or the URL-friendly key of the access_request (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**page** | Option<**i32**> | Page number of the results to fetch, starting at 1. |  |[default to 1]
**per_page** | Option<**i32**> | The number of results per page (max 100). |  |[default to 30]

### Return type

[**models::AccessRequestRead**](AccessRequestRead.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_access_requests

> models::PaginatedResultAccessRequestRead list_access_requests(proj_id, elements_config_id, env_id, status, tenant, role, resource, resource_instance, page, per_page)
List Access Requests

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**elements_config_id** | **String** | Either the unique id of the elements_config, or the URL-friendly key of the elements_config (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**status** | Option<[**models::RequestStatus**](.md)> | Optional status filter, will only return access requests with this status. |  |
**tenant** | Option<**String**> | Optional tenant filter, will only return access request granted in that tenant. |  |
**role** | Option<**String**> | Optional role filter, will only return access request granted with that role. |  |
**resource** | Option<**String**> | Optional resource filter, will only return access request granted in that resource. |  |
**resource_instance** | Option<**String**> | Optional resource_instance filter, will only return access request granted in that resource instance. |  |
**page** | Option<**i32**> | Page number of the results to fetch, starting at 1. |  |[default to 1]
**per_page** | Option<**i32**> | The number of results per page (max 100). |  |[default to 30]

### Return type

[**models::PaginatedResultAccessRequestRead**](PaginatedResult_AccessRequestRead_.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_access_request_reviewer

> models::AccessRequestRead update_access_request_reviewer(proj_id, access_request_id, env_id, elements_config_id, access_request_review)
Update Access Request Reviewer

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**access_request_id** | **String** | Either the unique id of the access_request, or the URL-friendly key of the access_request (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**elements_config_id** | **String** | Either the unique id of the elements_config, or the URL-friendly key of the elements_config (i.e: the \"slug\"). | [required] |
**access_request_review** | [**AccessRequestReview**](AccessRequestReview.md) |  | [required] |

### Return type

[**models::AccessRequestRead**](AccessRequestRead.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

